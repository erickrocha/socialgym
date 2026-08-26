// This is a generated file - do not edit.
//
// Generated from resource.proto.

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

@$core.Deprecated('Use resourceRequestDescriptor instead')
const ResourceRequest$json = {
  '1': 'ResourceRequest',
  '2': [
    {'1': 'userId', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'userId'},
    {'1': 'ownerUuid', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'ownerUuid'},
  ],
  '8': [
    {'1': 'identifier'},
  ],
};

/// Descriptor for `ResourceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resourceRequestDescriptor = $convert.base64Decode(
    'Cg9SZXNvdXJjZVJlcXVlc3QSGAoGdXNlcklkGAEgASgFSABSBnVzZXJJZBIeCglvd25lclV1aW'
    'QYAiABKAlIAFIJb3duZXJVdWlkQgwKCmlkZW50aWZpZXI=');

@$core.Deprecated('Use resourceResponseDescriptor instead')
const ResourceResponse$json = {
  '1': 'ResourceResponse',
  '2': [
    {
      '1': 'countries',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.country.Country',
      '10': 'countries'
    },
    {
      '1': 'setting',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.grpc.settings.Setting',
      '10': 'setting'
    },
    {
      '1': 'provinces',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.grpc.province.Province',
      '10': 'provinces'
    },
  ],
};

/// Descriptor for `ResourceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resourceResponseDescriptor = $convert.base64Decode(
    'ChBSZXNvdXJjZVJlc3BvbnNlEjMKCWNvdW50cmllcxgBIAMoCzIVLmdycGMuY291bnRyeS5Db3'
    'VudHJ5Ugljb3VudHJpZXMSMAoHc2V0dGluZxgCIAEoCzIWLmdycGMuc2V0dGluZ3MuU2V0dGlu'
    'Z1IHc2V0dGluZxI1Cglwcm92aW5jZXMYAyADKAsyFy5ncnBjLnByb3ZpbmNlLlByb3ZpbmNlUg'
    'lwcm92aW5jZXM=');
