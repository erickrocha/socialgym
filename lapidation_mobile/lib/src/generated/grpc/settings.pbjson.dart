// This is a generated file - do not edit.
//
// Generated from settings.proto.

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

@$core.Deprecated('Use settingIdRequestDescriptor instead')
const SettingIdRequest$json = {
  '1': 'SettingIdRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
  ],
};

/// Descriptor for `SettingIdRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingIdRequestDescriptor = $convert.base64Decode(
    'ChBTZXR0aW5nSWRSZXF1ZXN0Eg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUgR1dWlk');

@$core.Deprecated('Use settingOwnerIdRequestDescriptor instead')
const SettingOwnerIdRequest$json = {
  '1': 'SettingOwnerIdRequest',
  '2': [
    {'1': 'owner_id', '3': 1, '4': 1, '5': 5, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 2, '4': 1, '5': 9, '10': 'ownerUuid'},
  ],
};

/// Descriptor for `SettingOwnerIdRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingOwnerIdRequestDescriptor = $convert.base64Decode(
    'ChVTZXR0aW5nT3duZXJJZFJlcXVlc3QSGQoIb3duZXJfaWQYASABKAVSB293bmVySWQSHQoKb3'
    'duZXJfdXVpZBgCIAEoCVIJb3duZXJVdWlk');

@$core.Deprecated('Use settingsResponseDescriptor instead')
const SettingsResponse$json = {
  '1': 'SettingsResponse',
  '2': [
    {
      '1': 'settings',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.settings.Setting',
      '10': 'settings'
    },
  ],
};

/// Descriptor for `SettingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsResponseDescriptor = $convert.base64Decode(
    'ChBTZXR0aW5nc1Jlc3BvbnNlEjIKCHNldHRpbmdzGAEgAygLMhYuZ3JwYy5zZXR0aW5ncy5TZX'
    'R0aW5nUghzZXR0aW5ncw==');

@$core.Deprecated('Use settingDescriptor instead')
const Setting$json = {
  '1': 'Setting',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'owner_id', '3': 3, '4': 1, '5': 5, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 4, '4': 1, '5': 9, '10': 'ownerUuid'},
    {'1': 'language', '3': 5, '4': 1, '5': 9, '10': 'language'},
    {'1': 'theme', '3': 6, '4': 1, '5': 9, '10': 'theme'},
    {
      '1': 'notifications_enabled',
      '3': 7,
      '4': 1,
      '5': 8,
      '10': 'notificationsEnabled'
    },
    {
      '1': 'context_menu_position',
      '3': 8,
      '4': 1,
      '5': 9,
      '10': 'contextMenuPosition'
    },
    {'1': 'home_page', '3': 9, '4': 1, '5': 9, '10': 'homePage'},
    {'1': 'created_at', '3': 10, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 11, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
};

/// Descriptor for `Setting`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingDescriptor = $convert.base64Decode(
    'CgdTZXR0aW5nEg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUgR1dWlkEhkKCG93bmVyX2'
    'lkGAMgASgFUgdvd25lcklkEh0KCm93bmVyX3V1aWQYBCABKAlSCW93bmVyVXVpZBIaCghsYW5n'
    'dWFnZRgFIAEoCVIIbGFuZ3VhZ2USFAoFdGhlbWUYBiABKAlSBXRoZW1lEjMKFW5vdGlmaWNhdG'
    'lvbnNfZW5hYmxlZBgHIAEoCFIUbm90aWZpY2F0aW9uc0VuYWJsZWQSMgoVY29udGV4dF9tZW51'
    'X3Bvc2l0aW9uGAggASgJUhNjb250ZXh0TWVudVBvc2l0aW9uEhsKCWhvbWVfcGFnZRgJIAEoCV'
    'IIaG9tZVBhZ2USHQoKY3JlYXRlZF9hdBgKIAEoCVIJY3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQY'
    'CyABKAlSCXVwZGF0ZWRBdA==');
