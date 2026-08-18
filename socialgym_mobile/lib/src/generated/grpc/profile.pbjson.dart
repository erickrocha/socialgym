// This is a generated file - do not edit.
//
// Generated from profile.proto.

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

@$core.Deprecated('Use profileRequestDescriptor instead')
const ProfileRequest$json = {
  '1': 'ProfileRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
  ],
};

/// Descriptor for `ProfileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileRequestDescriptor = $convert.base64Decode(
    'Cg5Qcm9maWxlUmVxdWVzdBIOCgJpZBgBIAEoBVICaWQSEgoEdXVpZBgCIAEoCVIEdXVpZA==');

@$core.Deprecated('Use profileRequestByPersonIdDescriptor instead')
const ProfileRequestByPersonId$json = {
  '1': 'ProfileRequestByPersonId',
  '2': [
    {'1': 'person_id', '3': 1, '4': 1, '5': 5, '10': 'personId'},
    {'1': 'person_uuid', '3': 2, '4': 1, '5': 9, '10': 'personUuid'},
  ],
};

/// Descriptor for `ProfileRequestByPersonId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileRequestByPersonIdDescriptor =
    $convert.base64Decode(
        'ChhQcm9maWxlUmVxdWVzdEJ5UGVyc29uSWQSGwoJcGVyc29uX2lkGAEgASgFUghwZXJzb25JZB'
        'IfCgtwZXJzb25fdXVpZBgCIAEoCVIKcGVyc29uVXVpZA==');

@$core.Deprecated('Use profileResponseDescriptor instead')
const ProfileResponse$json = {
  '1': 'ProfileResponse',
  '2': [
    {
      '1': 'profiles',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.profile.Profile',
      '10': 'profiles'
    },
  ],
};

/// Descriptor for `ProfileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileResponseDescriptor = $convert.base64Decode(
    'Cg9Qcm9maWxlUmVzcG9uc2USMQoIcHJvZmlsZXMYASADKAsyFS5ncnBjLnByb2ZpbGUuUHJvZm'
    'lsZVIIcHJvZmlsZXM=');

@$core.Deprecated('Use profileDescriptor instead')
const Profile$json = {
  '1': 'Profile',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'person_id', '3': 3, '4': 1, '5': 5, '10': 'personId'},
    {'1': 'person_uuid', '3': 4, '4': 1, '5': 9, '10': 'personUuid'},
    {
      '1': 'business_profile_id',
      '3': 5,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
    {
      '1': 'business_profile_uuid',
      '3': 6,
      '4': 1,
      '5': 9,
      '10': 'businessProfileUuid'
    },
    {'1': 'created_at', '3': 7, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 8, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
};

/// Descriptor for `Profile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileDescriptor = $convert.base64Decode(
    'CgdQcm9maWxlEg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUgR1dWlkEhsKCXBlcnNvbl'
    '9pZBgDIAEoBVIIcGVyc29uSWQSHwoLcGVyc29uX3V1aWQYBCABKAlSCnBlcnNvblV1aWQSLgoT'
    'YnVzaW5lc3NfcHJvZmlsZV9pZBgFIAEoBVIRYnVzaW5lc3NQcm9maWxlSWQSMgoVYnVzaW5lc3'
    'NfcHJvZmlsZV91dWlkGAYgASgJUhNidXNpbmVzc1Byb2ZpbGVVdWlkEh0KCmNyZWF0ZWRfYXQY'
    'ByABKAlSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GAggASgJUgl1cGRhdGVkQXQ=');
