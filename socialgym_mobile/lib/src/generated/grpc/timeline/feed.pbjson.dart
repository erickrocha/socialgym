// This is a generated file - do not edit.
//
// Generated from timeline/feed.proto.

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

@$core.Deprecated('Use getFeedRequestDescriptor instead')
const GetFeedRequest$json = {
  '1': 'GetFeedRequest',
  '2': [
    {'1': 'page', '3': 1, '4': 1, '5': 13, '10': 'page'},
  ],
};

/// Descriptor for `GetFeedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getFeedRequestDescriptor =
    $convert.base64Decode('Cg5HZXRGZWVkUmVxdWVzdBISCgRwYWdlGAEgASgNUgRwYWdl');

@$core.Deprecated('Use getFeedByAuthorRequestDescriptor instead')
const GetFeedByAuthorRequest$json = {
  '1': 'GetFeedByAuthorRequest',
  '2': [
    {'1': 'author_uuid', '3': 1, '4': 1, '5': 9, '10': 'authorUuid'},
    {'1': 'page', '3': 2, '4': 1, '5': 13, '10': 'page'},
  ],
};

/// Descriptor for `GetFeedByAuthorRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getFeedByAuthorRequestDescriptor =
    $convert.base64Decode(
        'ChZHZXRGZWVkQnlBdXRob3JSZXF1ZXN0Eh8KC2F1dGhvcl91dWlkGAEgASgJUgphdXRob3JVdW'
        'lkEhIKBHBhZ2UYAiABKA1SBHBhZ2U=');

@$core.Deprecated('Use feedResponseDescriptor instead')
const FeedResponse$json = {
  '1': 'FeedResponse',
  '2': [
    {
      '1': 'posts',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Post',
      '10': 'posts'
    },
  ],
};

/// Descriptor for `FeedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedResponseDescriptor = $convert.base64Decode(
    'CgxGZWVkUmVzcG9uc2USKQoFcG9zdHMYASADKAsyEy5ncnBjLnRpbWVsaW5lLlBvc3RSBXBvc3'
    'Rz');
