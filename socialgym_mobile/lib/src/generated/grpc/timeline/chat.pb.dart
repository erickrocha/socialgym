// This is a generated file - do not edit.
//
// Generated from timeline/chat.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class MessageMedia extends $pb.GeneratedMessage {
  factory MessageMedia({
    $core.String? mediaType,
    $core.String? objectKey,
    $core.String? url,
  }) {
    final result = create();
    if (mediaType != null) result.mediaType = mediaType;
    if (objectKey != null) result.objectKey = objectKey;
    if (url != null) result.url = url;
    return result;
  }

  MessageMedia._();

  factory MessageMedia.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MessageMedia.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MessageMedia',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'mediaType')
    ..aOS(2, _omitFieldNames ? '' : 'objectKey')
    ..aOS(3, _omitFieldNames ? '' : 'url')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageMedia clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageMedia copyWith(void Function(MessageMedia) updates) =>
      super.copyWith((message) => updates(message as MessageMedia))
          as MessageMedia;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MessageMedia create() => MessageMedia._();
  @$core.override
  MessageMedia createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MessageMedia getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MessageMedia>(create);
  static MessageMedia? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get mediaType => $_getSZ(0);
  @$pb.TagNumber(1)
  set mediaType($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMediaType() => $_has(0);
  @$pb.TagNumber(1)
  void clearMediaType() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get objectKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjectKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectKey() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get url => $_getSZ(2);
  @$pb.TagNumber(3)
  set url($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearUrl() => $_clearField(3);
}

class ConversationParticipant extends $pb.GeneratedMessage {
  factory ConversationParticipant({
    $core.String? personUuid,
    $core.String? role,
    $core.String? lastReadAt,
    $core.String? lastReadMessageUuid,
  }) {
    final result = create();
    if (personUuid != null) result.personUuid = personUuid;
    if (role != null) result.role = role;
    if (lastReadAt != null) result.lastReadAt = lastReadAt;
    if (lastReadMessageUuid != null)
      result.lastReadMessageUuid = lastReadMessageUuid;
    return result;
  }

  ConversationParticipant._();

  factory ConversationParticipant.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConversationParticipant.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConversationParticipant',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'personUuid')
    ..aOS(2, _omitFieldNames ? '' : 'role')
    ..aOS(3, _omitFieldNames ? '' : 'lastReadAt')
    ..aOS(4, _omitFieldNames ? '' : 'lastReadMessageUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConversationParticipant clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConversationParticipant copyWith(
          void Function(ConversationParticipant) updates) =>
      super.copyWith((message) => updates(message as ConversationParticipant))
          as ConversationParticipant;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConversationParticipant create() => ConversationParticipant._();
  @$core.override
  ConversationParticipant createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConversationParticipant getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConversationParticipant>(create);
  static ConversationParticipant? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get personUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set personUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPersonUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearPersonUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get role => $_getSZ(1);
  @$pb.TagNumber(2)
  set role($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get lastReadAt => $_getSZ(2);
  @$pb.TagNumber(3)
  set lastReadAt($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLastReadAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearLastReadAt() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get lastReadMessageUuid => $_getSZ(3);
  @$pb.TagNumber(4)
  set lastReadMessageUuid($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLastReadMessageUuid() => $_has(3);
  @$pb.TagNumber(4)
  void clearLastReadMessageUuid() => $_clearField(4);
}

class LastMessagePreview extends $pb.GeneratedMessage {
  factory LastMessagePreview({
    $core.String? messageUuid,
    $core.String? senderPersonUuid,
    $core.String? senderDisplayName,
    $core.String? snippet,
    $core.String? sentAt,
    $core.bool? hasMedia,
  }) {
    final result = create();
    if (messageUuid != null) result.messageUuid = messageUuid;
    if (senderPersonUuid != null) result.senderPersonUuid = senderPersonUuid;
    if (senderDisplayName != null) result.senderDisplayName = senderDisplayName;
    if (snippet != null) result.snippet = snippet;
    if (sentAt != null) result.sentAt = sentAt;
    if (hasMedia != null) result.hasMedia = hasMedia;
    return result;
  }

  LastMessagePreview._();

  factory LastMessagePreview.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LastMessagePreview.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LastMessagePreview',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'messageUuid')
    ..aOS(2, _omitFieldNames ? '' : 'senderPersonUuid')
    ..aOS(3, _omitFieldNames ? '' : 'senderDisplayName')
    ..aOS(4, _omitFieldNames ? '' : 'snippet')
    ..aOS(5, _omitFieldNames ? '' : 'sentAt')
    ..aOB(6, _omitFieldNames ? '' : 'hasMedia')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LastMessagePreview clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LastMessagePreview copyWith(void Function(LastMessagePreview) updates) =>
      super.copyWith((message) => updates(message as LastMessagePreview))
          as LastMessagePreview;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LastMessagePreview create() => LastMessagePreview._();
  @$core.override
  LastMessagePreview createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LastMessagePreview getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LastMessagePreview>(create);
  static LastMessagePreview? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get messageUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set messageUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessageUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get senderPersonUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set senderPersonUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSenderPersonUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderPersonUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get senderDisplayName => $_getSZ(2);
  @$pb.TagNumber(3)
  set senderDisplayName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSenderDisplayName() => $_has(2);
  @$pb.TagNumber(3)
  void clearSenderDisplayName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get snippet => $_getSZ(3);
  @$pb.TagNumber(4)
  set snippet($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSnippet() => $_has(3);
  @$pb.TagNumber(4)
  void clearSnippet() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get sentAt => $_getSZ(4);
  @$pb.TagNumber(5)
  set sentAt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSentAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearSentAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get hasMedia => $_getBF(5);
  @$pb.TagNumber(6)
  set hasMedia($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasHasMedia() => $_has(5);
  @$pb.TagNumber(6)
  void clearHasMedia() => $_clearField(6);
}

class Conversation extends $pb.GeneratedMessage {
  factory Conversation({
    $core.String? uuid,
    $core.String? conversationType,
    $core.String? businessProfileUuid,
    $core.String? businessProfileName,
    $core.String? businessProfileLogoUrl,
    $core.Iterable<$core.String>? participantPersonUuids,
    $core.Iterable<ConversationParticipant>? participants,
    LastMessagePreview? lastMessage,
    $core.bool? unread,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (conversationType != null) result.conversationType = conversationType;
    if (businessProfileUuid != null)
      result.businessProfileUuid = businessProfileUuid;
    if (businessProfileName != null)
      result.businessProfileName = businessProfileName;
    if (businessProfileLogoUrl != null)
      result.businessProfileLogoUrl = businessProfileLogoUrl;
    if (participantPersonUuids != null)
      result.participantPersonUuids.addAll(participantPersonUuids);
    if (participants != null) result.participants.addAll(participants);
    if (lastMessage != null) result.lastMessage = lastMessage;
    if (unread != null) result.unread = unread;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Conversation._();

  factory Conversation.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Conversation.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Conversation',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'conversationType')
    ..aOS(3, _omitFieldNames ? '' : 'businessProfileUuid')
    ..aOS(4, _omitFieldNames ? '' : 'businessProfileName')
    ..aOS(5, _omitFieldNames ? '' : 'businessProfileLogoUrl')
    ..pPS(6, _omitFieldNames ? '' : 'participantPersonUuids')
    ..pPM<ConversationParticipant>(7, _omitFieldNames ? '' : 'participants',
        subBuilder: ConversationParticipant.create)
    ..aOM<LastMessagePreview>(8, _omitFieldNames ? '' : 'lastMessage',
        subBuilder: LastMessagePreview.create)
    ..aOB(9, _omitFieldNames ? '' : 'unread')
    ..aOS(10, _omitFieldNames ? '' : 'createdAt')
    ..aOS(11, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Conversation clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Conversation copyWith(void Function(Conversation) updates) =>
      super.copyWith((message) => updates(message as Conversation))
          as Conversation;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Conversation create() => Conversation._();
  @$core.override
  Conversation createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Conversation getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Conversation>(create);
  static Conversation? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get conversationType => $_getSZ(1);
  @$pb.TagNumber(2)
  set conversationType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasConversationType() => $_has(1);
  @$pb.TagNumber(2)
  void clearConversationType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get businessProfileUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set businessProfileUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBusinessProfileUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearBusinessProfileUuid() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get businessProfileName => $_getSZ(3);
  @$pb.TagNumber(4)
  set businessProfileName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBusinessProfileName() => $_has(3);
  @$pb.TagNumber(4)
  void clearBusinessProfileName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get businessProfileLogoUrl => $_getSZ(4);
  @$pb.TagNumber(5)
  set businessProfileLogoUrl($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasBusinessProfileLogoUrl() => $_has(4);
  @$pb.TagNumber(5)
  void clearBusinessProfileLogoUrl() => $_clearField(5);

  @$pb.TagNumber(6)
  $pb.PbList<$core.String> get participantPersonUuids => $_getList(5);

  @$pb.TagNumber(7)
  $pb.PbList<ConversationParticipant> get participants => $_getList(6);

  @$pb.TagNumber(8)
  LastMessagePreview get lastMessage => $_getN(7);
  @$pb.TagNumber(8)
  set lastMessage(LastMessagePreview value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasLastMessage() => $_has(7);
  @$pb.TagNumber(8)
  void clearLastMessage() => $_clearField(8);
  @$pb.TagNumber(8)
  LastMessagePreview ensureLastMessage() => $_ensure(7);

  @$pb.TagNumber(9)
  $core.bool get unread => $_getBF(8);
  @$pb.TagNumber(9)
  set unread($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasUnread() => $_has(8);
  @$pb.TagNumber(9)
  void clearUnread() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get createdAt => $_getSZ(9);
  @$pb.TagNumber(10)
  set createdAt($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCreatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearCreatedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get updatedAt => $_getSZ(10);
  @$pb.TagNumber(11)
  set updatedAt($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasUpdatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearUpdatedAt() => $_clearField(11);
}

class Message extends $pb.GeneratedMessage {
  factory Message({
    $core.String? uuid,
    $core.String? conversationUuid,
    $core.String? senderPersonUuid,
    $core.String? senderKind,
    $core.String? senderDisplayName,
    $core.String? senderAvatarUrl,
    $core.String? senderBusinessProfileUuid,
    $core.String? body,
    $core.Iterable<MessageMedia>? media,
    $core.String? clientMessageId,
    $core.String? sentAt,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (senderPersonUuid != null) result.senderPersonUuid = senderPersonUuid;
    if (senderKind != null) result.senderKind = senderKind;
    if (senderDisplayName != null) result.senderDisplayName = senderDisplayName;
    if (senderAvatarUrl != null) result.senderAvatarUrl = senderAvatarUrl;
    if (senderBusinessProfileUuid != null)
      result.senderBusinessProfileUuid = senderBusinessProfileUuid;
    if (body != null) result.body = body;
    if (media != null) result.media.addAll(media);
    if (clientMessageId != null) result.clientMessageId = clientMessageId;
    if (sentAt != null) result.sentAt = sentAt;
    return result;
  }

  Message._();

  factory Message.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Message.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Message',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(3, _omitFieldNames ? '' : 'senderPersonUuid')
    ..aOS(4, _omitFieldNames ? '' : 'senderKind')
    ..aOS(5, _omitFieldNames ? '' : 'senderDisplayName')
    ..aOS(6, _omitFieldNames ? '' : 'senderAvatarUrl')
    ..aOS(7, _omitFieldNames ? '' : 'senderBusinessProfileUuid')
    ..aOS(8, _omitFieldNames ? '' : 'body')
    ..pPM<MessageMedia>(9, _omitFieldNames ? '' : 'media',
        subBuilder: MessageMedia.create)
    ..aOS(10, _omitFieldNames ? '' : 'clientMessageId')
    ..aOS(11, _omitFieldNames ? '' : 'sentAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Message clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Message copyWith(void Function(Message) updates) =>
      super.copyWith((message) => updates(message as Message)) as Message;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Message create() => Message._();
  @$core.override
  Message createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Message getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Message>(create);
  static Message? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get conversationUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set conversationUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasConversationUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearConversationUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get senderPersonUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set senderPersonUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSenderPersonUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearSenderPersonUuid() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get senderKind => $_getSZ(3);
  @$pb.TagNumber(4)
  set senderKind($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSenderKind() => $_has(3);
  @$pb.TagNumber(4)
  void clearSenderKind() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get senderDisplayName => $_getSZ(4);
  @$pb.TagNumber(5)
  set senderDisplayName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSenderDisplayName() => $_has(4);
  @$pb.TagNumber(5)
  void clearSenderDisplayName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get senderAvatarUrl => $_getSZ(5);
  @$pb.TagNumber(6)
  set senderAvatarUrl($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasSenderAvatarUrl() => $_has(5);
  @$pb.TagNumber(6)
  void clearSenderAvatarUrl() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get senderBusinessProfileUuid => $_getSZ(6);
  @$pb.TagNumber(7)
  set senderBusinessProfileUuid($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSenderBusinessProfileUuid() => $_has(6);
  @$pb.TagNumber(7)
  void clearSenderBusinessProfileUuid() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get body => $_getSZ(7);
  @$pb.TagNumber(8)
  set body($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBody() => $_has(7);
  @$pb.TagNumber(8)
  void clearBody() => $_clearField(8);

  @$pb.TagNumber(9)
  $pb.PbList<MessageMedia> get media => $_getList(8);

  @$pb.TagNumber(10)
  $core.String get clientMessageId => $_getSZ(9);
  @$pb.TagNumber(10)
  set clientMessageId($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasClientMessageId() => $_has(9);
  @$pb.TagNumber(10)
  void clearClientMessageId() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get sentAt => $_getSZ(10);
  @$pb.TagNumber(11)
  set sentAt($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasSentAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearSentAt() => $_clearField(11);
}

class ListConversationsRequest extends $pb.GeneratedMessage {
  factory ListConversationsRequest({
    $core.int? page,
  }) {
    final result = create();
    if (page != null) result.page = page;
    return result;
  }

  ListConversationsRequest._();

  factory ListConversationsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListConversationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListConversationsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'page', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConversationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConversationsRequest copyWith(
          void Function(ListConversationsRequest) updates) =>
      super.copyWith((message) => updates(message as ListConversationsRequest))
          as ListConversationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListConversationsRequest create() => ListConversationsRequest._();
  @$core.override
  ListConversationsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListConversationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListConversationsRequest>(create);
  static ListConversationsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get page => $_getIZ(0);
  @$pb.TagNumber(1)
  set page($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPage() => $_has(0);
  @$pb.TagNumber(1)
  void clearPage() => $_clearField(1);
}

class ListConversationsResponse extends $pb.GeneratedMessage {
  factory ListConversationsResponse({
    $core.Iterable<Conversation>? conversations,
  }) {
    final result = create();
    if (conversations != null) result.conversations.addAll(conversations);
    return result;
  }

  ListConversationsResponse._();

  factory ListConversationsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListConversationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListConversationsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<Conversation>(1, _omitFieldNames ? '' : 'conversations',
        subBuilder: Conversation.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConversationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConversationsResponse copyWith(
          void Function(ListConversationsResponse) updates) =>
      super.copyWith((message) => updates(message as ListConversationsResponse))
          as ListConversationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListConversationsResponse create() => ListConversationsResponse._();
  @$core.override
  ListConversationsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListConversationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListConversationsResponse>(create);
  static ListConversationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Conversation> get conversations => $_getList(0);
}

class GetPresenceRequest extends $pb.GeneratedMessage {
  factory GetPresenceRequest({
    $core.Iterable<$core.String>? personUuids,
  }) {
    final result = create();
    if (personUuids != null) result.personUuids.addAll(personUuids);
    return result;
  }

  GetPresenceRequest._();

  factory GetPresenceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetPresenceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetPresenceRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'personUuids')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPresenceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPresenceRequest copyWith(void Function(GetPresenceRequest) updates) =>
      super.copyWith((message) => updates(message as GetPresenceRequest))
          as GetPresenceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetPresenceRequest create() => GetPresenceRequest._();
  @$core.override
  GetPresenceRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetPresenceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetPresenceRequest>(create);
  static GetPresenceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get personUuids => $_getList(0);
}

class GetPresenceResponse extends $pb.GeneratedMessage {
  factory GetPresenceResponse({
    $core.Iterable<$core.String>? online,
  }) {
    final result = create();
    if (online != null) result.online.addAll(online);
    return result;
  }

  GetPresenceResponse._();

  factory GetPresenceResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetPresenceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetPresenceResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'online')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPresenceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetPresenceResponse copyWith(void Function(GetPresenceResponse) updates) =>
      super.copyWith((message) => updates(message as GetPresenceResponse))
          as GetPresenceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetPresenceResponse create() => GetPresenceResponse._();
  @$core.override
  GetPresenceResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetPresenceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetPresenceResponse>(create);
  static GetPresenceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get online => $_getList(0);
}

class CreateDirectConversationRequest extends $pb.GeneratedMessage {
  factory CreateDirectConversationRequest({
    $core.String? targetPersonUuid,
  }) {
    final result = create();
    if (targetPersonUuid != null) result.targetPersonUuid = targetPersonUuid;
    return result;
  }

  CreateDirectConversationRequest._();

  factory CreateDirectConversationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateDirectConversationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDirectConversationRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'targetPersonUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDirectConversationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDirectConversationRequest copyWith(
          void Function(CreateDirectConversationRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateDirectConversationRequest))
          as CreateDirectConversationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateDirectConversationRequest create() =>
      CreateDirectConversationRequest._();
  @$core.override
  CreateDirectConversationRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateDirectConversationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDirectConversationRequest>(
          create);
  static CreateDirectConversationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get targetPersonUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set targetPersonUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTargetPersonUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetPersonUuid() => $_clearField(1);
}

class CreateBusinessTeamGroupRequest extends $pb.GeneratedMessage {
  factory CreateBusinessTeamGroupRequest({
    $core.String? businessProfileUuid,
  }) {
    final result = create();
    if (businessProfileUuid != null)
      result.businessProfileUuid = businessProfileUuid;
    return result;
  }

  CreateBusinessTeamGroupRequest._();

  factory CreateBusinessTeamGroupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateBusinessTeamGroupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateBusinessTeamGroupRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'businessProfileUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBusinessTeamGroupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBusinessTeamGroupRequest copyWith(
          void Function(CreateBusinessTeamGroupRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateBusinessTeamGroupRequest))
          as CreateBusinessTeamGroupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateBusinessTeamGroupRequest create() =>
      CreateBusinessTeamGroupRequest._();
  @$core.override
  CreateBusinessTeamGroupRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateBusinessTeamGroupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateBusinessTeamGroupRequest>(create);
  static CreateBusinessTeamGroupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get businessProfileUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set businessProfileUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBusinessProfileUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearBusinessProfileUuid() => $_clearField(1);
}

class CreateBusinessDirectConversationRequest extends $pb.GeneratedMessage {
  factory CreateBusinessDirectConversationRequest({
    $core.String? businessProfileUuid,
    $core.String? memberPersonUuid,
  }) {
    final result = create();
    if (businessProfileUuid != null)
      result.businessProfileUuid = businessProfileUuid;
    if (memberPersonUuid != null) result.memberPersonUuid = memberPersonUuid;
    return result;
  }

  CreateBusinessDirectConversationRequest._();

  factory CreateBusinessDirectConversationRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateBusinessDirectConversationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateBusinessDirectConversationRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'businessProfileUuid')
    ..aOS(2, _omitFieldNames ? '' : 'memberPersonUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBusinessDirectConversationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateBusinessDirectConversationRequest copyWith(
          void Function(CreateBusinessDirectConversationRequest) updates) =>
      super.copyWith((message) =>
              updates(message as CreateBusinessDirectConversationRequest))
          as CreateBusinessDirectConversationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateBusinessDirectConversationRequest create() =>
      CreateBusinessDirectConversationRequest._();
  @$core.override
  CreateBusinessDirectConversationRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateBusinessDirectConversationRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
          CreateBusinessDirectConversationRequest>(create);
  static CreateBusinessDirectConversationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get businessProfileUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set businessProfileUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBusinessProfileUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearBusinessProfileUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get memberPersonUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set memberPersonUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMemberPersonUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearMemberPersonUuid() => $_clearField(2);
}

class ListMessagesRequest extends $pb.GeneratedMessage {
  factory ListMessagesRequest({
    $core.String? conversationUuid,
    $core.int? page,
    $fixnum.Int64? sinceEpochMs,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (page != null) result.page = page;
    if (sinceEpochMs != null) result.sinceEpochMs = sinceEpochMs;
    return result;
  }

  ListMessagesRequest._();

  factory ListMessagesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListMessagesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListMessagesRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aI(2, _omitFieldNames ? '' : 'page', fieldType: $pb.PbFieldType.OU3)
    ..aInt64(3, _omitFieldNames ? '' : 'sinceEpochMs')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListMessagesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListMessagesRequest copyWith(void Function(ListMessagesRequest) updates) =>
      super.copyWith((message) => updates(message as ListMessagesRequest))
          as ListMessagesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListMessagesRequest create() => ListMessagesRequest._();
  @$core.override
  ListMessagesRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListMessagesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListMessagesRequest>(create);
  static ListMessagesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get page => $_getIZ(1);
  @$pb.TagNumber(2)
  set page($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(2)
  void clearPage() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get sinceEpochMs => $_getI64(2);
  @$pb.TagNumber(3)
  set sinceEpochMs($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSinceEpochMs() => $_has(2);
  @$pb.TagNumber(3)
  void clearSinceEpochMs() => $_clearField(3);
}

class ListMessagesResponse extends $pb.GeneratedMessage {
  factory ListMessagesResponse({
    $core.Iterable<Message>? messages,
  }) {
    final result = create();
    if (messages != null) result.messages.addAll(messages);
    return result;
  }

  ListMessagesResponse._();

  factory ListMessagesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListMessagesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListMessagesResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<Message>(1, _omitFieldNames ? '' : 'messages',
        subBuilder: Message.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListMessagesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListMessagesResponse copyWith(void Function(ListMessagesResponse) updates) =>
      super.copyWith((message) => updates(message as ListMessagesResponse))
          as ListMessagesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListMessagesResponse create() => ListMessagesResponse._();
  @$core.override
  ListMessagesResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListMessagesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListMessagesResponse>(create);
  static ListMessagesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Message> get messages => $_getList(0);
}

class SendMessageRequest extends $pb.GeneratedMessage {
  factory SendMessageRequest({
    $core.String? conversationUuid,
    $core.String? body,
    $core.Iterable<MessageMedia>? media,
    $core.String? clientMessageId,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (body != null) result.body = body;
    if (media != null) result.media.addAll(media);
    if (clientMessageId != null) result.clientMessageId = clientMessageId;
    return result;
  }

  SendMessageRequest._();

  factory SendMessageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SendMessageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SendMessageRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'body')
    ..pPM<MessageMedia>(3, _omitFieldNames ? '' : 'media',
        subBuilder: MessageMedia.create)
    ..aOS(4, _omitFieldNames ? '' : 'clientMessageId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendMessageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendMessageRequest copyWith(void Function(SendMessageRequest) updates) =>
      super.copyWith((message) => updates(message as SendMessageRequest))
          as SendMessageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendMessageRequest create() => SendMessageRequest._();
  @$core.override
  SendMessageRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SendMessageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SendMessageRequest>(create);
  static SendMessageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get body => $_getSZ(1);
  @$pb.TagNumber(2)
  set body($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBody() => $_has(1);
  @$pb.TagNumber(2)
  void clearBody() => $_clearField(2);

  @$pb.TagNumber(3)
  $pb.PbList<MessageMedia> get media => $_getList(2);

  @$pb.TagNumber(4)
  $core.String get clientMessageId => $_getSZ(3);
  @$pb.TagNumber(4)
  set clientMessageId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasClientMessageId() => $_has(3);
  @$pb.TagNumber(4)
  void clearClientMessageId() => $_clearField(4);
}

class MarkConversationReadRequest extends $pb.GeneratedMessage {
  factory MarkConversationReadRequest({
    $core.String? conversationUuid,
    $core.String? lastReadMessageUuid,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (lastReadMessageUuid != null)
      result.lastReadMessageUuid = lastReadMessageUuid;
    return result;
  }

  MarkConversationReadRequest._();

  factory MarkConversationReadRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MarkConversationReadRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MarkConversationReadRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'lastReadMessageUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkConversationReadRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkConversationReadRequest copyWith(
          void Function(MarkConversationReadRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MarkConversationReadRequest))
          as MarkConversationReadRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkConversationReadRequest create() =>
      MarkConversationReadRequest._();
  @$core.override
  MarkConversationReadRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MarkConversationReadRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MarkConversationReadRequest>(create);
  static MarkConversationReadRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get lastReadMessageUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set lastReadMessageUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLastReadMessageUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastReadMessageUuid() => $_clearField(2);
}

class MarkConversationReadResponse extends $pb.GeneratedMessage {
  factory MarkConversationReadResponse({
    $core.bool? read,
  }) {
    final result = create();
    if (read != null) result.read = read;
    return result;
  }

  MarkConversationReadResponse._();

  factory MarkConversationReadResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MarkConversationReadResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MarkConversationReadResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'read')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkConversationReadResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkConversationReadResponse copyWith(
          void Function(MarkConversationReadResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MarkConversationReadResponse))
          as MarkConversationReadResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkConversationReadResponse create() =>
      MarkConversationReadResponse._();
  @$core.override
  MarkConversationReadResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MarkConversationReadResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MarkConversationReadResponse>(create);
  static MarkConversationReadResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get read => $_getBF(0);
  @$pb.TagNumber(1)
  set read($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRead() => $_has(0);
  @$pb.TagNumber(1)
  void clearRead() => $_clearField(1);
}

class SendFrame extends $pb.GeneratedMessage {
  factory SendFrame({
    $core.String? conversationUuid,
    $core.String? body,
    $core.Iterable<MessageMedia>? media,
    $core.String? clientMessageId,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (body != null) result.body = body;
    if (media != null) result.media.addAll(media);
    if (clientMessageId != null) result.clientMessageId = clientMessageId;
    return result;
  }

  SendFrame._();

  factory SendFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SendFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SendFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'body')
    ..pPM<MessageMedia>(3, _omitFieldNames ? '' : 'media',
        subBuilder: MessageMedia.create)
    ..aOS(4, _omitFieldNames ? '' : 'clientMessageId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendFrame copyWith(void Function(SendFrame) updates) =>
      super.copyWith((message) => updates(message as SendFrame)) as SendFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendFrame create() => SendFrame._();
  @$core.override
  SendFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SendFrame getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendFrame>(create);
  static SendFrame? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get body => $_getSZ(1);
  @$pb.TagNumber(2)
  set body($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBody() => $_has(1);
  @$pb.TagNumber(2)
  void clearBody() => $_clearField(2);

  @$pb.TagNumber(3)
  $pb.PbList<MessageMedia> get media => $_getList(2);

  @$pb.TagNumber(4)
  $core.String get clientMessageId => $_getSZ(3);
  @$pb.TagNumber(4)
  set clientMessageId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasClientMessageId() => $_has(3);
  @$pb.TagNumber(4)
  void clearClientMessageId() => $_clearField(4);
}

class ReadFrame extends $pb.GeneratedMessage {
  factory ReadFrame({
    $core.String? conversationUuid,
    $core.String? lastReadMessageUuid,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (lastReadMessageUuid != null)
      result.lastReadMessageUuid = lastReadMessageUuid;
    return result;
  }

  ReadFrame._();

  factory ReadFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ReadFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ReadFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'lastReadMessageUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReadFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReadFrame copyWith(void Function(ReadFrame) updates) =>
      super.copyWith((message) => updates(message as ReadFrame)) as ReadFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ReadFrame create() => ReadFrame._();
  @$core.override
  ReadFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ReadFrame getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ReadFrame>(create);
  static ReadFrame? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get lastReadMessageUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set lastReadMessageUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLastReadMessageUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastReadMessageUuid() => $_clearField(2);
}

class TypingFrame extends $pb.GeneratedMessage {
  factory TypingFrame({
    $core.String? conversationUuid,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    return result;
  }

  TypingFrame._();

  factory TypingFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TypingFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TypingFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TypingFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TypingFrame copyWith(void Function(TypingFrame) updates) =>
      super.copyWith((message) => updates(message as TypingFrame))
          as TypingFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TypingFrame create() => TypingFrame._();
  @$core.override
  TypingFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static TypingFrame getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TypingFrame>(create);
  static TypingFrame? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);
}

class PingFrame extends $pb.GeneratedMessage {
  factory PingFrame() => create();

  PingFrame._();

  factory PingFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PingFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PingFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PingFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PingFrame copyWith(void Function(PingFrame) updates) =>
      super.copyWith((message) => updates(message as PingFrame)) as PingFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PingFrame create() => PingFrame._();
  @$core.override
  PingFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PingFrame getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PingFrame>(create);
  static PingFrame? _defaultInstance;
}

enum ClientFrame_Frame { send, read, typing, ping, notSet }

class ClientFrame extends $pb.GeneratedMessage {
  factory ClientFrame({
    SendFrame? send,
    ReadFrame? read,
    TypingFrame? typing,
    PingFrame? ping,
  }) {
    final result = create();
    if (send != null) result.send = send;
    if (read != null) result.read = read;
    if (typing != null) result.typing = typing;
    if (ping != null) result.ping = ping;
    return result;
  }

  ClientFrame._();

  factory ClientFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ClientFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ClientFrame_Frame> _ClientFrame_FrameByTag =
      {
    1: ClientFrame_Frame.send,
    2: ClientFrame_Frame.read,
    3: ClientFrame_Frame.typing,
    4: ClientFrame_Frame.ping,
    0: ClientFrame_Frame.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<SendFrame>(1, _omitFieldNames ? '' : 'send',
        subBuilder: SendFrame.create)
    ..aOM<ReadFrame>(2, _omitFieldNames ? '' : 'read',
        subBuilder: ReadFrame.create)
    ..aOM<TypingFrame>(3, _omitFieldNames ? '' : 'typing',
        subBuilder: TypingFrame.create)
    ..aOM<PingFrame>(4, _omitFieldNames ? '' : 'ping',
        subBuilder: PingFrame.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientFrame copyWith(void Function(ClientFrame) updates) =>
      super.copyWith((message) => updates(message as ClientFrame))
          as ClientFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ClientFrame create() => ClientFrame._();
  @$core.override
  ClientFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ClientFrame getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientFrame>(create);
  static ClientFrame? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  ClientFrame_Frame whichFrame() => _ClientFrame_FrameByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  void clearFrame() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  SendFrame get send => $_getN(0);
  @$pb.TagNumber(1)
  set send(SendFrame value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSend() => $_has(0);
  @$pb.TagNumber(1)
  void clearSend() => $_clearField(1);
  @$pb.TagNumber(1)
  SendFrame ensureSend() => $_ensure(0);

  @$pb.TagNumber(2)
  ReadFrame get read => $_getN(1);
  @$pb.TagNumber(2)
  set read(ReadFrame value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRead() => $_has(1);
  @$pb.TagNumber(2)
  void clearRead() => $_clearField(2);
  @$pb.TagNumber(2)
  ReadFrame ensureRead() => $_ensure(1);

  @$pb.TagNumber(3)
  TypingFrame get typing => $_getN(2);
  @$pb.TagNumber(3)
  set typing(TypingFrame value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasTyping() => $_has(2);
  @$pb.TagNumber(3)
  void clearTyping() => $_clearField(3);
  @$pb.TagNumber(3)
  TypingFrame ensureTyping() => $_ensure(2);

  @$pb.TagNumber(4)
  PingFrame get ping => $_getN(3);
  @$pb.TagNumber(4)
  set ping(PingFrame value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasPing() => $_has(3);
  @$pb.TagNumber(4)
  void clearPing() => $_clearField(4);
  @$pb.TagNumber(4)
  PingFrame ensurePing() => $_ensure(3);
}

class MessageNewEvent extends $pb.GeneratedMessage {
  factory MessageNewEvent({
    $core.String? conversationUuid,
    $core.String? conversationType,
    Message? message,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (conversationType != null) result.conversationType = conversationType;
    if (message != null) result.message = message;
    return result;
  }

  MessageNewEvent._();

  factory MessageNewEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MessageNewEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MessageNewEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'conversationType')
    ..aOM<Message>(3, _omitFieldNames ? '' : 'message',
        subBuilder: Message.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageNewEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageNewEvent copyWith(void Function(MessageNewEvent) updates) =>
      super.copyWith((message) => updates(message as MessageNewEvent))
          as MessageNewEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MessageNewEvent create() => MessageNewEvent._();
  @$core.override
  MessageNewEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MessageNewEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MessageNewEvent>(create);
  static MessageNewEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get conversationType => $_getSZ(1);
  @$pb.TagNumber(2)
  set conversationType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasConversationType() => $_has(1);
  @$pb.TagNumber(2)
  void clearConversationType() => $_clearField(2);

  @$pb.TagNumber(3)
  Message get message => $_getN(2);
  @$pb.TagNumber(3)
  set message(Message value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessage() => $_clearField(3);
  @$pb.TagNumber(3)
  Message ensureMessage() => $_ensure(2);
}

class ConversationUpdatedEvent extends $pb.GeneratedMessage {
  factory ConversationUpdatedEvent({
    Conversation? conversation,
  }) {
    final result = create();
    if (conversation != null) result.conversation = conversation;
    return result;
  }

  ConversationUpdatedEvent._();

  factory ConversationUpdatedEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConversationUpdatedEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConversationUpdatedEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOM<Conversation>(1, _omitFieldNames ? '' : 'conversation',
        subBuilder: Conversation.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConversationUpdatedEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConversationUpdatedEvent copyWith(
          void Function(ConversationUpdatedEvent) updates) =>
      super.copyWith((message) => updates(message as ConversationUpdatedEvent))
          as ConversationUpdatedEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConversationUpdatedEvent create() => ConversationUpdatedEvent._();
  @$core.override
  ConversationUpdatedEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConversationUpdatedEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConversationUpdatedEvent>(create);
  static ConversationUpdatedEvent? _defaultInstance;

  @$pb.TagNumber(1)
  Conversation get conversation => $_getN(0);
  @$pb.TagNumber(1)
  set conversation(Conversation value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasConversation() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversation() => $_clearField(1);
  @$pb.TagNumber(1)
  Conversation ensureConversation() => $_ensure(0);
}

class MessageReadEvent extends $pb.GeneratedMessage {
  factory MessageReadEvent({
    $core.String? conversationUuid,
    $core.String? personUuid,
    $core.String? lastReadMessageUuid,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (personUuid != null) result.personUuid = personUuid;
    if (lastReadMessageUuid != null)
      result.lastReadMessageUuid = lastReadMessageUuid;
    return result;
  }

  MessageReadEvent._();

  factory MessageReadEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MessageReadEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MessageReadEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'personUuid')
    ..aOS(3, _omitFieldNames ? '' : 'lastReadMessageUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageReadEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageReadEvent copyWith(void Function(MessageReadEvent) updates) =>
      super.copyWith((message) => updates(message as MessageReadEvent))
          as MessageReadEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MessageReadEvent create() => MessageReadEvent._();
  @$core.override
  MessageReadEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MessageReadEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MessageReadEvent>(create);
  static MessageReadEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get personUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set personUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get lastReadMessageUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set lastReadMessageUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLastReadMessageUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearLastReadMessageUuid() => $_clearField(3);
}

class TypingEvent extends $pb.GeneratedMessage {
  factory TypingEvent({
    $core.String? conversationUuid,
    $core.String? personUuid,
  }) {
    final result = create();
    if (conversationUuid != null) result.conversationUuid = conversationUuid;
    if (personUuid != null) result.personUuid = personUuid;
    return result;
  }

  TypingEvent._();

  factory TypingEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TypingEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TypingEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'conversationUuid')
    ..aOS(2, _omitFieldNames ? '' : 'personUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TypingEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TypingEvent copyWith(void Function(TypingEvent) updates) =>
      super.copyWith((message) => updates(message as TypingEvent))
          as TypingEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TypingEvent create() => TypingEvent._();
  @$core.override
  TypingEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static TypingEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TypingEvent>(create);
  static TypingEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get conversationUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set conversationUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasConversationUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get personUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set personUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonUuid() => $_clearField(2);
}

class PongEvent extends $pb.GeneratedMessage {
  factory PongEvent() => create();

  PongEvent._();

  factory PongEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PongEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PongEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PongEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PongEvent copyWith(void Function(PongEvent) updates) =>
      super.copyWith((message) => updates(message as PongEvent)) as PongEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PongEvent create() => PongEvent._();
  @$core.override
  PongEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PongEvent getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PongEvent>(create);
  static PongEvent? _defaultInstance;
}

class ErrorEvent extends $pb.GeneratedMessage {
  factory ErrorEvent({
    $core.String? message,
  }) {
    final result = create();
    if (message != null) result.message = message;
    return result;
  }

  ErrorEvent._();

  factory ErrorEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ErrorEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ErrorEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ErrorEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ErrorEvent copyWith(void Function(ErrorEvent) updates) =>
      super.copyWith((message) => updates(message as ErrorEvent)) as ErrorEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ErrorEvent create() => ErrorEvent._();
  @$core.override
  ErrorEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ErrorEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ErrorEvent>(create);
  static ErrorEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(1)
  set message($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessage() => $_clearField(1);
}

enum ServerFrame_Event {
  messageNew,
  conversationUpdated,
  messageRead,
  typing,
  pong,
  error,
  notSet
}

class ServerFrame extends $pb.GeneratedMessage {
  factory ServerFrame({
    MessageNewEvent? messageNew,
    ConversationUpdatedEvent? conversationUpdated,
    MessageReadEvent? messageRead,
    TypingEvent? typing,
    PongEvent? pong,
    ErrorEvent? error,
  }) {
    final result = create();
    if (messageNew != null) result.messageNew = messageNew;
    if (conversationUpdated != null)
      result.conversationUpdated = conversationUpdated;
    if (messageRead != null) result.messageRead = messageRead;
    if (typing != null) result.typing = typing;
    if (pong != null) result.pong = pong;
    if (error != null) result.error = error;
    return result;
  }

  ServerFrame._();

  factory ServerFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ServerFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ServerFrame_Event> _ServerFrame_EventByTag =
      {
    1: ServerFrame_Event.messageNew,
    2: ServerFrame_Event.conversationUpdated,
    3: ServerFrame_Event.messageRead,
    4: ServerFrame_Event.typing,
    5: ServerFrame_Event.pong,
    6: ServerFrame_Event.error,
    0: ServerFrame_Event.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<MessageNewEvent>(1, _omitFieldNames ? '' : 'messageNew',
        subBuilder: MessageNewEvent.create)
    ..aOM<ConversationUpdatedEvent>(
        2, _omitFieldNames ? '' : 'conversationUpdated',
        subBuilder: ConversationUpdatedEvent.create)
    ..aOM<MessageReadEvent>(3, _omitFieldNames ? '' : 'messageRead',
        subBuilder: MessageReadEvent.create)
    ..aOM<TypingEvent>(4, _omitFieldNames ? '' : 'typing',
        subBuilder: TypingEvent.create)
    ..aOM<PongEvent>(5, _omitFieldNames ? '' : 'pong',
        subBuilder: PongEvent.create)
    ..aOM<ErrorEvent>(6, _omitFieldNames ? '' : 'error',
        subBuilder: ErrorEvent.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerFrame copyWith(void Function(ServerFrame) updates) =>
      super.copyWith((message) => updates(message as ServerFrame))
          as ServerFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ServerFrame create() => ServerFrame._();
  @$core.override
  ServerFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ServerFrame getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerFrame>(create);
  static ServerFrame? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  ServerFrame_Event whichEvent() => _ServerFrame_EventByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearEvent() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  MessageNewEvent get messageNew => $_getN(0);
  @$pb.TagNumber(1)
  set messageNew(MessageNewEvent value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasMessageNew() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageNew() => $_clearField(1);
  @$pb.TagNumber(1)
  MessageNewEvent ensureMessageNew() => $_ensure(0);

  @$pb.TagNumber(2)
  ConversationUpdatedEvent get conversationUpdated => $_getN(1);
  @$pb.TagNumber(2)
  set conversationUpdated(ConversationUpdatedEvent value) =>
      $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasConversationUpdated() => $_has(1);
  @$pb.TagNumber(2)
  void clearConversationUpdated() => $_clearField(2);
  @$pb.TagNumber(2)
  ConversationUpdatedEvent ensureConversationUpdated() => $_ensure(1);

  @$pb.TagNumber(3)
  MessageReadEvent get messageRead => $_getN(2);
  @$pb.TagNumber(3)
  set messageRead(MessageReadEvent value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasMessageRead() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageRead() => $_clearField(3);
  @$pb.TagNumber(3)
  MessageReadEvent ensureMessageRead() => $_ensure(2);

  @$pb.TagNumber(4)
  TypingEvent get typing => $_getN(3);
  @$pb.TagNumber(4)
  set typing(TypingEvent value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasTyping() => $_has(3);
  @$pb.TagNumber(4)
  void clearTyping() => $_clearField(4);
  @$pb.TagNumber(4)
  TypingEvent ensureTyping() => $_ensure(3);

  @$pb.TagNumber(5)
  PongEvent get pong => $_getN(4);
  @$pb.TagNumber(5)
  set pong(PongEvent value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasPong() => $_has(4);
  @$pb.TagNumber(5)
  void clearPong() => $_clearField(5);
  @$pb.TagNumber(5)
  PongEvent ensurePong() => $_ensure(4);

  @$pb.TagNumber(6)
  ErrorEvent get error => $_getN(5);
  @$pb.TagNumber(6)
  set error(ErrorEvent value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasError() => $_has(5);
  @$pb.TagNumber(6)
  void clearError() => $_clearField(6);
  @$pb.TagNumber(6)
  ErrorEvent ensureError() => $_ensure(5);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
