// This is a generated file - do not edit.
//
// Generated from legal.proto.

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

import 'legal.pb.dart' as $0;

export 'legal.pb.dart';

/// The text of the legal documents (terms, privacy, health_data): the gRPC twin of the REST
/// /legal/documents routes (C-010). Public (no access token), limited per address like the REST routes.
@$pb.GrpcServiceName('grpc.legal.LegalDocumentService')
class LegalDocumentServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  LegalDocumentServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListLegalDocumentsResponse> listLegalDocuments(
    $0.ListLegalDocumentsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listLegalDocuments, request, options: options);
  }

  $grpc.ResponseFuture<$0.LegalDocument> getLegalDocument(
    $0.GetLegalDocumentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getLegalDocument, request, options: options);
  }

  // method descriptors

  static final _$listLegalDocuments = $grpc.ClientMethod<
          $0.ListLegalDocumentsRequest, $0.ListLegalDocumentsResponse>(
      '/grpc.legal.LegalDocumentService/ListLegalDocuments',
      ($0.ListLegalDocumentsRequest value) => value.writeToBuffer(),
      $0.ListLegalDocumentsResponse.fromBuffer);
  static final _$getLegalDocument =
      $grpc.ClientMethod<$0.GetLegalDocumentRequest, $0.LegalDocument>(
          '/grpc.legal.LegalDocumentService/GetLegalDocument',
          ($0.GetLegalDocumentRequest value) => value.writeToBuffer(),
          $0.LegalDocument.fromBuffer);
}

@$pb.GrpcServiceName('grpc.legal.LegalDocumentService')
abstract class LegalDocumentServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.legal.LegalDocumentService';

  LegalDocumentServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListLegalDocumentsRequest,
            $0.ListLegalDocumentsResponse>(
        'ListLegalDocuments',
        listLegalDocuments_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListLegalDocumentsRequest.fromBuffer(value),
        ($0.ListLegalDocumentsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetLegalDocumentRequest, $0.LegalDocument>(
            'GetLegalDocument',
            getLegalDocument_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetLegalDocumentRequest.fromBuffer(value),
            ($0.LegalDocument value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListLegalDocumentsResponse> listLegalDocuments_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListLegalDocumentsRequest> $request) async {
    return listLegalDocuments($call, await $request);
  }

  $async.Future<$0.ListLegalDocumentsResponse> listLegalDocuments(
      $grpc.ServiceCall call, $0.ListLegalDocumentsRequest request);

  $async.Future<$0.LegalDocument> getLegalDocument_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetLegalDocumentRequest> $request) async {
    return getLegalDocument($call, await $request);
  }

  $async.Future<$0.LegalDocument> getLegalDocument(
      $grpc.ServiceCall call, $0.GetLegalDocumentRequest request);
}
