// This is a generated file - do not edit.
//
// Generated from timeline/evolution.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class BodyComposition extends $pb.GeneratedMessage {
  factory BodyComposition({
    $core.String? uuid,
    $core.double? weight,
    $core.double? bodyFatPct,
    $core.double? muscleMassPct,
    $core.double? visceralFat,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (weight != null) result.weight = weight;
    if (bodyFatPct != null) result.bodyFatPct = bodyFatPct;
    if (muscleMassPct != null) result.muscleMassPct = muscleMassPct;
    if (visceralFat != null) result.visceralFat = visceralFat;
    return result;
  }

  BodyComposition._();

  factory BodyComposition.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BodyComposition.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BodyComposition',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aD(2, _omitFieldNames ? '' : 'weight')
    ..aD(3, _omitFieldNames ? '' : 'bodyFatPct')
    ..aD(4, _omitFieldNames ? '' : 'muscleMassPct')
    ..aD(5, _omitFieldNames ? '' : 'visceralFat')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BodyComposition clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BodyComposition copyWith(void Function(BodyComposition) updates) =>
      super.copyWith((message) => updates(message as BodyComposition))
          as BodyComposition;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BodyComposition create() => BodyComposition._();
  @$core.override
  BodyComposition createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BodyComposition getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BodyComposition>(create);
  static BodyComposition? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.double get weight => $_getN(1);
  @$pb.TagNumber(2)
  set weight($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(2)
  $core.bool hasWeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearWeight() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.double get bodyFatPct => $_getN(2);
  @$pb.TagNumber(3)
  set bodyFatPct($core.double value) => $_setDouble(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBodyFatPct() => $_has(2);
  @$pb.TagNumber(3)
  void clearBodyFatPct() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.double get muscleMassPct => $_getN(3);
  @$pb.TagNumber(4)
  set muscleMassPct($core.double value) => $_setDouble(3, value);
  @$pb.TagNumber(4)
  $core.bool hasMuscleMassPct() => $_has(3);
  @$pb.TagNumber(4)
  void clearMuscleMassPct() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.double get visceralFat => $_getN(4);
  @$pb.TagNumber(5)
  set visceralFat($core.double value) => $_setDouble(4, value);
  @$pb.TagNumber(5)
  $core.bool hasVisceralFat() => $_has(4);
  @$pb.TagNumber(5)
  void clearVisceralFat() => $_clearField(5);
}

class Circumferences extends $pb.GeneratedMessage {
  factory Circumferences({
    $core.String? uuid,
    $core.double? neck,
    $core.double? chest,
    $core.double? waist,
    $core.double? abdomen,
    $core.double? hip,
    $core.double? bicepsRight,
    $core.double? bicepsLeft,
    $core.double? thighRight,
    $core.double? thighLeft,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (neck != null) result.neck = neck;
    if (chest != null) result.chest = chest;
    if (waist != null) result.waist = waist;
    if (abdomen != null) result.abdomen = abdomen;
    if (hip != null) result.hip = hip;
    if (bicepsRight != null) result.bicepsRight = bicepsRight;
    if (bicepsLeft != null) result.bicepsLeft = bicepsLeft;
    if (thighRight != null) result.thighRight = thighRight;
    if (thighLeft != null) result.thighLeft = thighLeft;
    return result;
  }

  Circumferences._();

  factory Circumferences.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Circumferences.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Circumferences',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aD(2, _omitFieldNames ? '' : 'neck')
    ..aD(3, _omitFieldNames ? '' : 'chest')
    ..aD(4, _omitFieldNames ? '' : 'waist')
    ..aD(5, _omitFieldNames ? '' : 'abdomen')
    ..aD(6, _omitFieldNames ? '' : 'hip')
    ..aD(7, _omitFieldNames ? '' : 'bicepsRight')
    ..aD(8, _omitFieldNames ? '' : 'bicepsLeft')
    ..aD(9, _omitFieldNames ? '' : 'thighRight')
    ..aD(10, _omitFieldNames ? '' : 'thighLeft')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Circumferences clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Circumferences copyWith(void Function(Circumferences) updates) =>
      super.copyWith((message) => updates(message as Circumferences))
          as Circumferences;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Circumferences create() => Circumferences._();
  @$core.override
  Circumferences createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Circumferences getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Circumferences>(create);
  static Circumferences? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.double get neck => $_getN(1);
  @$pb.TagNumber(2)
  set neck($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNeck() => $_has(1);
  @$pb.TagNumber(2)
  void clearNeck() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.double get chest => $_getN(2);
  @$pb.TagNumber(3)
  set chest($core.double value) => $_setDouble(2, value);
  @$pb.TagNumber(3)
  $core.bool hasChest() => $_has(2);
  @$pb.TagNumber(3)
  void clearChest() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.double get waist => $_getN(3);
  @$pb.TagNumber(4)
  set waist($core.double value) => $_setDouble(3, value);
  @$pb.TagNumber(4)
  $core.bool hasWaist() => $_has(3);
  @$pb.TagNumber(4)
  void clearWaist() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.double get abdomen => $_getN(4);
  @$pb.TagNumber(5)
  set abdomen($core.double value) => $_setDouble(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAbdomen() => $_has(4);
  @$pb.TagNumber(5)
  void clearAbdomen() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.double get hip => $_getN(5);
  @$pb.TagNumber(6)
  set hip($core.double value) => $_setDouble(5, value);
  @$pb.TagNumber(6)
  $core.bool hasHip() => $_has(5);
  @$pb.TagNumber(6)
  void clearHip() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.double get bicepsRight => $_getN(6);
  @$pb.TagNumber(7)
  set bicepsRight($core.double value) => $_setDouble(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBicepsRight() => $_has(6);
  @$pb.TagNumber(7)
  void clearBicepsRight() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.double get bicepsLeft => $_getN(7);
  @$pb.TagNumber(8)
  set bicepsLeft($core.double value) => $_setDouble(7, value);
  @$pb.TagNumber(8)
  $core.bool hasBicepsLeft() => $_has(7);
  @$pb.TagNumber(8)
  void clearBicepsLeft() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.double get thighRight => $_getN(8);
  @$pb.TagNumber(9)
  set thighRight($core.double value) => $_setDouble(8, value);
  @$pb.TagNumber(9)
  $core.bool hasThighRight() => $_has(8);
  @$pb.TagNumber(9)
  void clearThighRight() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.double get thighLeft => $_getN(9);
  @$pb.TagNumber(10)
  set thighLeft($core.double value) => $_setDouble(9, value);
  @$pb.TagNumber(10)
  $core.bool hasThighLeft() => $_has(9);
  @$pb.TagNumber(10)
  void clearThighLeft() => $_clearField(10);
}

class EvolutionCheckIn extends $pb.GeneratedMessage {
  factory EvolutionCheckIn({
    $core.String? uuid,
    $core.String? personUuid,
    $core.String? createdAt,
    $core.String? note,
    $core.String? visibility,
    BodyComposition? composition,
    Circumferences? circumferences,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (personUuid != null) result.personUuid = personUuid;
    if (createdAt != null) result.createdAt = createdAt;
    if (note != null) result.note = note;
    if (visibility != null) result.visibility = visibility;
    if (composition != null) result.composition = composition;
    if (circumferences != null) result.circumferences = circumferences;
    return result;
  }

  EvolutionCheckIn._();

  factory EvolutionCheckIn.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory EvolutionCheckIn.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EvolutionCheckIn',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'personUuid')
    ..aOS(3, _omitFieldNames ? '' : 'createdAt')
    ..aOS(4, _omitFieldNames ? '' : 'note')
    ..aOS(5, _omitFieldNames ? '' : 'visibility')
    ..aOM<BodyComposition>(6, _omitFieldNames ? '' : 'composition',
        subBuilder: BodyComposition.create)
    ..aOM<Circumferences>(7, _omitFieldNames ? '' : 'circumferences',
        subBuilder: Circumferences.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EvolutionCheckIn clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EvolutionCheckIn copyWith(void Function(EvolutionCheckIn) updates) =>
      super.copyWith((message) => updates(message as EvolutionCheckIn))
          as EvolutionCheckIn;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EvolutionCheckIn create() => EvolutionCheckIn._();
  @$core.override
  EvolutionCheckIn createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static EvolutionCheckIn getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EvolutionCheckIn>(create);
  static EvolutionCheckIn? _defaultInstance;

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
  $core.String get createdAt => $_getSZ(2);
  @$pb.TagNumber(3)
  set createdAt($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCreatedAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearCreatedAt() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get note => $_getSZ(3);
  @$pb.TagNumber(4)
  set note($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasNote() => $_has(3);
  @$pb.TagNumber(4)
  void clearNote() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get visibility => $_getSZ(4);
  @$pb.TagNumber(5)
  set visibility($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasVisibility() => $_has(4);
  @$pb.TagNumber(5)
  void clearVisibility() => $_clearField(5);

  @$pb.TagNumber(6)
  BodyComposition get composition => $_getN(5);
  @$pb.TagNumber(6)
  set composition(BodyComposition value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasComposition() => $_has(5);
  @$pb.TagNumber(6)
  void clearComposition() => $_clearField(6);
  @$pb.TagNumber(6)
  BodyComposition ensureComposition() => $_ensure(5);

  @$pb.TagNumber(7)
  Circumferences get circumferences => $_getN(6);
  @$pb.TagNumber(7)
  set circumferences(Circumferences value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasCircumferences() => $_has(6);
  @$pb.TagNumber(7)
  void clearCircumferences() => $_clearField(7);
  @$pb.TagNumber(7)
  Circumferences ensureCircumferences() => $_ensure(6);
}

class AddEvolutionCheckInRequest extends $pb.GeneratedMessage {
  factory AddEvolutionCheckInRequest({
    $core.String? createdAt,
    $core.String? note,
    $core.String? visibility,
    BodyComposition? composition,
    Circumferences? circumferences,
  }) {
    final result = create();
    if (createdAt != null) result.createdAt = createdAt;
    if (note != null) result.note = note;
    if (visibility != null) result.visibility = visibility;
    if (composition != null) result.composition = composition;
    if (circumferences != null) result.circumferences = circumferences;
    return result;
  }

  AddEvolutionCheckInRequest._();

  factory AddEvolutionCheckInRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AddEvolutionCheckInRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AddEvolutionCheckInRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'createdAt')
    ..aOS(2, _omitFieldNames ? '' : 'note')
    ..aOS(3, _omitFieldNames ? '' : 'visibility')
    ..aOM<BodyComposition>(4, _omitFieldNames ? '' : 'composition',
        subBuilder: BodyComposition.create)
    ..aOM<Circumferences>(5, _omitFieldNames ? '' : 'circumferences',
        subBuilder: Circumferences.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddEvolutionCheckInRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddEvolutionCheckInRequest copyWith(
          void Function(AddEvolutionCheckInRequest) updates) =>
      super.copyWith(
              (message) => updates(message as AddEvolutionCheckInRequest))
          as AddEvolutionCheckInRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddEvolutionCheckInRequest create() => AddEvolutionCheckInRequest._();
  @$core.override
  AddEvolutionCheckInRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AddEvolutionCheckInRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AddEvolutionCheckInRequest>(create);
  static AddEvolutionCheckInRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get createdAt => $_getSZ(0);
  @$pb.TagNumber(1)
  set createdAt($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCreatedAt() => $_has(0);
  @$pb.TagNumber(1)
  void clearCreatedAt() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get note => $_getSZ(1);
  @$pb.TagNumber(2)
  set note($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNote() => $_has(1);
  @$pb.TagNumber(2)
  void clearNote() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get visibility => $_getSZ(2);
  @$pb.TagNumber(3)
  set visibility($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVisibility() => $_has(2);
  @$pb.TagNumber(3)
  void clearVisibility() => $_clearField(3);

  @$pb.TagNumber(4)
  BodyComposition get composition => $_getN(3);
  @$pb.TagNumber(4)
  set composition(BodyComposition value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasComposition() => $_has(3);
  @$pb.TagNumber(4)
  void clearComposition() => $_clearField(4);
  @$pb.TagNumber(4)
  BodyComposition ensureComposition() => $_ensure(3);

  @$pb.TagNumber(5)
  Circumferences get circumferences => $_getN(4);
  @$pb.TagNumber(5)
  set circumferences(Circumferences value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasCircumferences() => $_has(4);
  @$pb.TagNumber(5)
  void clearCircumferences() => $_clearField(5);
  @$pb.TagNumber(5)
  Circumferences ensureCircumferences() => $_ensure(4);
}

class ListEvolutionCheckInsRequest extends $pb.GeneratedMessage {
  factory ListEvolutionCheckInsRequest({
    $core.String? startDate,
    $core.String? endDate,
  }) {
    final result = create();
    if (startDate != null) result.startDate = startDate;
    if (endDate != null) result.endDate = endDate;
    return result;
  }

  ListEvolutionCheckInsRequest._();

  factory ListEvolutionCheckInsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListEvolutionCheckInsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListEvolutionCheckInsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'startDate')
    ..aOS(2, _omitFieldNames ? '' : 'endDate')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEvolutionCheckInsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEvolutionCheckInsRequest copyWith(
          void Function(ListEvolutionCheckInsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListEvolutionCheckInsRequest))
          as ListEvolutionCheckInsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListEvolutionCheckInsRequest create() =>
      ListEvolutionCheckInsRequest._();
  @$core.override
  ListEvolutionCheckInsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListEvolutionCheckInsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEvolutionCheckInsRequest>(create);
  static ListEvolutionCheckInsRequest? _defaultInstance;

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

class ListEvolutionCheckInsResponse extends $pb.GeneratedMessage {
  factory ListEvolutionCheckInsResponse({
    $core.Iterable<EvolutionCheckIn>? checkIns,
  }) {
    final result = create();
    if (checkIns != null) result.checkIns.addAll(checkIns);
    return result;
  }

  ListEvolutionCheckInsResponse._();

  factory ListEvolutionCheckInsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListEvolutionCheckInsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListEvolutionCheckInsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<EvolutionCheckIn>(1, _omitFieldNames ? '' : 'checkIns',
        subBuilder: EvolutionCheckIn.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEvolutionCheckInsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListEvolutionCheckInsResponse copyWith(
          void Function(ListEvolutionCheckInsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListEvolutionCheckInsResponse))
          as ListEvolutionCheckInsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListEvolutionCheckInsResponse create() =>
      ListEvolutionCheckInsResponse._();
  @$core.override
  ListEvolutionCheckInsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListEvolutionCheckInsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEvolutionCheckInsResponse>(create);
  static ListEvolutionCheckInsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<EvolutionCheckIn> get checkIns => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
