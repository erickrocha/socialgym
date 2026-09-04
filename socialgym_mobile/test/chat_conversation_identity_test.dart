import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/l10n/app_localizations_en.dart';
import 'package:socialgym_mobile/models/conversation.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/widgets/chat/conversation_tile.dart';

Person _person(String uuid, String first, String last) => Person(
  id: 1,
  uuid: uuid,
  firstname: first,
  surname: last,
  addresses: const [],
  hasBusinessProfiles: false,
  businessProfiles: const [],
);

LastMessagePreview _preview(String senderUuid, String senderName) =>
    LastMessagePreview(
      messageUuid: 'm1',
      senderPersonUuid: senderUuid,
      senderDisplayName: senderName,
      snippet: 'oi',
      sentAt: DateTime(2026, 1, 1),
      hasMedia: false,
    );

Conversation _conversation({
  required String type,
  required List<String> participants,
  LastMessagePreview? lastMessage,
}) => Conversation(
  uuid: 'c1',
  conversationType: type,
  participantPersonUuids: participants,
  participants: const [],
  lastMessage: lastMessage,
  unread: false,
  updatedAt: DateTime(2026, 1, 1),
);

void main() {
  group('counterpartUuidFor', () {
    test('returns the other participant of a direct conversation', () {
      final conversation = _conversation(
        type: 'DirectPerson',
        participants: ['me', 'you'],
      );
      expect(conversation.counterpartUuidFor('me'), 'you');
      expect(conversation.counterpartUuidFor('you'), 'me');
    });

    test('returns null for business and group conversations', () {
      for (final type in ['BusinessTeamGroup', 'BusinessDirect']) {
        final conversation = _conversation(
          type: type,
          participants: ['me', 'you'],
        );
        expect(conversation.counterpartUuidFor('me'), isNull, reason: type);
      }
    });

    test('returns null when I am somehow the only participant', () {
      final conversation = _conversation(
        type: 'DirectPerson',
        participants: ['me'],
      );
      expect(conversation.counterpartUuidFor('me'), isNull);
    });

    test('uses the type string the backend actually sends', () {
      // domain/src/conversation.rs: CONVERSATION_TYPE_DIRECT_PERSON
      expect(
        _conversation(type: 'DirectPerson', participants: ['me', 'you'])
            .isDirect,
        isTrue,
      );
      expect(
        _conversation(type: 'Direct', participants: ['me', 'you']).isDirect,
        isFalse,
      );
    });
  });

  _titleTests();
}

// The headline regression: a conversation must never be titled with the
// viewer's own name, nor with the generic "Messages" label once we know who it
// is with.

void _titleTests() {
  final l10n = AppLocalizationsEn();
  final friends = [_person('you', 'Ana', 'Souza')];

  group('conversationTitle', () {
    test('a brand-new conversation is named after the friend, not "Messages"', () {
      final conversation = _conversation(
        type: 'DirectPerson',
        participants: ['me', 'you'],
      );
      final counterpart = counterpartOf(conversation, 'me', friends);
      expect(
        conversationTitle(conversation, counterpart, 'me', l10n),
        'Ana Souza',
      );
    });

    test('never shows my own name when I sent the last message', () {
      final conversation = _conversation(
        type: 'DirectPerson',
        participants: ['me', 'you'],
        lastMessage: _preview('me', 'Erick Rocha'),
      );
      // Even with no friend match to fall back on.
      final title = conversationTitle(conversation, null, 'me', l10n);
      expect(title, isNot('Erick Rocha'));
      expect(title, l10n.chatConversationsTitle);
    });

    test('falls back to the sender name for an ex-friend', () {
      final conversation = _conversation(
        type: 'DirectPerson',
        participants: ['me', 'stranger'],
        lastMessage: _preview('stranger', 'Ex Amigo'),
      );
      expect(
        conversationTitle(conversation, null, 'me', l10n),
        'Ex Amigo',
      );
    });
  });
}
