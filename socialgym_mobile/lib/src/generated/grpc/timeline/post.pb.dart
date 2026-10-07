// This is a generated file - do not edit.
//
// Generated from timeline/post.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class Media extends $pb.GeneratedMessage {
  factory Media({
    $core.String? uuid,
    $core.String? url,
    $core.String? mediaType,
    $core.String? objectKey,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (url != null) result.url = url;
    if (mediaType != null) result.mediaType = mediaType;
    if (objectKey != null) result.objectKey = objectKey;
    return result;
  }

  Media._();

  factory Media.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Media.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Media',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'url')
    ..aOS(3, _omitFieldNames ? '' : 'mediaType')
    ..aOS(4, _omitFieldNames ? '' : 'objectKey')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Media clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Media copyWith(void Function(Media) updates) =>
      super.copyWith((message) => updates(message as Media)) as Media;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Media create() => Media._();
  @$core.override
  Media createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Media getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Media>(create);
  static Media? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get url => $_getSZ(1);
  @$pb.TagNumber(2)
  set url($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearUrl() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get mediaType => $_getSZ(2);
  @$pb.TagNumber(3)
  set mediaType($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMediaType() => $_has(2);
  @$pb.TagNumber(3)
  void clearMediaType() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get objectKey => $_getSZ(3);
  @$pb.TagNumber(4)
  set objectKey($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasObjectKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearObjectKey() => $_clearField(4);
}

class Reaction extends $pb.GeneratedMessage {
  factory Reaction({
    $core.String? uuid,
    $core.String? authorId,
    $core.String? authorName,
    $core.String? reactionType,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (authorId != null) result.authorId = authorId;
    if (authorName != null) result.authorName = authorName;
    if (reactionType != null) result.reactionType = reactionType;
    return result;
  }

  Reaction._();

  factory Reaction.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Reaction.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Reaction',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'authorId')
    ..aOS(3, _omitFieldNames ? '' : 'authorName')
    ..aOS(4, _omitFieldNames ? '' : 'reactionType')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Reaction clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Reaction copyWith(void Function(Reaction) updates) =>
      super.copyWith((message) => updates(message as Reaction)) as Reaction;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Reaction create() => Reaction._();
  @$core.override
  Reaction createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Reaction getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Reaction>(create);
  static Reaction? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get authorId => $_getSZ(1);
  @$pb.TagNumber(2)
  set authorId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAuthorId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAuthorId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get authorName => $_getSZ(2);
  @$pb.TagNumber(3)
  set authorName($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAuthorName() => $_has(2);
  @$pb.TagNumber(3)
  void clearAuthorName() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get reactionType => $_getSZ(3);
  @$pb.TagNumber(4)
  set reactionType($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasReactionType() => $_has(3);
  @$pb.TagNumber(4)
  void clearReactionType() => $_clearField(4);
}

class Mention extends $pb.GeneratedMessage {
  factory Mention({
    $core.String? name,
    $core.String? mentionedUuid,
  }) {
    final result = create();
    if (name != null) result.name = name;
    if (mentionedUuid != null) result.mentionedUuid = mentionedUuid;
    return result;
  }

  Mention._();

  factory Mention.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Mention.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Mention',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'mentionedUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Mention clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Mention copyWith(void Function(Mention) updates) =>
      super.copyWith((message) => updates(message as Mention)) as Mention;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Mention create() => Mention._();
  @$core.override
  Mention createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Mention getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Mention>(create);
  static Mention? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get mentionedUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set mentionedUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMentionedUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearMentionedUuid() => $_clearField(2);
}

class Comment extends $pb.GeneratedMessage {
  factory Comment({
    $core.String? uuid,
    $core.String? postUuid,
    $core.String? authorUuid,
    $core.String? authorName,
    $core.String? authorObjectKey,
    $core.String? authorAvatar,
    $core.String? content,
    $core.String? parentUuid,
    $core.String? createdAt,
    $core.String? updatedAt,
    $core.Iterable<Mention>? mentions,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (postUuid != null) result.postUuid = postUuid;
    if (authorUuid != null) result.authorUuid = authorUuid;
    if (authorName != null) result.authorName = authorName;
    if (authorObjectKey != null) result.authorObjectKey = authorObjectKey;
    if (authorAvatar != null) result.authorAvatar = authorAvatar;
    if (content != null) result.content = content;
    if (parentUuid != null) result.parentUuid = parentUuid;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (mentions != null) result.mentions.addAll(mentions);
    return result;
  }

  Comment._();

  factory Comment.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Comment.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Comment',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aOS(2, _omitFieldNames ? '' : 'postUuid')
    ..aOS(3, _omitFieldNames ? '' : 'authorUuid')
    ..aOS(4, _omitFieldNames ? '' : 'authorName')
    ..aOS(5, _omitFieldNames ? '' : 'authorObjectKey')
    ..aOS(6, _omitFieldNames ? '' : 'authorAvatar')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..aOS(8, _omitFieldNames ? '' : 'parentUuid')
    ..aOS(9, _omitFieldNames ? '' : 'createdAt')
    ..aOS(10, _omitFieldNames ? '' : 'updatedAt')
    ..pPM<Mention>(11, _omitFieldNames ? '' : 'mentions',
        subBuilder: Mention.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Comment clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Comment copyWith(void Function(Comment) updates) =>
      super.copyWith((message) => updates(message as Comment)) as Comment;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Comment create() => Comment._();
  @$core.override
  Comment createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Comment getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Comment>(create);
  static Comment? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get postUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set postUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPostUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearPostUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get authorUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set authorUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAuthorUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearAuthorUuid() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get authorName => $_getSZ(3);
  @$pb.TagNumber(4)
  set authorName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAuthorName() => $_has(3);
  @$pb.TagNumber(4)
  void clearAuthorName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get authorObjectKey => $_getSZ(4);
  @$pb.TagNumber(5)
  set authorObjectKey($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAuthorObjectKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearAuthorObjectKey() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get authorAvatar => $_getSZ(5);
  @$pb.TagNumber(6)
  set authorAvatar($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAuthorAvatar() => $_has(5);
  @$pb.TagNumber(6)
  void clearAuthorAvatar() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get parentUuid => $_getSZ(7);
  @$pb.TagNumber(8)
  set parentUuid($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasParentUuid() => $_has(7);
  @$pb.TagNumber(8)
  void clearParentUuid() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get createdAt => $_getSZ(8);
  @$pb.TagNumber(9)
  set createdAt($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCreatedAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearCreatedAt() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get updatedAt => $_getSZ(9);
  @$pb.TagNumber(10)
  set updatedAt($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasUpdatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearUpdatedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $pb.PbList<Mention> get mentions => $_getList(10);
}

class Post extends $pb.GeneratedMessage {
  factory Post({
    $core.String? uuid,
    $core.int? authorId,
    $core.String? authorUuid,
    $core.String? authorName,
    $core.String? authorObjectKey,
    $core.String? authorAvatar,
    $core.String? content,
    $core.Iterable<Media>? media,
    $core.Iterable<Reaction>? reactions,
    $core.Iterable<Comment>? comments,
    $core.String? createdAt,
    $core.String? updatedAt,
    $core.Iterable<Mention>? mentions,
  }) {
    final result = create();
    if (uuid != null) result.uuid = uuid;
    if (authorId != null) result.authorId = authorId;
    if (authorUuid != null) result.authorUuid = authorUuid;
    if (authorName != null) result.authorName = authorName;
    if (authorObjectKey != null) result.authorObjectKey = authorObjectKey;
    if (authorAvatar != null) result.authorAvatar = authorAvatar;
    if (content != null) result.content = content;
    if (media != null) result.media.addAll(media);
    if (reactions != null) result.reactions.addAll(reactions);
    if (comments != null) result.comments.addAll(comments);
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (mentions != null) result.mentions.addAll(mentions);
    return result;
  }

  Post._();

  factory Post.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Post.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Post',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'uuid')
    ..aI(2, _omitFieldNames ? '' : 'authorId')
    ..aOS(3, _omitFieldNames ? '' : 'authorUuid')
    ..aOS(4, _omitFieldNames ? '' : 'authorName')
    ..aOS(5, _omitFieldNames ? '' : 'authorObjectKey')
    ..aOS(6, _omitFieldNames ? '' : 'authorAvatar')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..pPM<Media>(8, _omitFieldNames ? '' : 'media', subBuilder: Media.create)
    ..pPM<Reaction>(9, _omitFieldNames ? '' : 'reactions',
        subBuilder: Reaction.create)
    ..pPM<Comment>(10, _omitFieldNames ? '' : 'comments',
        subBuilder: Comment.create)
    ..aOS(11, _omitFieldNames ? '' : 'createdAt')
    ..aOS(12, _omitFieldNames ? '' : 'updatedAt')
    ..pPM<Mention>(13, _omitFieldNames ? '' : 'mentions',
        subBuilder: Mention.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Post clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Post copyWith(void Function(Post) updates) =>
      super.copyWith((message) => updates(message as Post)) as Post;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Post create() => Post._();
  @$core.override
  Post createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Post getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Post>(create);
  static Post? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get uuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set uuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get authorId => $_getIZ(1);
  @$pb.TagNumber(2)
  set authorId($core.int value) => $_setSignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAuthorId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAuthorId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get authorUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set authorUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasAuthorUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearAuthorUuid() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get authorName => $_getSZ(3);
  @$pb.TagNumber(4)
  set authorName($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasAuthorName() => $_has(3);
  @$pb.TagNumber(4)
  void clearAuthorName() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get authorObjectKey => $_getSZ(4);
  @$pb.TagNumber(5)
  set authorObjectKey($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAuthorObjectKey() => $_has(4);
  @$pb.TagNumber(5)
  void clearAuthorObjectKey() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get authorAvatar => $_getSZ(5);
  @$pb.TagNumber(6)
  set authorAvatar($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAuthorAvatar() => $_has(5);
  @$pb.TagNumber(6)
  void clearAuthorAvatar() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => $_clearField(7);

  @$pb.TagNumber(8)
  $pb.PbList<Media> get media => $_getList(7);

  @$pb.TagNumber(9)
  $pb.PbList<Reaction> get reactions => $_getList(8);

  @$pb.TagNumber(10)
  $pb.PbList<Comment> get comments => $_getList(9);

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

  @$pb.TagNumber(13)
  $pb.PbList<Mention> get mentions => $_getList(12);
}

class CreatePostRequest extends $pb.GeneratedMessage {
  factory CreatePostRequest({
    $core.String? content,
    $core.Iterable<Media>? media,
    $core.Iterable<Mention>? mentions,
    $core.bool? thirdPartyConsentConfirmed,
  }) {
    final result = create();
    if (content != null) result.content = content;
    if (media != null) result.media.addAll(media);
    if (mentions != null) result.mentions.addAll(mentions);
    if (thirdPartyConsentConfirmed != null)
      result.thirdPartyConsentConfirmed = thirdPartyConsentConfirmed;
    return result;
  }

  CreatePostRequest._();

  factory CreatePostRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreatePostRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreatePostRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..pPM<Media>(2, _omitFieldNames ? '' : 'media', subBuilder: Media.create)
    ..pPM<Mention>(3, _omitFieldNames ? '' : 'mentions',
        subBuilder: Mention.create)
    ..aOB(4, _omitFieldNames ? '' : 'thirdPartyConsentConfirmed')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePostRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreatePostRequest copyWith(void Function(CreatePostRequest) updates) =>
      super.copyWith((message) => updates(message as CreatePostRequest))
          as CreatePostRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreatePostRequest create() => CreatePostRequest._();
  @$core.override
  CreatePostRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreatePostRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreatePostRequest>(create);
  static CreatePostRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<Media> get media => $_getList(1);

  @$pb.TagNumber(3)
  $pb.PbList<Mention> get mentions => $_getList(2);

  /// Required (true) when media is attached.
  @$pb.TagNumber(4)
  $core.bool get thirdPartyConsentConfirmed => $_getBF(3);
  @$pb.TagNumber(4)
  set thirdPartyConsentConfirmed($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasThirdPartyConsentConfirmed() => $_has(3);
  @$pb.TagNumber(4)
  void clearThirdPartyConsentConfirmed() => $_clearField(4);
}

class DeletePostRequest extends $pb.GeneratedMessage {
  factory DeletePostRequest({
    $core.String? postUuid,
  }) {
    final result = create();
    if (postUuid != null) result.postUuid = postUuid;
    return result;
  }

  DeletePostRequest._();

  factory DeletePostRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeletePostRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeletePostRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'postUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePostRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePostRequest copyWith(void Function(DeletePostRequest) updates) =>
      super.copyWith((message) => updates(message as DeletePostRequest))
          as DeletePostRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeletePostRequest create() => DeletePostRequest._();
  @$core.override
  DeletePostRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeletePostRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeletePostRequest>(create);
  static DeletePostRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get postUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set postUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPostUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearPostUuid() => $_clearField(1);
}

class DeletePostResponse extends $pb.GeneratedMessage {
  factory DeletePostResponse() => create();

  DeletePostResponse._();

  factory DeletePostResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeletePostResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeletePostResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePostResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePostResponse copyWith(void Function(DeletePostResponse) updates) =>
      super.copyWith((message) => updates(message as DeletePostResponse))
          as DeletePostResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeletePostResponse create() => DeletePostResponse._();
  @$core.override
  DeletePostResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeletePostResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeletePostResponse>(create);
  static DeletePostResponse? _defaultInstance;
}

class AddCommentRequest extends $pb.GeneratedMessage {
  factory AddCommentRequest({
    $core.String? postUuid,
    $core.String? content,
    $core.String? parentUuid,
    $core.Iterable<Mention>? mentions,
  }) {
    final result = create();
    if (postUuid != null) result.postUuid = postUuid;
    if (content != null) result.content = content;
    if (parentUuid != null) result.parentUuid = parentUuid;
    if (mentions != null) result.mentions.addAll(mentions);
    return result;
  }

  AddCommentRequest._();

  factory AddCommentRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AddCommentRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AddCommentRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'postUuid')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..aOS(3, _omitFieldNames ? '' : 'parentUuid')
    ..pPM<Mention>(4, _omitFieldNames ? '' : 'mentions',
        subBuilder: Mention.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddCommentRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddCommentRequest copyWith(void Function(AddCommentRequest) updates) =>
      super.copyWith((message) => updates(message as AddCommentRequest))
          as AddCommentRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddCommentRequest create() => AddCommentRequest._();
  @$core.override
  AddCommentRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AddCommentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AddCommentRequest>(create);
  static AddCommentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get postUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set postUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPostUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearPostUuid() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get parentUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set parentUuid($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasParentUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearParentUuid() => $_clearField(3);

  @$pb.TagNumber(4)
  $pb.PbList<Mention> get mentions => $_getList(3);
}

class AddReactionRequest extends $pb.GeneratedMessage {
  factory AddReactionRequest({
    $core.String? postUuid,
    $core.String? reactionType,
  }) {
    final result = create();
    if (postUuid != null) result.postUuid = postUuid;
    if (reactionType != null) result.reactionType = reactionType;
    return result;
  }

  AddReactionRequest._();

  factory AddReactionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AddReactionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AddReactionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'postUuid')
    ..aOS(2, _omitFieldNames ? '' : 'reactionType')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddReactionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AddReactionRequest copyWith(void Function(AddReactionRequest) updates) =>
      super.copyWith((message) => updates(message as AddReactionRequest))
          as AddReactionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddReactionRequest create() => AddReactionRequest._();
  @$core.override
  AddReactionRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AddReactionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AddReactionRequest>(create);
  static AddReactionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get postUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set postUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPostUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearPostUuid() => $_clearField(1);

  /// like, love, haha, wow, sad or angry; read case-insensitively.
  @$pb.TagNumber(2)
  $core.String get reactionType => $_getSZ(1);
  @$pb.TagNumber(2)
  set reactionType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReactionType() => $_has(1);
  @$pb.TagNumber(2)
  void clearReactionType() => $_clearField(2);
}

class RemoveReactionRequest extends $pb.GeneratedMessage {
  factory RemoveReactionRequest({
    $core.String? postUuid,
  }) {
    final result = create();
    if (postUuid != null) result.postUuid = postUuid;
    return result;
  }

  RemoveReactionRequest._();

  factory RemoveReactionRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemoveReactionRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoveReactionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'postUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveReactionRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveReactionRequest copyWith(
          void Function(RemoveReactionRequest) updates) =>
      super.copyWith((message) => updates(message as RemoveReactionRequest))
          as RemoveReactionRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveReactionRequest create() => RemoveReactionRequest._();
  @$core.override
  RemoveReactionRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemoveReactionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemoveReactionRequest>(create);
  static RemoveReactionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get postUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set postUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPostUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearPostUuid() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
