// This is a generated file - do not edit.
//
// Generated from address.proto.

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

import 'address.pb.dart' as $0;

export 'address.pb.dart';

/// Address autocomplete (C-010): the gRPC twin of the REST /address/search route, backed by the places
/// provider. Needs an access token; the typed text is never logged.
@$pb.GrpcServiceName('grpc.address.AddressSearchService')
class AddressSearchServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  AddressSearchServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.SearchAddressResponse> searchAddress(
    $0.SearchAddressRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$searchAddress, request, options: options);
  }

  // method descriptors

  static final _$searchAddress =
      $grpc.ClientMethod<$0.SearchAddressRequest, $0.SearchAddressResponse>(
          '/grpc.address.AddressSearchService/SearchAddress',
          ($0.SearchAddressRequest value) => value.writeToBuffer(),
          $0.SearchAddressResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.address.AddressSearchService')
abstract class AddressSearchServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.address.AddressSearchService';

  AddressSearchServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.SearchAddressRequest, $0.SearchAddressResponse>(
            'SearchAddress',
            searchAddress_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SearchAddressRequest.fromBuffer(value),
            ($0.SearchAddressResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.SearchAddressResponse> searchAddress_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SearchAddressRequest> $request) async {
    return searchAddress($call, await $request);
  }

  $async.Future<$0.SearchAddressResponse> searchAddress(
      $grpc.ServiceCall call, $0.SearchAddressRequest request);
}
