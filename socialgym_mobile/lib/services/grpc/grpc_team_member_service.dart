import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/commons/team_member_mapper.dart';

import '../../config/api_config.dart';
import '../../models/team_member_data.dart';
import '../../src/generated/grpc/team_member.pbgrpc.dart' as $tm;
import '../base_service.dart';
import 'grpc_channel_factory.dart';

class GrpcTeamMemberService {
  GrpcTeamMemberService._();

  static $tm.TeamMemberServiceClient? _client;

  /// Either id may be omitted (0): businessProfileId populates members/sentRequests,
  /// personId populates teams/receivedRequests.
  static Future<TeamMemberPageData> getTeamMemberPage({int? businessProfileId, int? personId}) async {
    try {
      final response = await _ensureClient().getTeamMemberPage(
        $tm.TeamMemberPageRequest(businessProfileId: businessProfileId ?? 0, personId: personId ?? 0),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return TeamMemberPageMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load team members');
    }
  }

  static Future<TeamMember> getTeamMember({required int businessProfileId, required int personId}) async {
    try {
      final response = await _ensureClient().getTeamMember(
        $tm.TeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return TeamMemberMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load team member');
    }
  }

  static Future<TeamMember> sendTeamMemberRequest({required int businessProfileId, required int personId}) async {
    try {
      final response = await _ensureClient().sendTeamMemberRequest(
        $tm.TeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return TeamMemberMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to send team member request');
    }
  }

  static Future<TeamMember> acceptTeamMemberRequest({required int businessProfileId, required int personId}) async {
    try {
      final response = await _ensureClient().acceptTeamMemberRequest(
        $tm.TeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return TeamMemberMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to accept team member request');
    }
  }

  static Future<TeamMember> denyTeamMemberRequest({required int businessProfileId, required int personId}) async {
    try {
      final response = await _ensureClient().denyTeamMemberRequest(
        $tm.TeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return TeamMemberMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to deny team member request');
    }
  }

  static Future<TeamMember> cancelTeamMemberRequest({required int businessProfileId, required int personId}) async {
    try {
      final response = await _ensureClient().cancelTeamMemberRequest(
        $tm.TeamMemberRequest(businessProfileId: businessProfileId, personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return TeamMemberMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to cancel team member request');
    }
  }

  static $tm.TeamMemberServiceClient _ensureClient() {
    if (_client != null) return _client!;
    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $tm.TeamMemberServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  static Future<void> shutdown() async {
    _client = null;
  }
}
