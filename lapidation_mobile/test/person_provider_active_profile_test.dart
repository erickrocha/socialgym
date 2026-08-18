import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';

Person _personWithProfiles() {
  return Person(
    id: 1,
    uuid: 'person-uuid',
    firstname: 'Jane',
    surname: 'Doe',
    businessProfiles: [
      BusinessProfile(
        id: 1,
        uuid: 'profile-uuid-1',
        ownerId: 1,
        ownerUuid: 'person-uuid',
        businessType: "Professional",
        businessName: 'Personal',
        taxId: '1234656'
      ),
    ],
  );
}

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  group('PersonProvider active profile', () {
    test('starts in personal mode', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);

      expect(provider.isProfessional, isFalse);
      expect(provider.activeBusinessProfile, isNull);
    });

    // switchProfile/switchToPersonal now call the backend activate/deactivate
    // REST endpoints (see business_profile_rest_service.dart), which are
    // unreachable in this unit-test environment, so they resolve to null and
    // leave local state unchanged. Exercise the local-only setters instead
    // to cover the state/storage plumbing.
    test('setActiveBusinessProfile sets isProfessional', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());

      provider.setActiveBusinessProfile(_personWithProfiles().businessProfiles.first);

      expect(provider.isProfessional, isTrue);
      expect(provider.activeBusinessProfile?.businessName, 'Personal');
    });

    test('switchProfile returns null when the activate call cannot reach the backend', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());

      final result = await provider.switchProfile(0, 'token');

      expect(result, isNull);
    });

    test('switchToPersonal returns null when the deactivate call cannot reach the backend', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());
      provider.setActiveBusinessProfile(_personWithProfiles().businessProfiles.first);

      final result = await provider.switchToPersonal('token');

      expect(result, isNull);
    });

    test('clear() resets both person and active profile', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());
      provider.setActiveBusinessProfile(_personWithProfiles().businessProfiles.first);

      provider.clear();

      expect(provider.person, isNull);
      expect(provider.activeBusinessProfile, isNull);
    });
  });
}
