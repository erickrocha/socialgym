// This is a generated file - do not edit.
//
// Generated from media.proto.

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

@$core.Deprecated('Use mediaUploadRequestDescriptor instead')
const MediaUploadRequest$json = {
  '1': 'MediaUploadRequest',
  '2': [
    {'1': 'album', '3': 1, '4': 1, '5': 9, '10': 'album'},
    {'1': 'format', '3': 2, '4': 1, '5': 9, '10': 'format'},
  ],
};

/// Descriptor for `MediaUploadRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mediaUploadRequestDescriptor = $convert.base64Decode(
    'ChJNZWRpYVVwbG9hZFJlcXVlc3QSFAoFYWxidW0YASABKAlSBWFsYnVtEhYKBmZvcm1hdBgCIA'
    'EoCVIGZm9ybWF0');

@$core.Deprecated('Use mediaUploadResponseDescriptor instead')
const MediaUploadResponse$json = {
  '1': 'MediaUploadResponse',
  '2': [
    {'1': 'url', '3': 1, '4': 1, '5': 9, '10': 'url'},
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '10': 'objectKey'},
    {'1': 'person_id', '3': 3, '4': 1, '5': 5, '10': 'personId'},
  ],
};

/// Descriptor for `MediaUploadResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mediaUploadResponseDescriptor = $convert.base64Decode(
    'ChNNZWRpYVVwbG9hZFJlc3BvbnNlEhAKA3VybBgBIAEoCVIDdXJsEh0KCm9iamVjdF9rZXkYAi'
    'ABKAlSCW9iamVjdEtleRIbCglwZXJzb25faWQYAyABKAVSCHBlcnNvbklk');
