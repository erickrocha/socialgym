import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/utils/mention_text_utils.dart';

void main() {
  group('findActiveMentionQuery', () {
    test('finds active mention token at cursor', () {
      const text = 'Hello @pe';
      final mention = findActiveMentionQuery(text, text.length);
      expect(mention, isNotNull);
      expect(mention!.token, '@pe');
      expect(mention.start, 6);
      expect(mention.end, text.length);
    });

    test('ignores @ that is part of non-mention token', () {
      const text = 'email@test.com';
      final mention = findActiveMentionQuery(text, text.length);
      expect(mention, isNull);
    });
  });

  test(
    'replaceMentionQuery replaces mention token and adds trailing space',
    () {
      const original = 'Hi @pe how are you';
      const query = MentionQuery(start: 3, end: 6, token: '@pe');

      final replaced = replaceMentionQuery(
        text: original,
        query: query,
        mentionDisplay: 'Pedro Silva',
      );

      expect(replaced, 'Hi @Pedro Silva how are you');
    },
  );
}
