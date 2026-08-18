import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/utils/display_name_helper.dart';

void main() {
  group('DisplayNameHelper', () {
    group('getPersonFullName', () {
      test('returns full name when both firstname and surname present', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: 'Doe',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John Doe');
      });

      test('returns firstname only when surname is empty', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: '',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John');
      });

      test('handles whitespace correctly', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: '  John  ',
          surname: '  Doe  ',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John Doe');
      });

      test('handles whitespace-only surname as empty', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: '   ',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John');
      });
    });

    group('getBusinessProfileDisplayName', () {
      test('returns social name when defined', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: 'John\'s Gym',
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'John\'s Gym',
        );
      });

      test('returns business name when social name is empty', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '',
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'Fitness Corp',
        );
      });

      test('returns business name when social name is null', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: null,
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'Fitness Corp',
        );
      });

      test('handles whitespace-only social name as empty', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '   ',
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'Fitness Corp',
        );
      });
    });

    group('formatBusinessProfileForMenu', () {
      test('uses social name as primary when defined', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: 'John\'s Gym',
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'John\'s Gym');
        expect(result['secondary'], 'Fitness Corp');
      });

      test('uses business name as primary when social name is empty', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '',
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'Fitness Corp');
        expect(result['secondary'], 'Professional');
      });

      test('uses business name as primary when social name is null', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: null,
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'Fitness Corp');
        expect(result['secondary'], 'Professional');
      });

      test('handles whitespace-only social name', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '   ',
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'Fitness Corp');
        expect(result['secondary'], 'Professional');
      });
    });
  });
}
