// This is a generated file - do not edit.
//
// Generated from person_info.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class PersonInfo extends $pb.GeneratedMessage {
  factory PersonInfo({
    $core.int? id,
    $core.int? personId,
    $core.String? biography,
    $core.String? relationship,
    $core.String? job,
    $core.String? homeTown,
    $core.String? currentCity,
    $core.double? weight,
    $core.double? height,
    $core.String? uuid,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (personId != null) result.personId = personId;
    if (biography != null) result.biography = biography;
    if (relationship != null) result.relationship = relationship;
    if (job != null) result.job = job;
    if (homeTown != null) result.homeTown = homeTown;
    if (currentCity != null) result.currentCity = currentCity;
    if (weight != null) result.weight = weight;
    if (height != null) result.height = height;
    if (uuid != null) result.uuid = uuid;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  PersonInfo._();

  factory PersonInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonInfo',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person_info'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aI(2, _omitFieldNames ? '' : 'personId')
    ..aOS(3, _omitFieldNames ? '' : 'biography')
    ..aOS(4, _omitFieldNames ? '' : 'relationship')
    ..aOS(5, _omitFieldNames ? '' : 'job')
    ..aOS(6, _omitFieldNames ? '' : 'homeTown')
    ..aOS(7, _omitFieldNames ? '' : 'currentCity')
    ..aD(8, _omitFieldNames ? '' : 'weight', fieldType: $pb.PbFieldType.OF)
    ..aD(9, _omitFieldNames ? '' : 'height', fieldType: $pb.PbFieldType.OF)
    ..aOS(10, _omitFieldNames ? '' : 'uuid')
    ..aOS(11, _omitFieldNames ? '' : 'createdAt')
    ..aOS(12, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonInfo copyWith(void Function(PersonInfo) updates) =>
      super.copyWith((message) => updates(message as PersonInfo)) as PersonInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonInfo create() => PersonInfo._();
  @$core.override
  PersonInfo createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonInfo getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonInfo>(create);
  static PersonInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get personId => $_getIZ(1);
  @$pb.TagNumber(2)
  set personId($core.int value) => $_setSignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonId() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get biography => $_getSZ(2);
  @$pb.TagNumber(3)
  set biography($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBiography() => $_has(2);
  @$pb.TagNumber(3)
  void clearBiography() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get relationship => $_getSZ(3);
  @$pb.TagNumber(4)
  set relationship($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRelationship() => $_has(3);
  @$pb.TagNumber(4)
  void clearRelationship() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get job => $_getSZ(4);
  @$pb.TagNumber(5)
  set job($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasJob() => $_has(4);
  @$pb.TagNumber(5)
  void clearJob() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get homeTown => $_getSZ(5);
  @$pb.TagNumber(6)
  set homeTown($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasHomeTown() => $_has(5);
  @$pb.TagNumber(6)
  void clearHomeTown() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get currentCity => $_getSZ(6);
  @$pb.TagNumber(7)
  set currentCity($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasCurrentCity() => $_has(6);
  @$pb.TagNumber(7)
  void clearCurrentCity() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.double get weight => $_getN(7);
  @$pb.TagNumber(8)
  set weight($core.double value) => $_setFloat(7, value);
  @$pb.TagNumber(8)
  $core.bool hasWeight() => $_has(7);
  @$pb.TagNumber(8)
  void clearWeight() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.double get height => $_getN(8);
  @$pb.TagNumber(9)
  set height($core.double value) => $_setFloat(8, value);
  @$pb.TagNumber(9)
  $core.bool hasHeight() => $_has(8);
  @$pb.TagNumber(9)
  void clearHeight() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get uuid => $_getSZ(9);
  @$pb.TagNumber(10)
  set uuid($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUuid() => $_has(9);
  @$pb.TagNumber(10)
  void clearUuid() => $_clearField(10);

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

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
