// This is a generated file - do not edit.
//
// Generated from timeline/content_report.proto.

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

import 'content_report.pb.dart' as $0;

export 'content_report.pb.dart';

/// Mirrors /timeline/api/reports and /timeline/api/moderation. Listing and deciding need the
/// moderator role (PERMISSION_DENIED otherwise, also when the role check itself fails).
@$pb.GrpcServiceName('grpc.timeline.ContentReportService')
class ContentReportServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ContentReportServiceClient(super.channel,
      {super.options, super.interceptors});

  /// Needs permission to read the post; the target must belong to it. Anything else is NOT_FOUND.
  $grpc.ResponseFuture<$0.ContentReport> createReport(
    $0.CreateReportRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createReport, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListReportsResponse> listReports(
    $0.ListReportsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listReports, request, options: options);
  }

  $grpc.ResponseFuture<$0.ContentReport> decideReport(
    $0.DecideReportRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$decideReport, request, options: options);
  }

  // method descriptors

  static final _$createReport =
      $grpc.ClientMethod<$0.CreateReportRequest, $0.ContentReport>(
          '/grpc.timeline.ContentReportService/CreateReport',
          ($0.CreateReportRequest value) => value.writeToBuffer(),
          $0.ContentReport.fromBuffer);
  static final _$listReports =
      $grpc.ClientMethod<$0.ListReportsRequest, $0.ListReportsResponse>(
          '/grpc.timeline.ContentReportService/ListReports',
          ($0.ListReportsRequest value) => value.writeToBuffer(),
          $0.ListReportsResponse.fromBuffer);
  static final _$decideReport =
      $grpc.ClientMethod<$0.DecideReportRequest, $0.ContentReport>(
          '/grpc.timeline.ContentReportService/DecideReport',
          ($0.DecideReportRequest value) => value.writeToBuffer(),
          $0.ContentReport.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.ContentReportService')
abstract class ContentReportServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.ContentReportService';

  ContentReportServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.CreateReportRequest, $0.ContentReport>(
        'CreateReport',
        createReport_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateReportRequest.fromBuffer(value),
        ($0.ContentReport value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListReportsRequest, $0.ListReportsResponse>(
            'ListReports',
            listReports_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListReportsRequest.fromBuffer(value),
            ($0.ListReportsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DecideReportRequest, $0.ContentReport>(
        'DecideReport',
        decideReport_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DecideReportRequest.fromBuffer(value),
        ($0.ContentReport value) => value.writeToBuffer()));
  }

  $async.Future<$0.ContentReport> createReport_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateReportRequest> $request) async {
    return createReport($call, await $request);
  }

  $async.Future<$0.ContentReport> createReport(
      $grpc.ServiceCall call, $0.CreateReportRequest request);

  $async.Future<$0.ListReportsResponse> listReports_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ListReportsRequest> $request) async {
    return listReports($call, await $request);
  }

  $async.Future<$0.ListReportsResponse> listReports(
      $grpc.ServiceCall call, $0.ListReportsRequest request);

  $async.Future<$0.ContentReport> decideReport_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DecideReportRequest> $request) async {
    return decideReport($call, await $request);
  }

  $async.Future<$0.ContentReport> decideReport(
      $grpc.ServiceCall call, $0.DecideReportRequest request);
}
