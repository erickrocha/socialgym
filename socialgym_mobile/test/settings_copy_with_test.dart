import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/models/enums.dart';
import 'package:socialgym_mobile/models/settings.dart';

void main() {
  group('Settings.copyWith', () {
    test('preserves id, personId, personUuid, theme and notificationsEnabled when only language changes', () {
      final original = Settings(
        id: 42,
        uuid: 'setting-uuid',
        personId: 7,
        personUuid: 'person-uuid',
        language: 'en',
        theme: 'dark',
        notificationsEnabled: false,
        contextMenuPosition: ContextMenuPosition.right,
        homePage: Pages.gallery,
      );

      final updated = original.copyWith(language: 'es');

      expect(updated.id, 42);
      expect(updated.uuid, 'setting-uuid');
      expect(updated.personId, 7);
      expect(updated.personUuid, 'person-uuid');
      expect(updated.theme, 'dark');
      expect(updated.notificationsEnabled, false);
      expect(updated.contextMenuPosition, ContextMenuPosition.right);
      expect(updated.homePage, Pages.gallery);
      expect(updated.language, 'es');
    });

    test('updates notificationsEnabled when passed explicitly', () {
      final original = Settings(notificationsEnabled: true);
      final updated = original.copyWith(notificationsEnabled: false);
      expect(updated.notificationsEnabled, false);
    });
  });
}
