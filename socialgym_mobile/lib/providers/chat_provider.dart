import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:image_picker/image_picker.dart';
import 'package:socialgym_mobile/models/chat_message.dart';
import 'package:socialgym_mobile/models/conversation.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/chat_service.dart';
import 'package:socialgym_mobile/services/chat_socket.dart';
import 'package:uuid/uuid.dart';

class ChatProvider extends ChangeNotifier {
  /// How long a "typing" frame keeps the indicator up. The protocol has no
  /// "stopped typing", so it expires on its own.
  static const Duration _typingTtl = Duration(seconds: 4);
  static const Duration _typingThrottleGap = Duration(seconds: 3);
  /// How long a socket-sent message may stay unacknowledged before it is shown
  /// as failed.
  static const Duration _ackTimeout = Duration(seconds: 10);

  final ChatSocket _socket = ChatSocket();
  final Uuid _uuid = const Uuid();

  StreamSubscription? _eventsSub;
  StreamSubscription? _statusSub;

  String _token = '';
  String _myPersonUuid = '';

  List<Conversation> _conversations = [];
  final Map<String, List<ChatMessage>> _messages = {};
  final Map<String, DateTime> _lastSeenSentAt = {};
  Set<String> _onlinePersonUuids = {};
  /// conversationUuid -> when the counterpart's "typing" signal goes stale.
  final Map<String, DateTime> _typingUntil = {};
  /// conversationUuid -> last message uuid the counterpart has read.
  final Map<String, String> _counterpartLastRead = {};
  final Map<String, int> _pagesLoaded = {};
  final Set<String> _fullyLoaded = {};
  bool _loadingOlder = false;
  Timer? _typingThrottle;
  Timer? _typingExpiry;

  bool _loadingConversations = false;
  bool _loadingMessages = false;
  bool _sending = false;
  String? _error;
  ChatSocketStatus _socketStatus = ChatSocketStatus.disconnected;

  List<Conversation> get conversations => List.unmodifiable(_conversations);
  List<ChatMessage> messagesFor(String conversationUuid) =>
      List.unmodifiable(_messages[conversationUuid] ?? const []);
  bool get loadingConversations => _loadingConversations;
  bool get loadingMessages => _loadingMessages;
  bool get sending => _sending;
  String? get error => _error;
  ChatSocketStatus get socketStatus => _socketStatus;
  String get myPersonUuid => _myPersonUuid;
  bool isOnline(String personUuid) => _onlinePersonUuids.contains(personUuid);
  bool get loadingOlder => _loadingOlder;
  bool hasMoreMessages(String conversationUuid) =>
      !_fullyLoaded.contains(conversationUuid);

  /// ponytail: a bool, not "who is typing" — enough for a direct chat, which is
  /// all the app creates today. Carry the person uuid through when groups land.
  bool isTypingIn(String conversationUuid) {
    final until = _typingUntil[conversationUuid];
    return until != null && until.isAfter(DateTime.now());
  }

  /// True once the counterpart has read [messageUuid] (or anything newer).
  bool isReadByCounterpart(String conversationUuid, String messageUuid) {
    final lastRead = _counterpartLastRead[conversationUuid];
    if (lastRead == null || messageUuid.isEmpty) return false;
    final list = _messages[conversationUuid] ?? const [];
    final readIdx = list.indexWhere((m) => m.uuid == lastRead);
    final thisIdx = list.indexWhere((m) => m.uuid == messageUuid);
    return readIdx != -1 && thisIdx != -1 && thisIdx <= readIdx;
  }

  /// ponytail: polled on demand by whoever renders a people list. A presence
  /// event on the socket would remove the poll — add it when a screen needs
  /// status to change while the user is looking at it.
  Future<void> refreshPresence(List<String> personUuids) async {
    if (_token.isEmpty) return;
    _onlinePersonUuids = await ChatService.presence(_token, personUuids);
    notifyListeners();
  }

  void connect(String token, String myPersonUuid) {
    if (token.isEmpty) return;
    _token = token;
    _myPersonUuid = myPersonUuid;

    _eventsSub ??= _socket.events.listen(_onServerEvent);
    _statusSub ??= _socket.statusStream.listen((status) {
      final reconnected =
          _socketStatus != ChatSocketStatus.connected &&
          status == ChatSocketStatus.connected;
      _socketStatus = status;
      notifyListeners();
      if (reconnected) _replayMissed();
    });

    _socket.connect(token);
  }

  void disconnect() {
    _socket.disconnect();
  }

  @override
  void dispose() {
    _typingThrottle?.cancel();
    _typingExpiry?.cancel();
    _eventsSub?.cancel();
    _statusSub?.cancel();
    _socket.dispose();
    super.dispose();
  }

  Future<void> fetchConversations() async {
    if (_token.isEmpty) return;
    _loadingConversations = true;
    _error = null;
    notifyListeners();
    try {
      _conversations = await ChatService.listConversations(_token);
    } on AppException catch (e) {
      _error = e.message;
    } catch (_) {
      _error = 'Failed to load conversations.';
    }
    _loadingConversations = false;
    notifyListeners();
  }

  Future<void> fetchMessages(String conversationUuid, {int page = 0}) async {
    if (_token.isEmpty) return;
    _loadingMessages = true;
    notifyListeners();
    try {
      final fetched = await ChatService.listMessages(
        _token,
        conversationUuid,
        page: page,
      );
      // API returns newest-first; store oldest-first.
      final ordered = fetched.reversed.toList();
      if (fetched.isEmpty) _fullyLoaded.add(conversationUuid);
      _pagesLoaded[conversationUuid] = page;
      if (page == 0) {
        _fullyLoaded.remove(conversationUuid);
        _messages[conversationUuid] = ordered;
      } else {
        _messages[conversationUuid] = [
          ...ordered,
          ...(_messages[conversationUuid] ?? const []),
        ];
      }
      _trackLastSeen(conversationUuid);
    } on AppException catch (e) {
      _error = e.message;
    } catch (_) {
      _error = 'Failed to load messages.';
    }
    _loadingMessages = false;
    notifyListeners();
  }

  /// Pulls the previous page of history, for the thread's scroll-to-top.
  Future<void> loadOlder(String conversationUuid) async {
    if (_loadingOlder ||
        _fullyLoaded.contains(conversationUuid) ||
        (_messages[conversationUuid] ?? const []).isEmpty) {
      return;
    }
    _loadingOlder = true;
    notifyListeners();
    await fetchMessages(
      conversationUuid,
      page: (_pagesLoaded[conversationUuid] ?? 0) + 1,
    );
    _loadingOlder = false;
    notifyListeners();
  }

  Future<Conversation?> openDirect(String targetPersonUuid) =>
      _open(() => ChatService.createDirect(_token, targetPersonUuid));

  Future<Conversation?> openTeamGroup(String businessProfileUuid) =>
      _open(() => ChatService.createTeamGroup(_token, businessProfileUuid));

  Future<Conversation?> openBusinessDirect(
    String businessProfileUuid, {
    String? memberPersonUuid,
  }) => _open(
    () => ChatService.createBusinessDirect(
      _token,
      businessProfileUuid,
      memberPersonUuid: memberPersonUuid,
    ),
  );

  Future<Conversation?> _open(Future<Conversation> Function() call) async {
    if (_token.isEmpty) return null;
    try {
      final conversation = await call();
      final idx = _conversations.indexWhere((c) => c.uuid == conversation.uuid);
      if (idx == -1) {
        _conversations = [conversation, ..._conversations];
      } else {
        _conversations[idx] = conversation;
      }
      notifyListeners();
      return conversation;
    } on AppException catch (e) {
      _error = e.message;
      notifyListeners();
      return null;
    }
  }

  Future<void> send(
    String conversationUuid, {
    String body = '',
    List<XFile> images = const [],
  }) async {
    if (_token.isEmpty) return;
    final trimmed = body.trim();
    if (trimmed.isEmpty && images.isEmpty) return;

    final clientMessageId = _uuid.v4();
    // Render it before any I/O — the server's copy replaces this one through
    // the clientMessageId dedupe in [_appendMessage].
    _appendMessage(
      conversationUuid,
      ChatMessage.pending(
        conversationUuid: conversationUuid,
        senderPersonUuid: _myPersonUuid,
        senderDisplayName: '',
        body: trimmed,
        media: const [],
        clientMessageId: clientMessageId,
      ),
    );

    await _deliver(
      conversationUuid,
      body: trimmed,
      images: images,
      clientMessageId: clientMessageId,
    );
  }

  /// Retries a message that failed to leave the device. Its text and images are
  /// gone by then, so only text messages can be retried — an image send that
  /// failed mid-upload has to be re-picked.
  Future<void> resend(String conversationUuid, String clientMessageId) async {
    final list = _messages[conversationUuid] ?? const [];
    final message = list.firstWhere(
      (m) => m.clientMessageId == clientMessageId,
      orElse: () => throw StateError('unknown message'),
    );
    _replaceByClientId(
      conversationUuid,
      clientMessageId,
      message.copyWith(pending: true, failed: false),
    );
    await _deliver(
      conversationUuid,
      body: message.body,
      images: const [],
      clientMessageId: clientMessageId,
    );
  }

  Future<void> _deliver(
    String conversationUuid, {
    required String body,
    required List<XFile> images,
    required String clientMessageId,
  }) async {
    _sending = true;
    notifyListeners();

    try {
      final media = images.isEmpty
          ? const <Map<String, dynamic>>[]
          : await ChatService.uploadImages(_token, images);

      final sentViaSocket =
          media.isEmpty &&
          _socket.sendMessage(
            conversationUuid: conversationUuid,
            body: body,
            media: const [],
            clientMessageId: clientMessageId,
          );

      if (!sentViaSocket) {
        final message = await ChatService.sendMessage(
          _token,
          conversationUuid,
          body: body,
          media: media,
          clientMessageId: clientMessageId,
        );
        _appendMessage(conversationUuid, message);
      }
      if (sentViaSocket) {
        // The socket send is fire-and-forget: nothing tells us synchronously
        // that the server accepted it. If no `message.new` echo lands, fail the
        // message so the user gets a retry instead of a permanent clock icon.
        Timer(_ackTimeout, () {
          final list = _messages[conversationUuid] ?? const [];
          final idx = list.indexWhere(
            (m) => m.clientMessageId == clientMessageId,
          );
          if (idx != -1 && list[idx].pending) {
            _markFailed(conversationUuid, clientMessageId);
          }
        });
      }
    } on AppException catch (e) {
      _error = e.message;
      _markFailed(conversationUuid, clientMessageId);
    } catch (_) {
      _error = 'Failed to send message.';
      _markFailed(conversationUuid, clientMessageId);
    }
    _sending = false;
    notifyListeners();
  }

  void _markFailed(String conversationUuid, String clientMessageId) {
    final list = _messages[conversationUuid] ?? const [];
    final idx = list.indexWhere((m) => m.clientMessageId == clientMessageId);
    if (idx == -1) return;
    _replaceByClientId(
      conversationUuid,
      clientMessageId,
      list[idx].copyWith(pending: false, failed: true),
    );
  }

  void _replaceByClientId(
    String conversationUuid,
    String clientMessageId,
    ChatMessage message,
  ) {
    final list = List<ChatMessage>.from(_messages[conversationUuid] ?? const []);
    final idx = list.indexWhere((m) => m.clientMessageId == clientMessageId);
    if (idx == -1) return;
    list[idx] = message;
    _messages[conversationUuid] = list;
    notifyListeners();
  }

  Future<void> markRead(String conversationUuid) async {
    final list = _messages[conversationUuid];
    if (list == null || list.isEmpty) return;
    final last = list.last;
    if (last.senderPersonUuid == _myPersonUuid) return;

    final sentViaSocket = _socket.sendRead(
      conversationUuid: conversationUuid,
      lastReadMessageUuid: last.uuid,
    );
    if (!sentViaSocket) {
      try {
        await ChatService.markRead(_token, conversationUuid, last.uuid);
      } catch (_) {
        /* best effort */
      }
    }
    final idx = _conversations.indexWhere((c) => c.uuid == conversationUuid);
    if (idx != -1 && _conversations[idx].unread) {
      _conversations[idx] = _conversations[idx].copyWith(unread: false);
      notifyListeners();
    }
  }

  /// Fired from `onChanged`, so throttled — one frame per [_typingThrottleGap]
  /// instead of one per keystroke.
  void sendTyping(String conversationUuid) {
    if (_typingThrottle?.isActive ?? false) return;
    _typingThrottle = Timer(_typingThrottleGap, () {});
    _socket.sendTyping(conversationUuid);
  }

  void clearError() {
    _error = null;
    notifyListeners();
  }

  // ── internals ────────────────────────────────────────────────────────────

  /// Frames are camelCase (locked by `frames_use_camel_case_field_names` in
  /// timeline's chat_hub.rs); the snake_case fallback keeps an older server
  /// from silently filing messages under the empty conversation.
  static String _frameString(Map<String, dynamic> frame, String camel, String snake) =>
      (frame[camel] ?? frame[snake] ?? '') as String;

  void _onServerEvent(Map<String, dynamic> frame) {
    switch (frame['type']) {
      case 'message.new':
        final conversationUuid = _frameString(
          frame,
          'conversationUuid',
          'conversation_uuid',
        );
        final raw = frame['message'];
        if (conversationUuid.isNotEmpty && raw is Map<String, dynamic>) {
          final message = ChatMessage.fromJson(raw);
          final known = _conversations.any((c) => c.uuid == conversationUuid);
          _appendMessage(conversationUuid, message);
          // Their message answers ours: they are clearly done typing.
          _typingUntil.remove(conversationUuid);
          if (!known) fetchConversations();
        }
        break;
      case 'message.read':
        final conversationUuid = _frameString(
          frame,
          'conversationUuid',
          'conversation_uuid',
        );
        final personUuid = _frameString(frame, 'personUuid', 'person_uuid');
        final lastRead = _frameString(
          frame,
          'lastReadMessageUuid',
          'last_read_message_uuid',
        );
        if (conversationUuid.isNotEmpty &&
            lastRead.isNotEmpty &&
            personUuid != _myPersonUuid) {
          _counterpartLastRead[conversationUuid] = lastRead;
          notifyListeners();
        }
        break;
      case 'typing':
        final conversationUuid = _frameString(
          frame,
          'conversationUuid',
          'conversation_uuid',
        );
        final personUuid = _frameString(frame, 'personUuid', 'person_uuid');
        if (conversationUuid.isEmpty || personUuid == _myPersonUuid) break;
        _typingUntil[conversationUuid] = DateTime.now().add(_typingTtl);
        notifyListeners();
        // Nothing pushes "stopped typing", so expire it on a timer.
        _typingExpiry?.cancel();
        _typingExpiry = Timer(_typingTtl, notifyListeners);
        break;
      case 'error':
        // Swallowing this is what hid the camelCase frame mismatch for so long.
        final message = _frameString(frame, 'message', 'message');
        if (message.isNotEmpty) {
          _error = message;
          notifyListeners();
        }
        break;
      case 'pong':
      default:
        break;
    }
  }

  void _appendMessage(String conversationUuid, ChatMessage message) {
    final list = List<ChatMessage>.from(_messages[conversationUuid] ?? const []);
    final existing = list.indexWhere(
      (m) =>
          // An optimistic message has an empty uuid — matching on it would make
          // every pending message collide with the previous one.
          (m.uuid.isNotEmpty && m.uuid == message.uuid) ||
          (m.clientMessageId.isNotEmpty &&
              m.clientMessageId == message.clientMessageId),
    );
    if (existing != -1) {
      list[existing] = message;
    } else {
      list.add(message);
    }
    _messages[conversationUuid] = list;
    _lastSeenSentAt[conversationUuid] = message.sentAt;

    final idx = _conversations.indexWhere((c) => c.uuid == conversationUuid);
    if (idx != -1) {
      final updated = _conversations[idx].copyWith(
        updatedAt: message.sentAt,
        lastMessage: LastMessagePreview(
          messageUuid: message.uuid,
          senderPersonUuid: message.senderPersonUuid,
          senderDisplayName: message.senderDisplayName,
          snippet: message.body.isEmpty ? '📷 Photo' : message.body,
          sentAt: message.sentAt,
          hasMedia: message.media.isNotEmpty,
        ),
        unread: message.senderPersonUuid != _myPersonUuid,
      );
      _conversations
        ..removeAt(idx)
        ..insert(0, updated);
    }
    notifyListeners();
  }

  void _trackLastSeen(String conversationUuid) {
    final list = _messages[conversationUuid];
    if (list != null && list.isNotEmpty) {
      _lastSeenSentAt[conversationUuid] = list.last.sentAt;
    }
  }

  void _replayMissed() {
    fetchConversations();
    _lastSeenSentAt.forEach((conversationUuid, sentAt) async {
      try {
        final missed = await ChatService.listMessages(
          _token,
          conversationUuid,
          since: sentAt.toUtc().millisecondsSinceEpoch,
        );
        for (final message in missed) {
          _appendMessage(conversationUuid, message);
        }
      } catch (_) {
        /* best effort */
      }
    });
  }
}
