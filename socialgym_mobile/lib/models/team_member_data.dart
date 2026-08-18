import 'business_profile.dart';
import 'person.dart';

/// A single team-membership record, returned by the mutating RPCs
/// (send/accept/deny/cancel) and by GetTeamMember.
class TeamMember {
  final int? id;
  final String? uuid;
  final int businessProfileId;
  final String businessProfileUuid;
  final int personId;
  final String personUuid;

  /// "Pending" | "Accepted" | "Rejected" | "Cancelled"
  final String status;

  TeamMember({
    this.id,
    this.uuid,
    required this.businessProfileId,
    required this.businessProfileUuid,
    required this.personId,
    required this.personUuid,
    required this.status,
  });
}

/// The combined result of GetTeamMemberPage: owner-side lists are populated
/// when a business_profile_id was supplied, person-side lists always are.
class TeamMemberPageData {
  final List<Person> members;
  final List<Person> sentRequests;
  final List<BusinessProfile> teams;
  final List<BusinessProfile> receivedRequests;

  TeamMemberPageData({
    required this.members,
    required this.sentRequests,
    required this.teams,
    required this.receivedRequests,
  });

  bool get isEmptyOwnerSide => members.isEmpty && sentRequests.isEmpty;
  bool get isEmptyPersonSide => teams.isEmpty && receivedRequests.isEmpty;
}
