// This is a generated file - do not edit.
//
// Generated from legal.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ListLegalDocumentsRequest extends $pb.GeneratedMessage {
  factory ListLegalDocumentsRequest() => create();

  ListLegalDocumentsRequest._();

  factory ListLegalDocumentsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListLegalDocumentsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListLegalDocumentsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.legal'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListLegalDocumentsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListLegalDocumentsRequest copyWith(
          void Function(ListLegalDocumentsRequest) updates) =>
      super.copyWith((message) => updates(message as ListLegalDocumentsRequest))
          as ListLegalDocumentsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListLegalDocumentsRequest create() => ListLegalDocumentsRequest._();
  @$core.override
  ListLegalDocumentsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListLegalDocumentsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListLegalDocumentsRequest>(create);
  static ListLegalDocumentsRequest? _defaultInstance;
}

class ListLegalDocumentsResponse extends $pb.GeneratedMessage {
  factory ListLegalDocumentsResponse({
    $core.Iterable<LegalDocument>? documents,
  }) {
    final result = create();
    if (documents != null) result.documents.addAll(documents);
    return result;
  }

  ListLegalDocumentsResponse._();

  factory ListLegalDocumentsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ListLegalDocumentsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListLegalDocumentsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.legal'),
      createEmptyInstance: create)
    ..pPM<LegalDocument>(1, _omitFieldNames ? '' : 'documents',
        subBuilder: LegalDocument.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListLegalDocumentsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListLegalDocumentsResponse copyWith(
          void Function(ListLegalDocumentsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as ListLegalDocumentsResponse))
          as ListLegalDocumentsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListLegalDocumentsResponse create() => ListLegalDocumentsResponse._();
  @$core.override
  ListLegalDocumentsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ListLegalDocumentsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListLegalDocumentsResponse>(create);
  static ListLegalDocumentsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<LegalDocument> get documents => $_getList(0);
}

class GetLegalDocumentRequest extends $pb.GeneratedMessage {
  factory GetLegalDocumentRequest({
    $core.String? document,
  }) {
    final result = create();
    if (document != null) result.document = document;
    return result;
  }

  GetLegalDocumentRequest._();

  factory GetLegalDocumentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetLegalDocumentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetLegalDocumentRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.legal'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'document')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetLegalDocumentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetLegalDocumentRequest copyWith(
          void Function(GetLegalDocumentRequest) updates) =>
      super.copyWith((message) => updates(message as GetLegalDocumentRequest))
          as GetLegalDocumentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetLegalDocumentRequest create() => GetLegalDocumentRequest._();
  @$core.override
  GetLegalDocumentRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetLegalDocumentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetLegalDocumentRequest>(create);
  static GetLegalDocumentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get document => $_getSZ(0);
  @$pb.TagNumber(1)
  set document($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDocument() => $_has(0);
  @$pb.TagNumber(1)
  void clearDocument() => $_clearField(1);
}

class LegalDocument extends $pb.GeneratedMessage {
  factory LegalDocument({
    $core.String? document,
    $core.String? version,
    $core.String? title,
    $core.String? content,
  }) {
    final result = create();
    if (document != null) result.document = document;
    if (version != null) result.version = version;
    if (title != null) result.title = title;
    if (content != null) result.content = content;
    return result;
  }

  LegalDocument._();

  factory LegalDocument.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LegalDocument.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LegalDocument',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.legal'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'document')
    ..aOS(2, _omitFieldNames ? '' : 'version')
    ..aOS(3, _omitFieldNames ? '' : 'title')
    ..aOS(4, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LegalDocument clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LegalDocument copyWith(void Function(LegalDocument) updates) =>
      super.copyWith((message) => updates(message as LegalDocument))
          as LegalDocument;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LegalDocument create() => LegalDocument._();
  @$core.override
  LegalDocument createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LegalDocument getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LegalDocument>(create);
  static LegalDocument? _defaultInstance;

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
  $core.String get title => $_getSZ(2);
  @$pb.TagNumber(3)
  set title($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTitle() => $_has(2);
  @$pb.TagNumber(3)
  void clearTitle() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get content => $_getSZ(3);
  @$pb.TagNumber(4)
  set content($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasContent() => $_has(3);
  @$pb.TagNumber(4)
  void clearContent() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
