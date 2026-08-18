import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/commons/business_profile_mapper.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/src/generated/grpc/business_profile.pb.dart' as $bp;
import 'package:socialgym_mobile/src/generated/grpc/business_profile_address.pb.dart' as $bpa;

void main() {
  group('BusinessProfile.fromProto', () {
    test('maps a persisted profile including addresses', () {
      final proto = $bp.BusinessProfile(
        id: 5,
        uuid: 'bp-uuid',
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        taxId: '12-3456789',
        businessName: 'Jane Doe Training',
        businessType: 'Professional',
        socialName: 'Jane Doe',
        addresses: [
          $bpa.BusinessProfileAddress(
            businessProfileId: 5,
            addressLine1: 'Gym Ave 1',
            locality: 'Metropolis',
            administrativeArea: 'NY',
            postalCode: '10001',
            countryCode: 'US',
          ),
        ],
      );

      final model = BusinessProfileMapper().fromProto(proto);

      expect(model.id, 5);
      expect(model.uuid, 'bp-uuid');
      expect(model.businessType, 'Professional');
      expect(model.socialName, 'Jane Doe');
      expect(model.addresses, hasLength(1));
      expect(model.addresses.first.addressLine1, 'Gym Ave 1');
    });

    test('newly created profile (not yet persisted) has null id/uuid', () {
      final proto = $bp.BusinessProfile(
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        taxId: '12-3456789',
        businessName: 'Iron Gym',
        businessType: 'Company',
      );

      final model = BusinessProfileMapper().fromProto(proto);

      expect(model.id, isNull);
      expect(model.uuid, isNull);
      expect(model.socialName, isNull);
    });
  });

  group('BusinessProfile.toProto', () {
    test('round-trips required fields through protobuf encoding', () {
      const model = BusinessProfile(
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        taxId: '12-3456789',
        businessName: 'Jane Doe Training',
        businessType: 'Professional',
        socialName: 'Jane Doe',
      );

      final encoded = BusinessProfileMapper().toProto(model).writeToBuffer();
      final decoded = $bp.BusinessProfile.fromBuffer(encoded);

      expect(decoded.ownerId, 1);
      expect(decoded.businessName, 'Jane Doe Training');
      expect(decoded.businessType, 'Professional');
    });
  });
}