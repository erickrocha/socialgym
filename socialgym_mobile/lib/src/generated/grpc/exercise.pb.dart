// This is a generated file - do not edit.
//
// Generated from exercise.proto.

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

class ExerciseParams extends $pb.GeneratedMessage {
  factory ExerciseParams({
    $core.String? ownerUuid,
    $core.String? category,
    $core.String? visibility,
    $core.Iterable<$core.String>? owners,
    $fixnum.Int64? pageNumber,
    $fixnum.Int64? pageSize,
    $core.String? sortBy,
  }) {
    final result = create();
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    if (category != null) result.category = category;
    if (visibility != null) result.visibility = visibility;
    if (owners != null) result.owners.addAll(owners);
    if (pageNumber != null) result.pageNumber = pageNumber;
    if (pageSize != null) result.pageSize = pageSize;
    if (sortBy != null) result.sortBy = sortBy;
    return result;
  }

  ExerciseParams._();

  factory ExerciseParams.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ExerciseParams.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExerciseParams',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.exercise'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'ownerUuid')
    ..aOS(2, _omitFieldNames ? '' : 'category')
    ..aOS(3, _omitFieldNames ? '' : 'visibility')
    ..pPS(4, _omitFieldNames ? '' : 'owners')
    ..aInt64(5, _omitFieldNames ? '' : 'pageNumber')
    ..aInt64(6, _omitFieldNames ? '' : 'pageSize')
    ..aOS(7, _omitFieldNames ? '' : 'sortBy')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExerciseParams clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExerciseParams copyWith(void Function(ExerciseParams) updates) =>
      super.copyWith((message) => updates(message as ExerciseParams))
          as ExerciseParams;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ExerciseParams create() => ExerciseParams._();
  @$core.override
  ExerciseParams createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ExerciseParams getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExerciseParams>(create);
  static ExerciseParams? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get ownerUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set ownerUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOwnerUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwnerUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get category => $_getSZ(1);
  @$pb.TagNumber(2)
  set category($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCategory() => $_has(1);
  @$pb.TagNumber(2)
  void clearCategory() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get visibility => $_getSZ(2);
  @$pb.TagNumber(3)
  set visibility($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVisibility() => $_has(2);
  @$pb.TagNumber(3)
  void clearVisibility() => $_clearField(3);

  @$pb.TagNumber(4)
  $pb.PbList<$core.String> get owners => $_getList(3);

  @$pb.TagNumber(5)
  $fixnum.Int64 get pageNumber => $_getI64(4);
  @$pb.TagNumber(5)
  set pageNumber($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPageNumber() => $_has(4);
  @$pb.TagNumber(5)
  void clearPageNumber() => $_clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get pageSize => $_getI64(5);
  @$pb.TagNumber(6)
  set pageSize($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPageSize() => $_has(5);
  @$pb.TagNumber(6)
  void clearPageSize() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get sortBy => $_getSZ(6);
  @$pb.TagNumber(7)
  set sortBy($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSortBy() => $_has(6);
  @$pb.TagNumber(7)
  void clearSortBy() => $_clearField(7);
}

enum ExerciseRequest_Identifier { id, uuid, notSet }

class ExerciseRequest extends $pb.GeneratedMessage {
  factory ExerciseRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  ExerciseRequest._();

  factory ExerciseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ExerciseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ExerciseRequest_Identifier>
      _ExerciseRequest_IdentifierByTag = {
    1: ExerciseRequest_Identifier.id,
    2: ExerciseRequest_Identifier.uuid,
    0: ExerciseRequest_Identifier.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExerciseRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.exercise'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExerciseRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExerciseRequest copyWith(void Function(ExerciseRequest) updates) =>
      super.copyWith((message) => updates(message as ExerciseRequest))
          as ExerciseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ExerciseRequest create() => ExerciseRequest._();
  @$core.override
  ExerciseRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ExerciseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExerciseRequest>(create);
  static ExerciseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  ExerciseRequest_Identifier whichIdentifier() =>
      _ExerciseRequest_IdentifierByTag[$_whichOneof(0)]!;
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

class Exercise extends $pb.GeneratedMessage {
  factory Exercise({
    $core.int? id,
    $core.String? name,
    $core.int? ownerId,
    $core.String? ownerUuid,
    $core.String? ownerName,
    $core.String? description,
    $core.int? sets,
    $core.String? category,
    $core.int? repsOrDuration,
    $core.String? uuid,
    $core.String? visibility,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    if (ownerName != null) result.ownerName = ownerName;
    if (description != null) result.description = description;
    if (sets != null) result.sets = sets;
    if (category != null) result.category = category;
    if (repsOrDuration != null) result.repsOrDuration = repsOrDuration;
    if (uuid != null) result.uuid = uuid;
    if (visibility != null) result.visibility = visibility;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Exercise._();

  factory Exercise.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Exercise.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Exercise',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.exercise'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aI(3, _omitFieldNames ? '' : 'ownerId')
    ..aOS(4, _omitFieldNames ? '' : 'ownerUuid')
    ..aOS(5, _omitFieldNames ? '' : 'ownerName')
    ..aOS(6, _omitFieldNames ? '' : 'description')
    ..aI(7, _omitFieldNames ? '' : 'sets')
    ..aOS(8, _omitFieldNames ? '' : 'category')
    ..aI(9, _omitFieldNames ? '' : 'repsOrDuration')
    ..aOS(10, _omitFieldNames ? '' : 'uuid')
    ..aOS(11, _omitFieldNames ? '' : 'visibility')
    ..aOS(12, _omitFieldNames ? '' : 'createdAt')
    ..aOS(13, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Exercise clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Exercise copyWith(void Function(Exercise) updates) =>
      super.copyWith((message) => updates(message as Exercise)) as Exercise;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Exercise create() => Exercise._();
  @$core.override
  Exercise createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Exercise getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Exercise>(create);
  static Exercise? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

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
  $core.String get ownerName => $_getSZ(4);
  @$pb.TagNumber(5)
  set ownerName($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasOwnerName() => $_has(4);
  @$pb.TagNumber(5)
  void clearOwnerName() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get description => $_getSZ(5);
  @$pb.TagNumber(6)
  set description($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasDescription() => $_has(5);
  @$pb.TagNumber(6)
  void clearDescription() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.int get sets => $_getIZ(6);
  @$pb.TagNumber(7)
  set sets($core.int value) => $_setSignedInt32(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSets() => $_has(6);
  @$pb.TagNumber(7)
  void clearSets() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get category => $_getSZ(7);
  @$pb.TagNumber(8)
  set category($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasCategory() => $_has(7);
  @$pb.TagNumber(8)
  void clearCategory() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.int get repsOrDuration => $_getIZ(8);
  @$pb.TagNumber(9)
  set repsOrDuration($core.int value) => $_setSignedInt32(8, value);
  @$pb.TagNumber(9)
  $core.bool hasRepsOrDuration() => $_has(8);
  @$pb.TagNumber(9)
  void clearRepsOrDuration() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get uuid => $_getSZ(9);
  @$pb.TagNumber(10)
  set uuid($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUuid() => $_has(9);
  @$pb.TagNumber(10)
  void clearUuid() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get visibility => $_getSZ(10);
  @$pb.TagNumber(11)
  set visibility($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasVisibility() => $_has(10);
  @$pb.TagNumber(11)
  void clearVisibility() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get createdAt => $_getSZ(11);
  @$pb.TagNumber(12)
  set createdAt($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasCreatedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearCreatedAt() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get updatedAt => $_getSZ(12);
  @$pb.TagNumber(13)
  set updatedAt($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasUpdatedAt() => $_has(12);
  @$pb.TagNumber(13)
  void clearUpdatedAt() => $_clearField(13);
}

class PaginatedExercise extends $pb.GeneratedMessage {
  factory PaginatedExercise({
    $core.Iterable<Exercise>? content,
    $fixnum.Int64? totalCount,
    $fixnum.Int64? pageNumber,
    $fixnum.Int64? pageSize,
    $core.bool? hasNextPage,
  }) {
    final result = create();
    if (content != null) result.content.addAll(content);
    if (totalCount != null) result.totalCount = totalCount;
    if (pageNumber != null) result.pageNumber = pageNumber;
    if (pageSize != null) result.pageSize = pageSize;
    if (hasNextPage != null) result.hasNextPage = hasNextPage;
    return result;
  }

  PaginatedExercise._();

  factory PaginatedExercise.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PaginatedExercise.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PaginatedExercise',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.exercise'),
      createEmptyInstance: create)
    ..pPM<Exercise>(1, _omitFieldNames ? '' : 'content',
        subBuilder: Exercise.create)
    ..aInt64(2, _omitFieldNames ? '' : 'totalCount')
    ..aInt64(3, _omitFieldNames ? '' : 'pageNumber')
    ..aInt64(4, _omitFieldNames ? '' : 'pageSize')
    ..aOB(5, _omitFieldNames ? '' : 'hasNextPage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PaginatedExercise clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PaginatedExercise copyWith(void Function(PaginatedExercise) updates) =>
      super.copyWith((message) => updates(message as PaginatedExercise))
          as PaginatedExercise;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PaginatedExercise create() => PaginatedExercise._();
  @$core.override
  PaginatedExercise createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PaginatedExercise getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PaginatedExercise>(create);
  static PaginatedExercise? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Exercise> get content => $_getList(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalCount => $_getI64(1);
  @$pb.TagNumber(2)
  set totalCount($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTotalCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalCount() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get pageNumber => $_getI64(2);
  @$pb.TagNumber(3)
  set pageNumber($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPageNumber() => $_has(2);
  @$pb.TagNumber(3)
  void clearPageNumber() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get pageSize => $_getI64(3);
  @$pb.TagNumber(4)
  set pageSize($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPageSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearPageSize() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get hasNextPage => $_getBF(4);
  @$pb.TagNumber(5)
  set hasNextPage($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasHasNextPage() => $_has(4);
  @$pb.TagNumber(5)
  void clearHasNextPage() => $_clearField(5);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
