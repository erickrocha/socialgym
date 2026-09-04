import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/models/chat_message.dart';

ChatMessage _fromServer({
  required String uuid,
  required String clientMessageId,
}) => ChatMessage(
  uuid: uuid,
  conversationUuid: 'c1',
  senderPersonUuid: 'me',
  senderKind: 'Person',
  senderDisplayName: 'Me',
  body: 'oi',
  media: const [],
  clientMessageId: clientMessageId,
  sentAt: DateTime(2026, 1, 1),
);

/// Mirrors the dedupe in ChatProvider._appendMessage. Two optimistic messages
/// both carry an empty uuid, so matching on uuid alone would make the second
/// one overwrite the first.
int _indexOfExisting(List<ChatMessage> list, ChatMessage message) =>
    list.indexWhere(
      (m) =>
          (m.uuid.isNotEmpty && m.uuid == message.uuid) ||
          (m.clientMessageId.isNotEmpty &&
              m.clientMessageId == message.clientMessageId),
    );

void main() {
  group('optimistic message dedupe', () {
    test('two pending messages do not collide on their empty uuid', () {
      final first = ChatMessage.pending(
        conversationUuid: 'c1',
        senderPersonUuid: 'me',
        senderDisplayName: '',
        body: 'primeira',
        media: const [],
        clientMessageId: 'a',
      );
      final second = ChatMessage.pending(
        conversationUuid: 'c1',
        senderPersonUuid: 'me',
        senderDisplayName: '',
        body: 'segunda',
        media: const [],
        clientMessageId: 'b',
      );
      expect(_indexOfExisting([first], second), -1);
    });

    test('the server copy replaces the pending one with the same client id', () {
      final pending = ChatMessage.pending(
        conversationUuid: 'c1',
        senderPersonUuid: 'me',
        senderDisplayName: '',
        body: 'oi',
        media: const [],
        clientMessageId: 'a',
      );
      final confirmed = _fromServer(uuid: 'server-1', clientMessageId: 'a');
      expect(_indexOfExisting([pending], confirmed), 0);
      expect(pending.pending, isTrue);
      expect(confirmed.pending, isFalse);
    });

    test('failed copy keeps the body so retry has something to send', () {
      final failed = ChatMessage.pending(
        conversationUuid: 'c1',
        senderPersonUuid: 'me',
        senderDisplayName: '',
        body: 'oi',
        media: const [],
        clientMessageId: 'a',
      ).copyWith(pending: false, failed: true);
      expect(failed.failed, isTrue);
      expect(failed.pending, isFalse);
      expect(failed.body, 'oi');
      expect(failed.clientMessageId, 'a');
    });
  });
}
