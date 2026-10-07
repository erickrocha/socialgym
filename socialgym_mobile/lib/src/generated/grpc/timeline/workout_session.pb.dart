// This is a generated file - do not edit.
//
// Generated from timeline/workout_session.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ExecutedSet extends $pb.GeneratedMessage {
  factory ExecutedSet({
    $core.String? uuid,
    $core.String? exerciseName,
    $core.int? ownerId,
    $core.String? ownerName,
    $core.String? category,
    $core.String? visibility,
    $core.int? setNumber,
    $core.int? repsOrDuration,
    $core.double? weight,
    $core.String? startedAt,
    $core.String? completedAt,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (exerciseName != null) result.exerciseName = exerciseName;
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerName != null) result.ownerName = ownerName;
    if (category != null) result.category = category;
    if (visibility != null) result.visibility = visibility;
    if (setNumber != null) result.setNumber = setNumber;
    if (repsOrDuration != null) result.repsOrDuration = repsOrDuration;
    if (weight != null) result.weight = weight;
    if (startedAt != null) result.startedAt = startedAt;
    if (completedAt != null) result.completedAt = completedAt;
    return result;
  }

  ExecutedSet._();

  factory ExecutedSet.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ExecutedSet.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExecutedSet',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'exerciseName')
    ..aI(3, _omitFieldNames ? '' : 'ownerId')
    ..aOS(4, _omitFieldNames ? '' : 'ownerName')
    ..aOS(5, _omitFieldNames ? '' : 'category')
    ..aOS(6, _omitFieldNames ? '' : 'visibility')
    ..aI(7, _omitFieldNames ? '' : 'setNumber')
    ..aI(8, _omitFieldNames ? '' : 'repsOrDuration')
    ..aD(9, _omitFieldNames ? '' : 'weight', fieldType: $pb.PbFieldType.OF)
    ..aOS(10, _omitFieldNames ? '' : 'startedAt')
    ..aOS(11, _omitFieldNames ? '' : 'completedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExecutedSet clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExecutedSet copyWith(void Function(ExecutedSet) updates) =>
      super.copyWith((message) => updates(message as ExecutedSet))
          as ExecutedSet;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ExecutedSet create() => ExecutedSet._();
  @$core.override
  ExecutedSet createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ExecutedSet getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExecutedSet>(create);
  static ExecutedSet? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get exerciseName => $_getSZ(1);
  @$pb.TagNumber(2)
  set exerciseName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasExerciseName() => $_has(1);
  @$pb.TagNumber(2)
  void clearExerciseName() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get ownerId => $_getIZ(2);
  @$pb.TagNumber(3)
  set ownerId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwnerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwnerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get ownerName => $_getSZ(3);
  @$pb.TagNumber(4)
  set ownerName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOwnerName() => $_has(3);
  @$pb.TagNumber(4)
  void clearOwnerName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get category => $_getSZ(4);
  @$pb.TagNumber(5)
  set category($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCategory() => $_has(4);
  @$pb.TagNumber(5)
  void clearCategory() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get visibility => $_getSZ(5);
  @$pb.TagNumber(6)
  set visibility($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasVisibility() => $_has(5);
  @$pb.TagNumber(6)
  void clearVisibility() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.int get setNumber => $_getIZ(6);
  @$pb.TagNumber(7)
  set setNumber($core.int value) => $_setSignedInt32(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSetNumber() => $_has(6);
  @$pb.TagNumber(7)
  void clearSetNumber() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.int get repsOrDuration => $_getIZ(7);
  @$pb.TagNumber(8)
  set repsOrDuration($core.int value) => $_setSignedInt32(7, value);
  @$pb.TagNumber(8)
  $core.bool hasRepsOrDuration() => $_has(7);
  @$pb.TagNumber(8)
  void clearRepsOrDuration() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.double get weight => $_getN(8);
  @$pb.TagNumber(9)
  set weight($core.double value) => $_setFloat(8, value);
  @$pb.TagNumber(9)
  $core.bool hasWeight() => $_has(8);
  @$pb.TagNumber(9)
  void clearWeight() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get startedAt => $_getSZ(9);
  @$pb.TagNumber(10)
  set startedAt($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasStartedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearStartedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get completedAt => $_getSZ(10);
  @$pb.TagNumber(11)
  set completedAt($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCompletedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearCompletedAt() => $_clearField(11);
}

class WorkoutSession extends $pb.GeneratedMessage {
  factory WorkoutSession({
    $core.String? uuid,
    $core.String? personUuid,
    $core.String? workoutName,
    $core.int? duration,
    $core.String? startedAt,
    $core.String? dayOfWeek,
    $core.String? completedAt,
    $core.Iterable<ExecutedSet>? executedSets,
    $core.double? totalVolume,
    $core.double? totalSets,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (personUuid != null) result.personUuid = personUuid;
    if (workoutName != null) result.workoutName = workoutName;
    if (duration != null) result.duration = duration;
    if (startedAt != null) result.startedAt = startedAt;
    if (dayOfWeek != null) result.dayOfWeek = dayOfWeek;
    if (completedAt != null) result.completedAt = completedAt;
    if (executedSets != null) result.executedSets.addAll(executedSets);
    if (totalVolume != null) result.totalVolume = totalVolume;
    if (totalSets != null) result.totalSets = totalSets;
    return result;
  }

  WorkoutSession._();

  factory WorkoutSession.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory WorkoutSession.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WorkoutSession',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'personUuid')
    ..aOS(3, _omitFieldNames ? '' : 'workoutName')
    ..aI(4, _omitFieldNames ? '' : 'duration')
    ..aOS(5, _omitFieldNames ? '' : 'startedAt')
    ..aOS(6, _omitFieldNames ? '' : 'dayOfWeek')
    ..aOS(7, _omitFieldNames ? '' : 'completedAt')
    ..pPM<ExecutedSet>(8, _omitFieldNames ? '' : 'executedSets',
        subBuilder: ExecutedSet.create)
    ..aD(9, _omitFieldNames ? '' : 'totalVolume', fieldType: $pb.PbFieldType.OF)
    ..aD(10, _omitFieldNames ? '' : 'totalSets', fieldType: $pb.PbFieldType.OF)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutSession clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutSession copyWith(void Function(WorkoutSession) updates) =>
      super.copyWith((message) => updates(message as WorkoutSession))
          as WorkoutSession;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static WorkoutSession create() => WorkoutSession._();
  @$core.override
  WorkoutSession createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static WorkoutSession getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<WorkoutSession>(create);
  static WorkoutSession? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get personUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set personUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get workoutName => $_getSZ(2);
  @$pb.TagNumber(3)
  set workoutName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasWorkoutName() => $_has(2);
  @$pb.TagNumber(3)
  void clearWorkoutName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get duration => $_getIZ(3);
  @$pb.TagNumber(4)
  set duration($core.int value) => $_setSignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasDuration() => $_has(3);
  @$pb.TagNumber(4)
  void clearDuration() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get startedAt => $_getSZ(4);
  @$pb.TagNumber(5)
  set startedAt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasStartedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearStartedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get dayOfWeek => $_getSZ(5);
  @$pb.TagNumber(6)
  set dayOfWeek($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDayOfWeek() => $_has(5);
  @$pb.TagNumber(6)
  void clearDayOfWeek() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get completedAt => $_getSZ(6);
  @$pb.TagNumber(7)
  set completedAt($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCompletedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearCompletedAt() => $_clearField(7);

  @$pb.TagNumber(8)
  $pb.PbList<ExecutedSet> get executedSets => $_getList(7);

  @$pb.TagNumber(9)
  $core.double get totalVolume => $_getN(8);
  @$pb.TagNumber(9)
  set totalVolume($core.double value) => $_setFloat(8, value);
  @$pb.TagNumber(9)
  $core.bool hasTotalVolume() => $_has(8);
  @$pb.TagNumber(9)
  void clearTotalVolume() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.double get totalSets => $_getN(9);
  @$pb.TagNumber(10)
  set totalSets($core.double value) => $_setFloat(9, value);
  @$pb.TagNumber(10)
  $core.bool hasTotalSets() => $_has(9);
  @$pb.TagNumber(10)
  void clearTotalSets() => $_clearField(10);
}

/// started_at, completed_at and every set's dates and owner_name are required (INVALID_ARGUMENT
/// otherwise); uuid and person_uuid are ignored.
class CreateWorkoutSessionRequest extends $pb.GeneratedMessage {
  factory CreateWorkoutSessionRequest({
    WorkoutSession? session,
  }) {
    final result = create();
    if (session != null) result.session = session;
    return result;
  }

  CreateWorkoutSessionRequest._();

  factory CreateWorkoutSessionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateWorkoutSessionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateWorkoutSessionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOM<WorkoutSession>(1, _omitFieldNames ? '' : 'session',
        subBuilder: WorkoutSession.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateWorkoutSessionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateWorkoutSessionRequest copyWith(
          void Function(CreateWorkoutSessionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CreateWorkoutSessionRequest))
          as CreateWorkoutSessionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateWorkoutSessionRequest create() =>
      CreateWorkoutSessionRequest._();
  @$core.override
  CreateWorkoutSessionRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateWorkoutSessionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateWorkoutSessionRequest>(create);
  static CreateWorkoutSessionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  WorkoutSession get session => $_getN(0);
  @$pb.TagNumber(1)
  set session(WorkoutSession value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSession() => $_has(0);
  @$pb.TagNumber(1)
  void clearSession() => $_clearField(1);
  @$pb.TagNumber(1)
  WorkoutSession ensureSession() => $_ensure(0);
}

class GetWorkoutSessionRequest extends $pb.GeneratedMessage {
  factory GetWorkoutSessionRequest({
    $core.String? sessionUuid,
  }) {
    final result = create();
    if (sessionUuid != null) result.sessionUuid = sessionUuid;
    return result;
  }

  GetWorkoutSessionRequest._();

  factory GetWorkoutSessionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetWorkoutSessionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetWorkoutSessionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetWorkoutSessionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetWorkoutSessionRequest copyWith(
          void Function(GetWorkoutSessionRequest) updates) =>
      super.copyWith((message) => updates(message as GetWorkoutSessionRequest))
          as GetWorkoutSessionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetWorkoutSessionRequest create() => GetWorkoutSessionRequest._();
  @$core.override
  GetWorkoutSessionRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetWorkoutSessionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetWorkoutSessionRequest>(create);
  static GetWorkoutSessionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionUuid() => $_clearField(1);
}

class ListWorkoutSessionsRequest extends $pb.GeneratedMessage {
  factory ListWorkoutSessionsRequest({
    $core.String? startDate,
    $core.String? endDate,
  }) {
    final result = create();
    if (startDate != null) result.startDate = startDate;
    if (endDate != null) result.endDate = endDate;
    return result;
  }

  ListWorkoutSessionsRequest._();

  factory ListWorkoutSessionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListWorkoutSessionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListWorkoutSessionsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'startDate')
    ..aOS(2, _omitFieldNames ? '' : 'endDate')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWorkoutSessionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWorkoutSessionsRequest copyWith(
          void Function(ListWorkoutSessionsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListWorkoutSessionsRequest))
          as ListWorkoutSessionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListWorkoutSessionsRequest create() => ListWorkoutSessionsRequest._();
  @$core.override
  ListWorkoutSessionsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListWorkoutSessionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListWorkoutSessionsRequest>(create);
  static ListWorkoutSessionsRequest? _defaultInstance;

  /// ISO-8601; the end defaults to now and the start to seven days before the end.
  @$pb.TagNumber(1)
  $core.String get startDate => $_getSZ(0);
  @$pb.TagNumber(1)
  set startDate($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStartDate() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartDate() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get endDate => $_getSZ(1);
  @$pb.TagNumber(2)
  set endDate($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEndDate() => $_has(1);
  @$pb.TagNumber(2)
  void clearEndDate() => $_clearField(2);
}

class ListWorkoutSessionsResponse extends $pb.GeneratedMessage {
  factory ListWorkoutSessionsResponse({
    $core.Iterable<WorkoutSession>? sessions,
  }) {
    final result = create();
    if (sessions != null) result.sessions.addAll(sessions);
    return result;
  }

  ListWorkoutSessionsResponse._();

  factory ListWorkoutSessionsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListWorkoutSessionsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListWorkoutSessionsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<WorkoutSession>(1, _omitFieldNames ? '' : 'sessions',
        subBuilder: WorkoutSession.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWorkoutSessionsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListWorkoutSessionsResponse copyWith(
          void Function(ListWorkoutSessionsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListWorkoutSessionsResponse))
          as ListWorkoutSessionsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListWorkoutSessionsResponse create() =>
      ListWorkoutSessionsResponse._();
  @$core.override
  ListWorkoutSessionsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListWorkoutSessionsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListWorkoutSessionsResponse>(create);
  static ListWorkoutSessionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<WorkoutSession> get sessions => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
