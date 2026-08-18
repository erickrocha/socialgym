import 'package:flutter/foundation.dart';

import '../models/business_profile.dart';
import '../models/person.dart';
import '../models/team_member_data.dart';
import '../services/grpc/grpc_team_member_service.dart';

class TeamMemberProvider extends ChangeNotifier {
  TeamMemberPageData? _data;
  bool _loading = false;
  bool _actionLoading = false;
  String? _error;

  int? _lastBusinessProfileId;
  int? _lastPersonId;

  TeamMemberPageData? get data => _data;
  bool get loading => _loading;
  bool get actionLoading => _actionLoading;
  String? get error => _error;

  List<Person> get members => _data?.members ?? [];
  List<Person> get sentRequests => _data?.sentRequests ?? [];
  List<BusinessProfile> get teams => _data?.teams ?? [];
  List<BusinessProfile> get receivedRequests => _data?.receivedRequests ?? [];

  /// businessProfileId populates members/sentRequests (owner side, pass it only
  /// when the caller currently has an active business profile); personId
  /// populates teams/receivedRequests (person side, always available).
  Future<void> fetchPage({int? businessProfileId, required int personId}) async {
    _lastBusinessProfileId = businessProfileId;
    _lastPersonId = personId;

    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _data = await GrpcTeamMemberService.getTeamMemberPage(
        businessProfileId: businessProfileId,
        personId: personId,
      );
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  Future<void> _refresh() async {
    await fetchPage(businessProfileId: _lastBusinessProfileId, personId: _lastPersonId ?? 0);
  }

  /// Owner side: invite a person to join the active business profile's team.
  Future<bool> inviteMember({required int businessProfileId, required int personId}) {
    return _mutate(
      () => GrpcTeamMemberService.sendTeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
    );
  }

  /// Owner side: cancel a pending invite this business profile sent.
  Future<bool> cancelSentInvite({required int businessProfileId, required int personId}) {
    return _mutate(
      () => GrpcTeamMemberService.cancelTeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
    );
  }

  /// Person side: accept an invite received from a business profile.
  Future<bool> acceptInvite({required int businessProfileId, required int personId}) {
    return _mutate(
      () => GrpcTeamMemberService.acceptTeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
    );
  }

  /// Person side: deny an invite received from a business profile.
  Future<bool> denyInvite({required int businessProfileId, required int personId}) {
    return _mutate(
      () => GrpcTeamMemberService.denyTeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
    );
  }

  Future<bool> _mutate(Future<TeamMember> Function() action) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await action();
      await _refresh();
      _actionLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  void clear() {
    _data = null;
    _loading = false;
    _actionLoading = false;
    _error = null;
    _lastBusinessProfileId = null;
    _lastPersonId = null;
    notifyListeners();
  }
}
