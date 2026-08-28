// This is a generated file - do not edit.
//
// Generated from country.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class Country extends $pb.GeneratedMessage {
  factory Country({
    $core.int? id,
    $core.String? ddi,
    $core.String? name,
    $core.String? acronym,
    $core.String? currency,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (ddi != null) result.ddi = ddi;
    if (name != null) result.name = name;
    if (acronym != null) result.acronym = acronym;
    if (currency != null) result.currency = currency;
    return result;
  }

  Country._();

  factory Country.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Country.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Country',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.country'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'ddi')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOS(4, _omitFieldNames ? '' : 'acronym')
    ..aOS(5, _omitFieldNames ? '' : 'currency')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Country clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Country copyWith(void Function(Country) updates) =>
      super.copyWith((message) => updates(message as Country)) as Country;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Country create() => Country._();
  @$core.override
  Country createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Country getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Country>(create);
  static Country? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get ddi => $_getSZ(1);
  @$pb.TagNumber(2)
  set ddi($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDdi() => $_has(1);
  @$pb.TagNumber(2)
  void clearDdi() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get acronym => $_getSZ(3);
  @$pb.TagNumber(4)
  set acronym($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAcronym() => $_has(3);
  @$pb.TagNumber(4)
  void clearAcronym() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get currency => $_getSZ(4);
  @$pb.TagNumber(5)
  set currency($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasCurrency() => $_has(4);
  @$pb.TagNumber(5)
  void clearCurrency() => $_clearField(5);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
