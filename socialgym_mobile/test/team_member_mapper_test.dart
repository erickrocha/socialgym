import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/commons/team_member_mapper.dart';
import 'package:socialgym_mobile/src/generated/grpc/business_profile.pb.dart' as $business_profile;
import 'package:socialgym_mobile/src/generated/grpc/person.pb.dart' as $person;
import 'package:socialgym_mobile/src/generated/grpc/team_member.pb.dart' as $tm;

void main() {
  group('TeamMemberMapper', () {
    test('fromProto maps all fields', () {
      final proto = $tm.TeamMember(
        id: 1,
        uuid: 'tm-uuid',
        businessProfileId: 5,
        businessProfileUuid: 'bp-uuid',
        personId: 42,
        personUuid: 'person-uuid',
        status: 'Accepted',
      );

      final domain = TeamMemberMapper().fromProto(proto);

      expect(domain.id, 1);
      expect(domain.uuid, 'tm-uuid');
      expect(domain.businessProfileId, 5);
      expect(domain.businessProfileUuid, 'bp-uuid');
      expect(domain.personId, 42);
      expect(domain.personUuid, 'person-uuid');
      expect(domain.status, 'Accepted');
    });

    test('fromProto treats zero id and empty uuid as absent', () {
      final proto = $tm.TeamMember(
        businessProfileId: 5,
        businessProfileUuid: 'bp-uuid',
        personId: 42,
        personUuid: 'person-uuid',
        status: 'Pending',
      );

      final domain = TeamMemberMapper().fromProto(proto);

      expect(domain.id, isNull);
      expect(domain.uuid, isNull);
    });
  });

  group('TeamMemberPageMapper', () {
    test('fromProto maps members/sentRequests to Person and teams/receivedRequests to BusinessProfile', () {
      final proto = $tm.TeamMemberPageResponse(
        members: [
          $person.Person(id: 1, uuid: 'p1', firstname: 'Carlos', surname: 'Silva', gender: 'Male'),
        ],
        sentRequests: [
          $person.Person(id: 2, uuid: 'p2', firstname: 'Ines', surname: 'Invitee', gender: 'Female'),
        ],
        teams: [
          $business_profile.BusinessProfile(
            id: 10,
            uuid: 'bp1',
            ownerId: 1,
            ownerUuid: 'p1',
            taxId: '123',
            businessName: 'Academia Teste',
            businessType: 'Company',
          ),
        ],
        receivedRequests: [
          $business_profile.BusinessProfile(
            id: 11,
            uuid: 'bp2',
            ownerId: 3,
            ownerUuid: 'p3',
            taxId: '456',
            businessName: 'Studio X',
            businessType: 'Company',
          ),
        ],
      );

      final page = TeamMemberPageMapper().fromProto(proto);

      expect(page.members, hasLength(1));
      expect(page.members.first.fullName, 'Carlos Silva');
      expect(page.sentRequests, hasLength(1));
      expect(page.sentRequests.first.fullName, 'Ines Invitee');
      expect(page.teams, hasLength(1));
      expect(page.teams.first.businessName, 'Academia Teste');
      expect(page.receivedRequests, hasLength(1));
      expect(page.receivedRequests.first.businessName, 'Studio X');
    });

    test('fromProto handles empty lists', () {
      final proto = $tm.TeamMemberPageResponse();

      final page = TeamMemberPageMapper().fromProto(proto);

      expect(page.isEmptyOwnerSide, isTrue);
      expect(page.isEmptyPersonSide, isTrue);
    });
  });
}
