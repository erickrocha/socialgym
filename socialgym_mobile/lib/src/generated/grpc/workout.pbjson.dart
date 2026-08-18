// This is a generated file - do not edit.
//
// Generated from workout.proto.

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

@$core.Deprecated('Use workoutDescriptor instead')
const Workout$json = {
  '1': 'Workout',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'owner_id', '3': 3, '4': 1, '5': 5, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 4, '4': 1, '5': 9, '10': 'ownerUuid'},
    {'1': 'name', '3': 5, '4': 1, '5': 9, '10': 'name'},
    {'1': 'description', '3': 6, '4': 1, '5': 9, '10': 'description'},
    {'1': 'difficulty', '3': 7, '4': 1, '5': 9, '10': 'difficulty'},
    {'1': 'muscle_group', '3': 8, '4': 1, '5': 9, '10': 'muscleGroup'},
    {'1': 'visibility', '3': 9, '4': 1, '5': 9, '10': 'visibility'},
    {
      '1': 'exercises',
      '3': 10,
      '4': 3,
      '5': 11,
      '6': '.grpc.exercise.Exercise',
      '10': 'exercises'
    },
    {'1': 'created_at', '3': 11, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 12, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
};

/// Descriptor for `Workout`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workoutDescriptor = $convert.base64Decode(
    'CgdXb3Jrb3V0Eg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUgR1dWlkEhkKCG93bmVyX2'
    'lkGAMgASgFUgdvd25lcklkEh0KCm93bmVyX3V1aWQYBCABKAlSCW93bmVyVXVpZBISCgRuYW1l'
    'GAUgASgJUgRuYW1lEiAKC2Rlc2NyaXB0aW9uGAYgASgJUgtkZXNjcmlwdGlvbhIeCgpkaWZmaW'
    'N1bHR5GAcgASgJUgpkaWZmaWN1bHR5EiEKDG11c2NsZV9ncm91cBgIIAEoCVILbXVzY2xlR3Jv'
    'dXASHgoKdmlzaWJpbGl0eRgJIAEoCVIKdmlzaWJpbGl0eRI1CglleGVyY2lzZXMYCiADKAsyFy'
    '5ncnBjLmV4ZXJjaXNlLkV4ZXJjaXNlUglleGVyY2lzZXMSHQoKY3JlYXRlZF9hdBgLIAEoCVIJ'
    'Y3JlYXRlZEF0Eh0KCnVwZGF0ZWRfYXQYDCABKAlSCXVwZGF0ZWRBdA==');

@$core.Deprecated('Use workoutRequestDescriptor instead')
const WorkoutRequest$json = {
  '1': 'WorkoutRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'uuid'},
  ],
  '8': [
    {'1': 'identifier'},
  ],
};

/// Descriptor for `WorkoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workoutRequestDescriptor = $convert.base64Decode(
    'Cg5Xb3Jrb3V0UmVxdWVzdBIQCgJpZBgBIAEoBUgAUgJpZBIUCgR1dWlkGAIgASgJSABSBHV1aW'
    'RCDAoKaWRlbnRpZmllcg==');

@$core.Deprecated('Use workoutExercisesRequestDescriptor instead')
const WorkoutExercisesRequest$json = {
  '1': 'WorkoutExercisesRequest',
  '2': [
    {'1': 'workout_uuid', '3': 1, '4': 1, '5': 9, '10': 'workoutUuid'},
    {
      '1': 'exercises',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.grpc.exercise.Exercise',
      '10': 'exercises'
    },
  ],
};

/// Descriptor for `WorkoutExercisesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workoutExercisesRequestDescriptor = $convert.base64Decode(
    'ChdXb3Jrb3V0RXhlcmNpc2VzUmVxdWVzdBIhCgx3b3Jrb3V0X3V1aWQYASABKAlSC3dvcmtvdX'
    'RVdWlkEjUKCWV4ZXJjaXNlcxgCIAMoCzIXLmdycGMuZXhlcmNpc2UuRXhlcmNpc2VSCWV4ZXJj'
    'aXNlcw==');

@$core.Deprecated('Use workoutListRequestDescriptor instead')
const WorkoutListRequest$json = {
  '1': 'WorkoutListRequest',
  '2': [
    {'1': 'owner_id', '3': 1, '4': 1, '5': 5, '9': 0, '10': 'ownerId'},
    {'1': 'owner_uuid', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'ownerUuid'},
  ],
  '8': [
    {'1': 'identifier'},
  ],
};

/// Descriptor for `WorkoutListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workoutListRequestDescriptor = $convert.base64Decode(
    'ChJXb3Jrb3V0TGlzdFJlcXVlc3QSGwoIb3duZXJfaWQYASABKAVIAFIHb3duZXJJZBIfCgpvd2'
    '5lcl91dWlkGAIgASgJSABSCW93bmVyVXVpZEIMCgppZGVudGlmaWVy');

@$core.Deprecated('Use workoutResponseDescriptor instead')
const WorkoutResponse$json = {
  '1': 'WorkoutResponse',
  '2': [
    {
      '1': 'workouts',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.workout.Workout',
      '10': 'workouts'
    },
  ],
};

/// Descriptor for `WorkoutResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workoutResponseDescriptor = $convert.base64Decode(
    'Cg9Xb3Jrb3V0UmVzcG9uc2USMQoId29ya291dHMYASADKAsyFS5ncnBjLndvcmtvdXQuV29ya2'
    '91dFIId29ya291dHM=');
