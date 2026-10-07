import 'dart:async';

import 'package:grpc/grpc.dart' as grpc;

import '../../src/generated/grpc/timeline/chat.pbgrpc.dart' as $chat;
import 'grpc_timeline.dart';

enum ChatSocketStatus { disconnected, connecting, connected }

/// The chat real-time stream over gRPC, replacing the WebSocket `ChatSocket` with the same
/// surface: decoded server frames (`{"type": ...}`, the shape the provider already handles) on
/// [events], connection state on [statusStream], and the same exponential-backoff reconnect.
/// The server ends the stream when the access token expires; the reconnect then opens it again
/// with the token stored by the last sign-in or refresh.
class GrpcChatStream {
  StreamController<$chat.ClientFrame>? _outgoing;
  StreamSubscription<$chat.ServerFrame>? _subscription;
  Timer? _reconnectTimer;
  Duration _backoff = const Duration(seconds: 1);
  bool _manualClose = false;
  bool _hasCredential = false;
  int _generation = 0;

  final _eventsController = StreamController<Map<String, dynamic>>.broadcast();
  final _statusController = StreamController<ChatSocketStatus>.broadcast();

  Stream<Map<String, dynamic>> get events => _eventsController.stream;
  Stream<ChatSocketStatus> get statusStream => _statusController.stream;

  ChatSocketStatus _status = ChatSocketStatus.disconnected;
  ChatSocketStatus get status => _status;

  /// [token] only says whether there is a signed-in person; the call itself carries the token the
  /// auth interceptor reads when each (re)connection starts.
  void connect(String token) {
    if (token.isEmpty) return;
    _hasCredential = true;
    _manualClose = false;
    _open();
  }

  void _open() {
    if (!_hasCredential) return;
    _reconnectTimer?.cancel();
    _closeCurrent();
    final generation = ++_generation;
    _setStatus(ChatSocketStatus.connecting);

    final outgoing = StreamController<$chat.ClientFrame>();
    _outgoing = outgoing;
    final client = $chat.ChatServiceClient(GrpcTimeline.channel, interceptors: GrpcTimeline.interceptors);
    final call = client.openStream(outgoing.stream);

    call.headers.then((_) {
      if (generation != _generation) return;
      _backoff = const Duration(seconds: 1);
      _setStatus(ChatSocketStatus.connected);
    }).catchError((_) {
      if (generation == _generation) _handleDrop(generation);
    });

    _subscription = call.listen(
      (frame) {
        final decoded = frameToMap(frame);
        if (decoded != null) _eventsController.add(decoded);
      },
      onDone: () => _handleDrop(generation),
      onError: (Object error) {
        // An UNAUTHENTICATED end is the normal token-expiry close; any end reconnects.
        if (error is grpc.GrpcError || error is Exception) _handleDrop(generation);
      },
      cancelOnError: true,
    );
  }

  void _handleDrop(int generation) {
    if (generation != _generation) return;
    _setStatus(ChatSocketStatus.disconnected);
    _closeCurrent();
    _scheduleReconnect();
  }

  void _closeCurrent() {
    _subscription?.cancel();
    _subscription = null;
    _outgoing?.close();
    _outgoing = null;
  }

  void _scheduleReconnect() {
    if (_manualClose) return;
    _reconnectTimer?.cancel();
    _reconnectTimer = Timer(_backoff, _open);
    final next = _backoff.inSeconds * 2;
    _backoff = Duration(seconds: next > 30 ? 30 : next);
  }

  bool _send($chat.ClientFrame frame) {
    final outgoing = _outgoing;
    if (outgoing == null || outgoing.isClosed || _status != ChatSocketStatus.connected) return false;
    outgoing.add(frame);
    return true;
  }

  bool sendMessage({
    required String conversationUuid,
    required String body,
    required List<Map<String, dynamic>> media,
    required String clientMessageId,
  }) => _send(
    $chat.ClientFrame(
      send: $chat.SendFrame(
        conversationUuid: conversationUuid,
        body: body,
        media: media.map(_mediaOf),
        clientMessageId: clientMessageId,
      ),
    ),
  );

  bool sendRead({required String conversationUuid, required String lastReadMessageUuid}) => _send(
    $chat.ClientFrame(
      read: $chat.ReadFrame(conversationUuid: conversationUuid, lastReadMessageUuid: lastReadMessageUuid),
    ),
  );

  bool sendTyping(String conversationUuid) =>
      _send($chat.ClientFrame(typing: $chat.TypingFrame(conversationUuid: conversationUuid)));

  void disconnect() {
    _manualClose = true;
    _hasCredential = false;
    _reconnectTimer?.cancel();
    _generation++;
    _closeCurrent();
    _setStatus(ChatSocketStatus.disconnected);
  }

  void dispose() {
    disconnect();
    _eventsController.close();
    _statusController.close();
  }

  void _setStatus(ChatSocketStatus status) {
    _status = status;
    if (!_statusController.isClosed) _statusController.add(status);
  }

  static $chat.MessageMedia _mediaOf(Map<String, dynamic> media) => $chat.MessageMedia(
    mediaType: '${media['mediaType'] ?? ''}',
    objectKey: '${media['objectKey'] ?? ''}',
  );

  /// A server frame in the JSON shape the WebSocket used to deliver.
  static Map<String, dynamic>? frameToMap($chat.ServerFrame frame) {
    switch (frame.whichEvent()) {
      case $chat.ServerFrame_Event.messageNew:
        return {
          'type': 'message.new',
          'conversationUuid': frame.messageNew.conversationUuid,
          'conversationType': frame.messageNew.conversationType,
          'message': GrpcTimeline.json(frame.messageNew.message),
        };
      case $chat.ServerFrame_Event.conversationUpdated:
        return {
          'type': 'conversation.updated',
          'conversation': GrpcTimeline.json(frame.conversationUpdated.conversation),
        };
      case $chat.ServerFrame_Event.messageRead:
        return {
          'type': 'message.read',
          'conversationUuid': frame.messageRead.conversationUuid,
          'personUuid': frame.messageRead.personUuid,
          'lastReadMessageUuid': frame.messageRead.lastReadMessageUuid,
        };
      case $chat.ServerFrame_Event.typing:
        return {
          'type': 'typing',
          'conversationUuid': frame.typing.conversationUuid,
          'personUuid': frame.typing.personUuid,
        };
      case $chat.ServerFrame_Event.pong:
        return {'type': 'pong'};
      case $chat.ServerFrame_Event.error:
        return {'type': 'error', 'message': frame.error.message};
      case $chat.ServerFrame_Event.notSet:
        return null;
    }
  }
}
