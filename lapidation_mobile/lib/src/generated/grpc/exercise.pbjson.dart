// This is a generated file - do not edit.
//
// Generated from exercise.proto.

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

@$core.Deprecated('Use exerciseParamsDescriptor instead')
const ExerciseParams$json = {
  '1': 'ExerciseParams',
  '2': [
    {'1': 'owner_uuid', '3': 1, '4': 1, '5': 9, '10': 'ownerUuid'},
    {'1': 'category', '3': 2, '4': 1, '5': 9, '10': 'category'},
    {'1': 'visibility', '3': 3, '4': 1, '5': 9, '10': 'visibility'},
    {'1': 'owners', '3': 4, '4': 3, '5': 9, '10': 'owners'},
    {'1': 'page_number', '3': 5, '4': 1, '5': 3, '10': 'pageNumber'},
    {'1': 'page_size', '3': 6, '4': 1, '5': 3, '10': 'pageSize'},
    {'1': 'sort_by', '3': 7, '4': 1, '5': 9, '10': 'sortBy'},
  ],
};

/// Descriptor for `ExerciseParams`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exerciseParamsDescriptor = $convert.base64Decode(
    'Cg5FeGVyY2lzZVBhcmFtcxIdCgpvd25lcl91dWlkGAEgASgJUglvd25lclV1aWQSGgoIY2F0ZW'
    'dvcnkYAiABKAlSCGNhdGVnb3J5Eh4KCnZpc2liaWxpdHkYAyABKAlSCnZpc2liaWxpdHkSFgoG'
    'b3duZXJzGAQgAygJUgZvd25lcnMSHwoLcGFnZV9udW1iZXIYBSABKANSCnBhZ2VOdW1iZXISGw'
    'oJcGFnZV9zaXplGAYgASgDUghwYWdlU2l6ZRIXCgdzb3J0X2J5GAcgASgJUgZzb3J0Qnk=');

@$core.Deprecated('Use exerciseRequestDescriptor instead')
const ExerciseRequest$json = {
  '1': 'ExerciseRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'uuid'},
  ],
  '8': [
    {'1': 'identifier'},
  ],
};

/// Descriptor for `ExerciseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exerciseRequestDescriptor = $convert.base64Decode(
    'Cg9FeGVyY2lzZVJlcXVlc3QSEAoCaWQYASABKAVIAFICaWQSFAoEdXVpZBgCIAEoCUgAUgR1dW'
    'lkQgwKCmlkZW50aWZpZXI=');

@$core.Deprecated('Use exerciseDescriptor instead')
const Exercise$json = {
  '1': 'Exercise',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'owner_id', '3': 3, '4': 1, '5': 5, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 4, '4': 1, '5': 9, '10': 'ownerUuid'},
    {'1': 'owner_name', '3': 5, '4': 1, '5': 9, '10': 'ownerName'},
    {'1': 'description', '3': 6, '4': 1, '5': 9, '10': 'description'},
    {'1': 'sets', '3': 7, '4': 1, '5': 5, '10': 'sets'},
    {'1': 'category', '3': 8, '4': 1, '5': 9, '10': 'category'},
    {'1': 'reps_or_duration', '3': 9, '4': 1, '5': 5, '10': 'repsOrDuration'},
    {'1': 'uuid', '3': 10, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'visibility', '3': 11, '4': 1, '5': 9, '10': 'visibility'},
    {'1': 'created_at', '3': 12, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 13, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
};

/// Descriptor for `Exercise`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exerciseDescriptor = $convert.base64Decode(
    'CghFeGVyY2lzZRIOCgJpZBgBIAEoBVICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIZCghvd25lcl'
    '9pZBgDIAEoBVIHb3duZXJJZBIdCgpvd25lcl91dWlkGAQgASgJUglvd25lclV1aWQSHQoKb3du'
    'ZXJfbmFtZRgFIAEoCVIJb3duZXJOYW1lEiAKC2Rlc2NyaXB0aW9uGAYgASgJUgtkZXNjcmlwdG'
    'lvbhISCgRzZXRzGAcgASgFUgRzZXRzEhoKCGNhdGVnb3J5GAggASgJUghjYXRlZ29yeRIoChBy'
    'ZXBzX29yX2R1cmF0aW9uGAkgASgFUg5yZXBzT3JEdXJhdGlvbhISCgR1dWlkGAogASgJUgR1dW'
    'lkEh4KCnZpc2liaWxpdHkYCyABKAlSCnZpc2liaWxpdHkSHQoKY3JlYXRlZF9hdBgMIAEoCVIJ'
    'Y3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYDSABKAlSCXVwZGF0ZWRBdA==');

@$core.Deprecated('Use paginatedExerciseDescriptor instead')
const PaginatedExercise$json = {
  '1': 'PaginatedExercise',
  '2': [
    {
      '1': 'content',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.exercise.Exercise',
      '10': 'content'
    },
    {'1': 'total_count', '3': 2, '4': 1, '5': 3, '10': 'totalCount'},
    {'1': 'page_number', '3': 3, '4': 1, '5': 3, '10': 'pageNumber'},
    {'1': 'page_size', '3': 4, '4': 1, '5': 3, '10': 'pageSize'},
    {'1': 'has_next_page', '3': 5, '4': 1, '5': 8, '10': 'hasNextPage'},
  ],
};

/// Descriptor for `PaginatedExercise`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List paginatedExerciseDescriptor = $convert.base64Decode(
    'ChFQYWdpbmF0ZWRFeGVyY2lzZRIxCgdjb250ZW50GAEgAygLMhcuZ3JwYy5leGVyY2lzZS5FeG'
    'VyY2lzZVIHY29udGVudBIfCgt0b3RhbF9jb3VudBgCIAEoA1IKdG90YWxDb3VudBIfCgtwYWdl'
    'X251bWJlchgDIAEoA1IKcGFnZU51bWJlchIbCglwYWdlX3NpemUYBCABKANSCHBhZ2VTaXplEi'
    'IKDWhhc19uZXh0X3BhZ2UYBSABKAhSC2hhc05leHRQYWdl');
