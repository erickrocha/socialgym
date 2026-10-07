// This is a generated file - do not edit.
//
// Generated from timeline/post.proto.

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

@$core.Deprecated('Use mediaDescriptor instead')
const Media$json = {
  '1': 'Media',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'url', '3': 2, '4': 1, '5': 9, '10': 'url'},
    {'1': 'media_type', '3': 3, '4': 1, '5': 9, '10': 'mediaType'},
    {'1': 'object_key', '3': 4, '4': 1, '5': 9, '10': 'objectKey'},
  ],
  '8': [
    {'1': '_uuid'},
  ],
};

/// Descriptor for `Media`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mediaDescriptor = $convert.base64Decode(
    'CgVNZWRpYRIXCgR1dWlkGAEgASgJSABSBHV1aWSIAQESEAoDdXJsGAIgASgJUgN1cmwSHQoKbW'
    'VkaWFfdHlwZRgDIAEoCVIJbWVkaWFUeXBlEh0KCm9iamVjdF9rZXkYBCABKAlSCW9iamVjdEtl'
    'eUIHCgVfdXVpZA==');

@$core.Deprecated('Use reactionDescriptor instead')
const Reaction$json = {
  '1': 'Reaction',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'author_id', '3': 2, '4': 1, '5': 9, '10': 'authorId'},
    {'1': 'author_name', '3': 3, '4': 1, '5': 9, '10': 'authorName'},
    {'1': 'reaction_type', '3': 4, '4': 1, '5': 9, '10': 'reactionType'},
  ],
  '8': [
    {'1': '_uuid'},
  ],
};

/// Descriptor for `Reaction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List reactionDescriptor = $convert.base64Decode(
    'CghSZWFjdGlvbhIXCgR1dWlkGAEgASgJSABSBHV1aWSIAQESGwoJYXV0aG9yX2lkGAIgASgJUg'
    'hhdXRob3JJZBIfCgthdXRob3JfbmFtZRgDIAEoCVIKYXV0aG9yTmFtZRIjCg1yZWFjdGlvbl90'
    'eXBlGAQgASgJUgxyZWFjdGlvblR5cGVCBwoFX3V1aWQ=');

@$core.Deprecated('Use mentionDescriptor instead')
const Mention$json = {
  '1': 'Mention',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'mentioned_uuid', '3': 2, '4': 1, '5': 9, '10': 'mentionedUuid'},
  ],
};

/// Descriptor for `Mention`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mentionDescriptor = $convert.base64Decode(
    'CgdNZW50aW9uEhIKBG5hbWUYASABKAlSBG5hbWUSJQoObWVudGlvbmVkX3V1aWQYAiABKAlSDW'
    '1lbnRpb25lZFV1aWQ=');

@$core.Deprecated('Use commentDescriptor instead')
const Comment$json = {
  '1': 'Comment',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'post_uuid', '3': 2, '4': 1, '5': 9, '10': 'postUuid'},
    {'1': 'author_uuid', '3': 3, '4': 1, '5': 9, '10': 'authorUuid'},
    {'1': 'author_name', '3': 4, '4': 1, '5': 9, '10': 'authorName'},
    {
      '1': 'author_object_key',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'authorObjectKey',
      '17': true
    },
    {
      '1': 'author_avatar',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'authorAvatar',
      '17': true
    },
    {'1': 'content', '3': 7, '4': 1, '5': 9, '10': 'content'},
    {
      '1': 'parent_uuid',
      '3': 8,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'parentUuid',
      '17': true
    },
    {
      '1': 'created_at',
      '3': 9,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'createdAt',
      '17': true
    },
    {
      '1': 'updated_at',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'updatedAt',
      '17': true
    },
    {
      '1': 'mentions',
      '3': 11,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Mention',
      '10': 'mentions'
    },
  ],
  '8': [
    {'1': '_uuid'},
    {'1': '_author_object_key'},
    {'1': '_author_avatar'},
    {'1': '_parent_uuid'},
    {'1': '_created_at'},
    {'1': '_updated_at'},
  ],
};

/// Descriptor for `Comment`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List commentDescriptor = $convert.base64Decode(
    'CgdDb21tZW50EhcKBHV1aWQYASABKAlIAFIEdXVpZIgBARIbCglwb3N0X3V1aWQYAiABKAlSCH'
    'Bvc3RVdWlkEh8KC2F1dGhvcl91dWlkGAMgASgJUgphdXRob3JVdWlkEh8KC2F1dGhvcl9uYW1l'
    'GAQgASgJUgphdXRob3JOYW1lEi8KEWF1dGhvcl9vYmplY3Rfa2V5GAUgASgJSAFSD2F1dGhvck'
    '9iamVjdEtleYgBARIoCg1hdXRob3JfYXZhdGFyGAYgASgJSAJSDGF1dGhvckF2YXRhcogBARIY'
    'Cgdjb250ZW50GAcgASgJUgdjb250ZW50EiQKC3BhcmVudF91dWlkGAggASgJSANSCnBhcmVudF'
    'V1aWSIAQESIgoKY3JlYXRlZF9hdBgJIAEoCUgEUgljcmVhdGVkQXSIAQESIgoKdXBkYXRlZF9h'
    'dBgKIAEoCUgFUgl1cGRhdGVkQXSIAQESMgoIbWVudGlvbnMYCyADKAsyFi5ncnBjLnRpbWVsaW'
    '5lLk1lbnRpb25SCG1lbnRpb25zQgcKBV91dWlkQhQKEl9hdXRob3Jfb2JqZWN0X2tleUIQCg5f'
    'YXV0aG9yX2F2YXRhckIOCgxfcGFyZW50X3V1aWRCDQoLX2NyZWF0ZWRfYXRCDQoLX3VwZGF0ZW'
    'RfYXQ=');

@$core.Deprecated('Use postDescriptor instead')
const Post$json = {
  '1': 'Post',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'author_id', '3': 2, '4': 1, '5': 5, '10': 'authorId'},
    {'1': 'author_uuid', '3': 3, '4': 1, '5': 9, '10': 'authorUuid'},
    {'1': 'author_name', '3': 4, '4': 1, '5': 9, '10': 'authorName'},
    {
      '1': 'author_object_key',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'authorObjectKey',
      '17': true
    },
    {
      '1': 'author_avatar',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'authorAvatar',
      '17': true
    },
    {'1': 'content', '3': 7, '4': 1, '5': 9, '10': 'content'},
    {
      '1': 'media',
      '3': 8,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Media',
      '10': 'media'
    },
    {
      '1': 'reactions',
      '3': 9,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Reaction',
      '10': 'reactions'
    },
    {
      '1': 'comments',
      '3': 10,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Comment',
      '10': 'comments'
    },
    {
      '1': 'created_at',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'createdAt',
      '17': true
    },
    {
      '1': 'updated_at',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'updatedAt',
      '17': true
    },
    {
      '1': 'mentions',
      '3': 13,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Mention',
      '10': 'mentions'
    },
  ],
  '8': [
    {'1': '_uuid'},
    {'1': '_author_object_key'},
    {'1': '_author_avatar'},
    {'1': '_created_at'},
    {'1': '_updated_at'},
  ],
};

/// Descriptor for `Post`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List postDescriptor = $convert.base64Decode(
    'CgRQb3N0EhcKBHV1aWQYASABKAlIAFIEdXVpZIgBARIbCglhdXRob3JfaWQYAiABKAVSCGF1dG'
    'hvcklkEh8KC2F1dGhvcl91dWlkGAMgASgJUgphdXRob3JVdWlkEh8KC2F1dGhvcl9uYW1lGAQg'
    'ASgJUgphdXRob3JOYW1lEi8KEWF1dGhvcl9vYmplY3Rfa2V5GAUgASgJSAFSD2F1dGhvck9iam'
    'VjdEtleYgBARIoCg1hdXRob3JfYXZhdGFyGAYgASgJSAJSDGF1dGhvckF2YXRhcogBARIYCgdj'
    'b250ZW50GAcgASgJUgdjb250ZW50EioKBW1lZGlhGAggAygLMhQuZ3JwYy50aW1lbGluZS5NZW'
    'RpYVIFbWVkaWESNQoJcmVhY3Rpb25zGAkgAygLMhcuZ3JwYy50aW1lbGluZS5SZWFjdGlvblIJ'
    'cmVhY3Rpb25zEjIKCGNvbW1lbnRzGAogAygLMhYuZ3JwYy50aW1lbGluZS5Db21tZW50Ughjb2'
    '1tZW50cxIiCgpjcmVhdGVkX2F0GAsgASgJSANSCWNyZWF0ZWRBdIgBARIiCgp1cGRhdGVkX2F0'
    'GAwgASgJSARSCXVwZGF0ZWRBdIgBARIyCghtZW50aW9ucxgNIAMoCzIWLmdycGMudGltZWxpbm'
    'UuTWVudGlvblIIbWVudGlvbnNCBwoFX3V1aWRCFAoSX2F1dGhvcl9vYmplY3Rfa2V5QhAKDl9h'
    'dXRob3JfYXZhdGFyQg0KC19jcmVhdGVkX2F0Qg0KC191cGRhdGVkX2F0');

@$core.Deprecated('Use createPostRequestDescriptor instead')
const CreatePostRequest$json = {
  '1': 'CreatePostRequest',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
    {
      '1': 'media',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Media',
      '10': 'media'
    },
    {
      '1': 'mentions',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Mention',
      '10': 'mentions'
    },
    {
      '1': 'third_party_consent_confirmed',
      '3': 4,
      '4': 1,
      '5': 8,
      '10': 'thirdPartyConsentConfirmed'
    },
  ],
};

/// Descriptor for `CreatePostRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createPostRequestDescriptor = $convert.base64Decode(
    'ChFDcmVhdGVQb3N0UmVxdWVzdBIYCgdjb250ZW50GAEgASgJUgdjb250ZW50EioKBW1lZGlhGA'
    'IgAygLMhQuZ3JwYy50aW1lbGluZS5NZWRpYVIFbWVkaWESMgoIbWVudGlvbnMYAyADKAsyFi5n'
    'cnBjLnRpbWVsaW5lLk1lbnRpb25SCG1lbnRpb25zEkEKHXRoaXJkX3BhcnR5X2NvbnNlbnRfY2'
    '9uZmlybWVkGAQgASgIUhp0aGlyZFBhcnR5Q29uc2VudENvbmZpcm1lZA==');

@$core.Deprecated('Use deletePostRequestDescriptor instead')
const DeletePostRequest$json = {
  '1': 'DeletePostRequest',
  '2': [
    {'1': 'post_uuid', '3': 1, '4': 1, '5': 9, '10': 'postUuid'},
  ],
};

/// Descriptor for `DeletePostRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deletePostRequestDescriptor = $convert.base64Decode(
    'ChFEZWxldGVQb3N0UmVxdWVzdBIbCglwb3N0X3V1aWQYASABKAlSCHBvc3RVdWlk');

@$core.Deprecated('Use deletePostResponseDescriptor instead')
const DeletePostResponse$json = {
  '1': 'DeletePostResponse',
};

/// Descriptor for `DeletePostResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deletePostResponseDescriptor =
    $convert.base64Decode('ChJEZWxldGVQb3N0UmVzcG9uc2U=');

@$core.Deprecated('Use addCommentRequestDescriptor instead')
const AddCommentRequest$json = {
  '1': 'AddCommentRequest',
  '2': [
    {'1': 'post_uuid', '3': 1, '4': 1, '5': 9, '10': 'postUuid'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
    {
      '1': 'parent_uuid',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'parentUuid',
      '17': true
    },
    {
      '1': 'mentions',
      '3': 4,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Mention',
      '10': 'mentions'
    },
  ],
  '8': [
    {'1': '_parent_uuid'},
  ],
};

/// Descriptor for `AddCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addCommentRequestDescriptor = $convert.base64Decode(
    'ChFBZGRDb21tZW50UmVxdWVzdBIbCglwb3N0X3V1aWQYASABKAlSCHBvc3RVdWlkEhgKB2Nvbn'
    'RlbnQYAiABKAlSB2NvbnRlbnQSJAoLcGFyZW50X3V1aWQYAyABKAlIAFIKcGFyZW50VXVpZIgB'
    'ARIyCghtZW50aW9ucxgEIAMoCzIWLmdycGMudGltZWxpbmUuTWVudGlvblIIbWVudGlvbnNCDg'
    'oMX3BhcmVudF91dWlk');

@$core.Deprecated('Use addReactionRequestDescriptor instead')
const AddReactionRequest$json = {
  '1': 'AddReactionRequest',
  '2': [
    {'1': 'post_uuid', '3': 1, '4': 1, '5': 9, '10': 'postUuid'},
    {'1': 'reaction_type', '3': 2, '4': 1, '5': 9, '10': 'reactionType'},
  ],
};

/// Descriptor for `AddReactionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addReactionRequestDescriptor = $convert.base64Decode(
    'ChJBZGRSZWFjdGlvblJlcXVlc3QSGwoJcG9zdF91dWlkGAEgASgJUghwb3N0VXVpZBIjCg1yZW'
    'FjdGlvbl90eXBlGAIgASgJUgxyZWFjdGlvblR5cGU=');

@$core.Deprecated('Use removeReactionRequestDescriptor instead')
const RemoveReactionRequest$json = {
  '1': 'RemoveReactionRequest',
  '2': [
    {'1': 'post_uuid', '3': 1, '4': 1, '5': 9, '10': 'postUuid'},
  ],
};

/// Descriptor for `RemoveReactionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeReactionRequestDescriptor = $convert.base64Decode(
    'ChVSZW1vdmVSZWFjdGlvblJlcXVlc3QSGwoJcG9zdF91dWlkGAEgASgJUghwb3N0VXVpZA==');
