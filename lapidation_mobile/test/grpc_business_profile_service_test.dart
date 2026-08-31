import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/src/generated/grpc/business_profile.pb.dart'
    as $bp;

void main() {
  group('BusinessProfileRequestId', () {
    test('round-trips id and uuid through protobuf encoding', () {
      final request = $bp.BusinessProfileRequestId(id: 5, uuid: 'bp-uuid');

      final encoded = request.writeToBuffer();
      final decoded = $bp.BusinessProfileRequestId.fromBuffer(encoded);

      expect(decoded.id, 5);
      expect(decoded.uuid, 'bp-uuid');
    });
  });

  group('BusinessProfileRequestOwnerId', () {
    test('round-trips ownerId and ownerUuid through protobuf encoding', () {
      final request = $bp.BusinessProfileRequestOwnerId(
        ownerId: 42,
        ownerUuid: 'owner-uuid-123',
      );

      final encoded = request.writeToBuffer();
      final decoded = $bp.BusinessProfileRequestOwnerId.fromBuffer(encoded);

      expect(decoded.ownerId, 42);
      expect(decoded.ownerUuid, 'owner-uuid-123');
    });
  });

  group('RemoveBusinessProfileAddressRequest', () {
    test('round-trips id and uuid through protobuf encoding', () {
      final request = $bp.RemoveBusinessProfileAddressRequest(
        id: 9,
        uuid: 'addr-uuid',
      );

      final encoded = request.writeToBuffer();
      final decoded = $bp.RemoveBusinessProfileAddressRequest.fromBuffer(
        encoded,
      );

      expect(decoded.id, 9);
      expect(decoded.uuid, 'addr-uuid');
    });
  });
}
