// This is a generated file - do not edit.
//
// Generated from address.proto.

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

@$core.Deprecated('Use searchAddressRequestDescriptor instead')
const SearchAddressRequest$json = {
  '1': 'SearchAddressRequest',
  '2': [
    {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
    {
      '1': 'latitude',
      '3': 2,
      '4': 1,
      '5': 1,
      '9': 0,
      '10': 'latitude',
      '17': true
    },
    {
      '1': 'longitude',
      '3': 3,
      '4': 1,
      '5': 1,
      '9': 1,
      '10': 'longitude',
      '17': true
    },
  ],
  '8': [
    {'1': '_latitude'},
    {'1': '_longitude'},
  ],
};

/// Descriptor for `SearchAddressRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchAddressRequestDescriptor = $convert.base64Decode(
    'ChRTZWFyY2hBZGRyZXNzUmVxdWVzdBISCgR0ZXh0GAEgASgJUgR0ZXh0Eh8KCGxhdGl0dWRlGA'
    'IgASgBSABSCGxhdGl0dWRliAEBEiEKCWxvbmdpdHVkZRgDIAEoAUgBUglsb25naXR1ZGWIAQFC'
    'CwoJX2xhdGl0dWRlQgwKCl9sb25naXR1ZGU=');

@$core.Deprecated('Use searchAddressResponseDescriptor instead')
const SearchAddressResponse$json = {
  '1': 'SearchAddressResponse',
  '2': [
    {
      '1': 'candidates',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.address.AddressCandidate',
      '10': 'candidates'
    },
  ],
};

/// Descriptor for `SearchAddressResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchAddressResponseDescriptor = $convert.base64Decode(
    'ChVTZWFyY2hBZGRyZXNzUmVzcG9uc2USPgoKY2FuZGlkYXRlcxgBIAMoCzIeLmdycGMuYWRkcm'
    'Vzcy5BZGRyZXNzQ2FuZGlkYXRlUgpjYW5kaWRhdGVz');

@$core.Deprecated('Use addressCandidateDescriptor instead')
const AddressCandidate$json = {
  '1': 'AddressCandidate',
  '2': [
    {'1': 'place_id', '3': 1, '4': 1, '5': 9, '10': 'placeId'},
    {
      '1': 'formatted_address',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'formattedAddress'
    },
    {'1': 'address_line_1', '3': 3, '4': 1, '5': 9, '10': 'addressLine1'},
    {
      '1': 'address_line_2',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'addressLine2',
      '17': true
    },
    {'1': 'locality', '3': 5, '4': 1, '5': 9, '10': 'locality'},
    {
      '1': 'administrative_area',
      '3': 6,
      '4': 1,
      '5': 9,
      '10': 'administrativeArea'
    },
    {
      '1': 'administrative_area_code',
      '3': 7,
      '4': 1,
      '5': 9,
      '10': 'administrativeAreaCode'
    },
    {
      '1': 'postal_code',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'postalCode',
      '17': true
    },
    {'1': 'country_code', '3': 9, '4': 1, '5': 9, '10': 'countryCode'},
    {'1': 'latitude', '3': 10, '4': 1, '5': 1, '10': 'latitude'},
    {'1': 'longitude', '3': 11, '4': 1, '5': 1, '10': 'longitude'},
  ],
  '8': [
    {'1': '_address_line_2'},
    {'1': '_postal_code'},
  ],
};

/// Descriptor for `AddressCandidate`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addressCandidateDescriptor = $convert.base64Decode(
    'ChBBZGRyZXNzQ2FuZGlkYXRlEhkKCHBsYWNlX2lkGAEgASgJUgdwbGFjZUlkEisKEWZvcm1hdH'
    'RlZF9hZGRyZXNzGAIgASgJUhBmb3JtYXR0ZWRBZGRyZXNzEiQKDmFkZHJlc3NfbGluZV8xGAMg'
    'ASgJUgxhZGRyZXNzTGluZTESKQoOYWRkcmVzc19saW5lXzIYBCABKAlIAFIMYWRkcmVzc0xpbm'
    'UyiAEBEhoKCGxvY2FsaXR5GAUgASgJUghsb2NhbGl0eRIvChNhZG1pbmlzdHJhdGl2ZV9hcmVh'
    'GAYgASgJUhJhZG1pbmlzdHJhdGl2ZUFyZWESOAoYYWRtaW5pc3RyYXRpdmVfYXJlYV9jb2RlGA'
    'cgASgJUhZhZG1pbmlzdHJhdGl2ZUFyZWFDb2RlEiQKC3Bvc3RhbF9jb2RlGAggASgJSAFSCnBv'
    'c3RhbENvZGWIAQESIQoMY291bnRyeV9jb2RlGAkgASgJUgtjb3VudHJ5Q29kZRIaCghsYXRpdH'
    'VkZRgKIAEoAVIIbGF0aXR1ZGUSHAoJbG9uZ2l0dWRlGAsgASgBUglsb25naXR1ZGVCEQoPX2Fk'
    'ZHJlc3NfbGluZV8yQg4KDF9wb3N0YWxfY29kZQ==');
