// This is a generated file - do not edit.
//
// Generated from timeline/notification.proto.

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

@$core.Deprecated('Use notificationDescriptor instead')
const Notification$json = {
  '1': 'Notification',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'uuid', '17': true},
    {
      '1': 'notification_type',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'notificationType'
    },
    {
      '1': 'recipient_person_uuid',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'recipientPersonUuid'
    },
    {'1': 'actor_person_uuid', '3': 4, '4': 1, '5': 9, '10': 'actorPersonUuid'},
    {'1': 'actor_name', '3': 5, '4': 1, '5': 9, '10': 'actorName'},
    {
      '1': 'post_uuid',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'postUuid',
      '17': true
    },
    {
      '1': 'comment_uuid',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'commentUuid',
      '17': true
    },
    {'1': 'entity_type', '3': 8, '4': 1, '5': 9, '10': 'entityType'},
    {'1': 'entity_uuid', '3': 9, '4': 1, '5': 9, '10': 'entityUuid'},
    {
      '1': 'target_type',
      '3': 10,
      '4': 1,
      '5': 9,
      '9': 3,
      '10': 'targetType',
      '17': true
    },
    {
      '1': 'target_uuid',
      '3': 11,
      '4': 1,
      '5': 9,
      '9': 4,
      '10': 'targetUuid',
      '17': true
    },
    {'1': 'snippet', '3': 12, '4': 1, '5': 9, '10': 'snippet'},
    {'1': 'read', '3': 13, '4': 1, '5': 8, '10': 'read'},
    {'1': 'created_at', '3': 14, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 15, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_uuid'},
    {'1': '_post_uuid'},
    {'1': '_comment_uuid'},
    {'1': '_target_type'},
    {'1': '_target_uuid'},
  ],
};

/// Descriptor for `Notification`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List notificationDescriptor = $convert.base64Decode(
    'CgxOb3RpZmljYXRpb24SFwoEdXVpZBgBIAEoCUgAUgR1dWlkiAEBEisKEW5vdGlmaWNhdGlvbl'
    '90eXBlGAIgASgJUhBub3RpZmljYXRpb25UeXBlEjIKFXJlY2lwaWVudF9wZXJzb25fdXVpZBgD'
    'IAEoCVITcmVjaXBpZW50UGVyc29uVXVpZBIqChFhY3Rvcl9wZXJzb25fdXVpZBgEIAEoCVIPYW'
    'N0b3JQZXJzb25VdWlkEh0KCmFjdG9yX25hbWUYBSABKAlSCWFjdG9yTmFtZRIgCglwb3N0X3V1'
    'aWQYBiABKAlIAVIIcG9zdFV1aWSIAQESJgoMY29tbWVudF91dWlkGAcgASgJSAJSC2NvbW1lbn'
    'RVdWlkiAEBEh8KC2VudGl0eV90eXBlGAggASgJUgplbnRpdHlUeXBlEh8KC2VudGl0eV91dWlk'
    'GAkgASgJUgplbnRpdHlVdWlkEiQKC3RhcmdldF90eXBlGAogASgJSANSCnRhcmdldFR5cGWIAQ'
    'ESJAoLdGFyZ2V0X3V1aWQYCyABKAlIBFIKdGFyZ2V0VXVpZIgBARIYCgdzbmlwcGV0GAwgASgJ'
    'UgdzbmlwcGV0EhIKBHJlYWQYDSABKAhSBHJlYWQSHQoKY3JlYXRlZF9hdBgOIAEoCVIJY3JlYX'
    'RlZEF0Eh0KCnVwZGF0ZWRfYXQYDyABKAlSCXVwZGF0ZWRBdEIHCgVfdXVpZEIMCgpfcG9zdF91'
    'dWlkQg8KDV9jb21tZW50X3V1aWRCDgoMX3RhcmdldF90eXBlQg4KDF90YXJnZXRfdXVpZA==');

@$core.Deprecated('Use listNotificationsRequestDescriptor instead')
const ListNotificationsRequest$json = {
  '1': 'ListNotificationsRequest',
  '2': [
    {'1': 'unread_only', '3': 1, '4': 1, '5': 8, '10': 'unreadOnly'},
    {'1': 'limit', '3': 2, '4': 1, '5': 13, '10': 'limit'},
  ],
};

/// Descriptor for `ListNotificationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationsRequestDescriptor =
    $convert.base64Decode(
        'ChhMaXN0Tm90aWZpY2F0aW9uc1JlcXVlc3QSHwoLdW5yZWFkX29ubHkYASABKAhSCnVucmVhZE'
        '9ubHkSFAoFbGltaXQYAiABKA1SBWxpbWl0');

@$core.Deprecated('Use listNotificationsResponseDescriptor instead')
const ListNotificationsResponse$json = {
  '1': 'ListNotificationsResponse',
  '2': [
    {
      '1': 'notifications',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Notification',
      '10': 'notifications'
    },
  ],
};

/// Descriptor for `ListNotificationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listNotificationsResponseDescriptor =
    $convert.base64Decode(
        'ChlMaXN0Tm90aWZpY2F0aW9uc1Jlc3BvbnNlEkEKDW5vdGlmaWNhdGlvbnMYASADKAsyGy5ncn'
        'BjLnRpbWVsaW5lLk5vdGlmaWNhdGlvblINbm90aWZpY2F0aW9ucw==');

@$core.Deprecated('Use markNotificationReadRequestDescriptor instead')
const MarkNotificationReadRequest$json = {
  '1': 'MarkNotificationReadRequest',
  '2': [
    {'1': 'idempotency_key', '3': 1, '4': 1, '5': 9, '10': 'idempotencyKey'},
  ],
};

/// Descriptor for `MarkNotificationReadRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markNotificationReadRequestDescriptor =
    $convert.base64Decode(
        'ChtNYXJrTm90aWZpY2F0aW9uUmVhZFJlcXVlc3QSJwoPaWRlbXBvdGVuY3lfa2V5GAEgASgJUg'
        '5pZGVtcG90ZW5jeUtleQ==');

@$core.Deprecated('Use markNotificationReadResponseDescriptor instead')
const MarkNotificationReadResponse$json = {
  '1': 'MarkNotificationReadResponse',
  '2': [
    {'1': 'read', '3': 1, '4': 1, '5': 8, '10': 'read'},
  ],
};

/// Descriptor for `MarkNotificationReadResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markNotificationReadResponseDescriptor =
    $convert.base64Decode(
        'ChxNYXJrTm90aWZpY2F0aW9uUmVhZFJlc3BvbnNlEhIKBHJlYWQYASABKAhSBHJlYWQ=');
