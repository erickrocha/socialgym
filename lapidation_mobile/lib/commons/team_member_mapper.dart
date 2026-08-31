import 'package:lapidation_mobile/commons/business_profile_mapper.dart';
import 'package:lapidation_mobile/commons/mapper.dart';
import 'package:lapidation_mobile/commons/person_mapper.dart';
import 'package:lapidation_mobile/models/team_member_data.dart';

import '../src/generated/grpc/team_member.pb.dart' as $team_member;

class TeamMemberMapper implements Mapper<TeamMember, $team_member.TeamMember> {
  @override
  TeamMember fromProto($team_member.TeamMember proto) {
    return TeamMember(
      id: proto.id != 0 ? proto.id : null,
      uuid: proto.uuid.isNotEmpty ? proto.uuid : null,
      businessProfileId: proto.businessProfileId,
      businessProfileUuid: proto.businessProfileUuid,
      personId: proto.personId,
      personUuid: proto.personUuid,
      status: proto.status,
    );
  }

  @override
  List<TeamMember> fromProtoList(List<$team_member.TeamMember> protoList) {
    return protoList.map(fromProto).toList();
  }

  @override
  $team_member.TeamMember toProto(TeamMember domain) {
    return $team_member.TeamMember(
      id: domain.id ?? 0,
      uuid: domain.uuid ?? '',
      businessProfileId: domain.businessProfileId,
      businessProfileUuid: domain.businessProfileUuid,
      personId: domain.personId,
      personUuid: domain.personUuid,
      status: domain.status,
    );
  }

  @override
  List<$team_member.TeamMember> toProtoList(List<TeamMember> domainList) {
    return domainList.map(toProto).toList();
  }
}

class TeamMemberPageMapper {
  TeamMemberPageData fromProto($team_member.TeamMemberPageResponse proto) {
    return TeamMemberPageData(
      members: PersonMapper().fromProtoList(proto.members),
      sentRequests: PersonMapper().fromProtoList(proto.sentRequests),
      teams: BusinessProfileMapper().fromProtoList(proto.teams),
      receivedRequests: BusinessProfileMapper().fromProtoList(
        proto.receivedRequests,
      ),
    );
  }
}
