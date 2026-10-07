// This is a generated file - do not edit.
//
// Generated from timeline/evolution.proto.

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

@$core.Deprecated('Use bodyCompositionDescriptor instead')
const BodyComposition$json = {
  '1': 'BodyComposition',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'weight', '3': 2, '4': 1, '5': 1, '10': 'weight'},
    {'1': 'body_fat_pct', '3': 3, '4': 1, '5': 1, '10': 'bodyFatPct'},
    {'1': 'muscle_mass_pct', '3': 4, '4': 1, '5': 1, '10': 'muscleMassPct'},
    {'1': 'visceral_fat', '3': 5, '4': 1, '5': 1, '10': 'visceralFat'},
  ],
  '8': [
    {'1': '_uuid'},
  ],
};

/// Descriptor for `BodyComposition`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bodyCompositionDescriptor = $convert.base64Decode(
    'Cg9Cb2R5Q29tcG9zaXRpb24SFwoEdXVpZBgBIAEoCUgAUgR1dWlkiAEBEhYKBndlaWdodBgCIA'
    'EoAVIGd2VpZ2h0EiAKDGJvZHlfZmF0X3BjdBgDIAEoAVIKYm9keUZhdFBjdBImCg9tdXNjbGVf'
    'bWFzc19wY3QYBCABKAFSDW11c2NsZU1hc3NQY3QSIQoMdmlzY2VyYWxfZmF0GAUgASgBUgt2aX'
    'NjZXJhbEZhdEIHCgVfdXVpZA==');

@$core.Deprecated('Use circumferencesDescriptor instead')
const Circumferences$json = {
  '1': 'Circumferences',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'neck', '3': 2, '4': 1, '5': 1, '10': 'neck'},
    {'1': 'chest', '3': 3, '4': 1, '5': 1, '10': 'chest'},
    {'1': 'waist', '3': 4, '4': 1, '5': 1, '10': 'waist'},
    {'1': 'abdomen', '3': 5, '4': 1, '5': 1, '10': 'abdomen'},
    {'1': 'hip', '3': 6, '4': 1, '5': 1, '10': 'hip'},
    {'1': 'biceps_right', '3': 7, '4': 1, '5': 1, '10': 'bicepsRight'},
    {'1': 'biceps_left', '3': 8, '4': 1, '5': 1, '10': 'bicepsLeft'},
    {'1': 'thigh_right', '3': 9, '4': 1, '5': 1, '10': 'thighRight'},
    {'1': 'thigh_left', '3': 10, '4': 1, '5': 1, '10': 'thighLeft'},
  ],
  '8': [
    {'1': '_uuid'},
  ],
};

/// Descriptor for `Circumferences`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List circumferencesDescriptor = $convert.base64Decode(
    'Cg5DaXJjdW1mZXJlbmNlcxIXCgR1dWlkGAEgASgJSABSBHV1aWSIAQESEgoEbmVjaxgCIAEoAV'
    'IEbmVjaxIUCgVjaGVzdBgDIAEoAVIFY2hlc3QSFAoFd2Fpc3QYBCABKAFSBXdhaXN0EhgKB2Fi'
    'ZG9tZW4YBSABKAFSB2FiZG9tZW4SEAoDaGlwGAYgASgBUgNoaXASIQoMYmljZXBzX3JpZ2h0GA'
    'cgASgBUgtiaWNlcHNSaWdodBIfCgtiaWNlcHNfbGVmdBgIIAEoAVIKYmljZXBzTGVmdBIfCgt0'
    'aGlnaF9yaWdodBgJIAEoAVIKdGhpZ2hSaWdodBIdCgp0aGlnaF9sZWZ0GAogASgBUgl0aGlnaE'
    'xlZnRCBwoFX3V1aWQ=');

@$core.Deprecated('Use evolutionCheckInDescriptor instead')
const EvolutionCheckIn$json = {
  '1': 'EvolutionCheckIn',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'person_uuid', '3': 2, '4': 1, '5': 9, '10': 'personUuid'},
    {'1': 'created_at', '3': 3, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'note', '3': 4, '4': 1, '5': 9, '9': 1, '10': 'note', '17': true},
    {'1': 'visibility', '3': 5, '4': 1, '5': 9, '10': 'visibility'},
    {
      '1': 'composition',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.BodyComposition',
      '9': 2,
      '10': 'composition',
      '17': true
    },
    {
      '1': 'circumferences',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.Circumferences',
      '9': 3,
      '10': 'circumferences',
      '17': true
    },
  ],
  '8': [
    {'1': '_uuid'},
    {'1': '_note'},
    {'1': '_composition'},
    {'1': '_circumferences'},
  ],
};

/// Descriptor for `EvolutionCheckIn`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List evolutionCheckInDescriptor = $convert.base64Decode(
    'ChBFdm9sdXRpb25DaGVja0luEhcKBHV1aWQYASABKAlIAFIEdXVpZIgBARIfCgtwZXJzb25fdX'
    'VpZBgCIAEoCVIKcGVyc29uVXVpZBIdCgpjcmVhdGVkX2F0GAMgASgJUgljcmVhdGVkQXQSFwoE'
    'bm90ZRgEIAEoCUgBUgRub3RliAEBEh4KCnZpc2liaWxpdHkYBSABKAlSCnZpc2liaWxpdHkSRQ'
    'oLY29tcG9zaXRpb24YBiABKAsyHi5ncnBjLnRpbWVsaW5lLkJvZHlDb21wb3NpdGlvbkgCUgtj'
    'b21wb3NpdGlvbogBARJKCg5jaXJjdW1mZXJlbmNlcxgHIAEoCzIdLmdycGMudGltZWxpbmUuQ2'
    'lyY3VtZmVyZW5jZXNIA1IOY2lyY3VtZmVyZW5jZXOIAQFCBwoFX3V1aWRCBwoFX25vdGVCDgoM'
    'X2NvbXBvc2l0aW9uQhEKD19jaXJjdW1mZXJlbmNlcw==');

@$core.Deprecated('Use addEvolutionCheckInRequestDescriptor instead')
const AddEvolutionCheckInRequest$json = {
  '1': 'AddEvolutionCheckInRequest',
  '2': [
    {'1': 'created_at', '3': 1, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'note', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'note', '17': true},
    {'1': 'visibility', '3': 3, '4': 1, '5': 9, '10': 'visibility'},
    {
      '1': 'composition',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.BodyComposition',
      '9': 1,
      '10': 'composition',
      '17': true
    },
    {
      '1': 'circumferences',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.Circumferences',
      '9': 2,
      '10': 'circumferences',
      '17': true
    },
  ],
  '8': [
    {'1': '_note'},
    {'1': '_composition'},
    {'1': '_circumferences'},
  ],
};

/// Descriptor for `AddEvolutionCheckInRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addEvolutionCheckInRequestDescriptor = $convert.base64Decode(
    'ChpBZGRFdm9sdXRpb25DaGVja0luUmVxdWVzdBIdCgpjcmVhdGVkX2F0GAEgASgJUgljcmVhdG'
    'VkQXQSFwoEbm90ZRgCIAEoCUgAUgRub3RliAEBEh4KCnZpc2liaWxpdHkYAyABKAlSCnZpc2li'
    'aWxpdHkSRQoLY29tcG9zaXRpb24YBCABKAsyHi5ncnBjLnRpbWVsaW5lLkJvZHlDb21wb3NpdG'
    'lvbkgBUgtjb21wb3NpdGlvbogBARJKCg5jaXJjdW1mZXJlbmNlcxgFIAEoCzIdLmdycGMudGlt'
    'ZWxpbmUuQ2lyY3VtZmVyZW5jZXNIAlIOY2lyY3VtZmVyZW5jZXOIAQFCBwoFX25vdGVCDgoMX2'
    'NvbXBvc2l0aW9uQhEKD19jaXJjdW1mZXJlbmNlcw==');

@$core.Deprecated('Use listEvolutionCheckInsRequestDescriptor instead')
const ListEvolutionCheckInsRequest$json = {
  '1': 'ListEvolutionCheckInsRequest',
  '2': [
    {
      '1': 'start_date',
      '3': 1,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'startDate',
      '17': true
    },
    {
      '1': 'end_date',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'endDate',
      '17': true
    },
  ],
  '8': [
    {'1': '_start_date'},
    {'1': '_end_date'},
  ],
};

/// Descriptor for `ListEvolutionCheckInsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEvolutionCheckInsRequestDescriptor =
    $convert.base64Decode(
        'ChxMaXN0RXZvbHV0aW9uQ2hlY2tJbnNSZXF1ZXN0EiIKCnN0YXJ0X2RhdGUYASABKAlIAFIJc3'
        'RhcnREYXRliAEBEh4KCGVuZF9kYXRlGAIgASgJSAFSB2VuZERhdGWIAQFCDQoLX3N0YXJ0X2Rh'
        'dGVCCwoJX2VuZF9kYXRl');

@$core.Deprecated('Use listEvolutionCheckInsResponseDescriptor instead')
const ListEvolutionCheckInsResponse$json = {
  '1': 'ListEvolutionCheckInsResponse',
  '2': [
    {
      '1': 'check_ins',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.EvolutionCheckIn',
      '10': 'checkIns'
    },
  ],
};

/// Descriptor for `ListEvolutionCheckInsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEvolutionCheckInsResponseDescriptor =
    $convert.base64Decode(
        'Ch1MaXN0RXZvbHV0aW9uQ2hlY2tJbnNSZXNwb25zZRI8CgljaGVja19pbnMYASADKAsyHy5ncn'
        'BjLnRpbWVsaW5lLkV2b2x1dGlvbkNoZWNrSW5SCGNoZWNrSW5z');
