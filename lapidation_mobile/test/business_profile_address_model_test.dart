import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/commons/business_profile_mapper.dart';
import 'package:lapidation_mobile/models/business_profile_address.dart';
import 'package:lapidation_mobile/src/generated/grpc/business_profile_address.pb.dart'
    as $bpa;

void main() {
  group('BusinessProfileAddress.fromProto', () {
    test('maps set fields and treats zero/empty as unset for id/uuid', () {
      final proto = $bpa.BusinessProfileAddress(
        id: 9,
        uuid: 'addr-uuid',
        businessProfileId: 3,
        addressLine1: 'Main St 1',
        locality: 'Springfield',
        administrativeArea: 'IL',
        postalCode: '62704',
        countryCode: 'US',
      );

      final model = BusinessProfileAddressMapper().fromProto(proto);

      expect(model.id, 9);
      expect(model.uuid, 'addr-uuid');
      expect(model.businessProfileId, 3);
      expect(model.addressLine1, 'Main St 1');
      expect(model.addressLine2, isNull);
      expect(model.locality, 'Springfield');
    });

    test('new address (never persisted) has null id/uuid', () {
      final proto = $bpa.BusinessProfileAddress(
        businessProfileId: 3,
        addressLine1: 'Main St 1',
        locality: 'Springfield',
        administrativeArea: 'IL',
        postalCode: '62704',
        countryCode: 'US',
      );

      final model = BusinessProfileAddressMapper().fromProto(proto);

      expect(model.id, isNull);
      expect(model.uuid, isNull);
    });
  });

  group('BusinessProfileAddress.toProto', () {
    test('round-trips through protobuf encoding', () {
      final model = BusinessProfileAddress(
        businessProfileId: 3,
        addressLine1: 'Main St 1',
        addressLine2: 'Suite 5',
        locality: 'Springfield',
        administrativeArea: 'IL',
        postalCode: '62704',
        countryCode: 'US',
        latitude: 39.78,
        longitude: -89.65,
      );

      final encoded = BusinessProfileAddressMapper()
          .toProto(model)
          .writeToBuffer();
      final decoded = $bpa.BusinessProfileAddress.fromBuffer(encoded);

      expect(decoded.addressLine1, 'Main St 1');
      expect(decoded.addressLine2, 'Suite 5');
      expect(decoded.latitude, 39.78);
      expect(decoded.longitude, -89.65);
    });
  });
}
