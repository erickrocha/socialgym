// This is a generated file - do not edit.
//
// Generated from business_profile_address.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class BusinessProfileAddress extends $pb.GeneratedMessage {
  factory BusinessProfileAddress({
    $core.int? id,
    $core.String? uuid,
    $core.int? businessProfileId,
    $core.String? addressLine1,
    $core.String? addressLine2,
    $core.String? locality,
    $core.String? administrativeArea,
    $core.String? postalCode,
    $core.String? countryCode,
    $core.double? latitude,
    $core.double? longitude,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (businessProfileId != null) result.businessProfileId = businessProfileId;
    if (addressLine1 != null) result.addressLine1 = addressLine1;
    if (addressLine2 != null) result.addressLine2 = addressLine2;
    if (locality != null) result.locality = locality;
    if (administrativeArea != null)
      result.administrativeArea = administrativeArea;
    if (postalCode != null) result.postalCode = postalCode;
    if (countryCode != null) result.countryCode = countryCode;
    if (latitude != null) result.latitude = latitude;
    if (longitude != null) result.longitude = longitude;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  BusinessProfileAddress._();

  factory BusinessProfileAddress.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfileAddress.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfileAddress',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile_address'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aI(3, _omitFieldNames ? '' : 'businessProfileId')
    ..aOS(4, _omitFieldNames ? '' : 'addressLine1', protoName: 'address_line_1')
    ..aOS(5, _omitFieldNames ? '' : 'addressLine2', protoName: 'address_line_2')
    ..aOS(6, _omitFieldNames ? '' : 'locality')
    ..aOS(7, _omitFieldNames ? '' : 'administrativeArea')
    ..aOS(8, _omitFieldNames ? '' : 'postalCode')
    ..aOS(9, _omitFieldNames ? '' : 'countryCode')
    ..aD(10, _omitFieldNames ? '' : 'latitude')
    ..aD(11, _omitFieldNames ? '' : 'longitude')
    ..aOS(12, _omitFieldNames ? '' : 'createdAt')
    ..aOS(13, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileAddress clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileAddress copyWith(
          void Function(BusinessProfileAddress) updates) =>
      super.copyWith((message) => updates(message as BusinessProfileAddress))
          as BusinessProfileAddress;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfileAddress create() => BusinessProfileAddress._();
  @$core.override
  BusinessProfileAddress createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfileAddress getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfileAddress>(create);
  static BusinessProfileAddress? _defaultInstance;

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
  $core.int get businessProfileId => $_getIZ(2);
  @$pb.TagNumber(3)
  set businessProfileId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBusinessProfileId() => $_has(2);
  @$pb.TagNumber(3)
  void clearBusinessProfileId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get addressLine1 => $_getSZ(3);
  @$pb.TagNumber(4)
  set addressLine1($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAddressLine1() => $_has(3);
  @$pb.TagNumber(4)
  void clearAddressLine1() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get addressLine2 => $_getSZ(4);
  @$pb.TagNumber(5)
  set addressLine2($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAddressLine2() => $_has(4);
  @$pb.TagNumber(5)
  void clearAddressLine2() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get locality => $_getSZ(5);
  @$pb.TagNumber(6)
  set locality($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasLocality() => $_has(5);
  @$pb.TagNumber(6)
  void clearLocality() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get administrativeArea => $_getSZ(6);
  @$pb.TagNumber(7)
  set administrativeArea($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAdministrativeArea() => $_has(6);
  @$pb.TagNumber(7)
  void clearAdministrativeArea() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get postalCode => $_getSZ(7);
  @$pb.TagNumber(8)
  set postalCode($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasPostalCode() => $_has(7);
  @$pb.TagNumber(8)
  void clearPostalCode() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get countryCode => $_getSZ(8);
  @$pb.TagNumber(9)
  set countryCode($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCountryCode() => $_has(8);
  @$pb.TagNumber(9)
  void clearCountryCode() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.double get latitude => $_getN(9);
  @$pb.TagNumber(10)
  set latitude($core.double value) => $_setDouble(9, value);
  @$pb.TagNumber(10)
  $core.bool hasLatitude() => $_has(9);
  @$pb.TagNumber(10)
  void clearLatitude() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.double get longitude => $_getN(10);
  @$pb.TagNumber(11)
  set longitude($core.double value) => $_setDouble(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLongitude() => $_has(10);
  @$pb.TagNumber(11)
  void clearLongitude() => $_clearField(11);

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

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
