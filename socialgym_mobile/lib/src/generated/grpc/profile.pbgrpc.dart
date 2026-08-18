// This is a generated file - do not edit.
//
// Generated from profile.proto.

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

import 'profile.pb.dart' as $0;

export 'profile.pb.dart';

@$pb.GrpcServiceName('grpc.profile.ProfileService')
class ProfileServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ProfileServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ProfileResponse> getProfiles(
    $0.ProfileRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getProfiles, request, options: options);
  }

  $grpc.ResponseFuture<$0.ProfileResponse> getProfilesByPerson(
    $0.ProfileRequestByPersonId request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getProfilesByPerson, request, options: options);
  }

  // method descriptors

  static final _$getProfiles =
      $grpc.ClientMethod<$0.ProfileRequest, $0.ProfileResponse>(
          '/grpc.profile.ProfileService/GetProfiles',
          ($0.ProfileRequest value) => value.writeToBuffer(),
          $0.ProfileResponse.fromBuffer);
  static final _$getProfilesByPerson =
      $grpc.ClientMethod<$0.ProfileRequestByPersonId, $0.ProfileResponse>(
          '/grpc.profile.ProfileService/GetProfilesByPerson',
          ($0.ProfileRequestByPersonId value) => value.writeToBuffer(),
          $0.ProfileResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.profile.ProfileService')
abstract class ProfileServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.profile.ProfileService';

  ProfileServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ProfileRequest, $0.ProfileResponse>(
        'GetProfiles',
        getProfiles_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ProfileRequest.fromBuffer(value),
        ($0.ProfileResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ProfileRequestByPersonId, $0.ProfileResponse>(
            'GetProfilesByPerson',
            getProfilesByPerson_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ProfileRequestByPersonId.fromBuffer(value),
            ($0.ProfileResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ProfileResponse> getProfiles_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ProfileRequest> $request) async {
    return getProfiles($call, await $request);
  }

  $async.Future<$0.ProfileResponse> getProfiles(
      $grpc.ServiceCall call, $0.ProfileRequest request);

  $async.Future<$0.ProfileResponse> getProfilesByPerson_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ProfileRequestByPersonId> $request) async {
    return getProfilesByPerson($call, await $request);
  }

  $async.Future<$0.ProfileResponse> getProfilesByPerson(
      $grpc.ServiceCall call, $0.ProfileRequestByPersonId request);
}
