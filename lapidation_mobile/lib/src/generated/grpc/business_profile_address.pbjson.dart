// This is a generated file - do not edit.
//
// Generated from business_profile_address.proto.

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

@$core.Deprecated('Use businessProfileAddressDescriptor instead')
const BusinessProfileAddress$json = {
  '1': 'BusinessProfileAddress',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {
      '1': 'business_profile_id',
      '3': 3,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
    {'1': 'address_line_1', '3': 4, '4': 1, '5': 9, '10': 'addressLine1'},
    {'1': 'address_line_2', '3': 5, '4': 1, '5': 9, '10': 'addressLine2'},
    {'1': 'locality', '3': 6, '4': 1, '5': 9, '10': 'locality'},
    {
      '1': 'administrative_area',
      '3': 7,
      '4': 1,
      '5': 9,
      '10': 'administrativeArea'
    },
    {'1': 'postal_code', '3': 8, '4': 1, '5': 9, '10': 'postalCode'},
    {'1': 'country_code', '3': 9, '4': 1, '5': 9, '10': 'countryCode'},
    {'1': 'latitude', '3': 10, '4': 1, '5': 1, '10': 'latitude'},
    {'1': 'longitude', '3': 11, '4': 1, '5': 1, '10': 'longitude'},
    {'1': 'created_at', '3': 12, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 13, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
};

/// Descriptor for `BusinessProfileAddress`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfileAddressDescriptor = $convert.base64Decode(
    'ChZCdXNpbmVzc1Byb2ZpbGVBZGRyZXNzEg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUg'
    'R1dWlkEi4KE2J1c2luZXNzX3Byb2ZpbGVfaWQYAyABKAVSEWJ1c2luZXNzUHJvZmlsZUlkEiQK'
    'DmFkZHJlc3NfbGluZV8xGAQgASgJUgxhZGRyZXNzTGluZTESJAoOYWRkcmVzc19saW5lXzIYBS'
    'ABKAlSDGFkZHJlc3NMaW5lMhIaCghsb2NhbGl0eRgGIAEoCVIIbG9jYWxpdHkSLwoTYWRtaW5p'
    'c3RyYXRpdmVfYXJlYRgHIAEoCVISYWRtaW5pc3RyYXRpdmVBcmVhEh8KC3Bvc3RhbF9jb2RlGA'
    'ggASgJUgpwb3N0YWxDb2RlEiEKDGNvdW50cnlfY29kZRgJIAEoCVILY291bnRyeUNvZGUSGgoI'
    'bGF0aXR1ZGUYCiABKAFSCGxhdGl0dWRlEhwKCWxvbmdpdHVkZRgLIAEoAVIJbG9uZ2l0dWRlEh'
    '0KCmNyZWF0ZWRfYXQYDCABKAlSCWNyZWF0ZWRBdBIdCgp1cGRhdGVkX2F0GA0gASgJUgl1cGRh'
    'dGVkQXQ=');
