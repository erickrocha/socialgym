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

@$core.Deprecated('Use businessProfileImageUploadRequestDescriptor instead')
const BusinessProfileImageUploadRequest$json = {
  '1': 'BusinessProfileImageUploadRequest',
  '2': [
    {'1': 'image_type', '3': 1, '4': 1, '5': 9, '10': 'imageType'},
    {'1': 'format', '3': 2, '4': 1, '5': 9, '10': 'format'},
  ],
};

/// Descriptor for `BusinessProfileImageUploadRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfileImageUploadRequestDescriptor =
    $convert.base64Decode(
        'CiFCdXNpbmVzc1Byb2ZpbGVJbWFnZVVwbG9hZFJlcXVlc3QSHQoKaW1hZ2VfdHlwZRgBIAEoCV'
        'IJaW1hZ2VUeXBlEhYKBmZvcm1hdBgCIAEoCVIGZm9ybWF0');

@$core.Deprecated('Use businessProfileImageUploadResponseDescriptor instead')
const BusinessProfileImageUploadResponse$json = {
  '1': 'BusinessProfileImageUploadResponse',
  '2': [
    {'1': 'url', '3': 1, '4': 1, '5': 9, '10': 'url'},
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '10': 'objectKey'},
    {
      '1': 'business_profile_id',
      '3': 3,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
  ],
};

/// Descriptor for `BusinessProfileImageUploadResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List businessProfileImageUploadResponseDescriptor =
    $convert.base64Decode(
        'CiJCdXNpbmVzc1Byb2ZpbGVJbWFnZVVwbG9hZFJlc3BvbnNlEhAKA3VybBgBIAEoCVIDdXJsEh'
        '0KCm9iamVjdF9rZXkYAiABKAlSCW9iamVjdEtleRIuChNidXNpbmVzc19wcm9maWxlX2lkGAMg'
        'ASgFUhFidXNpbmVzc1Byb2ZpbGVJZA==');

@$core.Deprecated('Use getActiveBusinessProfileRequestDescriptor instead')
const GetActiveBusinessProfileRequest$json = {
  '1': 'GetActiveBusinessProfileRequest',
};

/// Descriptor for `GetActiveBusinessProfileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getActiveBusinessProfileRequestDescriptor =
    $convert.base64Decode('Ch9HZXRBY3RpdmVCdXNpbmVzc1Byb2ZpbGVSZXF1ZXN0');

@$core.Deprecated('Use discoverBusinessProfilesRequestDescriptor instead')
const DiscoverBusinessProfilesRequest$json = {
  '1': 'DiscoverBusinessProfilesRequest',
  '2': [
    {'1': 'query', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'query', '17': true},
    {
      '1': 'business_type',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'businessType',
      '17': true
    },
    {
      '1': 'latitude',
      '3': 3,
      '4': 1,
      '5': 1,
      '9': 2,
      '10': 'latitude',
      '17': true
    },
    {
      '1': 'longitude',
      '3': 4,
      '4': 1,
      '5': 1,
      '9': 3,
      '10': 'longitude',
      '17': true
    },
    {
      '1': 'radius_km',
      '3': 5,
      '4': 1,
      '5': 1,
      '9': 4,
      '10': 'radiusKm',
      '17': true
    },
    {'1': 'limit', '3': 6, '4': 1, '5': 5, '9': 5, '10': 'limit', '17': true},
  ],
  '8': [
    {'1': '_query'},
    {'1': '_business_type'},
    {'1': '_latitude'},
    {'1': '_longitude'},
    {'1': '_radius_km'},
    {'1': '_limit'},
  ],
};

/// Descriptor for `DiscoverBusinessProfilesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List discoverBusinessProfilesRequestDescriptor = $convert.base64Decode(
    'Ch9EaXNjb3ZlckJ1c2luZXNzUHJvZmlsZXNSZXF1ZXN0EhkKBXF1ZXJ5GAEgASgJSABSBXF1ZX'
    'J5iAEBEigKDWJ1c2luZXNzX3R5cGUYAiABKAlIAVIMYnVzaW5lc3NUeXBliAEBEh8KCGxhdGl0'
    'dWRlGAMgASgBSAJSCGxhdGl0dWRliAEBEiEKCWxvbmdpdHVkZRgEIAEoAUgDUglsb25naXR1ZG'
    'WIAQESIAoJcmFkaXVzX2ttGAUgASgBSARSCHJhZGl1c0ttiAEBEhkKBWxpbWl0GAYgASgFSAVS'
    'BWxpbWl0iAEBQggKBl9xdWVyeUIQCg5fYnVzaW5lc3NfdHlwZUILCglfbGF0aXR1ZGVCDAoKX2'
    'xvbmdpdHVkZUIMCgpfcmFkaXVzX2ttQggKBl9saW1pdA==');

@$core.Deprecated('Use deleteBusinessProfileRequestDescriptor instead')
const DeleteBusinessProfileRequest$json = {
  '1': 'DeleteBusinessProfileRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
  ],
};

/// Descriptor for `DeleteBusinessProfileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteBusinessProfileRequestDescriptor =
    $convert.base64Decode(
        'ChxEZWxldGVCdXNpbmVzc1Byb2ZpbGVSZXF1ZXN0Eg4KAmlkGAEgASgFUgJpZA==');

@$core.Deprecated('Use deleteBusinessProfileResponseDescriptor instead')
const DeleteBusinessProfileResponse$json = {
  '1': 'DeleteBusinessProfileResponse',
};

/// Descriptor for `DeleteBusinessProfileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteBusinessProfileResponseDescriptor =
    $convert.base64Decode('Ch1EZWxldGVCdXNpbmVzc1Byb2ZpbGVSZXNwb25zZQ==');
