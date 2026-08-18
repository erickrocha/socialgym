// This is a generated file - do not edit.
//
// Generated from team_member.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'business_profile.pb.dart' as $2;
import 'person.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class TeamMemberRequest extends $pb.GeneratedMessage {
  factory TeamMemberRequest({
    $core.int? businessProfileId,
    $core.int? personId,
  }) {
    final result = create();
    if (businessProfileId != null) result.businessProfileId = businessProfileId;
    if (personId != null) result.personId = personId;
    return result;
  }

  TeamMemberRequest._();

  factory TeamMemberRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TeamMemberRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TeamMemberRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'grpc.team_member'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'businessProfileId')
    ..aI(2, _omitFieldNames ? '' : 'personId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMemberRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMemberRequest copyWith(void Function(TeamMemberRequest) updates) =>
      super.copyWith((message) => updates(message as TeamMemberRequest))
          as TeamMemberRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TeamMemberRequest create() => TeamMemberRequest._();
  @$core.override
  TeamMemberRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static TeamMemberRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TeamMemberRequest>(create);
  static TeamMemberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get businessProfileId => $_getIZ(0);
  @$pb.TagNumber(1)
  set businessProfileId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBusinessProfileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBusinessProfileId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get personId => $_getIZ(1);
  @$pb.TagNumber(2)
  set personId($core.int value) => $_setSignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonId() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonId() => $_clearField(2);
}

class TeamMember extends $pb.GeneratedMessage {
  factory TeamMember({
    $core.int? id,
    $core.String? uuid,
    $core.int? businessProfileId,
    $core.String? businessProfileUuid,
    $core.int? personId,
    $core.String? personUuid,
    $core.String? status,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (businessProfileId != null) result.businessProfileId = businessProfileId;
    if (businessProfileUuid != null)
      result.businessProfileUuid = businessProfileUuid;
    if (personId != null) result.personId = personId;
    if (personUuid != null) result.personUuid = personUuid;
    if (status != null) result.status = status;
    return result;
  }

  TeamMember._();

  factory TeamMember.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TeamMember.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TeamMember',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'grpc.team_member'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aI(3, _omitFieldNames ? '' : 'businessProfileId')
    ..aOS(4, _omitFieldNames ? '' : 'businessProfileUuid')
    ..aI(5, _omitFieldNames ? '' : 'personId')
    ..aOS(6, _omitFieldNames ? '' : 'personUuid')
    ..aOS(7, _omitFieldNames ? '' : 'status')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMember clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMember copyWith(void Function(TeamMember) updates) =>
      super.copyWith((message) => updates(message as TeamMember)) as TeamMember;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TeamMember create() => TeamMember._();
  @$core.override
  TeamMember createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static TeamMember getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TeamMember>(create);
  static TeamMember? _defaultInstance;

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
  $core.String get businessProfileUuid => $_getSZ(3);
  @$pb.TagNumber(4)
  set businessProfileUuid($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBusinessProfileUuid() => $_has(3);
  @$pb.TagNumber(4)
  void clearBusinessProfileUuid() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get personId => $_getIZ(4);
  @$pb.TagNumber(5)
  set personId($core.int value) => $_setSignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasPersonId() => $_has(4);
  @$pb.TagNumber(5)
  void clearPersonId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get personUuid => $_getSZ(5);
  @$pb.TagNumber(6)
  set personUuid($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasPersonUuid() => $_has(5);
  @$pb.TagNumber(6)
  void clearPersonUuid() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get status => $_getSZ(6);
  @$pb.TagNumber(7)
  set status($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasStatus() => $_has(6);
  @$pb.TagNumber(7)
  void clearStatus() => $_clearField(7);
}

/// Either side may be asked for: business_profile_id fills members/sent_requests,
/// person_id fills teams/received_requests. At least one must be informed.
class TeamMemberPageRequest extends $pb.GeneratedMessage {
  factory TeamMemberPageRequest({
    $core.int? businessProfileId,
    $core.int? personId,
  }) {
    final result = create();
    if (businessProfileId != null) result.businessProfileId = businessProfileId;
    if (personId != null) result.personId = personId;
    return result;
  }

  TeamMemberPageRequest._();

  factory TeamMemberPageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TeamMemberPageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TeamMemberPageRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'grpc.team_member'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'businessProfileId')
    ..aI(2, _omitFieldNames ? '' : 'personId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMemberPageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMemberPageRequest copyWith(
          void Function(TeamMemberPageRequest) updates) =>
      super.copyWith((message) => updates(message as TeamMemberPageRequest))
          as TeamMemberPageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TeamMemberPageRequest create() => TeamMemberPageRequest._();
  @$core.override
  TeamMemberPageRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static TeamMemberPageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TeamMemberPageRequest>(create);
  static TeamMemberPageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get businessProfileId => $_getIZ(0);
  @$pb.TagNumber(1)
  set businessProfileId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBusinessProfileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBusinessProfileId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get personId => $_getIZ(1);
  @$pb.TagNumber(2)
  set personId($core.int value) => $_setSignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPersonId() => $_has(1);
  @$pb.TagNumber(2)
  void clearPersonId() => $_clearField(2);
}

class TeamMemberPageResponse extends $pb.GeneratedMessage {
  factory TeamMemberPageResponse({
    $core.Iterable<$1.Person>? members,
    $core.Iterable<$1.Person>? sentRequests,
    $core.Iterable<$2.BusinessProfile>? teams,
    $core.Iterable<$2.BusinessProfile>? receivedRequests,
  }) {
    final result = create();
    if (members != null) result.members.addAll(members);
    if (sentRequests != null) result.sentRequests.addAll(sentRequests);
    if (teams != null) result.teams.addAll(teams);
    if (receivedRequests != null)
      result.receivedRequests.addAll(receivedRequests);
    return result;
  }

  TeamMemberPageResponse._();

  factory TeamMemberPageResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TeamMemberPageResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TeamMemberPageResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'grpc.team_member'),
      createEmptyInstance: create)
    ..pPM<$1.Person>(1, _omitFieldNames ? '' : 'members',
        subBuilder: $1.Person.create)
    ..pPM<$1.Person>(2, _omitFieldNames ? '' : 'sentRequests',
        subBuilder: $1.Person.create)
    ..pPM<$2.BusinessProfile>(3, _omitFieldNames ? '' : 'teams',
        subBuilder: $2.BusinessProfile.create)
    ..pPM<$2.BusinessProfile>(4, _omitFieldNames ? '' : 'receivedRequests',
        subBuilder: $2.BusinessProfile.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMemberPageResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TeamMemberPageResponse copyWith(
          void Function(TeamMemberPageResponse) updates) =>
      super.copyWith((message) => updates(message as TeamMemberPageResponse))
          as TeamMemberPageResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TeamMemberPageResponse create() => TeamMemberPageResponse._();
  @$core.override
  TeamMemberPageResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static TeamMemberPageResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TeamMemberPageResponse>(create);
  static TeamMemberPageResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$1.Person> get members => $_getList(0);

  @$pb.TagNumber(2)
  $pb.PbList<$1.Person> get sentRequests => $_getList(1);

  @$pb.TagNumber(3)
  $pb.PbList<$2.BusinessProfile> get teams => $_getList(2);

  @$pb.TagNumber(4)
  $pb.PbList<$2.BusinessProfile> get receivedRequests => $_getList(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
