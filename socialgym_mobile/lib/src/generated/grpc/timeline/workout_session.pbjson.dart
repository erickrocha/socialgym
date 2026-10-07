// This is a generated file - do not edit.
//
// Generated from timeline/workout_session.proto.

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

@$core.Deprecated('Use executedSetDescriptor instead')
const ExecutedSet$json = {
  '1': 'ExecutedSet',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {
      '1': 'exercise_name',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'exerciseName',
      '17': true
    },
    {'1': 'owner_id', '3': 3, '4': 1, '5': 5, '10': 'ownerId'},
    {
      '1': 'owner_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'ownerName',
      '17': true
    },
    {
      '1': 'category',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'category',
      '17': true
    },
    {
      '1': 'visibility',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'visibility',
      '17': true
    },
    {'1': 'set_number', '3': 7, '4': 1, '5': 5, '10': 'setNumber'},
    {'1': 'reps_or_duration', '3': 8, '4': 1, '5': 5, '10': 'repsOrDuration'},
    {'1': 'weight', '3': 9, '4': 1, '5': 2, '10': 'weight'},
    {
      '1': 'started_at',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 5,
      '10': 'startedAt',
      '17': true
    },
    {
      '1': 'completed_at',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 6,
      '10': 'completedAt',
      '17': true
    },
  ],
  '8': [
    {'1': '_uuid'},
    {'1': '_exercise_name'},
    {'1': '_owner_name'},
    {'1': '_category'},
    {'1': '_visibility'},
    {'1': '_started_at'},
    {'1': '_completed_at'},
  ],
};

/// Descriptor for `ExecutedSet`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List executedSetDescriptor = $convert.base64Decode(
    'CgtFeGVjdXRlZFNldBIXCgR1dWlkGAEgASgJSABSBHV1aWSIAQESKAoNZXhlcmNpc2VfbmFtZR'
    'gCIAEoCUgBUgxleGVyY2lzZU5hbWWIAQESGQoIb3duZXJfaWQYAyABKAVSB293bmVySWQSIgoK'
    'b3duZXJfbmFtZRgEIAEoCUgCUglvd25lck5hbWWIAQESHwoIY2F0ZWdvcnkYBSABKAlIA1IIY2'
    'F0ZWdvcnmIAQESIwoKdmlzaWJpbGl0eRgGIAEoCUgEUgp2aXNpYmlsaXR5iAEBEh0KCnNldF9u'
    'dW1iZXIYByABKAVSCXNldE51bWJlchIoChByZXBzX29yX2R1cmF0aW9uGAggASgFUg5yZXBzT3'
    'JEdXJhdGlvbhIWCgZ3ZWlnaHQYCSABKAJSBndlaWdodBIiCgpzdGFydGVkX2F0GAogASgJSAVS'
    'CXN0YXJ0ZWRBdIgBARImCgxjb21wbGV0ZWRfYXQYCyABKAlIBlILY29tcGxldGVkQXSIAQFCBw'
    'oFX3V1aWRCEAoOX2V4ZXJjaXNlX25hbWVCDQoLX293bmVyX25hbWVCCwoJX2NhdGVnb3J5Qg0K'
    'C192aXNpYmlsaXR5Qg0KC19zdGFydGVkX2F0Qg8KDV9jb21wbGV0ZWRfYXQ=');

@$core.Deprecated('Use workoutSessionDescriptor instead')
const WorkoutSession$json = {
  '1': 'WorkoutSession',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {'1': 'person_uuid', '3': 2, '4': 1, '5': 9, '10': 'personUuid'},
    {
      '1': 'workout_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'workoutName',
      '17': true
    },
    {'1': 'duration', '3': 4, '4': 1, '5': 5, '10': 'duration'},
    {
      '1': 'started_at',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'startedAt',
      '17': true
    },
    {
      '1': 'day_of_week',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'dayOfWeek',
      '17': true
    },
    {
      '1': 'completed_at',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'completedAt',
      '17': true
    },
    {
      '1': 'executed_sets',
      '3': 8,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.ExecutedSet',
      '10': 'executedSets'
    },
    {'1': 'total_volume', '3': 9, '4': 1, '5': 2, '10': 'totalVolume'},
    {'1': 'total_sets', '3': 10, '4': 1, '5': 2, '10': 'totalSets'},
  ],
  '8': [
    {'1': '_uuid'},
    {'1': '_workout_name'},
    {'1': '_started_at'},
    {'1': '_day_of_week'},
    {'1': '_completed_at'},
  ],
};

/// Descriptor for `WorkoutSession`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List workoutSessionDescriptor = $convert.base64Decode(
    'Cg5Xb3Jrb3V0U2Vzc2lvbhIXCgR1dWlkGAEgASgJSABSBHV1aWSIAQESHwoLcGVyc29uX3V1aW'
    'QYAiABKAlSCnBlcnNvblV1aWQSJgoMd29ya291dF9uYW1lGAMgASgJSAFSC3dvcmtvdXROYW1l'
    'iAEBEhoKCGR1cmF0aW9uGAQgASgFUghkdXJhdGlvbhIiCgpzdGFydGVkX2F0GAUgASgJSAJSCX'
    'N0YXJ0ZWRBdIgBARIjCgtkYXlfb2Zfd2VlaxgGIAEoCUgDUglkYXlPZldlZWuIAQESJgoMY29t'
    'cGxldGVkX2F0GAcgASgJSARSC2NvbXBsZXRlZEF0iAEBEj8KDWV4ZWN1dGVkX3NldHMYCCADKA'
    'syGi5ncnBjLnRpbWVsaW5lLkV4ZWN1dGVkU2V0UgxleGVjdXRlZFNldHMSIQoMdG90YWxfdm9s'
    'dW1lGAkgASgCUgt0b3RhbFZvbHVtZRIdCgp0b3RhbF9zZXRzGAogASgCUgl0b3RhbFNldHNCBw'
    'oFX3V1aWRCDwoNX3dvcmtvdXRfbmFtZUINCgtfc3RhcnRlZF9hdEIOCgxfZGF5X29mX3dlZWtC'
    'DwoNX2NvbXBsZXRlZF9hdA==');

@$core.Deprecated('Use createWorkoutSessionRequestDescriptor instead')
const CreateWorkoutSessionRequest$json = {
  '1': 'CreateWorkoutSessionRequest',
  '2': [
    {
      '1': 'session',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.WorkoutSession',
      '10': 'session'
    },
  ],
};

/// Descriptor for `CreateWorkoutSessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createWorkoutSessionRequestDescriptor =
    $convert.base64Decode(
        'ChtDcmVhdGVXb3Jrb3V0U2Vzc2lvblJlcXVlc3QSNwoHc2Vzc2lvbhgBIAEoCzIdLmdycGMudG'
        'ltZWxpbmUuV29ya291dFNlc3Npb25SB3Nlc3Npb24=');

@$core.Deprecated('Use getWorkoutSessionRequestDescriptor instead')
const GetWorkoutSessionRequest$json = {
  '1': 'GetWorkoutSessionRequest',
  '2': [
    {'1': 'session_uuid', '3': 1, '4': 1, '5': 9, '10': 'sessionUuid'},
  ],
};

/// Descriptor for `GetWorkoutSessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getWorkoutSessionRequestDescriptor =
    $convert.base64Decode(
        'ChhHZXRXb3Jrb3V0U2Vzc2lvblJlcXVlc3QSIQoMc2Vzc2lvbl91dWlkGAEgASgJUgtzZXNzaW'
        '9uVXVpZA==');

@$core.Deprecated('Use listWorkoutSessionsRequestDescriptor instead')
const ListWorkoutSessionsRequest$json = {
  '1': 'ListWorkoutSessionsRequest',
  '2': [
    {
      '1': 'start_date',
      '3': 1,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'startDate',
      '17': true
    },
    {
      '1': 'end_date',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'endDate',
      '17': true
    },
  ],
  '8': [
    {'1': '_start_date'},
    {'1': '_end_date'},
  ],
};

/// Descriptor for `ListWorkoutSessionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listWorkoutSessionsRequestDescriptor =
    $convert.base64Decode(
        'ChpMaXN0V29ya291dFNlc3Npb25zUmVxdWVzdBIiCgpzdGFydF9kYXRlGAEgASgJSABSCXN0YX'
        'J0RGF0ZYgBARIeCghlbmRfZGF0ZRgCIAEoCUgBUgdlbmREYXRliAEBQg0KC19zdGFydF9kYXRl'
        'QgsKCV9lbmRfZGF0ZQ==');

@$core.Deprecated('Use listWorkoutSessionsResponseDescriptor instead')
const ListWorkoutSessionsResponse$json = {
  '1': 'ListWorkoutSessionsResponse',
  '2': [
    {
      '1': 'sessions',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.WorkoutSession',
      '10': 'sessions'
    },
  ],
};

/// Descriptor for `ListWorkoutSessionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listWorkoutSessionsResponseDescriptor =
    $convert.base64Decode(
        'ChtMaXN0V29ya291dFNlc3Npb25zUmVzcG9uc2USOQoIc2Vzc2lvbnMYASADKAsyHS5ncnBjLn'
        'RpbWVsaW5lLldvcmtvdXRTZXNzaW9uUghzZXNzaW9ucw==');
