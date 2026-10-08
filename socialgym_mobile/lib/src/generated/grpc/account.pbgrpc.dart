// This is a generated file - do not edit.
//
// Generated from account.proto.

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

import 'account.pb.dart' as $0;

export 'account.pb.dart';

/// The caller's account lifecycle and data exports: the gRPC twin of the REST
/// /people/me/account/... and /people/me/data-exports... routes (C-010). Reachable without current Terms
/// and Privacy consent, so a person who revoked consent can still export their data or delete (or keep)
/// the account. The export file itself is never carried over gRPC: GetDataExportDownloadUrl returns a
/// short-lived pre-signed link.
@$pb.GrpcServiceName('grpc.account.AccountService')
class AccountServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AccountServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.AccountDeletionStatus> requestAccountDeletion(
    $0.RequestAccountDeletionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$requestAccountDeletion, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.CancelAccountDeletionResponse> cancelAccountDeletion(
    $0.CancelAccountDeletionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelAccountDeletion, request, options: options);
  }

  $grpc.ResponseFuture<$0.DataExport> createDataExport(
    $0.CreateDataExportRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createDataExport, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListDataExportsResponse> listDataExports(
    $0.ListDataExportsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDataExports, request, options: options);
  }

  $grpc.ResponseFuture<$0.DataExport> getDataExport(
    $0.GetDataExportRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDataExport, request, options: options);
  }

  $grpc.ResponseFuture<$0.DataExportDownload> getDataExportDownloadUrl(
    $0.GetDataExportRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getDataExportDownloadUrl, request,
        options: options);
  }

  // method descriptors

  static final _$requestAccountDeletion = $grpc.ClientMethod<
          $0.RequestAccountDeletionRequest, $0.AccountDeletionStatus>(
      '/grpc.account.AccountService/RequestAccountDeletion',
      ($0.RequestAccountDeletionRequest value) => value.writeToBuffer(),
      $0.AccountDeletionStatus.fromBuffer);
  static final _$cancelAccountDeletion = $grpc.ClientMethod<
          $0.CancelAccountDeletionRequest, $0.CancelAccountDeletionResponse>(
      '/grpc.account.AccountService/CancelAccountDeletion',
      ($0.CancelAccountDeletionRequest value) => value.writeToBuffer(),
      $0.CancelAccountDeletionResponse.fromBuffer);
  static final _$createDataExport =
      $grpc.ClientMethod<$0.CreateDataExportRequest, $0.DataExport>(
          '/grpc.account.AccountService/CreateDataExport',
          ($0.CreateDataExportRequest value) => value.writeToBuffer(),
          $0.DataExport.fromBuffer);
  static final _$listDataExports =
      $grpc.ClientMethod<$0.ListDataExportsRequest, $0.ListDataExportsResponse>(
          '/grpc.account.AccountService/ListDataExports',
          ($0.ListDataExportsRequest value) => value.writeToBuffer(),
          $0.ListDataExportsResponse.fromBuffer);
  static final _$getDataExport =
      $grpc.ClientMethod<$0.GetDataExportRequest, $0.DataExport>(
          '/grpc.account.AccountService/GetDataExport',
          ($0.GetDataExportRequest value) => value.writeToBuffer(),
          $0.DataExport.fromBuffer);
  static final _$getDataExportDownloadUrl =
      $grpc.ClientMethod<$0.GetDataExportRequest, $0.DataExportDownload>(
          '/grpc.account.AccountService/GetDataExportDownloadUrl',
          ($0.GetDataExportRequest value) => value.writeToBuffer(),
          $0.DataExportDownload.fromBuffer);
}

@$pb.GrpcServiceName('grpc.account.AccountService')
abstract class AccountServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.account.AccountService';

  AccountServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.RequestAccountDeletionRequest,
            $0.AccountDeletionStatus>(
        'RequestAccountDeletion',
        requestAccountDeletion_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RequestAccountDeletionRequest.fromBuffer(value),
        ($0.AccountDeletionStatus value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CancelAccountDeletionRequest,
            $0.CancelAccountDeletionResponse>(
        'CancelAccountDeletion',
        cancelAccountDeletion_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CancelAccountDeletionRequest.fromBuffer(value),
        ($0.CancelAccountDeletionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateDataExportRequest, $0.DataExport>(
        'CreateDataExport',
        createDataExport_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateDataExportRequest.fromBuffer(value),
        ($0.DataExport value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListDataExportsRequest,
            $0.ListDataExportsResponse>(
        'ListDataExports',
        listDataExports_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListDataExportsRequest.fromBuffer(value),
        ($0.ListDataExportsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetDataExportRequest, $0.DataExport>(
        'GetDataExport',
        getDataExport_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetDataExportRequest.fromBuffer(value),
        ($0.DataExport value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetDataExportRequest, $0.DataExportDownload>(
            'GetDataExportDownloadUrl',
            getDataExportDownloadUrl_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetDataExportRequest.fromBuffer(value),
            ($0.DataExportDownload value) => value.writeToBuffer()));
  }

  $async.Future<$0.AccountDeletionStatus> requestAccountDeletion_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RequestAccountDeletionRequest> $request) async {
    return requestAccountDeletion($call, await $request);
  }

  $async.Future<$0.AccountDeletionStatus> requestAccountDeletion(
      $grpc.ServiceCall call, $0.RequestAccountDeletionRequest request);

  $async.Future<$0.CancelAccountDeletionResponse> cancelAccountDeletion_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CancelAccountDeletionRequest> $request) async {
    return cancelAccountDeletion($call, await $request);
  }

  $async.Future<$0.CancelAccountDeletionResponse> cancelAccountDeletion(
      $grpc.ServiceCall call, $0.CancelAccountDeletionRequest request);

  $async.Future<$0.DataExport> createDataExport_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateDataExportRequest> $request) async {
    return createDataExport($call, await $request);
  }

  $async.Future<$0.DataExport> createDataExport(
      $grpc.ServiceCall call, $0.CreateDataExportRequest request);

  $async.Future<$0.ListDataExportsResponse> listDataExports_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListDataExportsRequest> $request) async {
    return listDataExports($call, await $request);
  }

  $async.Future<$0.ListDataExportsResponse> listDataExports(
      $grpc.ServiceCall call, $0.ListDataExportsRequest request);

  $async.Future<$0.DataExport> getDataExport_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetDataExportRequest> $request) async {
    return getDataExport($call, await $request);
  }

  $async.Future<$0.DataExport> getDataExport(
      $grpc.ServiceCall call, $0.GetDataExportRequest request);

  $async.Future<$0.DataExportDownload> getDataExportDownloadUrl_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetDataExportRequest> $request) async {
    return getDataExportDownloadUrl($call, await $request);
  }

  $async.Future<$0.DataExportDownload> getDataExportDownloadUrl(
      $grpc.ServiceCall call, $0.GetDataExportRequest request);
}
