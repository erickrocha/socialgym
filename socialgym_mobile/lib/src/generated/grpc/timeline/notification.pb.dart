// This is a generated file - do not edit.
//
// Generated from timeline/notification.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class Notification extends $pb.GeneratedMessage {
  factory Notification({
    $core.String? uuid,
    $core.String? notificationType,
    $core.String? recipientPersonUuid,
    $core.String? actorPersonUuid,
    $core.String? actorName,
    $core.String? postUuid,
    $core.String? commentUuid,
    $core.String? entityType,
    $core.String? entityUuid,
    $core.String? targetType,
    $core.String? targetUuid,
    $core.String? snippet,
    $core.bool? read,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (notificationType != null) result.notificationType = notificationType;
    if (recipientPersonUuid != null)
      result.recipientPersonUuid = recipientPersonUuid;
    if (actorPersonUuid != null) result.actorPersonUuid = actorPersonUuid;
    if (actorName != null) result.actorName = actorName;
    if (postUuid != null) result.postUuid = postUuid;
    if (commentUuid != null) result.commentUuid = commentUuid;
    if (entityType != null) result.entityType = entityType;
    if (entityUuid != null) result.entityUuid = entityUuid;
    if (targetType != null) result.targetType = targetType;
    if (targetUuid != null) result.targetUuid = targetUuid;
    if (snippet != null) result.snippet = snippet;
    if (read != null) result.read = read;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Notification._();

  factory Notification.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Notification.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Notification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'notificationType')
    ..aOS(3, _omitFieldNames ? '' : 'recipientPersonUuid')
    ..aOS(4, _omitFieldNames ? '' : 'actorPersonUuid')
    ..aOS(5, _omitFieldNames ? '' : 'actorName')
    ..aOS(6, _omitFieldNames ? '' : 'postUuid')
    ..aOS(7, _omitFieldNames ? '' : 'commentUuid')
    ..aOS(8, _omitFieldNames ? '' : 'entityType')
    ..aOS(9, _omitFieldNames ? '' : 'entityUuid')
    ..aOS(10, _omitFieldNames ? '' : 'targetType')
    ..aOS(11, _omitFieldNames ? '' : 'targetUuid')
    ..aOS(12, _omitFieldNames ? '' : 'snippet')
    ..aOB(13, _omitFieldNames ? '' : 'read')
    ..aOS(14, _omitFieldNames ? '' : 'createdAt')
    ..aOS(15, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Notification clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Notification copyWith(void Function(Notification) updates) =>
      super.copyWith((message) => updates(message as Notification))
          as Notification;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Notification create() => Notification._();
  @$core.override
  Notification createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Notification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Notification>(create);
  static Notification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get notificationType => $_getSZ(1);
  @$pb.TagNumber(2)
  set notificationType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNotificationType() => $_has(1);
  @$pb.TagNumber(2)
  void clearNotificationType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get recipientPersonUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set recipientPersonUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRecipientPersonUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearRecipientPersonUuid() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get actorPersonUuid => $_getSZ(3);
  @$pb.TagNumber(4)
  set actorPersonUuid($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasActorPersonUuid() => $_has(3);
  @$pb.TagNumber(4)
  void clearActorPersonUuid() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get actorName => $_getSZ(4);
  @$pb.TagNumber(5)
  set actorName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasActorName() => $_has(4);
  @$pb.TagNumber(5)
  void clearActorName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get postUuid => $_getSZ(5);
  @$pb.TagNumber(6)
  set postUuid($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPostUuid() => $_has(5);
  @$pb.TagNumber(6)
  void clearPostUuid() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get commentUuid => $_getSZ(6);
  @$pb.TagNumber(7)
  set commentUuid($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCommentUuid() => $_has(6);
  @$pb.TagNumber(7)
  void clearCommentUuid() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get entityType => $_getSZ(7);
  @$pb.TagNumber(8)
  set entityType($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasEntityType() => $_has(7);
  @$pb.TagNumber(8)
  void clearEntityType() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get entityUuid => $_getSZ(8);
  @$pb.TagNumber(9)
  set entityUuid($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasEntityUuid() => $_has(8);
  @$pb.TagNumber(9)
  void clearEntityUuid() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get targetType => $_getSZ(9);
  @$pb.TagNumber(10)
  set targetType($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasTargetType() => $_has(9);
  @$pb.TagNumber(10)
  void clearTargetType() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get targetUuid => $_getSZ(10);
  @$pb.TagNumber(11)
  set targetUuid($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasTargetUuid() => $_has(10);
  @$pb.TagNumber(11)
  void clearTargetUuid() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get snippet => $_getSZ(11);
  @$pb.TagNumber(12)
  set snippet($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasSnippet() => $_has(11);
  @$pb.TagNumber(12)
  void clearSnippet() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.bool get read => $_getBF(12);
  @$pb.TagNumber(13)
  set read($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(13)
  $core.bool hasRead() => $_has(12);
  @$pb.TagNumber(13)
  void clearRead() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get createdAt => $_getSZ(13);
  @$pb.TagNumber(14)
  set createdAt($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasCreatedAt() => $_has(13);
  @$pb.TagNumber(14)
  void clearCreatedAt() => $_clearField(14);

  @$pb.TagNumber(15)
  $core.String get updatedAt => $_getSZ(14);
  @$pb.TagNumber(15)
  set updatedAt($core.String value) => $_setString(14, value);
  @$pb.TagNumber(15)
  $core.bool hasUpdatedAt() => $_has(14);
  @$pb.TagNumber(15)
  void clearUpdatedAt() => $_clearField(15);
}

class ListNotificationsRequest extends $pb.GeneratedMessage {
  factory ListNotificationsRequest({
    $core.bool? unreadOnly,
    $core.int? limit,
  }) {
    final result = create();
    if (unreadOnly != null) result.unreadOnly = unreadOnly;
    if (limit != null) result.limit = limit;
    return result;
  }

  ListNotificationsRequest._();

  factory ListNotificationsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListNotificationsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'unreadOnly')
    ..aI(2, _omitFieldNames ? '' : 'limit', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsRequest copyWith(
          void Function(ListNotificationsRequest) updates) =>
      super.copyWith((message) => updates(message as ListNotificationsRequest))
          as ListNotificationsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListNotificationsRequest create() => ListNotificationsRequest._();
  @$core.override
  ListNotificationsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListNotificationsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListNotificationsRequest>(create);
  static ListNotificationsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get unreadOnly => $_getBF(0);
  @$pb.TagNumber(1)
  set unreadOnly($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUnreadOnly() => $_has(0);
  @$pb.TagNumber(1)
  void clearUnreadOnly() => $_clearField(1);

  /// 1..100, default 50 when 0.
  @$pb.TagNumber(2)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(2)
  set limit($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);
}

class ListNotificationsResponse extends $pb.GeneratedMessage {
  factory ListNotificationsResponse({
    $core.Iterable<Notification>? notifications,
  }) {
    final result = create();
    if (notifications != null) result.notifications.addAll(notifications);
    return result;
  }

  ListNotificationsResponse._();

  factory ListNotificationsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListNotificationsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListNotificationsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<Notification>(1, _omitFieldNames ? '' : 'notifications',
        subBuilder: Notification.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListNotificationsResponse copyWith(
          void Function(ListNotificationsResponse) updates) =>
      super.copyWith((message) => updates(message as ListNotificationsResponse))
          as ListNotificationsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListNotificationsResponse create() => ListNotificationsResponse._();
  @$core.override
  ListNotificationsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListNotificationsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListNotificationsResponse>(create);
  static ListNotificationsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Notification> get notifications => $_getList(0);
}

class MarkNotificationReadRequest extends $pb.GeneratedMessage {
  factory MarkNotificationReadRequest({
    $core.String? idempotencyKey,
  }) {
    final result = create();
    if (idempotencyKey != null) result.idempotencyKey = idempotencyKey;
    return result;
  }

  MarkNotificationReadRequest._();

  factory MarkNotificationReadRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MarkNotificationReadRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MarkNotificationReadRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'idempotencyKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkNotificationReadRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkNotificationReadRequest copyWith(
          void Function(MarkNotificationReadRequest) updates) =>
      super.copyWith(
              (message) => updates(message as MarkNotificationReadRequest))
          as MarkNotificationReadRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkNotificationReadRequest create() =>
      MarkNotificationReadRequest._();
  @$core.override
  MarkNotificationReadRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MarkNotificationReadRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MarkNotificationReadRequest>(create);
  static MarkNotificationReadRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get idempotencyKey => $_getSZ(0);
  @$pb.TagNumber(1)
  set idempotencyKey($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIdempotencyKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearIdempotencyKey() => $_clearField(1);
}

class MarkNotificationReadResponse extends $pb.GeneratedMessage {
  factory MarkNotificationReadResponse({
    $core.bool? read,
  }) {
    final result = create();
    if (read != null) result.read = read;
    return result;
  }

  MarkNotificationReadResponse._();

  factory MarkNotificationReadResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MarkNotificationReadResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MarkNotificationReadResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'read')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkNotificationReadResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MarkNotificationReadResponse copyWith(
          void Function(MarkNotificationReadResponse) updates) =>
      super.copyWith(
              (message) => updates(message as MarkNotificationReadResponse))
          as MarkNotificationReadResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkNotificationReadResponse create() =>
      MarkNotificationReadResponse._();
  @$core.override
  MarkNotificationReadResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MarkNotificationReadResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MarkNotificationReadResponse>(create);
  static MarkNotificationReadResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get read => $_getBF(0);
  @$pb.TagNumber(1)
  set read($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRead() => $_has(0);
  @$pb.TagNumber(1)
  void clearRead() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
