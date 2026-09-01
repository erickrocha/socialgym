import 'dart:async';
import 'dart:convert';

import 'package:socialgym_mobile/config/api_config.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

enum ChatSocketStatus { disconnected, connecting, connected }

/// Wraps the chat WebSocket with exponential-backoff reconnect. Emits decoded
/// server frames (`{"type": ...}`) on [events] and connection state on
/// [statusStream]. Sending is best-effort — [sendMessage] returns false when
/// the socket is not open so the caller can fall back to REST.
class ChatSocket {
  WebSocketChannel? _channel;
  StreamSubscription? _subscription;
  Timer? _reconnectTimer;
  Duration _backoff = const Duration(seconds: 1);
  bool _manualClose = false;
  String? _token;

  final _eventsController = StreamController<Map<String, dynamic>>.broadcast();
  final _statusController = StreamController<ChatSocketStatus>.broadcast();

  Stream<Map<String, dynamic>> get events => _eventsController.stream;
  Stream<ChatSocketStatus> get statusStream => _statusController.stream;

  ChatSocketStatus _status = ChatSocketStatus.disconnected;
  ChatSocketStatus get status => _status;

  void connect(String token) {
    if (token.isEmpty) return;
    _token = token;
    _manualClose = false;
    _open();
  }

  void _open() {
    _reconnectTimer?.cancel();
    _setStatus(ChatSocketStatus.connecting);

    final uri = Uri.parse(
      '${ApiConfig.wsBaseUrl}/timeline/api/chat/ws'
      '?access_token=${Uri.encodeComponent(_token ?? '')}',
    );

    try {
      _channel = WebSocketChannel.connect(uri);
    } catch (_) {
      _scheduleReconnect();
      return;
    }

    _channel!.ready.then((_) {
      _backoff = const Duration(seconds: 1);
      _setStatus(ChatSocketStatus.connected);
    }).catchError((_) {
      _scheduleReconnect();
    });

    _subscription = _channel!.stream.listen(
      (raw) {
        try {
          final decoded = jsonDecode(raw as String);
          if (decoded is Map<String, dynamic>) {
            _eventsController.add(decoded);
          }
        } catch (_) {
          /* ignore malformed frame */
        }
      },
      onDone: _handleDrop,
      onError: (_) => _handleDrop(),
      cancelOnError: true,
    );
  }

  void _handleDrop() {
    _setStatus(ChatSocketStatus.disconnected);
    _subscription?.cancel();
    _subscription = null;
    _channel = null;
    _scheduleReconnect();
  }

  void _scheduleReconnect() {
    if (_manualClose) return;
    _reconnectTimer?.cancel();
    _reconnectTimer = Timer(_backoff, _open);
    final next = _backoff.inSeconds * 2;
    _backoff = Duration(seconds: next > 30 ? 30 : next);
  }

  bool _send(Map<String, dynamic> frame) {
    final channel = _channel;
    if (channel == null || _status != ChatSocketStatus.connected) return false;
    try {
      channel.sink.add(jsonEncode(frame));
      return true;
    } catch (_) {
      return false;
    }
  }

  bool sendMessage({
    required String conversationUuid,
    required String body,
    required List<Map<String, dynamic>> media,
    required String clientMessageId,
  }) => _send({
    'type': 'send',
    'conversationUuid': conversationUuid,
    'body': body,
    'media': media,
    'clientMessageId': clientMessageId,
  });

  bool sendRead({
    required String conversationUuid,
    required String lastReadMessageUuid,
  }) => _send({
    'type': 'read',
    'conversationUuid': conversationUuid,
    'lastReadMessageUuid': lastReadMessageUuid,
  });

  bool sendTyping(String conversationUuid) =>
      _send({'type': 'typing', 'conversationUuid': conversationUuid});

  void disconnect() {
    _manualClose = true;
    _reconnectTimer?.cancel();
    _subscription?.cancel();
    _channel?.sink.close();
    _channel = null;
    _subscription = null;
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
}
