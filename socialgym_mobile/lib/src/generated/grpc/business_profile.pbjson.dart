// This is a generated file - do not edit.
//
// Generated from business_profile.proto.

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

@$core.Deprecated('Use businessProfileDescriptor instead')
const BusinessProfile$json = {
  '1': 'BusinessProfile',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'owner_id', '3': 3, '4': 1, '5': 5, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 4, '4': 1, '5': 9, '10': 'ownerUuid'},
    {'1': 'tax_id', '3': 5, '4': 1, '5': 9, '10': 'taxId'},
    {'1': 'business_name', '3': 6, '4': 1, '5': 9, '10': 'businessName'},
    {'1': 'business_type', '3': 7, '4': 1, '5': 9, '10': 'businessType'},
    {'1': 'social_name', '3': 8, '4': 1, '5': 9, '10': 'socialName'},
    {'1': 'object_key', '3': 9, '4': 1, '5': 9, '10': 'objectKey'},
    {'1': 'logo', '3': 10, '4': 1, '5': 9, '10': 'logo'},
    {'1': 'cover_image', '3': 11, '4': 1, '5': 9, '10': 'coverImage'},
    {'1': 'created_at', '3': 12, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 13, '4': 1, '5': 9, '10': 'updatedAt'},
    {
      '1': 'addresses',
      '3': 14,
      '4': 3,
      '5': 11,
      '6': '.grpc.business_profile_address.BusinessProfileAddress',
      '10': 'addresses'
    },
  ],
};

/// Descriptor for `BusinessProfile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfileDescriptor = $convert.base64Decode(
    'Cg9CdXNpbmVzc1Byb2ZpbGUSDgoCaWQYASABKAVSAmlkEhIKBHV1aWQYAiABKAlSBHV1aWQSGQ'
    'oIb3duZXJfaWQYAyABKAVSB293bmVySWQSHQoKb3duZXJfdXVpZBgEIAEoCVIJb3duZXJVdWlk'
    'EhUKBnRheF9pZBgFIAEoCVIFdGF4SWQSIwoNYnVzaW5lc3NfbmFtZRgGIAEoCVIMYnVzaW5lc3'
    'NOYW1lEiMKDWJ1c2luZXNzX3R5cGUYByABKAlSDGJ1c2luZXNzVHlwZRIfCgtzb2NpYWxfbmFt'
    'ZRgIIAEoCVIKc29jaWFsTmFtZRIdCgpvYmplY3Rfa2V5GAkgASgJUglvYmplY3RLZXkSEgoEbG'
    '9nbxgKIAEoCVIEbG9nbxIfCgtjb3Zlcl9pbWFnZRgLIAEoCVIKY292ZXJJbWFnZRIdCgpjcmVh'
    'dGVkX2F0GAwgASgJUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgNIAEoCVIJdXBkYXRlZEF0El'
    'MKCWFkZHJlc3NlcxgOIAMoCzI1LmdycGMuYnVzaW5lc3NfcHJvZmlsZV9hZGRyZXNzLkJ1c2lu'
    'ZXNzUHJvZmlsZUFkZHJlc3NSCWFkZHJlc3Nlcw==');

@$core.Deprecated('Use businessProfileRequestIdDescriptor instead')
const BusinessProfileRequestId$json = {
  '1': 'BusinessProfileRequestId',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
  ],
};

/// Descriptor for `BusinessProfileRequestId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfileRequestIdDescriptor =
    $convert.base64Decode(
        'ChhCdXNpbmVzc1Byb2ZpbGVSZXF1ZXN0SWQSDgoCaWQYASABKAVSAmlkEhIKBHV1aWQYAiABKA'
        'lSBHV1aWQ=');

@$core.Deprecated('Use businessProfileRequestOwnerIdDescriptor instead')
const BusinessProfileRequestOwnerId$json = {
  '1': 'BusinessProfileRequestOwnerId',
  '2': [
    {'1': 'owner_id', '3': 1, '4': 1, '5': 5, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 2, '4': 1, '5': 9, '10': 'ownerUuid'},
  ],
};

/// Descriptor for `BusinessProfileRequestOwnerId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfileRequestOwnerIdDescriptor =
    $convert.base64Decode(
        'Ch1CdXNpbmVzc1Byb2ZpbGVSZXF1ZXN0T3duZXJJZBIZCghvd25lcl9pZBgBIAEoBVIHb3duZX'
        'JJZBIdCgpvd25lcl91dWlkGAIgASgJUglvd25lclV1aWQ=');

@$core.Deprecated('Use businessProfilesResponseDescriptor instead')
const BusinessProfilesResponse$json = {
  '1': 'BusinessProfilesResponse',
  '2': [
    {
      '1': 'business_profiles',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.business_profile.BusinessProfile',
      '10': 'businessProfiles'
    },
  ],
};

/// Descriptor for `BusinessProfilesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfilesResponseDescriptor = $convert.base64Decode(
    'ChhCdXNpbmVzc1Byb2ZpbGVzUmVzcG9uc2USUwoRYnVzaW5lc3NfcHJvZmlsZXMYASADKAsyJi'
    '5ncnBjLmJ1c2luZXNzX3Byb2ZpbGUuQnVzaW5lc3NQcm9maWxlUhBidXNpbmVzc1Byb2ZpbGVz');

@$core.Deprecated('Use removeBusinessProfileAddressRequestDescriptor instead')
const RemoveBusinessProfileAddressRequest$json = {
  '1': 'RemoveBusinessProfileAddressRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
  ],
};

/// Descriptor for `RemoveBusinessProfileAddressRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeBusinessProfileAddressRequestDescriptor =
    $convert.base64Decode(
        'CiNSZW1vdmVCdXNpbmVzc1Byb2ZpbGVBZGRyZXNzUmVxdWVzdBIOCgJpZBgBIAEoBVICaWQSEg'
        'oEdXVpZBgCIAEoCVIEdXVpZA==');

@$core.Deprecated('Use removeBusinessProfileAddressResponseDescriptor instead')
const RemoveBusinessProfileAddressResponse$json = {
  '1': 'RemoveBusinessProfileAddressResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `RemoveBusinessProfileAddressResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeBusinessProfileAddressResponseDescriptor =
    $convert.base64Decode(
        'CiRSZW1vdmVCdXNpbmVzc1Byb2ZpbGVBZGRyZXNzUmVzcG9uc2USGAoHc3VjY2VzcxgBIAEoCF'
        'IHc3VjY2Vzcw==');
