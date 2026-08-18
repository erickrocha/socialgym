// This is a generated file - do not edit.
//
// Generated from friend.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'person.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class FriendsRequest extends $pb.GeneratedMessage {
  factory FriendsRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  FriendsRequest._();

  factory FriendsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FriendsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FriendsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.friend'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendsRequest copyWith(void Function(FriendsRequest) updates) =>
      super.copyWith((message) => updates(message as FriendsRequest))
          as FriendsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FriendsRequest create() => FriendsRequest._();
  @$core.override
  FriendsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FriendsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FriendsRequest>(create);
  static FriendsRequest? _defaultInstance;

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
}

class FriendsResponse extends $pb.GeneratedMessage {
  factory FriendsResponse({
    $core.Iterable<Friend>? friends,
  }) {
    final result = create();
    if (friends != null) result.friends.addAll(friends);
    return result;
  }

  FriendsResponse._();

  factory FriendsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FriendsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FriendsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.friend'),
      createEmptyInstance: create)
    ..pPM<Friend>(1, _omitFieldNames ? '' : 'friends',
        subBuilder: Friend.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendsResponse copyWith(void Function(FriendsResponse) updates) =>
      super.copyWith((message) => updates(message as FriendsResponse))
          as FriendsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FriendsResponse create() => FriendsResponse._();
  @$core.override
  FriendsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FriendsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FriendsResponse>(create);
  static FriendsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Friend> get friends => $_getList(0);
}

class Friend extends $pb.GeneratedMessage {
  factory Friend({
    $core.int? id,
    $core.String? uuid,
    $core.int? personId,
    $core.String? personUuid,
    $core.int? friendId,
    $core.String? friendUuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (personId != null) result.personId = personId;
    if (personUuid != null) result.personUuid = personUuid;
    if (friendId != null) result.friendId = friendId;
    if (friendUuid != null) result.friendUuid = friendUuid;
    return result;
  }

  Friend._();

  factory Friend.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Friend.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Friend',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.friend'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aI(3, _omitFieldNames ? '' : 'personId')
    ..aOS(4, _omitFieldNames ? '' : 'personUuid')
    ..aI(5, _omitFieldNames ? '' : 'friendId')
    ..aOS(6, _omitFieldNames ? '' : 'friendUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Friend clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Friend copyWith(void Function(Friend) updates) =>
      super.copyWith((message) => updates(message as Friend)) as Friend;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Friend create() => Friend._();
  @$core.override
  Friend createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Friend getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Friend>(create);
  static Friend? _defaultInstance;

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
  $core.int get personId => $_getIZ(2);
  @$pb.TagNumber(3)
  set personId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPersonId() => $_has(2);
  @$pb.TagNumber(3)
  void clearPersonId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get personUuid => $_getSZ(3);
  @$pb.TagNumber(4)
  set personUuid($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPersonUuid() => $_has(3);
  @$pb.TagNumber(4)
  void clearPersonUuid() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get friendId => $_getIZ(4);
  @$pb.TagNumber(5)
  set friendId($core.int value) => $_setSignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFriendId() => $_has(4);
  @$pb.TagNumber(5)
  void clearFriendId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get friendUuid => $_getSZ(5);
  @$pb.TagNumber(6)
  set friendUuid($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasFriendUuid() => $_has(5);
  @$pb.TagNumber(6)
  void clearFriendUuid() => $_clearField(6);
}

class FriendPageRequest extends $pb.GeneratedMessage {
  factory FriendPageRequest({
    $core.int? personId,
  }) {
    final result = create();
    if (personId != null) result.personId = personId;
    return result;
  }

  FriendPageRequest._();

  factory FriendPageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FriendPageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FriendPageRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.friend'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'personId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendPageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendPageRequest copyWith(void Function(FriendPageRequest) updates) =>
      super.copyWith((message) => updates(message as FriendPageRequest))
          as FriendPageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FriendPageRequest create() => FriendPageRequest._();
  @$core.override
  FriendPageRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FriendPageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FriendPageRequest>(create);
  static FriendPageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get personId => $_getIZ(0);
  @$pb.TagNumber(1)
  set personId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPersonId() => $_has(0);
  @$pb.TagNumber(1)
  void clearPersonId() => $_clearField(1);
}

class FriendPageResponse extends $pb.GeneratedMessage {
  factory FriendPageResponse({
    $core.Iterable<$1.Person>? suggestions,
    $core.Iterable<$1.Person>? friends,
    $core.Iterable<$1.Person>? receiveRequests,
    $core.Iterable<$1.Person>? sentRequests,
  }) {
    final result = create();
    if (suggestions != null) result.suggestions.addAll(suggestions);
    if (friends != null) result.friends.addAll(friends);
    if (receiveRequests != null) result.receiveRequests.addAll(receiveRequests);
    if (sentRequests != null) result.sentRequests.addAll(sentRequests);
    return result;
  }

  FriendPageResponse._();

  factory FriendPageResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FriendPageResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FriendPageResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.friend'),
      createEmptyInstance: create)
    ..pPM<$1.Person>(1, _omitFieldNames ? '' : 'suggestions',
        subBuilder: $1.Person.create)
    ..pPM<$1.Person>(2, _omitFieldNames ? '' : 'friends',
        subBuilder: $1.Person.create)
    ..pPM<$1.Person>(3, _omitFieldNames ? '' : 'receiveRequests',
        subBuilder: $1.Person.create)
    ..pPM<$1.Person>(4, _omitFieldNames ? '' : 'sentRequests',
        subBuilder: $1.Person.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendPageResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FriendPageResponse copyWith(void Function(FriendPageResponse) updates) =>
      super.copyWith((message) => updates(message as FriendPageResponse))
          as FriendPageResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FriendPageResponse create() => FriendPageResponse._();
  @$core.override
  FriendPageResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FriendPageResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FriendPageResponse>(create);
  static FriendPageResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$1.Person> get suggestions => $_getList(0);

  @$pb.TagNumber(2)
  $pb.PbList<$1.Person> get friends => $_getList(1);

  @$pb.TagNumber(3)
  $pb.PbList<$1.Person> get receiveRequests => $_getList(2);

  @$pb.TagNumber(4)
  $pb.PbList<$1.Person> get sentRequests => $_getList(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
