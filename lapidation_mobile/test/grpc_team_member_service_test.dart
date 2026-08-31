import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/src/generated/grpc/team_member.pb.dart'
    as $tm;

void main() {
  group('TeamMemberRequest', () {
    test(
      'round-trips businessProfileId and personId through protobuf encoding',
      () {
        final request = $tm.TeamMemberRequest(
          businessProfileId: 5,
          personId: 42,
        );

        final encoded = request.writeToBuffer();
        final decoded = $tm.TeamMemberRequest.fromBuffer(encoded);

        expect(decoded.businessProfileId, 5);
        expect(decoded.personId, 42);
      },
    );
  });

  group('TeamMember', () {
    test('round-trips all fields through protobuf encoding', () {
      final member = $tm.TeamMember(
        id: 1,
        uuid: 'tm-uuid',
        businessProfileId: 5,
        businessProfileUuid: 'bp-uuid',
        personId: 42,
        personUuid: 'person-uuid',
        status: 'Pending',
      );

      final decoded = $tm.TeamMember.fromBuffer(member.writeToBuffer());

      expect(decoded.id, 1);
      expect(decoded.uuid, 'tm-uuid');
      expect(decoded.businessProfileId, 5);
      expect(decoded.businessProfileUuid, 'bp-uuid');
      expect(decoded.personId, 42);
      expect(decoded.personUuid, 'person-uuid');
      expect(decoded.status, 'Pending');
    });
  });

  group('TeamMemberPageRequest', () {
    test(
      'round-trips businessProfileId and personId through protobuf encoding',
      () {
        final request = $tm.TeamMemberPageRequest(
          businessProfileId: 5,
          personId: 42,
        );

        final decoded = $tm.TeamMemberPageRequest.fromBuffer(
          request.writeToBuffer(),
        );

        expect(decoded.businessProfileId, 5);
        expect(decoded.personId, 42);
      },
    );
  });
}
