// This is a generated file - do not edit.
//
// Generated from person_address.proto.

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

@$core.Deprecated('Use personAddressDescriptor instead')
const PersonAddress$json = {
  '1': 'PersonAddress',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'person_id', '3': 3, '4': 1, '5': 5, '10': 'personId'},
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
    {'1': 'current', '3': 12, '4': 1, '5': 8, '10': 'current'},
    {'1': 'created_at', '3': 13, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 14, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
};

/// Descriptor for `PersonAddress`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personAddressDescriptor = $convert.base64Decode(
    'Cg1QZXJzb25BZGRyZXNzEg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUgR1dWlkEhsKCX'
    'BlcnNvbl9pZBgDIAEoBVIIcGVyc29uSWQSJAoOYWRkcmVzc19saW5lXzEYBCABKAlSDGFkZHJl'
    'c3NMaW5lMRIkCg5hZGRyZXNzX2xpbmVfMhgFIAEoCVIMYWRkcmVzc0xpbmUyEhoKCGxvY2FsaX'
    'R5GAYgASgJUghsb2NhbGl0eRIvChNhZG1pbmlzdHJhdGl2ZV9hcmVhGAcgASgJUhJhZG1pbmlz'
    'dHJhdGl2ZUFyZWESHwoLcG9zdGFsX2NvZGUYCCABKAlSCnBvc3RhbENvZGUSIQoMY291bnRyeV'
    '9jb2RlGAkgASgJUgtjb3VudHJ5Q29kZRIaCghsYXRpdHVkZRgKIAEoAVIIbGF0aXR1ZGUSHAoJ'
    'bG9uZ2l0dWRlGAsgASgBUglsb25naXR1ZGUSGAoHY3VycmVudBgMIAEoCFIHY3VycmVudBIdCg'
    'pjcmVhdGVkX2F0GA0gASgJUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgOIAEoCVIJdXBkYXRl'
    'ZEF0');
