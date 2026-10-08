// This is a generated file - do not edit.
//
// Generated from auth.proto.

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

import 'auth.pb.dart' as $0;

export 'auth.pb.dart';

/// Authentication and session for the mobile client: the gRPC twin of the REST /signup, /login, /refresh,
/// /logout and /auth/profile/... routes (C-010). Signup, Login and Refresh are reachable without an access
/// token and are limited per IP; every other method needs one. Error messages are localized from the
/// `accept-language` metadata, and the stable REST `errorKey` travels in the `error-key` trailer.
@$pb.GrpcServiceName('grpc.auth.AuthService')
class AuthServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AuthServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.AccessToken> signup(
    $0.SignupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$signup, request, options: options);
  }

  $grpc.ResponseFuture<$0.AccessToken> login(
    $0.LoginRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$login, request, options: options);
  }

  $grpc.ResponseFuture<$0.AccessToken> refresh(
    $0.RefreshRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$refresh, request, options: options);
  }

  $grpc.ResponseFuture<$0.LogoutResponse> logout(
    $0.LogoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$logout, request, options: options);
  }

  $grpc.ResponseFuture<$0.AccessToken> activateBusinessProfile(
    $0.ActivateBusinessProfileRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$activateBusinessProfile, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.AccessToken> deactivateBusinessProfile(
    $0.DeactivateBusinessProfileRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deactivateBusinessProfile, request,
        options: options);
  }

  // method descriptors

  static final _$signup = $grpc.ClientMethod<$0.SignupRequest, $0.AccessToken>(
      '/grpc.auth.AuthService/Signup',
      ($0.SignupRequest value) => value.writeToBuffer(),
      $0.AccessToken.fromBuffer);
  static final _$login = $grpc.ClientMethod<$0.LoginRequest, $0.AccessToken>(
      '/grpc.auth.AuthService/Login',
      ($0.LoginRequest value) => value.writeToBuffer(),
      $0.AccessToken.fromBuffer);
  static final _$refresh =
      $grpc.ClientMethod<$0.RefreshRequest, $0.AccessToken>(
          '/grpc.auth.AuthService/Refresh',
          ($0.RefreshRequest value) => value.writeToBuffer(),
          $0.AccessToken.fromBuffer);
  static final _$logout =
      $grpc.ClientMethod<$0.LogoutRequest, $0.LogoutResponse>(
          '/grpc.auth.AuthService/Logout',
          ($0.LogoutRequest value) => value.writeToBuffer(),
          $0.LogoutResponse.fromBuffer);
  static final _$activateBusinessProfile =
      $grpc.ClientMethod<$0.ActivateBusinessProfileRequest, $0.AccessToken>(
          '/grpc.auth.AuthService/ActivateBusinessProfile',
          ($0.ActivateBusinessProfileRequest value) => value.writeToBuffer(),
          $0.AccessToken.fromBuffer);
  static final _$deactivateBusinessProfile =
      $grpc.ClientMethod<$0.DeactivateBusinessProfileRequest, $0.AccessToken>(
          '/grpc.auth.AuthService/DeactivateBusinessProfile',
          ($0.DeactivateBusinessProfileRequest value) => value.writeToBuffer(),
          $0.AccessToken.fromBuffer);
}

@$pb.GrpcServiceName('grpc.auth.AuthService')
abstract class AuthServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.auth.AuthService';

  AuthServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.SignupRequest, $0.AccessToken>(
        'Signup',
        signup_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SignupRequest.fromBuffer(value),
        ($0.AccessToken value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LoginRequest, $0.AccessToken>(
        'Login',
        login_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LoginRequest.fromBuffer(value),
        ($0.AccessToken value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RefreshRequest, $0.AccessToken>(
        'Refresh',
        refresh_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RefreshRequest.fromBuffer(value),
        ($0.AccessToken value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.LogoutRequest, $0.LogoutResponse>(
        'Logout',
        logout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LogoutRequest.fromBuffer(value),
        ($0.LogoutResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ActivateBusinessProfileRequest, $0.AccessToken>(
            'ActivateBusinessProfile',
            activateBusinessProfile_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ActivateBusinessProfileRequest.fromBuffer(value),
            ($0.AccessToken value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeactivateBusinessProfileRequest,
            $0.AccessToken>(
        'DeactivateBusinessProfile',
        deactivateBusinessProfile_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeactivateBusinessProfileRequest.fromBuffer(value),
        ($0.AccessToken value) => value.writeToBuffer()));
  }

  $async.Future<$0.AccessToken> signup_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.SignupRequest> $request) async {
    return signup($call, await $request);
  }

  $async.Future<$0.AccessToken> signup(
      $grpc.ServiceCall call, $0.SignupRequest request);

  $async.Future<$0.AccessToken> login_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.LoginRequest> $request) async {
    return login($call, await $request);
  }

  $async.Future<$0.AccessToken> login(
      $grpc.ServiceCall call, $0.LoginRequest request);

  $async.Future<$0.AccessToken> refresh_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RefreshRequest> $request) async {
    return refresh($call, await $request);
  }

  $async.Future<$0.AccessToken> refresh(
      $grpc.ServiceCall call, $0.RefreshRequest request);

  $async.Future<$0.LogoutResponse> logout_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.LogoutRequest> $request) async {
    return logout($call, await $request);
  }

  $async.Future<$0.LogoutResponse> logout(
      $grpc.ServiceCall call, $0.LogoutRequest request);

  $async.Future<$0.AccessToken> activateBusinessProfile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ActivateBusinessProfileRequest> $request) async {
    return activateBusinessProfile($call, await $request);
  }

  $async.Future<$0.AccessToken> activateBusinessProfile(
      $grpc.ServiceCall call, $0.ActivateBusinessProfileRequest request);

  $async.Future<$0.AccessToken> deactivateBusinessProfile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.DeactivateBusinessProfileRequest> $request) async {
    return deactivateBusinessProfile($call, await $request);
  }

  $async.Future<$0.AccessToken> deactivateBusinessProfile(
      $grpc.ServiceCall call, $0.DeactivateBusinessProfileRequest request);
}
