// This is a generated file - do not edit.
//
// Generated from timeline/content_report.proto.

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

@$core.Deprecated('Use moderationEventDescriptor instead')
const ModerationEvent$json = {
  '1': 'ModerationEvent',
  '2': [
    {'1': 'actor_person_uuid', '3': 1, '4': 1, '5': 9, '10': 'actorPersonUuid'},
    {'1': 'action', '3': 2, '4': 1, '5': 9, '10': 'action'},
    {'1': 'reason', '3': 3, '4': 1, '5': 9, '10': 'reason'},
    {'1': 'created_at', '3': 4, '4': 1, '5': 9, '10': 'createdAt'},
  ],
};

/// Descriptor for `ModerationEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List moderationEventDescriptor = $convert.base64Decode(
    'Cg9Nb2RlcmF0aW9uRXZlbnQSKgoRYWN0b3JfcGVyc29uX3V1aWQYASABKAlSD2FjdG9yUGVyc2'
    '9uVXVpZBIWCgZhY3Rpb24YAiABKAlSBmFjdGlvbhIWCgZyZWFzb24YAyABKAlSBnJlYXNvbhId'
    'CgpjcmVhdGVkX2F0GAQgASgJUgljcmVhdGVkQXQ=');

@$core.Deprecated('Use contentReportDescriptor instead')
const ContentReport$json = {
  '1': 'ContentReport',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'target_type', '3': 2, '4': 1, '5': 9, '10': 'targetType'},
    {'1': 'target_id', '3': 3, '4': 1, '5': 9, '10': 'targetId'},
    {'1': 'post_id', '3': 4, '4': 1, '5': 9, '10': 'postId'},
    {
      '1': 'reporter_person_uuid',
      '3': 5,
      '4': 1,
      '5': 9,
      '10': 'reporterPersonUuid'
    },
    {'1': 'reason', '3': 6, '4': 1, '5': 9, '10': 'reason'},
    {
      '1': 'details',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'details',
      '17': true
    },
    {'1': 'priority', '3': 8, '4': 1, '5': 9, '10': 'priority'},
    {'1': 'status', '3': 9, '4': 1, '5': 9, '10': 'status'},
    {
      '1': 'assigned_moderator_uuid',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'assignedModeratorUuid',
      '17': true
    },
    {
      '1': 'decision',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'decision',
      '17': true
    },
    {
      '1': 'removal_reason',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'removalReason',
      '17': true
    },
    {
      '1': 'history',
      '3': 13,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.ModerationEvent',
      '10': 'history'
    },
    {'1': 'created_at', '3': 14, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 15, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_details'},
    {'1': '_assigned_moderator_uuid'},
    {'1': '_decision'},
    {'1': '_removal_reason'},
  ],
};

/// Descriptor for `ContentReport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List contentReportDescriptor = $convert.base64Decode(
    'Cg1Db250ZW50UmVwb3J0EhIKBHV1aWQYASABKAlSBHV1aWQSHwoLdGFyZ2V0X3R5cGUYAiABKA'
    'lSCnRhcmdldFR5cGUSGwoJdGFyZ2V0X2lkGAMgASgJUgh0YXJnZXRJZBIXCgdwb3N0X2lkGAQg'
    'ASgJUgZwb3N0SWQSMAoUcmVwb3J0ZXJfcGVyc29uX3V1aWQYBSABKAlSEnJlcG9ydGVyUGVyc2'
    '9uVXVpZBIWCgZyZWFzb24YBiABKAlSBnJlYXNvbhIdCgdkZXRhaWxzGAcgASgJSABSB2RldGFp'
    'bHOIAQESGgoIcHJpb3JpdHkYCCABKAlSCHByaW9yaXR5EhYKBnN0YXR1cxgJIAEoCVIGc3RhdH'
    'VzEjsKF2Fzc2lnbmVkX21vZGVyYXRvcl91dWlkGAogASgJSAFSFWFzc2lnbmVkTW9kZXJhdG9y'
    'VXVpZIgBARIfCghkZWNpc2lvbhgLIAEoCUgCUghkZWNpc2lvbogBARIqCg5yZW1vdmFsX3JlYX'
    'NvbhgMIAEoCUgDUg1yZW1vdmFsUmVhc29uiAEBEjgKB2hpc3RvcnkYDSADKAsyHi5ncnBjLnRp'
    'bWVsaW5lLk1vZGVyYXRpb25FdmVudFIHaGlzdG9yeRIdCgpjcmVhdGVkX2F0GA4gASgJUgljcm'
    'VhdGVkQXQSHQoKdXBkYXRlZF9hdBgPIAEoCVIJdXBkYXRlZEF0QgoKCF9kZXRhaWxzQhoKGF9h'
    'c3NpZ25lZF9tb2RlcmF0b3JfdXVpZEILCglfZGVjaXNpb25CEQoPX3JlbW92YWxfcmVhc29u');

@$core.Deprecated('Use createReportRequestDescriptor instead')
const CreateReportRequest$json = {
  '1': 'CreateReportRequest',
  '2': [
    {'1': 'target_type', '3': 1, '4': 1, '5': 9, '10': 'targetType'},
    {'1': 'target_id', '3': 2, '4': 1, '5': 9, '10': 'targetId'},
    {'1': 'post_id', '3': 3, '4': 1, '5': 9, '10': 'postId'},
    {'1': 'reason', '3': 4, '4': 1, '5': 9, '10': 'reason'},
    {
      '1': 'details',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'details',
      '17': true
    },
  ],
  '8': [
    {'1': '_details'},
  ],
};

/// Descriptor for `CreateReportRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createReportRequestDescriptor = $convert.base64Decode(
    'ChNDcmVhdGVSZXBvcnRSZXF1ZXN0Eh8KC3RhcmdldF90eXBlGAEgASgJUgp0YXJnZXRUeXBlEh'
    'sKCXRhcmdldF9pZBgCIAEoCVIIdGFyZ2V0SWQSFwoHcG9zdF9pZBgDIAEoCVIGcG9zdElkEhYK'
    'BnJlYXNvbhgEIAEoCVIGcmVhc29uEh0KB2RldGFpbHMYBSABKAlIAFIHZGV0YWlsc4gBAUIKCg'
    'hfZGV0YWlscw==');

@$core.Deprecated('Use listReportsRequestDescriptor instead')
const ListReportsRequest$json = {
  '1': 'ListReportsRequest',
  '2': [
    {'1': 'status', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'status', '17': true},
  ],
  '8': [
    {'1': '_status'},
  ],
};

/// Descriptor for `ListReportsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listReportsRequestDescriptor = $convert.base64Decode(
    'ChJMaXN0UmVwb3J0c1JlcXVlc3QSGwoGc3RhdHVzGAEgASgJSABSBnN0YXR1c4gBAUIJCgdfc3'
    'RhdHVz');

@$core.Deprecated('Use listReportsResponseDescriptor instead')
const ListReportsResponse$json = {
  '1': 'ListReportsResponse',
  '2': [
    {
      '1': 'reports',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.ContentReport',
      '10': 'reports'
    },
  ],
};

/// Descriptor for `ListReportsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listReportsResponseDescriptor = $convert.base64Decode(
    'ChNMaXN0UmVwb3J0c1Jlc3BvbnNlEjYKB3JlcG9ydHMYASADKAsyHC5ncnBjLnRpbWVsaW5lLk'
    'NvbnRlbnRSZXBvcnRSB3JlcG9ydHM=');

@$core.Deprecated('Use decideReportRequestDescriptor instead')
const DecideReportRequest$json = {
  '1': 'DecideReportRequest',
  '2': [
    {'1': 'report_id', '3': 1, '4': 1, '5': 9, '10': 'reportId'},
    {'1': 'decision', '3': 2, '4': 1, '5': 9, '10': 'decision'},
    {'1': 'reason', '3': 3, '4': 1, '5': 9, '10': 'reason'},
  ],
};

/// Descriptor for `DecideReportRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decideReportRequestDescriptor = $convert.base64Decode(
    'ChNEZWNpZGVSZXBvcnRSZXF1ZXN0EhsKCXJlcG9ydF9pZBgBIAEoCVIIcmVwb3J0SWQSGgoIZG'
    'VjaXNpb24YAiABKAlSCGRlY2lzaW9uEhYKBnJlYXNvbhgDIAEoCVIGcmVhc29u');
