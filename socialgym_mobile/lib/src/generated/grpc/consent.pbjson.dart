// This is a generated file - do not edit.
//
// Generated from consent.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use listConsentsRequestDescriptor instead')
const ListConsentsRequest$json = {
  '1': 'ListConsentsRequest',
};

/// Descriptor for `ListConsentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listConsentsRequestDescriptor =
    $convert.base64Decode('ChNMaXN0Q29uc2VudHNSZXF1ZXN0');

@$core.Deprecated('Use listConsentsResponseDescriptor instead')
const ListConsentsResponse$json = {
  '1': 'ListConsentsResponse',
  '2': [
    {
      '1': 'consents',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.consent.Consent',
      '10': 'consents'
    },
  ],
};

/// Descriptor for `ListConsentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listConsentsResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0Q29uc2VudHNSZXNwb25zZRIxCghjb25zZW50cxgBIAMoCzIVLmdycGMuY29uc2VudC'
    '5Db25zZW50Ughjb25zZW50cw==');

@$core.Deprecated('Use listPendingConsentsRequestDescriptor instead')
const ListPendingConsentsRequest$json = {
  '1': 'ListPendingConsentsRequest',
};

/// Descriptor for `ListPendingConsentsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPendingConsentsRequestDescriptor =
    $convert.base64Decode('ChpMaXN0UGVuZGluZ0NvbnNlbnRzUmVxdWVzdA==');

@$core.Deprecated('Use listPendingConsentsResponseDescriptor instead')
const ListPendingConsentsResponse$json = {
  '1': 'ListPendingConsentsResponse',
  '2': [
    {
      '1': 'pending',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.consent.PendingConsent',
      '10': 'pending'
    },
  ],
};

/// Descriptor for `ListPendingConsentsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listPendingConsentsResponseDescriptor =
    $convert.base64Decode(
        'ChtMaXN0UGVuZGluZ0NvbnNlbnRzUmVzcG9uc2USNgoHcGVuZGluZxgBIAMoCzIcLmdycGMuY2'
        '9uc2VudC5QZW5kaW5nQ29uc2VudFIHcGVuZGluZw==');

@$core.Deprecated('Use acceptConsentRequestDescriptor instead')
const AcceptConsentRequest$json = {
  '1': 'AcceptConsentRequest',
  '2': [
    {'1': 'document', '3': 1, '4': 1, '5': 9, '10': 'document'},
    {'1': 'version', '3': 2, '4': 1, '5': 9, '10': 'version'},
    {'1': 'accepted', '3': 3, '4': 1, '5': 8, '10': 'accepted'},
  ],
};

/// Descriptor for `AcceptConsentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List acceptConsentRequestDescriptor = $convert.base64Decode(
    'ChRBY2NlcHRDb25zZW50UmVxdWVzdBIaCghkb2N1bWVudBgBIAEoCVIIZG9jdW1lbnQSGAoHdm'
    'Vyc2lvbhgCIAEoCVIHdmVyc2lvbhIaCghhY2NlcHRlZBgDIAEoCFIIYWNjZXB0ZWQ=');

@$core.Deprecated('Use revokeConsentRequestDescriptor instead')
const RevokeConsentRequest$json = {
  '1': 'RevokeConsentRequest',
  '2': [
    {'1': 'document', '3': 1, '4': 1, '5': 9, '10': 'document'},
  ],
};

/// Descriptor for `RevokeConsentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List revokeConsentRequestDescriptor =
    $convert.base64Decode(
        'ChRSZXZva2VDb25zZW50UmVxdWVzdBIaCghkb2N1bWVudBgBIAEoCVIIZG9jdW1lbnQ=');

@$core.Deprecated('Use revokeConsentResponseDescriptor instead')
const RevokeConsentResponse$json = {
  '1': 'RevokeConsentResponse',
};

/// Descriptor for `RevokeConsentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List revokeConsentResponseDescriptor =
    $convert.base64Decode('ChVSZXZva2VDb25zZW50UmVzcG9uc2U=');

@$core.Deprecated('Use consentDescriptor instead')
const Consent$json = {
  '1': 'Consent',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'document', '3': 2, '4': 1, '5': 9, '10': 'document'},
    {'1': 'version', '3': 3, '4': 1, '5': 9, '10': 'version'},
    {'1': 'accepted_at', '3': 4, '4': 1, '5': 9, '10': 'acceptedAt'},
    {
      '1': 'revoked_at',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'revokedAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_revoked_at'},
  ],
};

/// Descriptor for `Consent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List consentDescriptor = $convert.base64Decode(
    'CgdDb25zZW50Eg4KAmlkGAEgASgFUgJpZBIaCghkb2N1bWVudBgCIAEoCVIIZG9jdW1lbnQSGA'
    'oHdmVyc2lvbhgDIAEoCVIHdmVyc2lvbhIfCgthY2NlcHRlZF9hdBgEIAEoCVIKYWNjZXB0ZWRB'
    'dBIiCgpyZXZva2VkX2F0GAUgASgJSABSCXJldm9rZWRBdIgBAUINCgtfcmV2b2tlZF9hdA==');

@$core.Deprecated('Use pendingConsentDescriptor instead')
const PendingConsent$json = {
  '1': 'PendingConsent',
  '2': [
    {'1': 'document', '3': 1, '4': 1, '5': 9, '10': 'document'},
    {'1': 'current_version', '3': 2, '4': 1, '5': 9, '10': 'currentVersion'},
    {
      '1': 'accepted_version',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'acceptedVersion',
      '17': true
    },
  ],
  '8': [
    {'1': '_accepted_version'},
  ],
};

/// Descriptor for `PendingConsent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pendingConsentDescriptor = $convert.base64Decode(
    'Cg5QZW5kaW5nQ29uc2VudBIaCghkb2N1bWVudBgBIAEoCVIIZG9jdW1lbnQSJwoPY3VycmVudF'
    '92ZXJzaW9uGAIgASgJUg5jdXJyZW50VmVyc2lvbhIuChBhY2NlcHRlZF92ZXJzaW9uGAMgASgJ'
    'SABSD2FjY2VwdGVkVmVyc2lvbogBAUITChFfYWNjZXB0ZWRfdmVyc2lvbg==');
