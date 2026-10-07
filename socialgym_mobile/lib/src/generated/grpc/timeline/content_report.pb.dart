// This is a generated file - do not edit.
//
// Generated from timeline/content_report.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ModerationEvent extends $pb.GeneratedMessage {
  factory ModerationEvent({
    $core.String? actorPersonUuid,
    $core.String? action,
    $core.String? reason,
    $core.String? createdAt,
  }) {
    final result = create();
    if (actorPersonUuid != null) result.actorPersonUuid = actorPersonUuid;
    if (action != null) result.action = action;
    if (reason != null) result.reason = reason;
    if (createdAt != null) result.createdAt = createdAt;
    return result;
  }

  ModerationEvent._();

  factory ModerationEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ModerationEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ModerationEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'actorPersonUuid')
    ..aOS(2, _omitFieldNames ? '' : 'action')
    ..aOS(3, _omitFieldNames ? '' : 'reason')
    ..aOS(4, _omitFieldNames ? '' : 'createdAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ModerationEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ModerationEvent copyWith(void Function(ModerationEvent) updates) =>
      super.copyWith((message) => updates(message as ModerationEvent))
          as ModerationEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ModerationEvent create() => ModerationEvent._();
  @$core.override
  ModerationEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ModerationEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ModerationEvent>(create);
  static ModerationEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get actorPersonUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set actorPersonUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasActorPersonUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearActorPersonUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get action => $_getSZ(1);
  @$pb.TagNumber(2)
  set action($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAction() => $_has(1);
  @$pb.TagNumber(2)
  void clearAction() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get reason => $_getSZ(2);
  @$pb.TagNumber(3)
  set reason($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasReason() => $_has(2);
  @$pb.TagNumber(3)
  void clearReason() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get createdAt => $_getSZ(3);
  @$pb.TagNumber(4)
  set createdAt($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCreatedAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearCreatedAt() => $_clearField(4);
}

class ContentReport extends $pb.GeneratedMessage {
  factory ContentReport({
    $core.String? uuid,
    $core.String? targetType,
    $core.String? targetId,
    $core.String? postId,
    $core.String? reporterPersonUuid,
    $core.String? reason,
    $core.String? details,
    $core.String? priority,
    $core.String? status,
    $core.String? assignedModeratorUuid,
    $core.String? decision,
    $core.String? removalReason,
    $core.Iterable<ModerationEvent>? history,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (targetType != null) result.targetType = targetType;
    if (targetId != null) result.targetId = targetId;
    if (postId != null) result.postId = postId;
    if (reporterPersonUuid != null)
      result.reporterPersonUuid = reporterPersonUuid;
    if (reason != null) result.reason = reason;
    if (details != null) result.details = details;
    if (priority != null) result.priority = priority;
    if (status != null) result.status = status;
    if (assignedModeratorUuid != null)
      result.assignedModeratorUuid = assignedModeratorUuid;
    if (decision != null) result.decision = decision;
    if (removalReason != null) result.removalReason = removalReason;
    if (history != null) result.history.addAll(history);
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  ContentReport._();

  factory ContentReport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ContentReport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ContentReport',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'targetType')
    ..aOS(3, _omitFieldNames ? '' : 'targetId')
    ..aOS(4, _omitFieldNames ? '' : 'postId')
    ..aOS(5, _omitFieldNames ? '' : 'reporterPersonUuid')
    ..aOS(6, _omitFieldNames ? '' : 'reason')
    ..aOS(7, _omitFieldNames ? '' : 'details')
    ..aOS(8, _omitFieldNames ? '' : 'priority')
    ..aOS(9, _omitFieldNames ? '' : 'status')
    ..aOS(10, _omitFieldNames ? '' : 'assignedModeratorUuid')
    ..aOS(11, _omitFieldNames ? '' : 'decision')
    ..aOS(12, _omitFieldNames ? '' : 'removalReason')
    ..pPM<ModerationEvent>(13, _omitFieldNames ? '' : 'history',
        subBuilder: ModerationEvent.create)
    ..aOS(14, _omitFieldNames ? '' : 'createdAt')
    ..aOS(15, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ContentReport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ContentReport copyWith(void Function(ContentReport) updates) =>
      super.copyWith((message) => updates(message as ContentReport))
          as ContentReport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ContentReport create() => ContentReport._();
  @$core.override
  ContentReport createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ContentReport getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ContentReport>(create);
  static ContentReport? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  /// post, comment or media.
  @$pb.TagNumber(2)
  $core.String get targetType => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTargetType() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get targetId => $_getSZ(2);
  @$pb.TagNumber(3)
  set targetId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTargetId() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get postId => $_getSZ(3);
  @$pb.TagNumber(4)
  set postId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPostId() => $_has(3);
  @$pb.TagNumber(4)
  void clearPostId() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get reporterPersonUuid => $_getSZ(4);
  @$pb.TagNumber(5)
  set reporterPersonUuid($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasReporterPersonUuid() => $_has(4);
  @$pb.TagNumber(5)
  void clearReporterPersonUuid() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get reason => $_getSZ(5);
  @$pb.TagNumber(6)
  set reason($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasReason() => $_has(5);
  @$pb.TagNumber(6)
  void clearReason() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get details => $_getSZ(6);
  @$pb.TagNumber(7)
  set details($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasDetails() => $_has(6);
  @$pb.TagNumber(7)
  void clearDetails() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get priority => $_getSZ(7);
  @$pb.TagNumber(8)
  set priority($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasPriority() => $_has(7);
  @$pb.TagNumber(8)
  void clearPriority() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get status => $_getSZ(8);
  @$pb.TagNumber(9)
  set status($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasStatus() => $_has(8);
  @$pb.TagNumber(9)
  void clearStatus() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get assignedModeratorUuid => $_getSZ(9);
  @$pb.TagNumber(10)
  set assignedModeratorUuid($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasAssignedModeratorUuid() => $_has(9);
  @$pb.TagNumber(10)
  void clearAssignedModeratorUuid() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get decision => $_getSZ(10);
  @$pb.TagNumber(11)
  set decision($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasDecision() => $_has(10);
  @$pb.TagNumber(11)
  void clearDecision() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get removalReason => $_getSZ(11);
  @$pb.TagNumber(12)
  set removalReason($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasRemovalReason() => $_has(11);
  @$pb.TagNumber(12)
  void clearRemovalReason() => $_clearField(12);

  @$pb.TagNumber(13)
  $pb.PbList<ModerationEvent> get history => $_getList(12);

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

class CreateReportRequest extends $pb.GeneratedMessage {
  factory CreateReportRequest({
    $core.String? targetType,
    $core.String? targetId,
    $core.String? postId,
    $core.String? reason,
    $core.String? details,
  }) {
    final result = create();
    if (targetType != null) result.targetType = targetType;
    if (targetId != null) result.targetId = targetId;
    if (postId != null) result.postId = postId;
    if (reason != null) result.reason = reason;
    if (details != null) result.details = details;
    return result;
  }

  CreateReportRequest._();

  factory CreateReportRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateReportRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateReportRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'targetType')
    ..aOS(2, _omitFieldNames ? '' : 'targetId')
    ..aOS(3, _omitFieldNames ? '' : 'postId')
    ..aOS(4, _omitFieldNames ? '' : 'reason')
    ..aOS(5, _omitFieldNames ? '' : 'details')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateReportRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateReportRequest copyWith(void Function(CreateReportRequest) updates) =>
      super.copyWith((message) => updates(message as CreateReportRequest))
          as CreateReportRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateReportRequest create() => CreateReportRequest._();
  @$core.override
  CreateReportRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateReportRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateReportRequest>(create);
  static CreateReportRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get targetType => $_getSZ(0);
  @$pb.TagNumber(1)
  set targetType($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTargetType() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetType() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get targetId => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTargetId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get postId => $_getSZ(2);
  @$pb.TagNumber(3)
  set postId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPostId() => $_has(2);
  @$pb.TagNumber(3)
  void clearPostId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get reason => $_getSZ(3);
  @$pb.TagNumber(4)
  set reason($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasReason() => $_has(3);
  @$pb.TagNumber(4)
  void clearReason() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get details => $_getSZ(4);
  @$pb.TagNumber(5)
  set details($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDetails() => $_has(4);
  @$pb.TagNumber(5)
  void clearDetails() => $_clearField(5);
}

class ListReportsRequest extends $pb.GeneratedMessage {
  factory ListReportsRequest({
    $core.String? status,
  }) {
    final result = create();
    if (status != null) result.status = status;
    return result;
  }

  ListReportsRequest._();

  factory ListReportsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListReportsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListReportsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'status')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListReportsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListReportsRequest copyWith(void Function(ListReportsRequest) updates) =>
      super.copyWith((message) => updates(message as ListReportsRequest))
          as ListReportsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListReportsRequest create() => ListReportsRequest._();
  @$core.override
  ListReportsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListReportsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListReportsRequest>(create);
  static ListReportsRequest? _defaultInstance;

  /// open or resolved; all reports when empty.
  @$pb.TagNumber(1)
  $core.String get status => $_getSZ(0);
  @$pb.TagNumber(1)
  set status($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);
}

class ListReportsResponse extends $pb.GeneratedMessage {
  factory ListReportsResponse({
    $core.Iterable<ContentReport>? reports,
  }) {
    final result = create();
    if (reports != null) result.reports.addAll(reports);
    return result;
  }

  ListReportsResponse._();

  factory ListReportsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListReportsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListReportsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<ContentReport>(1, _omitFieldNames ? '' : 'reports',
        subBuilder: ContentReport.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListReportsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListReportsResponse copyWith(void Function(ListReportsResponse) updates) =>
      super.copyWith((message) => updates(message as ListReportsResponse))
          as ListReportsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListReportsResponse create() => ListReportsResponse._();
  @$core.override
  ListReportsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListReportsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListReportsResponse>(create);
  static ListReportsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<ContentReport> get reports => $_getList(0);
}

class DecideReportRequest extends $pb.GeneratedMessage {
  factory DecideReportRequest({
    $core.String? reportId,
    $core.String? decision,
    $core.String? reason,
  }) {
    final result = create();
    if (reportId != null) result.reportId = reportId;
    if (decision != null) result.decision = decision;
    if (reason != null) result.reason = reason;
    return result;
  }

  DecideReportRequest._();

  factory DecideReportRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DecideReportRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecideReportRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'reportId')
    ..aOS(2, _omitFieldNames ? '' : 'decision')
    ..aOS(3, _omitFieldNames ? '' : 'reason')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecideReportRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecideReportRequest copyWith(void Function(DecideReportRequest) updates) =>
      super.copyWith((message) => updates(message as DecideReportRequest))
          as DecideReportRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DecideReportRequest create() => DecideReportRequest._();
  @$core.override
  DecideReportRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DecideReportRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecideReportRequest>(create);
  static DecideReportRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get reportId => $_getSZ(0);
  @$pb.TagNumber(1)
  set reportId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasReportId() => $_has(0);
  @$pb.TagNumber(1)
  void clearReportId() => $_clearField(1);

  /// removed or dismissed.
  @$pb.TagNumber(2)
  $core.String get decision => $_getSZ(1);
  @$pb.TagNumber(2)
  set decision($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDecision() => $_has(1);
  @$pb.TagNumber(2)
  void clearDecision() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get reason => $_getSZ(2);
  @$pb.TagNumber(3)
  set reason($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasReason() => $_has(2);
  @$pb.TagNumber(3)
  void clearReason() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
