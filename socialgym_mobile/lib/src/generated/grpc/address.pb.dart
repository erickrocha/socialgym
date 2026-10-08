// This is a generated file - do not edit.
//
// Generated from address.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class SearchAddressRequest extends $pb.GeneratedMessage {
  factory SearchAddressRequest({
    $core.String? text,
    $core.double? latitude,
    $core.double? longitude,
  }) {
    final result = create();
    if (text != null) result.text = text;
    if (latitude != null) result.latitude = latitude;
    if (longitude != null) result.longitude = longitude;
    return result;
  }

  SearchAddressRequest._();

  factory SearchAddressRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SearchAddressRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SearchAddressRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.address'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..aD(2, _omitFieldNames ? '' : 'latitude')
    ..aD(3, _omitFieldNames ? '' : 'longitude')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchAddressRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchAddressRequest copyWith(void Function(SearchAddressRequest) updates) =>
      super.copyWith((message) => updates(message as SearchAddressRequest))
          as SearchAddressRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SearchAddressRequest create() => SearchAddressRequest._();
  @$core.override
  SearchAddressRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SearchAddressRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SearchAddressRequest>(create);
  static SearchAddressRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.double get latitude => $_getN(1);
  @$pb.TagNumber(2)
  set latitude($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLatitude() => $_has(1);
  @$pb.TagNumber(2)
  void clearLatitude() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.double get longitude => $_getN(2);
  @$pb.TagNumber(3)
  set longitude($core.double value) => $_setDouble(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLongitude() => $_has(2);
  @$pb.TagNumber(3)
  void clearLongitude() => $_clearField(3);
}

class SearchAddressResponse extends $pb.GeneratedMessage {
  factory SearchAddressResponse({
    $core.Iterable<AddressCandidate>? candidates,
  }) {
    final result = create();
    if (candidates != null) result.candidates.addAll(candidates);
    return result;
  }

  SearchAddressResponse._();

  factory SearchAddressResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SearchAddressResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SearchAddressResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.address'),
      createEmptyInstance: create)
    ..pPM<AddressCandidate>(1, _omitFieldNames ? '' : 'candidates',
        subBuilder: AddressCandidate.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchAddressResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchAddressResponse copyWith(
          void Function(SearchAddressResponse) updates) =>
      super.copyWith((message) => updates(message as SearchAddressResponse))
          as SearchAddressResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SearchAddressResponse create() => SearchAddressResponse._();
  @$core.override
  SearchAddressResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SearchAddressResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SearchAddressResponse>(create);
  static SearchAddressResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<AddressCandidate> get candidates => $_getList(0);
}

class AddressCandidate extends $pb.GeneratedMessage {
  factory AddressCandidate({
    $core.String? placeId,
    $core.String? formattedAddress,
    $core.String? addressLine1,
    $core.String? addressLine2,
    $core.String? locality,
    $core.String? administrativeArea,
    $core.String? administrativeAreaCode,
    $core.String? postalCode,
    $core.String? countryCode,
    $core.double? latitude,
    $core.double? longitude,
  }) {
    final result = create();
    if (placeId != null) result.placeId = placeId;
    if (formattedAddress != null) result.formattedAddress = formattedAddress;
    if (addressLine1 != null) result.addressLine1 = addressLine1;
    if (addressLine2 != null) result.addressLine2 = addressLine2;
    if (locality != null) result.locality = locality;
    if (administrativeArea != null)
      result.administrativeArea = administrativeArea;
    if (administrativeAreaCode != null)
      result.administrativeAreaCode = administrativeAreaCode;
    if (postalCode != null) result.postalCode = postalCode;
    if (countryCode != null) result.countryCode = countryCode;
    if (latitude != null) result.latitude = latitude;
    if (longitude != null) result.longitude = longitude;
    return result;
  }

  AddressCandidate._();

  factory AddressCandidate.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AddressCandidate.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AddressCandidate',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.address'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'placeId')
    ..aOS(2, _omitFieldNames ? '' : 'formattedAddress')
    ..aOS(3, _omitFieldNames ? '' : 'addressLine1', protoName: 'address_line_1')
    ..aOS(4, _omitFieldNames ? '' : 'addressLine2', protoName: 'address_line_2')
    ..aOS(5, _omitFieldNames ? '' : 'locality')
    ..aOS(6, _omitFieldNames ? '' : 'administrativeArea')
    ..aOS(7, _omitFieldNames ? '' : 'administrativeAreaCode')
    ..aOS(8, _omitFieldNames ? '' : 'postalCode')
    ..aOS(9, _omitFieldNames ? '' : 'countryCode')
    ..aD(10, _omitFieldNames ? '' : 'latitude')
    ..aD(11, _omitFieldNames ? '' : 'longitude')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddressCandidate clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddressCandidate copyWith(void Function(AddressCandidate) updates) =>
      super.copyWith((message) => updates(message as AddressCandidate))
          as AddressCandidate;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddressCandidate create() => AddressCandidate._();
  @$core.override
  AddressCandidate createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AddressCandidate getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AddressCandidate>(create);
  static AddressCandidate? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get placeId => $_getSZ(0);
  @$pb.TagNumber(1)
  set placeId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPlaceId() => $_has(0);
  @$pb.TagNumber(1)
  void clearPlaceId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get formattedAddress => $_getSZ(1);
  @$pb.TagNumber(2)
  set formattedAddress($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFormattedAddress() => $_has(1);
  @$pb.TagNumber(2)
  void clearFormattedAddress() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get addressLine1 => $_getSZ(2);
  @$pb.TagNumber(3)
  set addressLine1($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAddressLine1() => $_has(2);
  @$pb.TagNumber(3)
  void clearAddressLine1() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get addressLine2 => $_getSZ(3);
  @$pb.TagNumber(4)
  set addressLine2($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAddressLine2() => $_has(3);
  @$pb.TagNumber(4)
  void clearAddressLine2() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get locality => $_getSZ(4);
  @$pb.TagNumber(5)
  set locality($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLocality() => $_has(4);
  @$pb.TagNumber(5)
  void clearLocality() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get administrativeArea => $_getSZ(5);
  @$pb.TagNumber(6)
  set administrativeArea($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAdministrativeArea() => $_has(5);
  @$pb.TagNumber(6)
  void clearAdministrativeArea() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get administrativeAreaCode => $_getSZ(6);
  @$pb.TagNumber(7)
  set administrativeAreaCode($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasAdministrativeAreaCode() => $_has(6);
  @$pb.TagNumber(7)
  void clearAdministrativeAreaCode() => $_clearField(7);

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
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
