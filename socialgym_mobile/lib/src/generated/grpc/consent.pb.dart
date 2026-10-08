// This is a generated file - do not edit.
//
// Generated from consent.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ListConsentsRequest extends $pb.GeneratedMessage {
  factory ListConsentsRequest() => create();

  ListConsentsRequest._();

  factory ListConsentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListConsentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListConsentsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConsentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConsentsRequest copyWith(void Function(ListConsentsRequest) updates) =>
      super.copyWith((message) => updates(message as ListConsentsRequest))
          as ListConsentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListConsentsRequest create() => ListConsentsRequest._();
  @$core.override
  ListConsentsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListConsentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListConsentsRequest>(create);
  static ListConsentsRequest? _defaultInstance;
}

class ListConsentsResponse extends $pb.GeneratedMessage {
  factory ListConsentsResponse({
    $core.Iterable<Consent>? consents,
  }) {
    final result = create();
    if (consents != null) result.consents.addAll(consents);
    return result;
  }

  ListConsentsResponse._();

  factory ListConsentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListConsentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListConsentsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..pPM<Consent>(1, _omitFieldNames ? '' : 'consents',
        subBuilder: Consent.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConsentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListConsentsResponse copyWith(void Function(ListConsentsResponse) updates) =>
      super.copyWith((message) => updates(message as ListConsentsResponse))
          as ListConsentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListConsentsResponse create() => ListConsentsResponse._();
  @$core.override
  ListConsentsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListConsentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListConsentsResponse>(create);
  static ListConsentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Consent> get consents => $_getList(0);
}

class ListPendingConsentsRequest extends $pb.GeneratedMessage {
  factory ListPendingConsentsRequest() => create();

  ListPendingConsentsRequest._();

  factory ListPendingConsentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListPendingConsentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPendingConsentsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPendingConsentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPendingConsentsRequest copyWith(
          void Function(ListPendingConsentsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as ListPendingConsentsRequest))
          as ListPendingConsentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListPendingConsentsRequest create() => ListPendingConsentsRequest._();
  @$core.override
  ListPendingConsentsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListPendingConsentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPendingConsentsRequest>(create);
  static ListPendingConsentsRequest? _defaultInstance;
}

class ListPendingConsentsResponse extends $pb.GeneratedMessage {
  factory ListPendingConsentsResponse({
    $core.Iterable<PendingConsent>? pending,
  }) {
    final result = create();
    if (pending != null) result.pending.addAll(pending);
    return result;
  }

  ListPendingConsentsResponse._();

  factory ListPendingConsentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListPendingConsentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListPendingConsentsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..pPM<PendingConsent>(1, _omitFieldNames ? '' : 'pending',
        subBuilder: PendingConsent.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPendingConsentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListPendingConsentsResponse copyWith(
          void Function(ListPendingConsentsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListPendingConsentsResponse))
          as ListPendingConsentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListPendingConsentsResponse create() =>
      ListPendingConsentsResponse._();
  @$core.override
  ListPendingConsentsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListPendingConsentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListPendingConsentsResponse>(create);
  static ListPendingConsentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<PendingConsent> get pending => $_getList(0);
}

class AcceptConsentRequest extends $pb.GeneratedMessage {
  factory AcceptConsentRequest({
    $core.String? document,
    $core.String? version,
    $core.bool? accepted,
  }) {
    final result = create();
    if (document != null) result.document = document;
    if (version != null) result.version = version;
    if (accepted != null) result.accepted = accepted;
    return result;
  }

  AcceptConsentRequest._();

  factory AcceptConsentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AcceptConsentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AcceptConsentRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'document')
    ..aOS(2, _omitFieldNames ? '' : 'version')
    ..aOB(3, _omitFieldNames ? '' : 'accepted')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AcceptConsentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AcceptConsentRequest copyWith(void Function(AcceptConsentRequest) updates) =>
      super.copyWith((message) => updates(message as AcceptConsentRequest))
          as AcceptConsentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AcceptConsentRequest create() => AcceptConsentRequest._();
  @$core.override
  AcceptConsentRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AcceptConsentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AcceptConsentRequest>(create);
  static AcceptConsentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get document => $_getSZ(0);
  @$pb.TagNumber(1)
  set document($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDocument() => $_has(0);
  @$pb.TagNumber(1)
  void clearDocument() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get version => $_getSZ(1);
  @$pb.TagNumber(2)
  set version($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearVersion() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.bool get accepted => $_getBF(2);
  @$pb.TagNumber(3)
  set accepted($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAccepted() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccepted() => $_clearField(3);
}

class RevokeConsentRequest extends $pb.GeneratedMessage {
  factory RevokeConsentRequest({
    $core.String? document,
  }) {
    final result = create();
    if (document != null) result.document = document;
    return result;
  }

  RevokeConsentRequest._();

  factory RevokeConsentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RevokeConsentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RevokeConsentRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'document')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokeConsentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokeConsentRequest copyWith(void Function(RevokeConsentRequest) updates) =>
      super.copyWith((message) => updates(message as RevokeConsentRequest))
          as RevokeConsentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RevokeConsentRequest create() => RevokeConsentRequest._();
  @$core.override
  RevokeConsentRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RevokeConsentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RevokeConsentRequest>(create);
  static RevokeConsentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get document => $_getSZ(0);
  @$pb.TagNumber(1)
  set document($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDocument() => $_has(0);
  @$pb.TagNumber(1)
  void clearDocument() => $_clearField(1);
}

class RevokeConsentResponse extends $pb.GeneratedMessage {
  factory RevokeConsentResponse() => create();

  RevokeConsentResponse._();

  factory RevokeConsentResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RevokeConsentResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RevokeConsentResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokeConsentResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RevokeConsentResponse copyWith(
          void Function(RevokeConsentResponse) updates) =>
      super.copyWith((message) => updates(message as RevokeConsentResponse))
          as RevokeConsentResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RevokeConsentResponse create() => RevokeConsentResponse._();
  @$core.override
  RevokeConsentResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RevokeConsentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RevokeConsentResponse>(create);
  static RevokeConsentResponse? _defaultInstance;
}

class Consent extends $pb.GeneratedMessage {
  factory Consent({
    $core.int? id,
    $core.String? document,
    $core.String? version,
    $core.String? acceptedAt,
    $core.String? revokedAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (document != null) result.document = document;
    if (version != null) result.version = version;
    if (acceptedAt != null) result.acceptedAt = acceptedAt;
    if (revokedAt != null) result.revokedAt = revokedAt;
    return result;
  }

  Consent._();

  factory Consent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Consent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Consent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'document')
    ..aOS(3, _omitFieldNames ? '' : 'version')
    ..aOS(4, _omitFieldNames ? '' : 'acceptedAt')
    ..aOS(5, _omitFieldNames ? '' : 'revokedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Consent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Consent copyWith(void Function(Consent) updates) =>
      super.copyWith((message) => updates(message as Consent)) as Consent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Consent create() => Consent._();
  @$core.override
  Consent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Consent getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Consent>(create);
  static Consent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get document => $_getSZ(1);
  @$pb.TagNumber(2)
  set document($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDocument() => $_has(1);
  @$pb.TagNumber(2)
  void clearDocument() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get version => $_getSZ(2);
  @$pb.TagNumber(3)
  set version($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVersion() => $_has(2);
  @$pb.TagNumber(3)
  void clearVersion() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get acceptedAt => $_getSZ(3);
  @$pb.TagNumber(4)
  set acceptedAt($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAcceptedAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearAcceptedAt() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get revokedAt => $_getSZ(4);
  @$pb.TagNumber(5)
  set revokedAt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRevokedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearRevokedAt() => $_clearField(5);
}

/// A document whose current version the person has not accepted.
class PendingConsent extends $pb.GeneratedMessage {
  factory PendingConsent({
    $core.String? document,
    $core.String? currentVersion,
    $core.String? acceptedVersion,
  }) {
    final result = create();
    if (document != null) result.document = document;
    if (currentVersion != null) result.currentVersion = currentVersion;
    if (acceptedVersion != null) result.acceptedVersion = acceptedVersion;
    return result;
  }

  PendingConsent._();

  factory PendingConsent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PendingConsent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PendingConsent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.consent'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'document')
    ..aOS(2, _omitFieldNames ? '' : 'currentVersion')
    ..aOS(3, _omitFieldNames ? '' : 'acceptedVersion')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PendingConsent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PendingConsent copyWith(void Function(PendingConsent) updates) =>
      super.copyWith((message) => updates(message as PendingConsent))
          as PendingConsent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PendingConsent create() => PendingConsent._();
  @$core.override
  PendingConsent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PendingConsent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PendingConsent>(create);
  static PendingConsent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get document => $_getSZ(0);
  @$pb.TagNumber(1)
  set document($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDocument() => $_has(0);
  @$pb.TagNumber(1)
  void clearDocument() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get currentVersion => $_getSZ(1);
  @$pb.TagNumber(2)
  set currentVersion($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCurrentVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearCurrentVersion() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get acceptedVersion => $_getSZ(2);
  @$pb.TagNumber(3)
  set acceptedVersion($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAcceptedVersion() => $_has(2);
  @$pb.TagNumber(3)
  void clearAcceptedVersion() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
