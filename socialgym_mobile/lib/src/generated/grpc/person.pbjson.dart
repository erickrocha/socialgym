// This is a generated file - do not edit.
//
// Generated from person.proto.

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

@$core.Deprecated('Use getMeRequestDescriptor instead')
const GetMeRequest$json = {
  '1': 'GetMeRequest',
};

/// Descriptor for `GetMeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getMeRequestDescriptor =
    $convert.base64Decode('CgxHZXRNZVJlcXVlc3Q=');

@$core.Deprecated('Use removePersonAddressRequestDescriptor instead')
const RemovePersonAddressRequest$json = {
  '1': 'RemovePersonAddressRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
  ],
};

/// Descriptor for `RemovePersonAddressRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removePersonAddressRequestDescriptor =
    $convert.base64Decode(
        'ChpSZW1vdmVQZXJzb25BZGRyZXNzUmVxdWVzdBIOCgJpZBgBIAEoBVICaWQSEgoEdXVpZBgCIA'
        'EoCVIEdXVpZA==');

@$core.Deprecated('Use removePersonAddressResponseDescriptor instead')
const RemovePersonAddressResponse$json = {
  '1': 'RemovePersonAddressResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `RemovePersonAddressResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removePersonAddressResponseDescriptor =
    $convert.base64Decode(
        'ChtSZW1vdmVQZXJzb25BZGRyZXNzUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCFIHc3VjY2Vzcw'
        '==');

@$core.Deprecated('Use personImageUploadRequestDescriptor instead')
const PersonImageUploadRequest$json = {
  '1': 'PersonImageUploadRequest',
  '2': [
    {'1': 'image_type', '3': 1, '4': 1, '5': 9, '10': 'imageType'},
    {'1': 'format', '3': 2, '4': 1, '5': 9, '10': 'format'},
  ],
};

/// Descriptor for `PersonImageUploadRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personImageUploadRequestDescriptor =
    $convert.base64Decode(
        'ChhQZXJzb25JbWFnZVVwbG9hZFJlcXVlc3QSHQoKaW1hZ2VfdHlwZRgBIAEoCVIJaW1hZ2VUeX'
        'BlEhYKBmZvcm1hdBgCIAEoCVIGZm9ybWF0');

@$core.Deprecated('Use personImageUploadResponseDescriptor instead')
const PersonImageUploadResponse$json = {
  '1': 'PersonImageUploadResponse',
  '2': [
    {'1': 'url', '3': 1, '4': 1, '5': 9, '10': 'url'},
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '10': 'objectKey'},
    {'1': 'person_id', '3': 3, '4': 1, '5': 5, '10': 'personId'},
  ],
};

/// Descriptor for `PersonImageUploadResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personImageUploadResponseDescriptor =
    $convert.base64Decode(
        'ChlQZXJzb25JbWFnZVVwbG9hZFJlc3BvbnNlEhAKA3VybBgBIAEoCVIDdXJsEh0KCm9iamVjdF'
        '9rZXkYAiABKAlSCW9iamVjdEtleRIbCglwZXJzb25faWQYAyABKAVSCHBlcnNvbklk');

@$core.Deprecated('Use personImageRequestDescriptor instead')
const PersonImageRequest$json = {
  '1': 'PersonImageRequest',
  '2': [
    {'1': 'image_type', '3': 1, '4': 1, '5': 9, '10': 'imageType'},
  ],
};

/// Descriptor for `PersonImageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personImageRequestDescriptor =
    $convert.base64Decode(
        'ChJQZXJzb25JbWFnZVJlcXVlc3QSHQoKaW1hZ2VfdHlwZRgBIAEoCVIJaW1hZ2VUeXBl');

@$core.Deprecated('Use deletePersonImageResponseDescriptor instead')
const DeletePersonImageResponse$json = {
  '1': 'DeletePersonImageResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `DeletePersonImageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deletePersonImageResponseDescriptor =
    $convert.base64Decode(
        'ChlEZWxldGVQZXJzb25JbWFnZVJlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3M=');

@$core.Deprecated('Use personParamsDescriptor instead')
const PersonParams$json = {
  '1': 'PersonParams',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'uuid'},
    {'1': 'query', '3': 3, '4': 1, '5': 9, '10': 'query'},
    {'1': 'limit', '3': 4, '4': 1, '5': 5, '10': 'limit'},
  ],
  '8': [
    {'1': 'ParamIdentifier'},
  ],
};

/// Descriptor for `PersonParams`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personParamsDescriptor = $convert.base64Decode(
    'CgxQZXJzb25QYXJhbXMSEAoCaWQYASABKAVIAFICaWQSFAoEdXVpZBgCIAEoCUgAUgR1dWlkEh'
    'QKBXF1ZXJ5GAMgASgJUgVxdWVyeRIUCgVsaW1pdBgEIAEoBVIFbGltaXRCEQoPUGFyYW1JZGVu'
    'dGlmaWVy');

@$core.Deprecated('Use personIdRequestDescriptor instead')
const PersonIdRequest$json = {
  '1': 'PersonIdRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'uuid'},
  ],
  '8': [
    {'1': 'identifier'},
  ],
};

/// Descriptor for `PersonIdRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personIdRequestDescriptor = $convert.base64Decode(
    'Cg9QZXJzb25JZFJlcXVlc3QSEAoCaWQYASABKAVIAFICaWQSFAoEdXVpZBgCIAEoCUgAUgR1dW'
    'lkQgwKCmlkZW50aWZpZXI=');

@$core.Deprecated('Use searchMentionableFriendsRequestDescriptor instead')
const SearchMentionableFriendsRequest$json = {
  '1': 'SearchMentionableFriendsRequest',
  '2': [
    {'1': 'person_id', '3': 1, '4': 1, '5': 5, '10': 'personId'},
    {'1': 'query', '3': 2, '4': 1, '5': 9, '10': 'query'},
    {'1': 'limit', '3': 3, '4': 1, '5': 5, '10': 'limit'},
  ],
};

/// Descriptor for `SearchMentionableFriendsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchMentionableFriendsRequestDescriptor =
    $convert.base64Decode(
        'Ch9TZWFyY2hNZW50aW9uYWJsZUZyaWVuZHNSZXF1ZXN0EhsKCXBlcnNvbl9pZBgBIAEoBVIIcG'
        'Vyc29uSWQSFAoFcXVlcnkYAiABKAlSBXF1ZXJ5EhQKBWxpbWl0GAMgASgFUgVsaW1pdA==');

@$core.Deprecated('Use peopleResponseDescriptor instead')
const PeopleResponse$json = {
  '1': 'PeopleResponse',
  '2': [
    {
      '1': 'people',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'people'
    },
  ],
};

/// Descriptor for `PeopleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List peopleResponseDescriptor = $convert.base64Decode(
    'Cg5QZW9wbGVSZXNwb25zZRIrCgZwZW9wbGUYASADKAsyEy5ncnBjLnBlcnNvbi5QZXJzb25SBn'
    'Blb3BsZQ==');

@$core.Deprecated('Use personResponseDescriptor instead')
const PersonResponse$json = {
  '1': 'PersonResponse',
  '2': [
    {
      '1': 'person',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'person'
    },
  ],
};

/// Descriptor for `PersonResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personResponseDescriptor = $convert.base64Decode(
    'Cg5QZXJzb25SZXNwb25zZRIrCgZwZXJzb24YASABKAsyEy5ncnBjLnBlcnNvbi5QZXJzb25SBn'
    'BlcnNvbg==');

@$core.Deprecated('Use personDescriptor instead')
const Person$json = {
  '1': 'Person',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'firstname', '3': 3, '4': 1, '5': 9, '10': 'firstname'},
    {'1': 'surname', '3': 4, '4': 1, '5': 9, '10': 'surname'},
    {'1': 'date_of_birth', '3': 5, '4': 1, '5': 9, '10': 'dateOfBirth'},
    {'1': 'gender', '3': 6, '4': 1, '5': 9, '10': 'gender'},
    {'1': 'object_key', '3': 7, '4': 1, '5': 9, '10': 'objectKey'},
    {'1': 'avatar', '3': 8, '4': 1, '5': 9, '10': 'avatar'},
    {'1': 'cover', '3': 9, '4': 1, '5': 9, '10': 'cover'},
    {
      '1': 'user',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.grpc.user.User',
      '10': 'user'
    },
    {
      '1': 'person_info',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.grpc.person_info.PersonInfo',
      '10': 'personInfo'
    },
    {
      '1': 'addresses',
      '3': 12,
      '4': 3,
      '5': 11,
      '6': '.grpc.person_address.PersonAddress',
      '10': 'addresses'
    },
    {'1': 'created_at', '3': 13, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 14, '4': 1, '5': 9, '10': 'updatedAt'},
    {
      '1': 'business_profiles',
      '3': 15,
      '4': 3,
      '5': 11,
      '6': '.grpc.business_profile.BusinessProfile',
      '10': 'businessProfiles'
    },
  ],
};

/// Descriptor for `Person`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personDescriptor = $convert.base64Decode(
    'CgZQZXJzb24SDgoCaWQYASABKAVSAmlkEhIKBHV1aWQYAiABKAlSBHV1aWQSHAoJZmlyc3RuYW'
    '1lGAMgASgJUglmaXJzdG5hbWUSGAoHc3VybmFtZRgEIAEoCVIHc3VybmFtZRIiCg1kYXRlX29m'
    'X2JpcnRoGAUgASgJUgtkYXRlT2ZCaXJ0aBIWCgZnZW5kZXIYBiABKAlSBmdlbmRlchIdCgpvYm'
    'plY3Rfa2V5GAcgASgJUglvYmplY3RLZXkSFgoGYXZhdGFyGAggASgJUgZhdmF0YXISFAoFY292'
    'ZXIYCSABKAlSBWNvdmVyEiMKBHVzZXIYCiABKAsyDy5ncnBjLnVzZXIuVXNlclIEdXNlchI9Cg'
    'twZXJzb25faW5mbxgLIAEoCzIcLmdycGMucGVyc29uX2luZm8uUGVyc29uSW5mb1IKcGVyc29u'
    'SW5mbxJACglhZGRyZXNzZXMYDCADKAsyIi5ncnBjLnBlcnNvbl9hZGRyZXNzLlBlcnNvbkFkZH'
    'Jlc3NSCWFkZHJlc3NlcxIdCgpjcmVhdGVkX2F0GA0gASgJUgljcmVhdGVkQXQSHQoKdXBkYXRl'
    'ZF9hdBgOIAEoCVIJdXBkYXRlZEF0ElMKEWJ1c2luZXNzX3Byb2ZpbGVzGA8gAygLMiYuZ3JwYy'
    '5idXNpbmVzc19wcm9maWxlLkJ1c2luZXNzUHJvZmlsZVIQYnVzaW5lc3NQcm9maWxlcw==');
