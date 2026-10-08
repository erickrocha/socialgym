// This is a generated file - do not edit.
//
// Generated from account.proto.

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

@$core.Deprecated('Use requestAccountDeletionRequestDescriptor instead')
const RequestAccountDeletionRequest$json = {
  '1': 'RequestAccountDeletionRequest',
  '2': [
    {'1': 'immediate', '3': 1, '4': 1, '5': 8, '10': 'immediate'},
  ],
};

/// Descriptor for `RequestAccountDeletionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestAccountDeletionRequestDescriptor =
    $convert.base64Decode(
        'Ch1SZXF1ZXN0QWNjb3VudERlbGV0aW9uUmVxdWVzdBIcCglpbW1lZGlhdGUYASABKAhSCWltbW'
        'VkaWF0ZQ==');

@$core.Deprecated('Use accountDeletionStatusDescriptor instead')
const AccountDeletionStatus$json = {
  '1': 'AccountDeletionStatus',
  '2': [
    {'1': 'requested_at', '3': 1, '4': 1, '5': 9, '10': 'requestedAt'},
    {'1': 'scheduled_at', '3': 2, '4': 1, '5': 9, '10': 'scheduledAt'},
  ],
};

/// Descriptor for `AccountDeletionStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountDeletionStatusDescriptor = $convert.base64Decode(
    'ChVBY2NvdW50RGVsZXRpb25TdGF0dXMSIQoMcmVxdWVzdGVkX2F0GAEgASgJUgtyZXF1ZXN0ZW'
    'RBdBIhCgxzY2hlZHVsZWRfYXQYAiABKAlSC3NjaGVkdWxlZEF0');

@$core.Deprecated('Use cancelAccountDeletionRequestDescriptor instead')
const CancelAccountDeletionRequest$json = {
  '1': 'CancelAccountDeletionRequest',
};

/// Descriptor for `CancelAccountDeletionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cancelAccountDeletionRequestDescriptor =
    $convert.base64Decode('ChxDYW5jZWxBY2NvdW50RGVsZXRpb25SZXF1ZXN0');

@$core.Deprecated('Use cancelAccountDeletionResponseDescriptor instead')
const CancelAccountDeletionResponse$json = {
  '1': 'CancelAccountDeletionResponse',
};

/// Descriptor for `CancelAccountDeletionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cancelAccountDeletionResponseDescriptor =
    $convert.base64Decode('Ch1DYW5jZWxBY2NvdW50RGVsZXRpb25SZXNwb25zZQ==');

@$core.Deprecated('Use createDataExportRequestDescriptor instead')
const CreateDataExportRequest$json = {
  '1': 'CreateDataExportRequest',
};

/// Descriptor for `CreateDataExportRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDataExportRequestDescriptor =
    $convert.base64Decode('ChdDcmVhdGVEYXRhRXhwb3J0UmVxdWVzdA==');

@$core.Deprecated('Use listDataExportsRequestDescriptor instead')
const ListDataExportsRequest$json = {
  '1': 'ListDataExportsRequest',
};

/// Descriptor for `ListDataExportsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDataExportsRequestDescriptor =
    $convert.base64Decode('ChZMaXN0RGF0YUV4cG9ydHNSZXF1ZXN0');

@$core.Deprecated('Use listDataExportsResponseDescriptor instead')
const ListDataExportsResponse$json = {
  '1': 'ListDataExportsResponse',
  '2': [
    {
      '1': 'exports',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.account.DataExport',
      '10': 'exports'
    },
  ],
};

/// Descriptor for `ListDataExportsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDataExportsResponseDescriptor =
    $convert.base64Decode(
        'ChdMaXN0RGF0YUV4cG9ydHNSZXNwb25zZRIyCgdleHBvcnRzGAEgAygLMhguZ3JwYy5hY2NvdW'
        '50LkRhdGFFeHBvcnRSB2V4cG9ydHM=');

@$core.Deprecated('Use getDataExportRequestDescriptor instead')
const GetDataExportRequest$json = {
  '1': 'GetDataExportRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `GetDataExportRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDataExportRequestDescriptor = $convert
    .base64Decode('ChRHZXREYXRhRXhwb3J0UmVxdWVzdBIOCgJpZBgBIAEoCVICaWQ=');

@$core.Deprecated('Use dataExportDescriptor instead')
const DataExport$json = {
  '1': 'DataExport',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'status', '3': 2, '4': 1, '5': 9, '10': 'status'},
    {'1': 'error', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'error', '17': true},
    {'1': 'created_at', '3': 4, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 5, '4': 1, '5': 9, '10': 'updatedAt'},
    {
      '1': 'expires_at',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'expiresAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_error'},
    {'1': '_expires_at'},
  ],
};

/// Descriptor for `DataExport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataExportDescriptor = $convert.base64Decode(
    'CgpEYXRhRXhwb3J0Eg4KAmlkGAEgASgJUgJpZBIWCgZzdGF0dXMYAiABKAlSBnN0YXR1cxIZCg'
    'VlcnJvchgDIAEoCUgAUgVlcnJvcogBARIdCgpjcmVhdGVkX2F0GAQgASgJUgljcmVhdGVkQXQS'
    'HQoKdXBkYXRlZF9hdBgFIAEoCVIJdXBkYXRlZEF0EiIKCmV4cGlyZXNfYXQYBiABKAlIAVIJZX'
    'hwaXJlc0F0iAEBQggKBl9lcnJvckINCgtfZXhwaXJlc19hdA==');

@$core.Deprecated('Use dataExportDownloadDescriptor instead')
const DataExportDownload$json = {
  '1': 'DataExportDownload',
  '2': [
    {'1': 'url', '3': 1, '4': 1, '5': 9, '10': 'url'},
    {
      '1': 'expires_in_seconds',
      '3': 2,
      '4': 1,
      '5': 13,
      '10': 'expiresInSeconds'
    },
  ],
};

/// Descriptor for `DataExportDownload`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataExportDownloadDescriptor = $convert.base64Decode(
    'ChJEYXRhRXhwb3J0RG93bmxvYWQSEAoDdXJsGAEgASgJUgN1cmwSLAoSZXhwaXJlc19pbl9zZW'
    'NvbmRzGAIgASgNUhBleHBpcmVzSW5TZWNvbmRz');
