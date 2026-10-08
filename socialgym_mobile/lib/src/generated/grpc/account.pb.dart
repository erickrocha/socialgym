// This is a generated file - do not edit.
//
// Generated from account.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class RequestAccountDeletionRequest extends $pb.GeneratedMessage {
  factory RequestAccountDeletionRequest({
    $core.bool? immediate,
  }) {
    final result = create();
    if (immediate != null) result.immediate = immediate;
    return result;
  }

  RequestAccountDeletionRequest._();

  factory RequestAccountDeletionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RequestAccountDeletionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RequestAccountDeletionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'immediate')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RequestAccountDeletionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RequestAccountDeletionRequest copyWith(
          void Function(RequestAccountDeletionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RequestAccountDeletionRequest))
          as RequestAccountDeletionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RequestAccountDeletionRequest create() =>
      RequestAccountDeletionRequest._();
  @$core.override
  RequestAccountDeletionRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RequestAccountDeletionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RequestAccountDeletionRequest>(create);
  static RequestAccountDeletionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get immediate => $_getBF(0);
  @$pb.TagNumber(1)
  set immediate($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasImmediate() => $_has(0);
  @$pb.TagNumber(1)
  void clearImmediate() => $_clearField(1);
}

class AccountDeletionStatus extends $pb.GeneratedMessage {
  factory AccountDeletionStatus({
    $core.String? requestedAt,
    $core.String? scheduledAt,
  }) {
    final result = create();
    if (requestedAt != null) result.requestedAt = requestedAt;
    if (scheduledAt != null) result.scheduledAt = scheduledAt;
    return result;
  }

  AccountDeletionStatus._();

  factory AccountDeletionStatus.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AccountDeletionStatus.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AccountDeletionStatus',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'requestedAt')
    ..aOS(2, _omitFieldNames ? '' : 'scheduledAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AccountDeletionStatus clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AccountDeletionStatus copyWith(
          void Function(AccountDeletionStatus) updates) =>
      super.copyWith((message) => updates(message as AccountDeletionStatus))
          as AccountDeletionStatus;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountDeletionStatus create() => AccountDeletionStatus._();
  @$core.override
  AccountDeletionStatus createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AccountDeletionStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AccountDeletionStatus>(create);
  static AccountDeletionStatus? _defaultInstance;

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

class CancelAccountDeletionRequest extends $pb.GeneratedMessage {
  factory CancelAccountDeletionRequest() => create();

  CancelAccountDeletionRequest._();

  factory CancelAccountDeletionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CancelAccountDeletionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CancelAccountDeletionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CancelAccountDeletionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CancelAccountDeletionRequest copyWith(
          void Function(CancelAccountDeletionRequest) updates) =>
      super.copyWith(
              (message) => updates(message as CancelAccountDeletionRequest))
          as CancelAccountDeletionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CancelAccountDeletionRequest create() =>
      CancelAccountDeletionRequest._();
  @$core.override
  CancelAccountDeletionRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CancelAccountDeletionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CancelAccountDeletionRequest>(create);
  static CancelAccountDeletionRequest? _defaultInstance;
}

class CancelAccountDeletionResponse extends $pb.GeneratedMessage {
  factory CancelAccountDeletionResponse() => create();

  CancelAccountDeletionResponse._();

  factory CancelAccountDeletionResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CancelAccountDeletionResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CancelAccountDeletionResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CancelAccountDeletionResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CancelAccountDeletionResponse copyWith(
          void Function(CancelAccountDeletionResponse) updates) =>
      super.copyWith(
              (message) => updates(message as CancelAccountDeletionResponse))
          as CancelAccountDeletionResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CancelAccountDeletionResponse create() =>
      CancelAccountDeletionResponse._();
  @$core.override
  CancelAccountDeletionResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CancelAccountDeletionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CancelAccountDeletionResponse>(create);
  static CancelAccountDeletionResponse? _defaultInstance;
}

class CreateDataExportRequest extends $pb.GeneratedMessage {
  factory CreateDataExportRequest() => create();

  CreateDataExportRequest._();

  factory CreateDataExportRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateDataExportRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateDataExportRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDataExportRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateDataExportRequest copyWith(
          void Function(CreateDataExportRequest) updates) =>
      super.copyWith((message) => updates(message as CreateDataExportRequest))
          as CreateDataExportRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateDataExportRequest create() => CreateDataExportRequest._();
  @$core.override
  CreateDataExportRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateDataExportRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateDataExportRequest>(create);
  static CreateDataExportRequest? _defaultInstance;
}

class ListDataExportsRequest extends $pb.GeneratedMessage {
  factory ListDataExportsRequest() => create();

  ListDataExportsRequest._();

  factory ListDataExportsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListDataExportsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDataExportsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDataExportsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDataExportsRequest copyWith(
          void Function(ListDataExportsRequest) updates) =>
      super.copyWith((message) => updates(message as ListDataExportsRequest))
          as ListDataExportsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListDataExportsRequest create() => ListDataExportsRequest._();
  @$core.override
  ListDataExportsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListDataExportsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDataExportsRequest>(create);
  static ListDataExportsRequest? _defaultInstance;
}

class ListDataExportsResponse extends $pb.GeneratedMessage {
  factory ListDataExportsResponse({
    $core.Iterable<DataExport>? exports,
  }) {
    final result = create();
    if (exports != null) result.exports.addAll(exports);
    return result;
  }

  ListDataExportsResponse._();

  factory ListDataExportsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListDataExportsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDataExportsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..pPM<DataExport>(1, _omitFieldNames ? '' : 'exports',
        subBuilder: DataExport.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDataExportsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListDataExportsResponse copyWith(
          void Function(ListDataExportsResponse) updates) =>
      super.copyWith((message) => updates(message as ListDataExportsResponse))
          as ListDataExportsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListDataExportsResponse create() => ListDataExportsResponse._();
  @$core.override
  ListDataExportsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListDataExportsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListDataExportsResponse>(create);
  static ListDataExportsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<DataExport> get exports => $_getList(0);
}

class GetDataExportRequest extends $pb.GeneratedMessage {
  factory GetDataExportRequest({
    $core.String? id,
  }) {
    final result = create();
    if (id != null) result.id = id;
    return result;
  }

  GetDataExportRequest._();

  factory GetDataExportRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetDataExportRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetDataExportRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDataExportRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetDataExportRequest copyWith(void Function(GetDataExportRequest) updates) =>
      super.copyWith((message) => updates(message as GetDataExportRequest))
          as GetDataExportRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetDataExportRequest create() => GetDataExportRequest._();
  @$core.override
  GetDataExportRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetDataExportRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetDataExportRequest>(create);
  static GetDataExportRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DataExport extends $pb.GeneratedMessage {
  factory DataExport({
    $core.String? id,
    $core.String? status,
    $core.String? error,
    $core.String? createdAt,
    $core.String? updatedAt,
    $core.String? expiresAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (status != null) result.status = status;
    if (error != null) result.error = error;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (expiresAt != null) result.expiresAt = expiresAt;
    return result;
  }

  DataExport._();

  factory DataExport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DataExport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DataExport',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'status')
    ..aOS(3, _omitFieldNames ? '' : 'error')
    ..aOS(4, _omitFieldNames ? '' : 'createdAt')
    ..aOS(5, _omitFieldNames ? '' : 'updatedAt')
    ..aOS(6, _omitFieldNames ? '' : 'expiresAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DataExport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DataExport copyWith(void Function(DataExport) updates) =>
      super.copyWith((message) => updates(message as DataExport)) as DataExport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DataExport create() => DataExport._();
  @$core.override
  DataExport createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DataExport getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DataExport>(create);
  static DataExport? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get status => $_getSZ(1);
  @$pb.TagNumber(2)
  set status($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get error => $_getSZ(2);
  @$pb.TagNumber(3)
  set error($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasError() => $_has(2);
  @$pb.TagNumber(3)
  void clearError() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get createdAt => $_getSZ(3);
  @$pb.TagNumber(4)
  set createdAt($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasCreatedAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearCreatedAt() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get updatedAt => $_getSZ(4);
  @$pb.TagNumber(5)
  set updatedAt($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUpdatedAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearUpdatedAt() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get expiresAt => $_getSZ(5);
  @$pb.TagNumber(6)
  set expiresAt($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasExpiresAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearExpiresAt() => $_clearField(6);
}

class DataExportDownload extends $pb.GeneratedMessage {
  factory DataExportDownload({
    $core.String? url,
    $core.int? expiresInSeconds,
  }) {
    final result = create();
    if (url != null) result.url = url;
    if (expiresInSeconds != null) result.expiresInSeconds = expiresInSeconds;
    return result;
  }

  DataExportDownload._();

  factory DataExportDownload.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DataExportDownload.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DataExportDownload',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.account'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'url')
    ..aI(2, _omitFieldNames ? '' : 'expiresInSeconds',
        fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DataExportDownload clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DataExportDownload copyWith(void Function(DataExportDownload) updates) =>
      super.copyWith((message) => updates(message as DataExportDownload))
          as DataExportDownload;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DataExportDownload create() => DataExportDownload._();
  @$core.override
  DataExportDownload createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DataExportDownload getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DataExportDownload>(create);
  static DataExportDownload? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get url => $_getSZ(0);
  @$pb.TagNumber(1)
  set url($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get expiresInSeconds => $_getIZ(1);
  @$pb.TagNumber(2)
  set expiresInSeconds($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasExpiresInSeconds() => $_has(1);
  @$pb.TagNumber(2)
  void clearExpiresInSeconds() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
