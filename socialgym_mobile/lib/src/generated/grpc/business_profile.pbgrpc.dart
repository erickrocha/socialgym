// This is a generated file - do not edit.
//
// Generated from business_profile.proto.

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

import 'business_profile.pb.dart' as $0;
import 'business_profile_address.pb.dart' as $1;

export 'business_profile.pb.dart';

@$pb.GrpcServiceName('grpc.business_profile.BusinessProfileService')
class BusinessProfileServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  BusinessProfileServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.BusinessProfile> getBusinessProfileById(
    $0.BusinessProfileRequestId request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getBusinessProfileById, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.BusinessProfilesResponse> getBusinessProfileByOwnerId(
    $0.BusinessProfileRequestOwnerId request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getBusinessProfileByOwnerId, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.BusinessProfile> addBusinessProfile(
    $0.BusinessProfile request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addBusinessProfile, request, options: options);
  }

  $grpc.ResponseFuture<$0.BusinessProfile> updateBusinessProfile(
    $0.BusinessProfile request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateBusinessProfile, request, options: options);
  }

  $grpc.ResponseFuture<$1.BusinessProfileAddress> addBusinessProfileAddress(
    $1.BusinessProfileAddress request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addBusinessProfileAddress, request,
        options: options);
  }

  $grpc.ResponseFuture<$1.BusinessProfileAddress> updateBusinessProfileAddress(
    $1.BusinessProfileAddress request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateBusinessProfileAddress, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.RemoveBusinessProfileAddressResponse>
      removeBusinessProfileAddress(
    $0.RemoveBusinessProfileAddressRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeBusinessProfileAddress, request,
        options: options);
  }

  // method descriptors

  static final _$getBusinessProfileById = $grpc.ClientMethod<
          $0.BusinessProfileRequestId, $0.BusinessProfile>(
      '/grpc.business_profile.BusinessProfileService/GetBusinessProfileById',
      ($0.BusinessProfileRequestId value) => value.writeToBuffer(),
      $0.BusinessProfile.fromBuffer);
  static final _$getBusinessProfileByOwnerId = $grpc.ClientMethod<
          $0.BusinessProfileRequestOwnerId, $0.BusinessProfilesResponse>(
      '/grpc.business_profile.BusinessProfileService/GetBusinessProfileByOwnerId',
      ($0.BusinessProfileRequestOwnerId value) => value.writeToBuffer(),
      $0.BusinessProfilesResponse.fromBuffer);
  static final _$addBusinessProfile =
      $grpc.ClientMethod<$0.BusinessProfile, $0.BusinessProfile>(
          '/grpc.business_profile.BusinessProfileService/AddBusinessProfile',
          ($0.BusinessProfile value) => value.writeToBuffer(),
          $0.BusinessProfile.fromBuffer);
  static final _$updateBusinessProfile =
      $grpc.ClientMethod<$0.BusinessProfile, $0.BusinessProfile>(
          '/grpc.business_profile.BusinessProfileService/UpdateBusinessProfile',
          ($0.BusinessProfile value) => value.writeToBuffer(),
          $0.BusinessProfile.fromBuffer);
  static final _$addBusinessProfileAddress = $grpc.ClientMethod<
          $1.BusinessProfileAddress, $1.BusinessProfileAddress>(
      '/grpc.business_profile.BusinessProfileService/AddBusinessProfileAddress',
      ($1.BusinessProfileAddress value) => value.writeToBuffer(),
      $1.BusinessProfileAddress.fromBuffer);
  static final _$updateBusinessProfileAddress = $grpc.ClientMethod<
          $1.BusinessProfileAddress, $1.BusinessProfileAddress>(
      '/grpc.business_profile.BusinessProfileService/UpdateBusinessProfileAddress',
      ($1.BusinessProfileAddress value) => value.writeToBuffer(),
      $1.BusinessProfileAddress.fromBuffer);
  static final _$removeBusinessProfileAddress = $grpc.ClientMethod<
          $0.RemoveBusinessProfileAddressRequest,
          $0.RemoveBusinessProfileAddressResponse>(
      '/grpc.business_profile.BusinessProfileService/RemoveBusinessProfileAddress',
      ($0.RemoveBusinessProfileAddressRequest value) => value.writeToBuffer(),
      $0.RemoveBusinessProfileAddressResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.business_profile.BusinessProfileService')
abstract class BusinessProfileServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.business_profile.BusinessProfileService';

  BusinessProfileServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.BusinessProfileRequestId, $0.BusinessProfile>(
            'GetBusinessProfileById',
            getBusinessProfileById_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.BusinessProfileRequestId.fromBuffer(value),
            ($0.BusinessProfile value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BusinessProfileRequestOwnerId,
            $0.BusinessProfilesResponse>(
        'GetBusinessProfileByOwnerId',
        getBusinessProfileByOwnerId_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.BusinessProfileRequestOwnerId.fromBuffer(value),
        ($0.BusinessProfilesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BusinessProfile, $0.BusinessProfile>(
        'AddBusinessProfile',
        addBusinessProfile_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.BusinessProfile.fromBuffer(value),
        ($0.BusinessProfile value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.BusinessProfile, $0.BusinessProfile>(
        'UpdateBusinessProfile',
        updateBusinessProfile_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.BusinessProfile.fromBuffer(value),
        ($0.BusinessProfile value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.BusinessProfileAddress,
            $1.BusinessProfileAddress>(
        'AddBusinessProfileAddress',
        addBusinessProfileAddress_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.BusinessProfileAddress.fromBuffer(value),
        ($1.BusinessProfileAddress value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.BusinessProfileAddress,
            $1.BusinessProfileAddress>(
        'UpdateBusinessProfileAddress',
        updateBusinessProfileAddress_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.BusinessProfileAddress.fromBuffer(value),
        ($1.BusinessProfileAddress value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoveBusinessProfileAddressRequest,
            $0.RemoveBusinessProfileAddressResponse>(
        'RemoveBusinessProfileAddress',
        removeBusinessProfileAddress_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoveBusinessProfileAddressRequest.fromBuffer(value),
        ($0.RemoveBusinessProfileAddressResponse value) =>
            value.writeToBuffer()));
  }

  $async.Future<$0.BusinessProfile> getBusinessProfileById_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BusinessProfileRequestId> $request) async {
    return getBusinessProfileById($call, await $request);
  }

  $async.Future<$0.BusinessProfile> getBusinessProfileById(
      $grpc.ServiceCall call, $0.BusinessProfileRequestId request);

  $async.Future<$0.BusinessProfilesResponse> getBusinessProfileByOwnerId_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BusinessProfileRequestOwnerId> $request) async {
    return getBusinessProfileByOwnerId($call, await $request);
  }

  $async.Future<$0.BusinessProfilesResponse> getBusinessProfileByOwnerId(
      $grpc.ServiceCall call, $0.BusinessProfileRequestOwnerId request);

  $async.Future<$0.BusinessProfile> addBusinessProfile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BusinessProfile> $request) async {
    return addBusinessProfile($call, await $request);
  }

  $async.Future<$0.BusinessProfile> addBusinessProfile(
      $grpc.ServiceCall call, $0.BusinessProfile request);

  $async.Future<$0.BusinessProfile> updateBusinessProfile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.BusinessProfile> $request) async {
    return updateBusinessProfile($call, await $request);
  }

  $async.Future<$0.BusinessProfile> updateBusinessProfile(
      $grpc.ServiceCall call, $0.BusinessProfile request);

  $async.Future<$1.BusinessProfileAddress> addBusinessProfileAddress_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$1.BusinessProfileAddress> $request) async {
    return addBusinessProfileAddress($call, await $request);
  }

  $async.Future<$1.BusinessProfileAddress> addBusinessProfileAddress(
      $grpc.ServiceCall call, $1.BusinessProfileAddress request);

  $async.Future<$1.BusinessProfileAddress> updateBusinessProfileAddress_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$1.BusinessProfileAddress> $request) async {
    return updateBusinessProfileAddress($call, await $request);
  }

  $async.Future<$1.BusinessProfileAddress> updateBusinessProfileAddress(
      $grpc.ServiceCall call, $1.BusinessProfileAddress request);

  $async.Future<$0.RemoveBusinessProfileAddressResponse>
      removeBusinessProfileAddress_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.RemoveBusinessProfileAddressRequest>
              $request) async {
    return removeBusinessProfileAddress($call, await $request);
  }

  $async.Future<$0.RemoveBusinessProfileAddressResponse>
      removeBusinessProfileAddress($grpc.ServiceCall call,
          $0.RemoveBusinessProfileAddressRequest request);
}
