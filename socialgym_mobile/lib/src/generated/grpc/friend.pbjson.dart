// This is a generated file - do not edit.
//
// Generated from friend.proto.

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

@$core.Deprecated('Use friendsRequestDescriptor instead')
const FriendsRequest$json = {
  '1': 'FriendsRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
  ],
};

/// Descriptor for `FriendsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List friendsRequestDescriptor = $convert.base64Decode(
    'Cg5GcmllbmRzUmVxdWVzdBIOCgJpZBgBIAEoBVICaWQSEgoEdXVpZBgCIAEoCVIEdXVpZA==');

@$core.Deprecated('Use friendsResponseDescriptor instead')
const FriendsResponse$json = {
  '1': 'FriendsResponse',
  '2': [
    {
      '1': 'friends',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.friend.Friend',
      '10': 'friends'
    },
  ],
};

/// Descriptor for `FriendsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List friendsResponseDescriptor = $convert.base64Decode(
    'Cg9GcmllbmRzUmVzcG9uc2USLQoHZnJpZW5kcxgBIAMoCzITLmdycGMuZnJpZW5kLkZyaWVuZF'
    'IHZnJpZW5kcw==');

@$core.Deprecated('Use friendDescriptor instead')
const Friend$json = {
  '1': 'Friend',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'person_id', '3': 3, '4': 1, '5': 5, '10': 'personId'},
    {'1': 'person_uuid', '3': 4, '4': 1, '5': 9, '10': 'personUuid'},
    {'1': 'friend_id', '3': 5, '4': 1, '5': 5, '10': 'friendId'},
    {'1': 'friend_uuid', '3': 6, '4': 1, '5': 9, '10': 'friendUuid'},
    {'1': 'status', '3': 7, '4': 1, '5': 9, '10': 'status'},
  ],
};

/// Descriptor for `Friend`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List friendDescriptor = $convert.base64Decode(
    'CgZGcmllbmQSDgoCaWQYASABKAVSAmlkEhIKBHV1aWQYAiABKAlSBHV1aWQSGwoJcGVyc29uX2'
    'lkGAMgASgFUghwZXJzb25JZBIfCgtwZXJzb25fdXVpZBgEIAEoCVIKcGVyc29uVXVpZBIbCglm'
    'cmllbmRfaWQYBSABKAVSCGZyaWVuZElkEh8KC2ZyaWVuZF91dWlkGAYgASgJUgpmcmllbmRVdW'
    'lkEhYKBnN0YXR1cxgHIAEoCVIGc3RhdHVz');

@$core.Deprecated('Use friendRequestRequestDescriptor instead')
const FriendRequestRequest$json = {
  '1': 'FriendRequestRequest',
  '2': [
    {'1': 'person_id', '3': 1, '4': 1, '5': 5, '10': 'personId'},
  ],
};

/// Descriptor for `FriendRequestRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List friendRequestRequestDescriptor =
    $convert.base64Decode(
        'ChRGcmllbmRSZXF1ZXN0UmVxdWVzdBIbCglwZXJzb25faWQYASABKAVSCHBlcnNvbklk');

@$core.Deprecated('Use removeFriendResponseDescriptor instead')
const RemoveFriendResponse$json = {
  '1': 'RemoveFriendResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `RemoveFriendResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeFriendResponseDescriptor =
    $convert.base64Decode(
        'ChRSZW1vdmVGcmllbmRSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNz');

@$core.Deprecated('Use friendPageRequestDescriptor instead')
const FriendPageRequest$json = {
  '1': 'FriendPageRequest',
  '2': [
    {'1': 'person_id', '3': 1, '4': 1, '5': 5, '10': 'personId'},
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
    {
      '1': 'radius_km',
      '3': 4,
      '4': 1,
      '5': 1,
      '9': 2,
      '10': 'radiusKm',
      '17': true
    },
  ],
  '8': [
    {'1': '_latitude'},
    {'1': '_longitude'},
    {'1': '_radius_km'},
  ],
};

/// Descriptor for `FriendPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List friendPageRequestDescriptor = $convert.base64Decode(
    'ChFGcmllbmRQYWdlUmVxdWVzdBIbCglwZXJzb25faWQYASABKAVSCHBlcnNvbklkEh8KCGxhdG'
    'l0dWRlGAIgASgBSABSCGxhdGl0dWRliAEBEiEKCWxvbmdpdHVkZRgDIAEoAUgBUglsb25naXR1'
    'ZGWIAQESIAoJcmFkaXVzX2ttGAQgASgBSAJSCHJhZGl1c0ttiAEBQgsKCV9sYXRpdHVkZUIMCg'
    'pfbG9uZ2l0dWRlQgwKCl9yYWRpdXNfa20=');

@$core.Deprecated('Use friendPageResponseDescriptor instead')
const FriendPageResponse$json = {
  '1': 'FriendPageResponse',
  '2': [
    {
      '1': 'suggestions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'suggestions'
    },
    {
      '1': 'friends',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'friends'
    },
    {
      '1': 'receive_requests',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'receiveRequests'
    },
    {
      '1': 'sent_requests',
      '3': 4,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'sentRequests'
    },
  ],
};

/// Descriptor for `FriendPageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List friendPageResponseDescriptor = $convert.base64Decode(
    'ChJGcmllbmRQYWdlUmVzcG9uc2USNQoLc3VnZ2VzdGlvbnMYASADKAsyEy5ncnBjLnBlcnNvbi'
    '5QZXJzb25SC3N1Z2dlc3Rpb25zEi0KB2ZyaWVuZHMYAiADKAsyEy5ncnBjLnBlcnNvbi5QZXJz'
    'b25SB2ZyaWVuZHMSPgoQcmVjZWl2ZV9yZXF1ZXN0cxgDIAMoCzITLmdycGMucGVyc29uLlBlcn'
    'NvblIPcmVjZWl2ZVJlcXVlc3RzEjgKDXNlbnRfcmVxdWVzdHMYBCADKAsyEy5ncnBjLnBlcnNv'
    'bi5QZXJzb25SDHNlbnRSZXF1ZXN0cw==');

@$core.Deprecated('Use searchFriendsRequestDescriptor instead')
const SearchFriendsRequest$json = {
  '1': 'SearchFriendsRequest',
  '2': [
    {'1': 'query', '3': 1, '4': 1, '5': 9, '10': 'query'},
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
    {
      '1': 'radius_km',
      '3': 4,
      '4': 1,
      '5': 1,
      '9': 2,
      '10': 'radiusKm',
      '17': true
    },
    {'1': 'limit', '3': 5, '4': 1, '5': 5, '10': 'limit'},
  ],
  '8': [
    {'1': '_latitude'},
    {'1': '_longitude'},
    {'1': '_radius_km'},
  ],
};

/// Descriptor for `SearchFriendsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchFriendsRequestDescriptor = $convert.base64Decode(
    'ChRTZWFyY2hGcmllbmRzUmVxdWVzdBIUCgVxdWVyeRgBIAEoCVIFcXVlcnkSHwoIbGF0aXR1ZG'
    'UYAiABKAFIAFIIbGF0aXR1ZGWIAQESIQoJbG9uZ2l0dWRlGAMgASgBSAFSCWxvbmdpdHVkZYgB'
    'ARIgCglyYWRpdXNfa20YBCABKAFIAlIIcmFkaXVzS22IAQESFAoFbGltaXQYBSABKAVSBWxpbW'
    'l0QgsKCV9sYXRpdHVkZUIMCgpfbG9uZ2l0dWRlQgwKCl9yYWRpdXNfa20=');

@$core.Deprecated('Use searchFriendsResponseDescriptor instead')
const SearchFriendsResponse$json = {
  '1': 'SearchFriendsResponse',
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

/// Descriptor for `SearchFriendsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchFriendsResponseDescriptor = $convert.base64Decode(
    'ChVTZWFyY2hGcmllbmRzUmVzcG9uc2USKwoGcGVvcGxlGAEgAygLMhMuZ3JwYy5wZXJzb24uUG'
    'Vyc29uUgZwZW9wbGU=');
