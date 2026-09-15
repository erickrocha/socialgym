// This is a generated file - do not edit.
//
// Generated from team_member.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use teamMemberRequestDescriptor instead')
const TeamMemberRequest$json = {
  '1': 'TeamMemberRequest',
  '2': [
    {
      '1': 'business_profile_id',
      '3': 1,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
    {'1': 'person_id', '3': 2, '4': 1, '5': 5, '10': 'personId'},
  ],
};

/// Descriptor for `TeamMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List teamMemberRequestDescriptor = $convert.base64Decode(
    'ChFUZWFtTWVtYmVyUmVxdWVzdBIuChNidXNpbmVzc19wcm9maWxlX2lkGAEgASgFUhFidXNpbm'
    'Vzc1Byb2ZpbGVJZBIbCglwZXJzb25faWQYAiABKAVSCHBlcnNvbklk');

@$core.Deprecated('Use teamMemberDescriptor instead')
const TeamMember$json = {
  '1': 'TeamMember',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'uuid', '3': 2, '4': 1, '5': 9, '10': 'uuid'},
    {
      '1': 'business_profile_id',
      '3': 3,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
    {
      '1': 'business_profile_uuid',
      '3': 4,
      '4': 1,
      '5': 9,
      '10': 'businessProfileUuid'
    },
    {'1': 'person_id', '3': 5, '4': 1, '5': 5, '10': 'personId'},
    {'1': 'person_uuid', '3': 6, '4': 1, '5': 9, '10': 'personUuid'},
    {'1': 'status', '3': 7, '4': 1, '5': 9, '10': 'status'},
  ],
};

/// Descriptor for `TeamMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List teamMemberDescriptor = $convert.base64Decode(
    'CgpUZWFtTWVtYmVyEg4KAmlkGAEgASgFUgJpZBISCgR1dWlkGAIgASgJUgR1dWlkEi4KE2J1c2'
    'luZXNzX3Byb2ZpbGVfaWQYAyABKAVSEWJ1c2luZXNzUHJvZmlsZUlkEjIKFWJ1c2luZXNzX3By'
    'b2ZpbGVfdXVpZBgEIAEoCVITYnVzaW5lc3NQcm9maWxlVXVpZBIbCglwZXJzb25faWQYBSABKA'
    'VSCHBlcnNvbklkEh8KC3BlcnNvbl91dWlkGAYgASgJUgpwZXJzb25VdWlkEhYKBnN0YXR1cxgH'
    'IAEoCVIGc3RhdHVz');

@$core.Deprecated('Use teamMemberPageRequestDescriptor instead')
const TeamMemberPageRequest$json = {
  '1': 'TeamMemberPageRequest',
  '2': [
    {
      '1': 'business_profile_id',
      '3': 1,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
    {'1': 'person_id', '3': 2, '4': 1, '5': 5, '10': 'personId'},
  ],
};

/// Descriptor for `TeamMemberPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List teamMemberPageRequestDescriptor = $convert.base64Decode(
    'ChVUZWFtTWVtYmVyUGFnZVJlcXVlc3QSLgoTYnVzaW5lc3NfcHJvZmlsZV9pZBgBIAEoBVIRYn'
    'VzaW5lc3NQcm9maWxlSWQSGwoJcGVyc29uX2lkGAIgASgFUghwZXJzb25JZA==');

@$core.Deprecated('Use teamMemberPageResponseDescriptor instead')
const TeamMemberPageResponse$json = {
  '1': 'TeamMemberPageResponse',
  '2': [
    {
      '1': 'members',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'members'
    },
    {
      '1': 'sent_requests',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.grpc.person.Person',
      '10': 'sentRequests'
    },
    {
      '1': 'teams',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.grpc.business_profile.BusinessProfile',
      '10': 'teams'
    },
    {
      '1': 'received_requests',
      '3': 4,
      '4': 3,
      '5': 11,
      '6': '.grpc.business_profile.BusinessProfile',
      '10': 'receivedRequests'
    },
  ],
};

/// Descriptor for `TeamMemberPageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List teamMemberPageResponseDescriptor = $convert.base64Decode(
    'ChZUZWFtTWVtYmVyUGFnZVJlc3BvbnNlEi0KB21lbWJlcnMYASADKAsyEy5ncnBjLnBlcnNvbi'
    '5QZXJzb25SB21lbWJlcnMSOAoNc2VudF9yZXF1ZXN0cxgCIAMoCzITLmdycGMucGVyc29uLlBl'
    'cnNvblIMc2VudFJlcXVlc3RzEjwKBXRlYW1zGAMgAygLMiYuZ3JwYy5idXNpbmVzc19wcm9maW'
    'xlLkJ1c2luZXNzUHJvZmlsZVIFdGVhbXMSUwoRcmVjZWl2ZWRfcmVxdWVzdHMYBCADKAsyJi5n'
    'cnBjLmJ1c2luZXNzX3Byb2ZpbGUuQnVzaW5lc3NQcm9maWxlUhByZWNlaXZlZFJlcXVlc3Rz');

@$core.Deprecated('Use teamRosterRequestDescriptor instead')
const TeamRosterRequest$json = {
  '1': 'TeamRosterRequest',
  '2': [
    {
      '1': 'business_profile_uuid',
      '3': 1,
      '4': 1,
      '5': 9,
      '10': 'businessProfileUuid'
    },
  ],
};

/// Descriptor for `TeamRosterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List teamRosterRequestDescriptor = $convert.base64Decode(
    'ChFUZWFtUm9zdGVyUmVxdWVzdBIyChVidXNpbmVzc19wcm9maWxlX3V1aWQYASABKAlSE2J1c2'
    'luZXNzUHJvZmlsZVV1aWQ=');

@$core.Deprecated('Use teamRosterResponseDescriptor instead')
const TeamRosterResponse$json = {
  '1': 'TeamRosterResponse',
  '2': [
    {
      '1': 'business_profile_id',
      '3': 1,
      '4': 1,
      '5': 5,
      '10': 'businessProfileId'
    },
    {
      '1': 'business_profile_uuid',
      '3': 2,
      '4': 1,
      '5': 9,
      '10': 'businessProfileUuid'
    },
    {
      '1': 'business_profile_name',
      '3': 3,
      '4': 1,
      '5': 9,
      '10': 'businessProfileName'
    },
    {
      '1': 'business_profile_logo_object_key',
      '3': 4,
      '4': 1,
      '5': 9,
      '10': 'businessProfileLogoObjectKey'
    },
    {'1': 'owner_person_uuid', '3': 5, '4': 1, '5': 9, '10': 'ownerPersonUuid'},
    {
      '1': 'accepted_member_person_uuids',
      '3': 6,
      '4': 3,
      '5': 9,
      '10': 'acceptedMemberPersonUuids'
    },
  ],
};

/// Descriptor for `TeamRosterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List teamRosterResponseDescriptor = $convert.base64Decode(
    'ChJUZWFtUm9zdGVyUmVzcG9uc2USLgoTYnVzaW5lc3NfcHJvZmlsZV9pZBgBIAEoBVIRYnVzaW'
    '5lc3NQcm9maWxlSWQSMgoVYnVzaW5lc3NfcHJvZmlsZV91dWlkGAIgASgJUhNidXNpbmVzc1By'
    'b2ZpbGVVdWlkEjIKFWJ1c2luZXNzX3Byb2ZpbGVfbmFtZRgDIAEoCVITYnVzaW5lc3NQcm9maW'
    'xlTmFtZRJGCiBidXNpbmVzc19wcm9maWxlX2xvZ29fb2JqZWN0X2tleRgEIAEoCVIcYnVzaW5l'
    'c3NQcm9maWxlTG9nb09iamVjdEtleRIqChFvd25lcl9wZXJzb25fdXVpZBgFIAEoCVIPb3duZX'
    'JQZXJzb25VdWlkEj8KHGFjY2VwdGVkX21lbWJlcl9wZXJzb25fdXVpZHMYBiADKAlSGWFjY2Vw'
    'dGVkTWVtYmVyUGVyc29uVXVpZHM=');
