// This is a generated file - do not edit.
//
// Generated from timeline/feed.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'post.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class GetFeedRequest extends $pb.GeneratedMessage {
  factory GetFeedRequest({
    $core.int? page,
  }) {
    final result = create();
    if (page != null) result.page = page;
    return result;
  }

  GetFeedRequest._();

  factory GetFeedRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetFeedRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetFeedRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'page', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetFeedRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetFeedRequest copyWith(void Function(GetFeedRequest) updates) =>
      super.copyWith((message) => updates(message as GetFeedRequest))
          as GetFeedRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetFeedRequest create() => GetFeedRequest._();
  @$core.override
  GetFeedRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetFeedRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetFeedRequest>(create);
  static GetFeedRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get page => $_getIZ(0);
  @$pb.TagNumber(1)
  set page($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPage() => $_has(0);
  @$pb.TagNumber(1)
  void clearPage() => $_clearField(1);
}

class GetFeedByAuthorRequest extends $pb.GeneratedMessage {
  factory GetFeedByAuthorRequest({
    $core.String? authorUuid,
    $core.int? page,
  }) {
    final result = create();
    if (authorUuid != null) result.authorUuid = authorUuid;
    if (page != null) result.page = page;
    return result;
  }

  GetFeedByAuthorRequest._();

  factory GetFeedByAuthorRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetFeedByAuthorRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetFeedByAuthorRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'authorUuid')
    ..aI(2, _omitFieldNames ? '' : 'page', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetFeedByAuthorRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetFeedByAuthorRequest copyWith(
          void Function(GetFeedByAuthorRequest) updates) =>
      super.copyWith((message) => updates(message as GetFeedByAuthorRequest))
          as GetFeedByAuthorRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetFeedByAuthorRequest create() => GetFeedByAuthorRequest._();
  @$core.override
  GetFeedByAuthorRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetFeedByAuthorRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetFeedByAuthorRequest>(create);
  static GetFeedByAuthorRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get authorUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set authorUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAuthorUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearAuthorUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get page => $_getIZ(1);
  @$pb.TagNumber(2)
  set page($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(2)
  void clearPage() => $_clearField(2);
}

class FeedResponse extends $pb.GeneratedMessage {
  factory FeedResponse({
    $core.Iterable<$1.Post>? posts,
  }) {
    final result = create();
    if (posts != null) result.posts.addAll(posts);
    return result;
  }

  FeedResponse._();

  factory FeedResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..pPM<$1.Post>(1, _omitFieldNames ? '' : 'posts',
        subBuilder: $1.Post.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedResponse copyWith(void Function(FeedResponse) updates) =>
      super.copyWith((message) => updates(message as FeedResponse))
          as FeedResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedResponse create() => FeedResponse._();
  @$core.override
  FeedResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedResponse>(create);
  static FeedResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$1.Post> get posts => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
