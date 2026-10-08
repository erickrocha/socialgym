// This is a generated file - do not edit.
//
// Generated from auth.proto.

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

class SignupRequest extends $pb.GeneratedMessage {
  factory SignupRequest({
    $core.String? firstname,
    $core.String? surname,
    $core.String? dateOfBirth,
    $core.String? gender,
    $core.String? email,
    $core.String? password,
    $core.String? termsVersion,
    $core.String? privacyVersion,
    $core.bool? termsAccepted,
    $core.bool? privacyAccepted,
  }) {
    final result = create();
    if (firstname != null) result.firstname = firstname;
    if (surname != null) result.surname = surname;
    if (dateOfBirth != null) result.dateOfBirth = dateOfBirth;
    if (gender != null) result.gender = gender;
    if (email != null) result.email = email;
    if (password != null) result.password = password;
    if (termsVersion != null) result.termsVersion = termsVersion;
    if (privacyVersion != null) result.privacyVersion = privacyVersion;
    if (termsAccepted != null) result.termsAccepted = termsAccepted;
    if (privacyAccepted != null) result.privacyAccepted = privacyAccepted;
    return result;
  }

  SignupRequest._();

  factory SignupRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SignupRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SignupRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'firstname')
    ..aOS(2, _omitFieldNames ? '' : 'surname')
    ..aOS(3, _omitFieldNames ? '' : 'dateOfBirth')
    ..aOS(4, _omitFieldNames ? '' : 'gender')
    ..aOS(5, _omitFieldNames ? '' : 'email')
    ..aOS(6, _omitFieldNames ? '' : 'password')
    ..aOS(7, _omitFieldNames ? '' : 'termsVersion')
    ..aOS(8, _omitFieldNames ? '' : 'privacyVersion')
    ..aOB(9, _omitFieldNames ? '' : 'termsAccepted')
    ..aOB(10, _omitFieldNames ? '' : 'privacyAccepted')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SignupRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SignupRequest copyWith(void Function(SignupRequest) updates) =>
      super.copyWith((message) => updates(message as SignupRequest))
          as SignupRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SignupRequest create() => SignupRequest._();
  @$core.override
  SignupRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SignupRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SignupRequest>(create);
  static SignupRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get firstname => $_getSZ(0);
  @$pb.TagNumber(1)
  set firstname($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFirstname() => $_has(0);
  @$pb.TagNumber(1)
  void clearFirstname() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get surname => $_getSZ(1);
  @$pb.TagNumber(2)
  set surname($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSurname() => $_has(1);
  @$pb.TagNumber(2)
  void clearSurname() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get dateOfBirth => $_getSZ(2);
  @$pb.TagNumber(3)
  set dateOfBirth($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDateOfBirth() => $_has(2);
  @$pb.TagNumber(3)
  void clearDateOfBirth() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get gender => $_getSZ(3);
  @$pb.TagNumber(4)
  set gender($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasGender() => $_has(3);
  @$pb.TagNumber(4)
  void clearGender() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get email => $_getSZ(4);
  @$pb.TagNumber(5)
  set email($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasEmail() => $_has(4);
  @$pb.TagNumber(5)
  void clearEmail() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get password => $_getSZ(5);
  @$pb.TagNumber(6)
  set password($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPassword() => $_has(5);
  @$pb.TagNumber(6)
  void clearPassword() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get termsVersion => $_getSZ(6);
  @$pb.TagNumber(7)
  set termsVersion($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasTermsVersion() => $_has(6);
  @$pb.TagNumber(7)
  void clearTermsVersion() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get privacyVersion => $_getSZ(7);
  @$pb.TagNumber(8)
  set privacyVersion($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasPrivacyVersion() => $_has(7);
  @$pb.TagNumber(8)
  void clearPrivacyVersion() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.bool get termsAccepted => $_getBF(8);
  @$pb.TagNumber(9)
  set termsAccepted($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasTermsAccepted() => $_has(8);
  @$pb.TagNumber(9)
  void clearTermsAccepted() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.bool get privacyAccepted => $_getBF(9);
  @$pb.TagNumber(10)
  set privacyAccepted($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(10)
  $core.bool hasPrivacyAccepted() => $_has(9);
  @$pb.TagNumber(10)
  void clearPrivacyAccepted() => $_clearField(10);
}

class LoginRequest extends $pb.GeneratedMessage {
  factory LoginRequest({
    $core.String? email,
    $core.String? password,
  }) {
    final result = create();
    if (email != null) result.email = email;
    if (password != null) result.password = password;
    return result;
  }

  LoginRequest._();

  factory LoginRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LoginRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LoginRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'email')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LoginRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LoginRequest copyWith(void Function(LoginRequest) updates) =>
      super.copyWith((message) => updates(message as LoginRequest))
          as LoginRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LoginRequest create() => LoginRequest._();
  @$core.override
  LoginRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LoginRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LoginRequest>(create);
  static LoginRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get email => $_getSZ(0);
  @$pb.TagNumber(1)
  set email($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEmail() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmail() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);
}

class RefreshRequest extends $pb.GeneratedMessage {
  factory RefreshRequest({
    $core.String? refreshToken,
  }) {
    final result = create();
    if (refreshToken != null) result.refreshToken = refreshToken;
    return result;
  }

  RefreshRequest._();

  factory RefreshRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RefreshRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RefreshRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'refreshToken')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RefreshRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RefreshRequest copyWith(void Function(RefreshRequest) updates) =>
      super.copyWith((message) => updates(message as RefreshRequest))
          as RefreshRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RefreshRequest create() => RefreshRequest._();
  @$core.override
  RefreshRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RefreshRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RefreshRequest>(create);
  static RefreshRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get refreshToken => $_getSZ(0);
  @$pb.TagNumber(1)
  set refreshToken($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRefreshToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearRefreshToken() => $_clearField(1);
}

class LogoutRequest extends $pb.GeneratedMessage {
  factory LogoutRequest({
    $core.String? refreshToken,
  }) {
    final result = create();
    if (refreshToken != null) result.refreshToken = refreshToken;
    return result;
  }

  LogoutRequest._();

  factory LogoutRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LogoutRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogoutRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'refreshToken')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutRequest copyWith(void Function(LogoutRequest) updates) =>
      super.copyWith((message) => updates(message as LogoutRequest))
          as LogoutRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LogoutRequest create() => LogoutRequest._();
  @$core.override
  LogoutRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LogoutRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LogoutRequest>(create);
  static LogoutRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get refreshToken => $_getSZ(0);
  @$pb.TagNumber(1)
  set refreshToken($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRefreshToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearRefreshToken() => $_clearField(1);
}

class LogoutResponse extends $pb.GeneratedMessage {
  factory LogoutResponse() => create();

  LogoutResponse._();

  factory LogoutResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LogoutResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogoutResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutResponse copyWith(void Function(LogoutResponse) updates) =>
      super.copyWith((message) => updates(message as LogoutResponse))
          as LogoutResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LogoutResponse create() => LogoutResponse._();
  @$core.override
  LogoutResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LogoutResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LogoutResponse>(create);
  static LogoutResponse? _defaultInstance;
}

class ActivateBusinessProfileRequest extends $pb.GeneratedMessage {
  factory ActivateBusinessProfileRequest({
    $core.String? businessProfileUuid,
  }) {
    final result = create();
    if (businessProfileUuid != null)
      result.businessProfileUuid = businessProfileUuid;
    return result;
  }

  ActivateBusinessProfileRequest._();

  factory ActivateBusinessProfileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ActivateBusinessProfileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ActivateBusinessProfileRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'businessProfileUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ActivateBusinessProfileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ActivateBusinessProfileRequest copyWith(
          void Function(ActivateBusinessProfileRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ActivateBusinessProfileRequest))
          as ActivateBusinessProfileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ActivateBusinessProfileRequest create() =>
      ActivateBusinessProfileRequest._();
  @$core.override
  ActivateBusinessProfileRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ActivateBusinessProfileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ActivateBusinessProfileRequest>(create);
  static ActivateBusinessProfileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get businessProfileUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set businessProfileUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBusinessProfileUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearBusinessProfileUuid() => $_clearField(1);
}

class DeactivateBusinessProfileRequest extends $pb.GeneratedMessage {
  factory DeactivateBusinessProfileRequest() => create();

  DeactivateBusinessProfileRequest._();

  factory DeactivateBusinessProfileRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeactivateBusinessProfileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeactivateBusinessProfileRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeactivateBusinessProfileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeactivateBusinessProfileRequest copyWith(
          void Function(DeactivateBusinessProfileRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeactivateBusinessProfileRequest))
          as DeactivateBusinessProfileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeactivateBusinessProfileRequest create() =>
      DeactivateBusinessProfileRequest._();
  @$core.override
  DeactivateBusinessProfileRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeactivateBusinessProfileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeactivateBusinessProfileRequest>(
          create);
  static DeactivateBusinessProfileRequest? _defaultInstance;
}

class AccessToken extends $pb.GeneratedMessage {
  factory AccessToken({
    $core.String? accessToken,
    $core.String? tokenType,
    $fixnum.Int64? expireIn,
    $core.String? refreshToken,
    $core.String? username,
    $core.String? uuid,
    $core.String? name,
    $core.int? personId,
    $core.String? personUuid,
    $core.String? personObjectKey,
    $core.int? activeBusinessProfileId,
    $core.String? activeBusinessProfileUuid,
    PendingAccountDeletion? pendingAccountDeletion,
  }) {
    final result = create();
    if (accessToken != null) result.accessToken = accessToken;
    if (tokenType != null) result.tokenType = tokenType;
    if (expireIn != null) result.expireIn = expireIn;
    if (refreshToken != null) result.refreshToken = refreshToken;
    if (username != null) result.username = username;
    if (uuid != null) result.uuid = uuid;
    if (name != null) result.name = name;
    if (personId != null) result.personId = personId;
    if (personUuid != null) result.personUuid = personUuid;
    if (personObjectKey != null) result.personObjectKey = personObjectKey;
    if (activeBusinessProfileId != null)
      result.activeBusinessProfileId = activeBusinessProfileId;
    if (activeBusinessProfileUuid != null)
      result.activeBusinessProfileUuid = activeBusinessProfileUuid;
    if (pendingAccountDeletion != null)
      result.pendingAccountDeletion = pendingAccountDeletion;
    return result;
  }

  AccessToken._();

  factory AccessToken.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AccessToken.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AccessToken',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'accessToken')
    ..aOS(2, _omitFieldNames ? '' : 'tokenType')
    ..aInt64(3, _omitFieldNames ? '' : 'expireIn')
    ..aOS(4, _omitFieldNames ? '' : 'refreshToken')
    ..aOS(5, _omitFieldNames ? '' : 'username')
    ..aOS(6, _omitFieldNames ? '' : 'uuid')
    ..aOS(7, _omitFieldNames ? '' : 'name')
    ..aI(8, _omitFieldNames ? '' : 'personId')
    ..aOS(9, _omitFieldNames ? '' : 'personUuid')
    ..aOS(10, _omitFieldNames ? '' : 'personObjectKey')
    ..aI(11, _omitFieldNames ? '' : 'activeBusinessProfileId')
    ..aOS(12, _omitFieldNames ? '' : 'activeBusinessProfileUuid')
    ..aOM<PendingAccountDeletion>(
        13, _omitFieldNames ? '' : 'pendingAccountDeletion',
        subBuilder: PendingAccountDeletion.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AccessToken clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AccessToken copyWith(void Function(AccessToken) updates) =>
      super.copyWith((message) => updates(message as AccessToken))
          as AccessToken;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccessToken create() => AccessToken._();
  @$core.override
  AccessToken createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AccessToken getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AccessToken>(create);
  static AccessToken? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get accessToken => $_getSZ(0);
  @$pb.TagNumber(1)
  set accessToken($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAccessToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccessToken() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get tokenType => $_getSZ(1);
  @$pb.TagNumber(2)
  set tokenType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTokenType() => $_has(1);
  @$pb.TagNumber(2)
  void clearTokenType() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get expireIn => $_getI64(2);
  @$pb.TagNumber(3)
  set expireIn($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasExpireIn() => $_has(2);
  @$pb.TagNumber(3)
  void clearExpireIn() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get refreshToken => $_getSZ(3);
  @$pb.TagNumber(4)
  set refreshToken($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRefreshToken() => $_has(3);
  @$pb.TagNumber(4)
  void clearRefreshToken() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get username => $_getSZ(4);
  @$pb.TagNumber(5)
  set username($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUsername() => $_has(4);
  @$pb.TagNumber(5)
  void clearUsername() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get uuid => $_getSZ(5);
  @$pb.TagNumber(6)
  set uuid($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasUuid() => $_has(5);
  @$pb.TagNumber(6)
  void clearUuid() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get name => $_getSZ(6);
  @$pb.TagNumber(7)
  set name($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasName() => $_has(6);
  @$pb.TagNumber(7)
  void clearName() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.int get personId => $_getIZ(7);
  @$pb.TagNumber(8)
  set personId($core.int value) => $_setSignedInt32(7, value);
  @$pb.TagNumber(8)
  $core.bool hasPersonId() => $_has(7);
  @$pb.TagNumber(8)
  void clearPersonId() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get personUuid => $_getSZ(8);
  @$pb.TagNumber(9)
  set personUuid($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasPersonUuid() => $_has(8);
  @$pb.TagNumber(9)
  void clearPersonUuid() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get personObjectKey => $_getSZ(9);
  @$pb.TagNumber(10)
  set personObjectKey($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasPersonObjectKey() => $_has(9);
  @$pb.TagNumber(10)
  void clearPersonObjectKey() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.int get activeBusinessProfileId => $_getIZ(10);
  @$pb.TagNumber(11)
  set activeBusinessProfileId($core.int value) => $_setSignedInt32(10, value);
  @$pb.TagNumber(11)
  $core.bool hasActiveBusinessProfileId() => $_has(10);
  @$pb.TagNumber(11)
  void clearActiveBusinessProfileId() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get activeBusinessProfileUuid => $_getSZ(11);
  @$pb.TagNumber(12)
  set activeBusinessProfileUuid($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasActiveBusinessProfileUuid() => $_has(11);
  @$pb.TagNumber(12)
  void clearActiveBusinessProfileUuid() => $_clearField(12);

  @$pb.TagNumber(13)
  PendingAccountDeletion get pendingAccountDeletion => $_getN(12);
  @$pb.TagNumber(13)
  set pendingAccountDeletion(PendingAccountDeletion value) =>
      $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasPendingAccountDeletion() => $_has(12);
  @$pb.TagNumber(13)
  void clearPendingAccountDeletion() => $_clearField(13);
  @$pb.TagNumber(13)
  PendingAccountDeletion ensurePendingAccountDeletion() => $_ensure(12);
}

/// Present only when the account has a deletion request that was not purged yet.
class PendingAccountDeletion extends $pb.GeneratedMessage {
  factory PendingAccountDeletion({
    $core.String? requestedAt,
    $core.String? scheduledAt,
  }) {
    final result = create();
    if (requestedAt != null) result.requestedAt = requestedAt;
    if (scheduledAt != null) result.scheduledAt = scheduledAt;
    return result;
  }

  PendingAccountDeletion._();

  factory PendingAccountDeletion.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PendingAccountDeletion.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PendingAccountDeletion',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.auth'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'requestedAt')
    ..aOS(2, _omitFieldNames ? '' : 'scheduledAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PendingAccountDeletion clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PendingAccountDeletion copyWith(
          void Function(PendingAccountDeletion) updates) =>
      super.copyWith((message) => updates(message as PendingAccountDeletion))
          as PendingAccountDeletion;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PendingAccountDeletion create() => PendingAccountDeletion._();
  @$core.override
  PendingAccountDeletion createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PendingAccountDeletion getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PendingAccountDeletion>(create);
  static PendingAccountDeletion? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get requestedAt => $_getSZ(0);
  @$pb.TagNumber(1)
  set requestedAt($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRequestedAt() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequestedAt() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get scheduledAt => $_getSZ(1);
  @$pb.TagNumber(2)
  set scheduledAt($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasScheduledAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearScheduledAt() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
