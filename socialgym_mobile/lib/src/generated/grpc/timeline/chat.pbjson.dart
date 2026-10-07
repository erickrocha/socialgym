// This is a generated file - do not edit.
//
// Generated from timeline/chat.proto.

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

@$core.Deprecated('Use messageMediaDescriptor instead')
const MessageMedia$json = {
  '1': 'MessageMedia',
  '2': [
    {'1': 'media_type', '3': 1, '4': 1, '5': 9, '10': 'mediaType'},
    {'1': 'object_key', '3': 2, '4': 1, '5': 9, '10': 'objectKey'},
    {'1': 'url', '3': 3, '4': 1, '5': 9, '10': 'url'},
  ],
};

/// Descriptor for `MessageMedia`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageMediaDescriptor = $convert.base64Decode(
    'CgxNZXNzYWdlTWVkaWESHQoKbWVkaWFfdHlwZRgBIAEoCVIJbWVkaWFUeXBlEh0KCm9iamVjdF'
    '9rZXkYAiABKAlSCW9iamVjdEtleRIQCgN1cmwYAyABKAlSA3VybA==');

@$core.Deprecated('Use conversationParticipantDescriptor instead')
const ConversationParticipant$json = {
  '1': 'ConversationParticipant',
  '2': [
    {'1': 'person_uuid', '3': 1, '4': 1, '5': 9, '10': 'personUuid'},
    {'1': 'role', '3': 2, '4': 1, '5': 9, '10': 'role'},
    {
      '1': 'last_read_at',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'lastReadAt',
      '17': true
    },
    {
      '1': 'last_read_message_uuid',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'lastReadMessageUuid',
      '17': true
    },
  ],
  '8': [
    {'1': '_last_read_at'},
    {'1': '_last_read_message_uuid'},
  ],
};

/// Descriptor for `ConversationParticipant`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List conversationParticipantDescriptor = $convert.base64Decode(
    'ChdDb252ZXJzYXRpb25QYXJ0aWNpcGFudBIfCgtwZXJzb25fdXVpZBgBIAEoCVIKcGVyc29uVX'
    'VpZBISCgRyb2xlGAIgASgJUgRyb2xlEiUKDGxhc3RfcmVhZF9hdBgDIAEoCUgAUgpsYXN0UmVh'
    'ZEF0iAEBEjgKFmxhc3RfcmVhZF9tZXNzYWdlX3V1aWQYBCABKAlIAVITbGFzdFJlYWRNZXNzYW'
    'dlVXVpZIgBAUIPCg1fbGFzdF9yZWFkX2F0QhkKF19sYXN0X3JlYWRfbWVzc2FnZV91dWlk');

@$core.Deprecated('Use lastMessagePreviewDescriptor instead')
const LastMessagePreview$json = {
  '1': 'LastMessagePreview',
  '2': [
    {'1': 'message_uuid', '3': 1, '4': 1, '5': 9, '10': 'messageUuid'},
    {
      '1': 'sender_person_uuid',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'senderPersonUuid'
    },
    {
      '1': 'sender_display_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'senderDisplayName'
    },
    {'1': 'snippet', '3': 4, '4': 1, '5': 9, '10': 'snippet'},
    {'1': 'sent_at', '3': 5, '4': 1, '5': 9, '10': 'sentAt'},
    {'1': 'has_media', '3': 6, '4': 1, '5': 8, '10': 'hasMedia'},
  ],
};

/// Descriptor for `LastMessagePreview`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List lastMessagePreviewDescriptor = $convert.base64Decode(
    'ChJMYXN0TWVzc2FnZVByZXZpZXcSIQoMbWVzc2FnZV91dWlkGAEgASgJUgttZXNzYWdlVXVpZB'
    'IsChJzZW5kZXJfcGVyc29uX3V1aWQYAiABKAlSEHNlbmRlclBlcnNvblV1aWQSLgoTc2VuZGVy'
    'X2Rpc3BsYXlfbmFtZRgDIAEoCVIRc2VuZGVyRGlzcGxheU5hbWUSGAoHc25pcHBldBgEIAEoCV'
    'IHc25pcHBldBIXCgdzZW50X2F0GAUgASgJUgZzZW50QXQSGwoJaGFzX21lZGlhGAYgASgIUgho'
    'YXNNZWRpYQ==');

@$core.Deprecated('Use conversationDescriptor instead')
const Conversation$json = {
  '1': 'Conversation',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '10': 'uuid'},
    {
      '1': 'conversation_type',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'conversationType'
    },
    {
      '1': 'business_profile_uuid',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'businessProfileUuid',
      '17': true
    },
    {
      '1': 'business_profile_name',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'businessProfileName',
      '17': true
    },
    {
      '1': 'business_profile_logo_url',
      '3': 5,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'businessProfileLogoUrl',
      '17': true
    },
    {
      '1': 'participant_person_uuids',
      '3': 6,
      '4': 3,
      '5': 9,
      '10': 'participantPersonUuids'
    },
    {
      '1': 'participants',
      '3': 7,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.ConversationParticipant',
      '10': 'participants'
    },
    {
      '1': 'last_message',
      '3': 8,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.LastMessagePreview',
      '9': 3,
      '10': 'lastMessage',
      '17': true
    },
    {'1': 'unread', '3': 9, '4': 1, '5': 8, '10': 'unread'},
    {'1': 'created_at', '3': 10, '4': 1, '5': 9, '10': 'createdAt'},
    {'1': 'updated_at', '3': 11, '4': 1, '5': 9, '10': 'updatedAt'},
  ],
  '8': [
    {'1': '_business_profile_uuid'},
    {'1': '_business_profile_name'},
    {'1': '_business_profile_logo_url'},
    {'1': '_last_message'},
  ],
};

/// Descriptor for `Conversation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List conversationDescriptor = $convert.base64Decode(
    'CgxDb252ZXJzYXRpb24SEgoEdXVpZBgBIAEoCVIEdXVpZBIrChFjb252ZXJzYXRpb25fdHlwZR'
    'gCIAEoCVIQY29udmVyc2F0aW9uVHlwZRI3ChVidXNpbmVzc19wcm9maWxlX3V1aWQYAyABKAlI'
    'AFITYnVzaW5lc3NQcm9maWxlVXVpZIgBARI3ChVidXNpbmVzc19wcm9maWxlX25hbWUYBCABKA'
    'lIAVITYnVzaW5lc3NQcm9maWxlTmFtZYgBARI+ChlidXNpbmVzc19wcm9maWxlX2xvZ29fdXJs'
    'GAUgASgJSAJSFmJ1c2luZXNzUHJvZmlsZUxvZ29VcmyIAQESOAoYcGFydGljaXBhbnRfcGVyc2'
    '9uX3V1aWRzGAYgAygJUhZwYXJ0aWNpcGFudFBlcnNvblV1aWRzEkoKDHBhcnRpY2lwYW50cxgH'
    'IAMoCzImLmdycGMudGltZWxpbmUuQ29udmVyc2F0aW9uUGFydGljaXBhbnRSDHBhcnRpY2lwYW'
    '50cxJJCgxsYXN0X21lc3NhZ2UYCCABKAsyIS5ncnBjLnRpbWVsaW5lLkxhc3RNZXNzYWdlUHJl'
    'dmlld0gDUgtsYXN0TWVzc2FnZYgBARIWCgZ1bnJlYWQYCSABKAhSBnVucmVhZBIdCgpjcmVhdG'
    'VkX2F0GAogASgJUgljcmVhdGVkQXQSHQoKdXBkYXRlZF9hdBgLIAEoCVIJdXBkYXRlZEF0QhgK'
    'Fl9idXNpbmVzc19wcm9maWxlX3V1aWRCGAoWX2J1c2luZXNzX3Byb2ZpbGVfbmFtZUIcChpfYn'
    'VzaW5lc3NfcHJvZmlsZV9sb2dvX3VybEIPCg1fbGFzdF9tZXNzYWdl');

@$core.Deprecated('Use messageDescriptor instead')
const Message$json = {
  '1': 'Message',
  '2': [
    {'1': 'uuid', '3': 1, '4': 1, '5': 9, '10': 'uuid'},
    {
      '1': 'conversation_uuid',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {
      '1': 'sender_person_uuid',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'senderPersonUuid'
    },
    {'1': 'sender_kind', '3': 4, '4': 1, '5': 9, '10': 'senderKind'},
    {
      '1': 'sender_display_name',
      '3': 5,
      '4': 1,
      '5': 9,
      '10': 'senderDisplayName'
    },
    {
      '1': 'sender_avatar_url',
      '3': 6,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'senderAvatarUrl',
      '17': true
    },
    {
      '1': 'sender_business_profile_uuid',
      '3': 7,
      '4': 1,
      '5': 9,
      '9': 1,
      '10': 'senderBusinessProfileUuid',
      '17': true
    },
    {'1': 'body', '3': 8, '4': 1, '5': 9, '10': 'body'},
    {
      '1': 'media',
      '3': 9,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.MessageMedia',
      '10': 'media'
    },
    {
      '1': 'client_message_id',
      '3': 10,
      '4': 1,
      '5': 9,
      '10': 'clientMessageId'
    },
    {'1': 'sent_at', '3': 11, '4': 1, '5': 9, '10': 'sentAt'},
  ],
  '8': [
    {'1': '_sender_avatar_url'},
    {'1': '_sender_business_profile_uuid'},
  ],
};

/// Descriptor for `Message`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageDescriptor = $convert.base64Decode(
    'CgdNZXNzYWdlEhIKBHV1aWQYASABKAlSBHV1aWQSKwoRY29udmVyc2F0aW9uX3V1aWQYAiABKA'
    'lSEGNvbnZlcnNhdGlvblV1aWQSLAoSc2VuZGVyX3BlcnNvbl91dWlkGAMgASgJUhBzZW5kZXJQ'
    'ZXJzb25VdWlkEh8KC3NlbmRlcl9raW5kGAQgASgJUgpzZW5kZXJLaW5kEi4KE3NlbmRlcl9kaX'
    'NwbGF5X25hbWUYBSABKAlSEXNlbmRlckRpc3BsYXlOYW1lEi8KEXNlbmRlcl9hdmF0YXJfdXJs'
    'GAYgASgJSABSD3NlbmRlckF2YXRhclVybIgBARJEChxzZW5kZXJfYnVzaW5lc3NfcHJvZmlsZV'
    '91dWlkGAcgASgJSAFSGXNlbmRlckJ1c2luZXNzUHJvZmlsZVV1aWSIAQESEgoEYm9keRgIIAEo'
    'CVIEYm9keRIxCgVtZWRpYRgJIAMoCzIbLmdycGMudGltZWxpbmUuTWVzc2FnZU1lZGlhUgVtZW'
    'RpYRIqChFjbGllbnRfbWVzc2FnZV9pZBgKIAEoCVIPY2xpZW50TWVzc2FnZUlkEhcKB3NlbnRf'
    'YXQYCyABKAlSBnNlbnRBdEIUChJfc2VuZGVyX2F2YXRhcl91cmxCHwodX3NlbmRlcl9idXNpbm'
    'Vzc19wcm9maWxlX3V1aWQ=');

@$core.Deprecated('Use listConversationsRequestDescriptor instead')
const ListConversationsRequest$json = {
  '1': 'ListConversationsRequest',
  '2': [
    {'1': 'page', '3': 1, '4': 1, '5': 13, '10': 'page'},
  ],
};

/// Descriptor for `ListConversationsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listConversationsRequestDescriptor =
    $convert.base64Decode(
        'ChhMaXN0Q29udmVyc2F0aW9uc1JlcXVlc3QSEgoEcGFnZRgBIAEoDVIEcGFnZQ==');

@$core.Deprecated('Use listConversationsResponseDescriptor instead')
const ListConversationsResponse$json = {
  '1': 'ListConversationsResponse',
  '2': [
    {
      '1': 'conversations',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Conversation',
      '10': 'conversations'
    },
  ],
};

/// Descriptor for `ListConversationsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listConversationsResponseDescriptor =
    $convert.base64Decode(
        'ChlMaXN0Q29udmVyc2F0aW9uc1Jlc3BvbnNlEkEKDWNvbnZlcnNhdGlvbnMYASADKAsyGy5ncn'
        'BjLnRpbWVsaW5lLkNvbnZlcnNhdGlvblINY29udmVyc2F0aW9ucw==');

@$core.Deprecated('Use getPresenceRequestDescriptor instead')
const GetPresenceRequest$json = {
  '1': 'GetPresenceRequest',
  '2': [
    {'1': 'person_uuids', '3': 1, '4': 3, '5': 9, '10': 'personUuids'},
  ],
};

/// Descriptor for `GetPresenceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPresenceRequestDescriptor = $convert.base64Decode(
    'ChJHZXRQcmVzZW5jZVJlcXVlc3QSIQoMcGVyc29uX3V1aWRzGAEgAygJUgtwZXJzb25VdWlkcw'
    '==');

@$core.Deprecated('Use getPresenceResponseDescriptor instead')
const GetPresenceResponse$json = {
  '1': 'GetPresenceResponse',
  '2': [
    {'1': 'online', '3': 1, '4': 3, '5': 9, '10': 'online'},
  ],
};

/// Descriptor for `GetPresenceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPresenceResponseDescriptor =
    $convert.base64Decode(
        'ChNHZXRQcmVzZW5jZVJlc3BvbnNlEhYKBm9ubGluZRgBIAMoCVIGb25saW5l');

@$core.Deprecated('Use createDirectConversationRequestDescriptor instead')
const CreateDirectConversationRequest$json = {
  '1': 'CreateDirectConversationRequest',
  '2': [
    {
      '1': 'target_person_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'targetPersonUuid'
    },
  ],
};

/// Descriptor for `CreateDirectConversationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createDirectConversationRequestDescriptor =
    $convert.base64Decode(
        'Ch9DcmVhdGVEaXJlY3RDb252ZXJzYXRpb25SZXF1ZXN0EiwKEnRhcmdldF9wZXJzb25fdXVpZB'
        'gBIAEoCVIQdGFyZ2V0UGVyc29uVXVpZA==');

@$core.Deprecated('Use createBusinessTeamGroupRequestDescriptor instead')
const CreateBusinessTeamGroupRequest$json = {
  '1': 'CreateBusinessTeamGroupRequest',
  '2': [
    {
      '1': 'business_profile_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'businessProfileUuid'
    },
  ],
};

/// Descriptor for `CreateBusinessTeamGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createBusinessTeamGroupRequestDescriptor =
    $convert.base64Decode(
        'Ch5DcmVhdGVCdXNpbmVzc1RlYW1Hcm91cFJlcXVlc3QSMgoVYnVzaW5lc3NfcHJvZmlsZV91dW'
        'lkGAEgASgJUhNidXNpbmVzc1Byb2ZpbGVVdWlk');

@$core
    .Deprecated('Use createBusinessDirectConversationRequestDescriptor instead')
const CreateBusinessDirectConversationRequest$json = {
  '1': 'CreateBusinessDirectConversationRequest',
  '2': [
    {
      '1': 'business_profile_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'businessProfileUuid'
    },
    {
      '1': 'member_person_uuid',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'memberPersonUuid',
      '17': true
    },
  ],
  '8': [
    {'1': '_member_person_uuid'},
  ],
};

/// Descriptor for `CreateBusinessDirectConversationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createBusinessDirectConversationRequestDescriptor =
    $convert.base64Decode(
        'CidDcmVhdGVCdXNpbmVzc0RpcmVjdENvbnZlcnNhdGlvblJlcXVlc3QSMgoVYnVzaW5lc3NfcH'
        'JvZmlsZV91dWlkGAEgASgJUhNidXNpbmVzc1Byb2ZpbGVVdWlkEjEKEm1lbWJlcl9wZXJzb25f'
        'dXVpZBgCIAEoCUgAUhBtZW1iZXJQZXJzb25VdWlkiAEBQhUKE19tZW1iZXJfcGVyc29uX3V1aW'
        'Q=');

@$core.Deprecated('Use listMessagesRequestDescriptor instead')
const ListMessagesRequest$json = {
  '1': 'ListMessagesRequest',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {'1': 'page', '3': 2, '4': 1, '5': 13, '10': 'page'},
    {'1': 'since_epoch_ms', '3': 3, '4': 1, '5': 3, '10': 'sinceEpochMs'},
  ],
};

/// Descriptor for `ListMessagesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listMessagesRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0TWVzc2FnZXNSZXF1ZXN0EisKEWNvbnZlcnNhdGlvbl91dWlkGAEgASgJUhBjb252ZX'
    'JzYXRpb25VdWlkEhIKBHBhZ2UYAiABKA1SBHBhZ2USJAoOc2luY2VfZXBvY2hfbXMYAyABKANS'
    'DHNpbmNlRXBvY2hNcw==');

@$core.Deprecated('Use listMessagesResponseDescriptor instead')
const ListMessagesResponse$json = {
  '1': 'ListMessagesResponse',
  '2': [
    {
      '1': 'messages',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.Message',
      '10': 'messages'
    },
  ],
};

/// Descriptor for `ListMessagesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listMessagesResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0TWVzc2FnZXNSZXNwb25zZRIyCghtZXNzYWdlcxgBIAMoCzIWLmdycGMudGltZWxpbm'
    'UuTWVzc2FnZVIIbWVzc2FnZXM=');

@$core.Deprecated('Use sendMessageRequestDescriptor instead')
const SendMessageRequest$json = {
  '1': 'SendMessageRequest',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {'1': 'body', '3': 2, '4': 1, '5': 9, '10': 'body'},
    {
      '1': 'media',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.MessageMedia',
      '10': 'media'
    },
    {'1': 'client_message_id', '3': 4, '4': 1, '5': 9, '10': 'clientMessageId'},
  ],
};

/// Descriptor for `SendMessageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendMessageRequestDescriptor = $convert.base64Decode(
    'ChJTZW5kTWVzc2FnZVJlcXVlc3QSKwoRY29udmVyc2F0aW9uX3V1aWQYASABKAlSEGNvbnZlcn'
    'NhdGlvblV1aWQSEgoEYm9keRgCIAEoCVIEYm9keRIxCgVtZWRpYRgDIAMoCzIbLmdycGMudGlt'
    'ZWxpbmUuTWVzc2FnZU1lZGlhUgVtZWRpYRIqChFjbGllbnRfbWVzc2FnZV9pZBgEIAEoCVIPY2'
    'xpZW50TWVzc2FnZUlk');

@$core.Deprecated('Use markConversationReadRequestDescriptor instead')
const MarkConversationReadRequest$json = {
  '1': 'MarkConversationReadRequest',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {
      '1': 'last_read_message_uuid',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'lastReadMessageUuid'
    },
  ],
};

/// Descriptor for `MarkConversationReadRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markConversationReadRequestDescriptor =
    $convert.base64Decode(
        'ChtNYXJrQ29udmVyc2F0aW9uUmVhZFJlcXVlc3QSKwoRY29udmVyc2F0aW9uX3V1aWQYASABKA'
        'lSEGNvbnZlcnNhdGlvblV1aWQSMwoWbGFzdF9yZWFkX21lc3NhZ2VfdXVpZBgCIAEoCVITbGFz'
        'dFJlYWRNZXNzYWdlVXVpZA==');

@$core.Deprecated('Use markConversationReadResponseDescriptor instead')
const MarkConversationReadResponse$json = {
  '1': 'MarkConversationReadResponse',
  '2': [
    {'1': 'read', '3': 1, '4': 1, '5': 8, '10': 'read'},
  ],
};

/// Descriptor for `MarkConversationReadResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markConversationReadResponseDescriptor =
    $convert.base64Decode(
        'ChxNYXJrQ29udmVyc2F0aW9uUmVhZFJlc3BvbnNlEhIKBHJlYWQYASABKAhSBHJlYWQ=');

@$core.Deprecated('Use sendFrameDescriptor instead')
const SendFrame$json = {
  '1': 'SendFrame',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {'1': 'body', '3': 2, '4': 1, '5': 9, '10': 'body'},
    {
      '1': 'media',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.grpc.timeline.MessageMedia',
      '10': 'media'
    },
    {'1': 'client_message_id', '3': 4, '4': 1, '5': 9, '10': 'clientMessageId'},
  ],
};

/// Descriptor for `SendFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFrameDescriptor = $convert.base64Decode(
    'CglTZW5kRnJhbWUSKwoRY29udmVyc2F0aW9uX3V1aWQYASABKAlSEGNvbnZlcnNhdGlvblV1aW'
    'QSEgoEYm9keRgCIAEoCVIEYm9keRIxCgVtZWRpYRgDIAMoCzIbLmdycGMudGltZWxpbmUuTWVz'
    'c2FnZU1lZGlhUgVtZWRpYRIqChFjbGllbnRfbWVzc2FnZV9pZBgEIAEoCVIPY2xpZW50TWVzc2'
    'FnZUlk');

@$core.Deprecated('Use readFrameDescriptor instead')
const ReadFrame$json = {
  '1': 'ReadFrame',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {
      '1': 'last_read_message_uuid',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'lastReadMessageUuid'
    },
  ],
};

/// Descriptor for `ReadFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List readFrameDescriptor = $convert.base64Decode(
    'CglSZWFkRnJhbWUSKwoRY29udmVyc2F0aW9uX3V1aWQYASABKAlSEGNvbnZlcnNhdGlvblV1aW'
    'QSMwoWbGFzdF9yZWFkX21lc3NhZ2VfdXVpZBgCIAEoCVITbGFzdFJlYWRNZXNzYWdlVXVpZA==');

@$core.Deprecated('Use typingFrameDescriptor instead')
const TypingFrame$json = {
  '1': 'TypingFrame',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
  ],
};

/// Descriptor for `TypingFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List typingFrameDescriptor = $convert.base64Decode(
    'CgtUeXBpbmdGcmFtZRIrChFjb252ZXJzYXRpb25fdXVpZBgBIAEoCVIQY29udmVyc2F0aW9uVX'
    'VpZA==');

@$core.Deprecated('Use pingFrameDescriptor instead')
const PingFrame$json = {
  '1': 'PingFrame',
};

/// Descriptor for `PingFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pingFrameDescriptor =
    $convert.base64Decode('CglQaW5nRnJhbWU=');

@$core.Deprecated('Use clientFrameDescriptor instead')
const ClientFrame$json = {
  '1': 'ClientFrame',
  '2': [
    {
      '1': 'send',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.SendFrame',
      '9': 0,
      '10': 'send'
    },
    {
      '1': 'read',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.ReadFrame',
      '9': 0,
      '10': 'read'
    },
    {
      '1': 'typing',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.TypingFrame',
      '9': 0,
      '10': 'typing'
    },
    {
      '1': 'ping',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.PingFrame',
      '9': 0,
      '10': 'ping'
    },
  ],
  '8': [
    {'1': 'frame'},
  ],
};

/// Descriptor for `ClientFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientFrameDescriptor = $convert.base64Decode(
    'CgtDbGllbnRGcmFtZRIuCgRzZW5kGAEgASgLMhguZ3JwYy50aW1lbGluZS5TZW5kRnJhbWVIAF'
    'IEc2VuZBIuCgRyZWFkGAIgASgLMhguZ3JwYy50aW1lbGluZS5SZWFkRnJhbWVIAFIEcmVhZBI0'
    'CgZ0eXBpbmcYAyABKAsyGi5ncnBjLnRpbWVsaW5lLlR5cGluZ0ZyYW1lSABSBnR5cGluZxIuCg'
    'RwaW5nGAQgASgLMhguZ3JwYy50aW1lbGluZS5QaW5nRnJhbWVIAFIEcGluZ0IHCgVmcmFtZQ==');

@$core.Deprecated('Use messageNewEventDescriptor instead')
const MessageNewEvent$json = {
  '1': 'MessageNewEvent',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {
      '1': 'conversation_type',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'conversationType'
    },
    {
      '1': 'message',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.Message',
      '10': 'message'
    },
  ],
};

/// Descriptor for `MessageNewEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageNewEventDescriptor = $convert.base64Decode(
    'Cg9NZXNzYWdlTmV3RXZlbnQSKwoRY29udmVyc2F0aW9uX3V1aWQYASABKAlSEGNvbnZlcnNhdG'
    'lvblV1aWQSKwoRY29udmVyc2F0aW9uX3R5cGUYAiABKAlSEGNvbnZlcnNhdGlvblR5cGUSMAoH'
    'bWVzc2FnZRgDIAEoCzIWLmdycGMudGltZWxpbmUuTWVzc2FnZVIHbWVzc2FnZQ==');

@$core.Deprecated('Use conversationUpdatedEventDescriptor instead')
const ConversationUpdatedEvent$json = {
  '1': 'ConversationUpdatedEvent',
  '2': [
    {
      '1': 'conversation',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.Conversation',
      '10': 'conversation'
    },
  ],
};

/// Descriptor for `ConversationUpdatedEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List conversationUpdatedEventDescriptor =
    $convert.base64Decode(
        'ChhDb252ZXJzYXRpb25VcGRhdGVkRXZlbnQSPwoMY29udmVyc2F0aW9uGAEgASgLMhsuZ3JwYy'
        '50aW1lbGluZS5Db252ZXJzYXRpb25SDGNvbnZlcnNhdGlvbg==');

@$core.Deprecated('Use messageReadEventDescriptor instead')
const MessageReadEvent$json = {
  '1': 'MessageReadEvent',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {'1': 'person_uuid', '3': 2, '4': 1, '5': 9, '10': 'personUuid'},
    {
      '1': 'last_read_message_uuid',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'lastReadMessageUuid'
    },
  ],
};

/// Descriptor for `MessageReadEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageReadEventDescriptor = $convert.base64Decode(
    'ChBNZXNzYWdlUmVhZEV2ZW50EisKEWNvbnZlcnNhdGlvbl91dWlkGAEgASgJUhBjb252ZXJzYX'
    'Rpb25VdWlkEh8KC3BlcnNvbl91dWlkGAIgASgJUgpwZXJzb25VdWlkEjMKFmxhc3RfcmVhZF9t'
    'ZXNzYWdlX3V1aWQYAyABKAlSE2xhc3RSZWFkTWVzc2FnZVV1aWQ=');

@$core.Deprecated('Use typingEventDescriptor instead')
const TypingEvent$json = {
  '1': 'TypingEvent',
  '2': [
    {
      '1': 'conversation_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'conversationUuid'
    },
    {'1': 'person_uuid', '3': 2, '4': 1, '5': 9, '10': 'personUuid'},
  ],
};

/// Descriptor for `TypingEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List typingEventDescriptor = $convert.base64Decode(
    'CgtUeXBpbmdFdmVudBIrChFjb252ZXJzYXRpb25fdXVpZBgBIAEoCVIQY29udmVyc2F0aW9uVX'
    'VpZBIfCgtwZXJzb25fdXVpZBgCIAEoCVIKcGVyc29uVXVpZA==');

@$core.Deprecated('Use pongEventDescriptor instead')
const PongEvent$json = {
  '1': 'PongEvent',
};

/// Descriptor for `PongEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pongEventDescriptor =
    $convert.base64Decode('CglQb25nRXZlbnQ=');

@$core.Deprecated('Use errorEventDescriptor instead')
const ErrorEvent$json = {
  '1': 'ErrorEvent',
  '2': [
    {'1': 'message', '3': 1, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `ErrorEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List errorEventDescriptor = $convert
    .base64Decode('CgpFcnJvckV2ZW50EhgKB21lc3NhZ2UYASABKAlSB21lc3NhZ2U=');

@$core.Deprecated('Use serverFrameDescriptor instead')
const ServerFrame$json = {
  '1': 'ServerFrame',
  '2': [
    {
      '1': 'message_new',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.MessageNewEvent',
      '9': 0,
      '10': 'messageNew'
    },
    {
      '1': 'conversation_updated',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.ConversationUpdatedEvent',
      '9': 0,
      '10': 'conversationUpdated'
    },
    {
      '1': 'message_read',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.MessageReadEvent',
      '9': 0,
      '10': 'messageRead'
    },
    {
      '1': 'typing',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.TypingEvent',
      '9': 0,
      '10': 'typing'
    },
    {
      '1': 'pong',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.PongEvent',
      '9': 0,
      '10': 'pong'
    },
    {
      '1': 'error',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.grpc.timeline.ErrorEvent',
      '9': 0,
      '10': 'error'
    },
  ],
  '8': [
    {'1': 'event'},
  ],
};

/// Descriptor for `ServerFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverFrameDescriptor = $convert.base64Decode(
    'CgtTZXJ2ZXJGcmFtZRJBCgttZXNzYWdlX25ldxgBIAEoCzIeLmdycGMudGltZWxpbmUuTWVzc2'
    'FnZU5ld0V2ZW50SABSCm1lc3NhZ2VOZXcSXAoUY29udmVyc2F0aW9uX3VwZGF0ZWQYAiABKAsy'
    'Jy5ncnBjLnRpbWVsaW5lLkNvbnZlcnNhdGlvblVwZGF0ZWRFdmVudEgAUhNjb252ZXJzYXRpb2'
    '5VcGRhdGVkEkQKDG1lc3NhZ2VfcmVhZBgDIAEoCzIfLmdycGMudGltZWxpbmUuTWVzc2FnZVJl'
    'YWRFdmVudEgAUgttZXNzYWdlUmVhZBI0CgZ0eXBpbmcYBCABKAsyGi5ncnBjLnRpbWVsaW5lLl'
    'R5cGluZ0V2ZW50SABSBnR5cGluZxIuCgRwb25nGAUgASgLMhguZ3JwYy50aW1lbGluZS5Qb25n'
    'RXZlbnRIAFIEcG9uZxIxCgVlcnJvchgGIAEoCzIZLmdycGMudGltZWxpbmUuRXJyb3JFdmVudE'
    'gAUgVlcnJvckIHCgVldmVudA==');
