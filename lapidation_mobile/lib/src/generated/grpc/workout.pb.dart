// This is a generated file - do not edit.
//
// Generated from workout.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'exercise.pb.dart' as $2;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class Workout extends $pb.GeneratedMessage {
  factory Workout({
    $core.int? id,
    $core.String? uuid,
    $core.int? ownerId,
    $core.String? ownerUuid,
    $core.String? name,
    $core.String? description,
    $core.String? difficulty,
    $core.String? muscleGroup,
    $core.String? visibility,
    $core.Iterable<$2.Exercise>? exercises,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    if (name != null) result.name = name;
    if (description != null) result.description = description;
    if (difficulty != null) result.difficulty = difficulty;
    if (muscleGroup != null) result.muscleGroup = muscleGroup;
    if (visibility != null) result.visibility = visibility;
    if (exercises != null) result.exercises.addAll(exercises);
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Workout._();

  factory Workout.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Workout.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Workout',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.workout'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aI(3, _omitFieldNames ? '' : 'ownerId')
    ..aOS(4, _omitFieldNames ? '' : 'ownerUuid')
    ..aOS(5, _omitFieldNames ? '' : 'name')
    ..aOS(6, _omitFieldNames ? '' : 'description')
    ..aOS(7, _omitFieldNames ? '' : 'difficulty')
    ..aOS(8, _omitFieldNames ? '' : 'muscleGroup')
    ..aOS(9, _omitFieldNames ? '' : 'visibility')
    ..pPM<$2.Exercise>(10, _omitFieldNames ? '' : 'exercises',
        subBuilder: $2.Exercise.create)
    ..aOS(11, _omitFieldNames ? '' : 'createdAt')
    ..aOS(12, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Workout clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Workout copyWith(void Function(Workout) updates) =>
      super.copyWith((message) => updates(message as Workout)) as Workout;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Workout create() => Workout._();
  @$core.override
  Workout createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Workout getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Workout>(create);
  static Workout? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get uuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set uuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get ownerId => $_getIZ(2);
  @$pb.TagNumber(3)
  set ownerId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwnerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwnerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get ownerUuid => $_getSZ(3);
  @$pb.TagNumber(4)
  set ownerUuid($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOwnerUuid() => $_has(3);
  @$pb.TagNumber(4)
  void clearOwnerUuid() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get name => $_getSZ(4);
  @$pb.TagNumber(5)
  set name($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasName() => $_has(4);
  @$pb.TagNumber(5)
  void clearName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get description => $_getSZ(5);
  @$pb.TagNumber(6)
  set description($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDescription() => $_has(5);
  @$pb.TagNumber(6)
  void clearDescription() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get difficulty => $_getSZ(6);
  @$pb.TagNumber(7)
  set difficulty($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasDifficulty() => $_has(6);
  @$pb.TagNumber(7)
  void clearDifficulty() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get muscleGroup => $_getSZ(7);
  @$pb.TagNumber(8)
  set muscleGroup($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasMuscleGroup() => $_has(7);
  @$pb.TagNumber(8)
  void clearMuscleGroup() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get visibility => $_getSZ(8);
  @$pb.TagNumber(9)
  set visibility($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasVisibility() => $_has(8);
  @$pb.TagNumber(9)
  void clearVisibility() => $_clearField(9);

  @$pb.TagNumber(10)
  $pb.PbList<$2.Exercise> get exercises => $_getList(9);

  @$pb.TagNumber(11)
  $core.String get createdAt => $_getSZ(10);
  @$pb.TagNumber(11)
  set createdAt($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCreatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearCreatedAt() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get updatedAt => $_getSZ(11);
  @$pb.TagNumber(12)
  set updatedAt($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasUpdatedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearUpdatedAt() => $_clearField(12);
}

enum WorkoutRequest_Identifier { id, uuid, notSet }

class WorkoutRequest extends $pb.GeneratedMessage {
  factory WorkoutRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  WorkoutRequest._();

  factory WorkoutRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory WorkoutRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, WorkoutRequest_Identifier>
      _WorkoutRequest_IdentifierByTag = {
    1: WorkoutRequest_Identifier.id,
    2: WorkoutRequest_Identifier.uuid,
    0: WorkoutRequest_Identifier.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WorkoutRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.workout'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutRequest copyWith(void Function(WorkoutRequest) updates) =>
      super.copyWith((message) => updates(message as WorkoutRequest))
          as WorkoutRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static WorkoutRequest create() => WorkoutRequest._();
  @$core.override
  WorkoutRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static WorkoutRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<WorkoutRequest>(create);
  static WorkoutRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  WorkoutRequest_Identifier whichIdentifier() =>
      _WorkoutRequest_IdentifierByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearIdentifier() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get uuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set uuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUuid() => $_clearField(2);
}

class WorkoutExercisesRequest extends $pb.GeneratedMessage {
  factory WorkoutExercisesRequest({
    $core.String? workoutUuid,
    $core.Iterable<$2.Exercise>? exercises,
  }) {
    final result = create();
    if (workoutUuid != null) result.workoutUuid = workoutUuid;
    if (exercises != null) result.exercises.addAll(exercises);
    return result;
  }

  WorkoutExercisesRequest._();

  factory WorkoutExercisesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory WorkoutExercisesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WorkoutExercisesRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.workout'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'workoutUuid')
    ..pPM<$2.Exercise>(2, _omitFieldNames ? '' : 'exercises',
        subBuilder: $2.Exercise.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutExercisesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutExercisesRequest copyWith(
          void Function(WorkoutExercisesRequest) updates) =>
      super.copyWith((message) => updates(message as WorkoutExercisesRequest))
          as WorkoutExercisesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static WorkoutExercisesRequest create() => WorkoutExercisesRequest._();
  @$core.override
  WorkoutExercisesRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static WorkoutExercisesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<WorkoutExercisesRequest>(create);
  static WorkoutExercisesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get workoutUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set workoutUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasWorkoutUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearWorkoutUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$2.Exercise> get exercises => $_getList(1);
}

enum WorkoutListRequest_Identifier { ownerId, ownerUuid, notSet }

class WorkoutListRequest extends $pb.GeneratedMessage {
  factory WorkoutListRequest({
    $core.int? ownerId,
    $core.String? ownerUuid,
  }) {
    final result = create();
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    return result;
  }

  WorkoutListRequest._();

  factory WorkoutListRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory WorkoutListRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, WorkoutListRequest_Identifier>
      _WorkoutListRequest_IdentifierByTag = {
    1: WorkoutListRequest_Identifier.ownerId,
    2: WorkoutListRequest_Identifier.ownerUuid,
    0: WorkoutListRequest_Identifier.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WorkoutListRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.workout'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aI(1, _omitFieldNames ? '' : 'ownerId')
    ..aOS(2, _omitFieldNames ? '' : 'ownerUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutListRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutListRequest copyWith(void Function(WorkoutListRequest) updates) =>
      super.copyWith((message) => updates(message as WorkoutListRequest))
          as WorkoutListRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static WorkoutListRequest create() => WorkoutListRequest._();
  @$core.override
  WorkoutListRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static WorkoutListRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<WorkoutListRequest>(create);
  static WorkoutListRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  WorkoutListRequest_Identifier whichIdentifier() =>
      _WorkoutListRequest_IdentifierByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearIdentifier() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.int get ownerId => $_getIZ(0);
  @$pb.TagNumber(1)
  set ownerId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOwnerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwnerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get ownerUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set ownerUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwnerUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwnerUuid() => $_clearField(2);
}

class WorkoutResponse extends $pb.GeneratedMessage {
  factory WorkoutResponse({
    $core.Iterable<Workout>? workouts,
  }) {
    final result = create();
    if (workouts != null) result.workouts.addAll(workouts);
    return result;
  }

  WorkoutResponse._();

  factory WorkoutResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory WorkoutResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'WorkoutResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.workout'),
      createEmptyInstance: create)
    ..pPM<Workout>(1, _omitFieldNames ? '' : 'workouts',
        subBuilder: Workout.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  WorkoutResponse copyWith(void Function(WorkoutResponse) updates) =>
      super.copyWith((message) => updates(message as WorkoutResponse))
          as WorkoutResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static WorkoutResponse create() => WorkoutResponse._();
  @$core.override
  WorkoutResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static WorkoutResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<WorkoutResponse>(create);
  static WorkoutResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Workout> get workouts => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
