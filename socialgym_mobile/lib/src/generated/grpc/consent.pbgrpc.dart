// This is a generated file - do not edit.
//
// Generated from consent.proto.

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

import 'consent.pb.dart' as $0;

export 'consent.pb.dart';

/// Consent to the legal documents (terms, privacy, health_data): the gRPC twin of the REST
/// /people/me/consents routes (C-010). Reachable without current Terms and Privacy consent, so a person
/// can accept again after a document changes or revoke consent.
@$pb.GrpcServiceName('grpc.consent.ConsentService')
class ConsentServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ConsentServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListConsentsResponse> listConsents(
    $0.ListConsentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listConsents, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListPendingConsentsResponse> listPendingConsents(
    $0.ListPendingConsentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listPendingConsents, request, options: options);
  }

  $grpc.ResponseFuture<$0.Consent> acceptConsent(
    $0.AcceptConsentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$acceptConsent, request, options: options);
  }

  $grpc.ResponseFuture<$0.RevokeConsentResponse> revokeConsent(
    $0.RevokeConsentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$revokeConsent, request, options: options);
  }

  // method descriptors

  static final _$listConsents =
      $grpc.ClientMethod<$0.ListConsentsRequest, $0.ListConsentsResponse>(
          '/grpc.consent.ConsentService/ListConsents',
          ($0.ListConsentsRequest value) => value.writeToBuffer(),
          $0.ListConsentsResponse.fromBuffer);
  static final _$listPendingConsents = $grpc.ClientMethod<
          $0.ListPendingConsentsRequest, $0.ListPendingConsentsResponse>(
      '/grpc.consent.ConsentService/ListPendingConsents',
      ($0.ListPendingConsentsRequest value) => value.writeToBuffer(),
      $0.ListPendingConsentsResponse.fromBuffer);
  static final _$acceptConsent =
      $grpc.ClientMethod<$0.AcceptConsentRequest, $0.Consent>(
          '/grpc.consent.ConsentService/AcceptConsent',
          ($0.AcceptConsentRequest value) => value.writeToBuffer(),
          $0.Consent.fromBuffer);
  static final _$revokeConsent =
      $grpc.ClientMethod<$0.RevokeConsentRequest, $0.RevokeConsentResponse>(
          '/grpc.consent.ConsentService/RevokeConsent',
          ($0.RevokeConsentRequest value) => value.writeToBuffer(),
          $0.RevokeConsentResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.consent.ConsentService')
abstract class ConsentServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.consent.ConsentService';

  ConsentServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.ListConsentsRequest, $0.ListConsentsResponse>(
            'ListConsents',
            listConsents_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListConsentsRequest.fromBuffer(value),
            ($0.ListConsentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListPendingConsentsRequest,
            $0.ListPendingConsentsResponse>(
        'ListPendingConsents',
        listPendingConsents_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListPendingConsentsRequest.fromBuffer(value),
        ($0.ListPendingConsentsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AcceptConsentRequest, $0.Consent>(
        'AcceptConsent',
        acceptConsent_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AcceptConsentRequest.fromBuffer(value),
        ($0.Consent value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RevokeConsentRequest, $0.RevokeConsentResponse>(
            'RevokeConsent',
            revokeConsent_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.RevokeConsentRequest.fromBuffer(value),
            ($0.RevokeConsentResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListConsentsResponse> listConsents_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListConsentsRequest> $request) async {
    return listConsents($call, await $request);
  }

  $async.Future<$0.ListConsentsResponse> listConsents(
      $grpc.ServiceCall call, $0.ListConsentsRequest request);

  $async.Future<$0.ListPendingConsentsResponse> listPendingConsents_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListPendingConsentsRequest> $request) async {
    return listPendingConsents($call, await $request);
  }

  $async.Future<$0.ListPendingConsentsResponse> listPendingConsents(
      $grpc.ServiceCall call, $0.ListPendingConsentsRequest request);

  $async.Future<$0.Consent> acceptConsent_Pre($grpc.ServiceCall $call,
      $async.Future<$0.AcceptConsentRequest> $request) async {
    return acceptConsent($call, await $request);
  }

  $async.Future<$0.Consent> acceptConsent(
      $grpc.ServiceCall call, $0.AcceptConsentRequest request);

  $async.Future<$0.RevokeConsentResponse> revokeConsent_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RevokeConsentRequest> $request) async {
    return revokeConsent($call, await $request);
  }

  $async.Future<$0.RevokeConsentResponse> revokeConsent(
      $grpc.ServiceCall call, $0.RevokeConsentRequest request);
}
