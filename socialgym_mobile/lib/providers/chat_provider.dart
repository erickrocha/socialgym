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
  final ChatSocket _socket = ChatSocket();
  final Uuid _uuid = const Uuid();

  StreamSubscription? _eventsSub;
  StreamSubscription? _statusSub;

  String _token = '';
  String _myPersonUuid = '';

  List<Conversation> _conversations = [];
  final Map<String, List<ChatMessage>> _messages = {};
  final Map<String, DateTime> _lastSeenSentAt = {};

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
      if (page == 0) {
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

    _sending = true;
    notifyListeners();

    try {
      final media = images.isEmpty
          ? const <Map<String, dynamic>>[]
          : await ChatService.uploadImages(_token, images);
      final clientMessageId = _uuid.v4();

      final sentViaSocket =
          images.isEmpty &&
          _socket.sendMessage(
            conversationUuid: conversationUuid,
            body: trimmed,
            media: const [],
            clientMessageId: clientMessageId,
          );

      if (!sentViaSocket) {
        final message = await ChatService.sendMessage(
          _token,
          conversationUuid,
          body: trimmed,
          media: media,
          clientMessageId: clientMessageId,
        );
        _appendMessage(conversationUuid, message);
      }
    } on AppException catch (e) {
      _error = e.message;
    } catch (_) {
      _error = 'Failed to send message.';
    }
    _sending = false;
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

  void sendTyping(String conversationUuid) =>
      _socket.sendTyping(conversationUuid);

  void clearError() {
    _error = null;
    notifyListeners();
  }

  // ── internals ────────────────────────────────────────────────────────────

  void _onServerEvent(Map<String, dynamic> frame) {
    switch (frame['type']) {
      case 'message.new':
        final conversationUuid = frame['conversationUuid'] as String? ?? '';
        final raw = frame['message'];
        if (raw is Map<String, dynamic>) {
          final message = ChatMessage.fromJson(raw);
          final known = _conversations.any((c) => c.uuid == conversationUuid);
          _appendMessage(conversationUuid, message);
          if (!known) fetchConversations();
        }
        break;
      case 'message.read':
      case 'typing':
      case 'pong':
      case 'error':
      default:
        break;
    }
  }

  void _appendMessage(String conversationUuid, ChatMessage message) {
    final list = List<ChatMessage>.from(_messages[conversationUuid] ?? const []);
    final existing = list.indexWhere(
      (m) =>
          m.uuid == message.uuid ||
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
