// This is a generated file - do not edit.
//
// Generated from team_member.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'team_member.pb.dart' as $0;

export 'team_member.pb.dart';

/// Mirrors the REST endpoints under /workout/api/team-members. gRPC has no ambient active
/// business profile, so both sides of the membership are always passed explicitly.
@$pb.GrpcServiceName('grpc.team_member.TeamMemberService')
class TeamMemberServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  TeamMemberServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.TeamMemberPageResponse> getTeamMemberPage(
    $0.TeamMemberPageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getTeamMemberPage, request, options: options);
  }

  $grpc.ResponseFuture<$0.TeamMember> getTeamMember(
    $0.TeamMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getTeamMember, request, options: options);
  }

  $grpc.ResponseFuture<$0.TeamMember> sendTeamMemberRequest(
    $0.TeamMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$sendTeamMemberRequest, request, options: options);
  }

  $grpc.ResponseFuture<$0.TeamMember> acceptTeamMemberRequest(
    $0.TeamMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$acceptTeamMemberRequest, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.TeamMember> denyTeamMemberRequest(
    $0.TeamMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$denyTeamMemberRequest, request, options: options);
  }

  $grpc.ResponseFuture<$0.TeamMember> cancelTeamMemberRequest(
    $0.TeamMemberRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelTeamMemberRequest, request,
        options: options);
  }

  // method descriptors

  static final _$getTeamMemberPage =
      $grpc.ClientMethod<$0.TeamMemberPageRequest, $0.TeamMemberPageResponse>(
          '/grpc.team_member.TeamMemberService/GetTeamMemberPage',
          ($0.TeamMemberPageRequest value) => value.writeToBuffer(),
          $0.TeamMemberPageResponse.fromBuffer);
  static final _$getTeamMember =
      $grpc.ClientMethod<$0.TeamMemberRequest, $0.TeamMember>(
          '/grpc.team_member.TeamMemberService/GetTeamMember',
          ($0.TeamMemberRequest value) => value.writeToBuffer(),
          $0.TeamMember.fromBuffer);
  static final _$sendTeamMemberRequest =
      $grpc.ClientMethod<$0.TeamMemberRequest, $0.TeamMember>(
          '/grpc.team_member.TeamMemberService/SendTeamMemberRequest',
          ($0.TeamMemberRequest value) => value.writeToBuffer(),
          $0.TeamMember.fromBuffer);
  static final _$acceptTeamMemberRequest =
      $grpc.ClientMethod<$0.TeamMemberRequest, $0.TeamMember>(
          '/grpc.team_member.TeamMemberService/AcceptTeamMemberRequest',
          ($0.TeamMemberRequest value) => value.writeToBuffer(),
          $0.TeamMember.fromBuffer);
  static final _$denyTeamMemberRequest =
      $grpc.ClientMethod<$0.TeamMemberRequest, $0.TeamMember>(
          '/grpc.team_member.TeamMemberService/DenyTeamMemberRequest',
          ($0.TeamMemberRequest value) => value.writeToBuffer(),
          $0.TeamMember.fromBuffer);
  static final _$cancelTeamMemberRequest =
      $grpc.ClientMethod<$0.TeamMemberRequest, $0.TeamMember>(
          '/grpc.team_member.TeamMemberService/CancelTeamMemberRequest',
          ($0.TeamMemberRequest value) => value.writeToBuffer(),
          $0.TeamMember.fromBuffer);
}

@$pb.GrpcServiceName('grpc.team_member.TeamMemberService')
abstract class TeamMemberServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.team_member.TeamMemberService';

  TeamMemberServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.TeamMemberPageRequest,
            $0.TeamMemberPageResponse>(
        'GetTeamMemberPage',
        getTeamMemberPage_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TeamMemberPageRequest.fromBuffer(value),
        ($0.TeamMemberPageResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TeamMemberRequest, $0.TeamMember>(
        'GetTeamMember',
        getTeamMember_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.TeamMemberRequest.fromBuffer(value),
        ($0.TeamMember value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TeamMemberRequest, $0.TeamMember>(
        'SendTeamMemberRequest',
        sendTeamMemberRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.TeamMemberRequest.fromBuffer(value),
        ($0.TeamMember value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TeamMemberRequest, $0.TeamMember>(
        'AcceptTeamMemberRequest',
        acceptTeamMemberRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.TeamMemberRequest.fromBuffer(value),
        ($0.TeamMember value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TeamMemberRequest, $0.TeamMember>(
        'DenyTeamMemberRequest',
        denyTeamMemberRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.TeamMemberRequest.fromBuffer(value),
        ($0.TeamMember value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TeamMemberRequest, $0.TeamMember>(
        'CancelTeamMemberRequest',
        cancelTeamMemberRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.TeamMemberRequest.fromBuffer(value),
        ($0.TeamMember value) => value.writeToBuffer()));
  }

  $async.Future<$0.TeamMemberPageResponse> getTeamMemberPage_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TeamMemberPageRequest> $request) async {
    return getTeamMemberPage($call, await $request);
  }

  $async.Future<$0.TeamMemberPageResponse> getTeamMemberPage(
      $grpc.ServiceCall call, $0.TeamMemberPageRequest request);

  $async.Future<$0.TeamMember> getTeamMember_Pre($grpc.ServiceCall $call,
      $async.Future<$0.TeamMemberRequest> $request) async {
    return getTeamMember($call, await $request);
  }

  $async.Future<$0.TeamMember> getTeamMember(
      $grpc.ServiceCall call, $0.TeamMemberRequest request);

  $async.Future<$0.TeamMember> sendTeamMemberRequest_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TeamMemberRequest> $request) async {
    return sendTeamMemberRequest($call, await $request);
  }

  $async.Future<$0.TeamMember> sendTeamMemberRequest(
      $grpc.ServiceCall call, $0.TeamMemberRequest request);

  $async.Future<$0.TeamMember> acceptTeamMemberRequest_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TeamMemberRequest> $request) async {
    return acceptTeamMemberRequest($call, await $request);
  }

  $async.Future<$0.TeamMember> acceptTeamMemberRequest(
      $grpc.ServiceCall call, $0.TeamMemberRequest request);

  $async.Future<$0.TeamMember> denyTeamMemberRequest_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TeamMemberRequest> $request) async {
    return denyTeamMemberRequest($call, await $request);
  }

  $async.Future<$0.TeamMember> denyTeamMemberRequest(
      $grpc.ServiceCall call, $0.TeamMemberRequest request);

  $async.Future<$0.TeamMember> cancelTeamMemberRequest_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TeamMemberRequest> $request) async {
    return cancelTeamMemberRequest($call, await $request);
  }

  $async.Future<$0.TeamMember> cancelTeamMemberRequest(
      $grpc.ServiceCall call, $0.TeamMemberRequest request);
}
