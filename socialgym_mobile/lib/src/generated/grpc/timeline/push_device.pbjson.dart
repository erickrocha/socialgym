// This is a generated file - do not edit.
//
// Generated from timeline/push_device.proto.

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

@$core.Deprecated('Use registerPushDeviceRequestDescriptor instead')
const RegisterPushDeviceRequest$json = {
  '1': 'RegisterPushDeviceRequest',
  '2': [
    {'1': 'device_uuid', '3': 1, '4': 1, '5': 9, '10': 'deviceUuid'},
    {'1': 'platform', '3': 2, '4': 1, '5': 9, '10': 'platform'},
    {
      '1': 'registration_token',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'registrationToken'
    },
  ],
};

/// Descriptor for `RegisterPushDeviceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerPushDeviceRequestDescriptor = $convert.base64Decode(
    'ChlSZWdpc3RlclB1c2hEZXZpY2VSZXF1ZXN0Eh8KC2RldmljZV91dWlkGAEgASgJUgpkZXZpY2'
    'VVdWlkEhoKCHBsYXRmb3JtGAIgASgJUghwbGF0Zm9ybRItChJyZWdpc3RyYXRpb25fdG9rZW4Y'
    'AyABKAlSEXJlZ2lzdHJhdGlvblRva2Vu');

@$core.Deprecated('Use registerPushDeviceResponseDescriptor instead')
const RegisterPushDeviceResponse$json = {
  '1': 'RegisterPushDeviceResponse',
};

/// Descriptor for `RegisterPushDeviceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerPushDeviceResponseDescriptor =
    $convert.base64Decode('ChpSZWdpc3RlclB1c2hEZXZpY2VSZXNwb25zZQ==');

@$core.Deprecated('Use removePushDeviceRequestDescriptor instead')
const RemovePushDeviceRequest$json = {
  '1': 'RemovePushDeviceRequest',
  '2': [
    {'1': 'device_uuid', '3': 1, '4': 1, '5': 9, '10': 'deviceUuid'},
  ],
};

/// Descriptor for `RemovePushDeviceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removePushDeviceRequestDescriptor =
    $convert.base64Decode(
        'ChdSZW1vdmVQdXNoRGV2aWNlUmVxdWVzdBIfCgtkZXZpY2VfdXVpZBgBIAEoCVIKZGV2aWNlVX'
        'VpZA==');

@$core.Deprecated('Use removePushDeviceResponseDescriptor instead')
const RemovePushDeviceResponse$json = {
  '1': 'RemovePushDeviceResponse',
};

/// Descriptor for `RemovePushDeviceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removePushDeviceResponseDescriptor =
    $convert.base64Decode('ChhSZW1vdmVQdXNoRGV2aWNlUmVzcG9uc2U=');
