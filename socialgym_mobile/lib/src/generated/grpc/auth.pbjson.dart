// This is a generated file - do not edit.
//
// Generated from auth.proto.

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

@$core.Deprecated('Use signupRequestDescriptor instead')
const SignupRequest$json = {
  '1': 'SignupRequest',
  '2': [
    {'1': 'firstname', '3': 1, '4': 1, '5': 9, '10': 'firstname'},
    {'1': 'surname', '3': 2, '4': 1, '5': 9, '10': 'surname'},
    {'1': 'date_of_birth', '3': 3, '4': 1, '5': 9, '10': 'dateOfBirth'},
    {'1': 'gender', '3': 4, '4': 1, '5': 9, '10': 'gender'},
    {'1': 'email', '3': 5, '4': 1, '5': 9, '10': 'email'},
    {'1': 'password', '3': 6, '4': 1, '5': 9, '10': 'password'},
    {'1': 'terms_version', '3': 7, '4': 1, '5': 9, '10': 'termsVersion'},
    {'1': 'privacy_version', '3': 8, '4': 1, '5': 9, '10': 'privacyVersion'},
    {'1': 'terms_accepted', '3': 9, '4': 1, '5': 8, '10': 'termsAccepted'},
    {'1': 'privacy_accepted', '3': 10, '4': 1, '5': 8, '10': 'privacyAccepted'},
  ],
};

/// Descriptor for `SignupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signupRequestDescriptor = $convert.base64Decode(
    'Cg1TaWdudXBSZXF1ZXN0EhwKCWZpcnN0bmFtZRgBIAEoCVIJZmlyc3RuYW1lEhgKB3N1cm5hbW'
    'UYAiABKAlSB3N1cm5hbWUSIgoNZGF0ZV9vZl9iaXJ0aBgDIAEoCVILZGF0ZU9mQmlydGgSFgoG'
    'Z2VuZGVyGAQgASgJUgZnZW5kZXISFAoFZW1haWwYBSABKAlSBWVtYWlsEhoKCHBhc3N3b3JkGA'
    'YgASgJUghwYXNzd29yZBIjCg10ZXJtc192ZXJzaW9uGAcgASgJUgx0ZXJtc1ZlcnNpb24SJwoP'
    'cHJpdmFjeV92ZXJzaW9uGAggASgJUg5wcml2YWN5VmVyc2lvbhIlCg50ZXJtc19hY2NlcHRlZB'
    'gJIAEoCFINdGVybXNBY2NlcHRlZBIpChBwcml2YWN5X2FjY2VwdGVkGAogASgIUg9wcml2YWN5'
    'QWNjZXB0ZWQ=');

@$core.Deprecated('Use loginRequestDescriptor instead')
const LoginRequest$json = {
  '1': 'LoginRequest',
  '2': [
    {'1': 'email', '3': 1, '4': 1, '5': 9, '10': 'email'},
    {'1': 'password', '3': 2, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `LoginRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginRequestDescriptor = $convert.base64Decode(
    'CgxMb2dpblJlcXVlc3QSFAoFZW1haWwYASABKAlSBWVtYWlsEhoKCHBhc3N3b3JkGAIgASgJUg'
    'hwYXNzd29yZA==');

@$core.Deprecated('Use refreshRequestDescriptor instead')
const RefreshRequest$json = {
  '1': 'RefreshRequest',
  '2': [
    {'1': 'refresh_token', '3': 1, '4': 1, '5': 9, '10': 'refreshToken'},
  ],
};

/// Descriptor for `RefreshRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List refreshRequestDescriptor = $convert.base64Decode(
    'Cg5SZWZyZXNoUmVxdWVzdBIjCg1yZWZyZXNoX3Rva2VuGAEgASgJUgxyZWZyZXNoVG9rZW4=');

@$core.Deprecated('Use logoutRequestDescriptor instead')
const LogoutRequest$json = {
  '1': 'LogoutRequest',
  '2': [
    {'1': 'refresh_token', '3': 1, '4': 1, '5': 9, '10': 'refreshToken'},
  ],
};

/// Descriptor for `LogoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logoutRequestDescriptor = $convert.base64Decode(
    'Cg1Mb2dvdXRSZXF1ZXN0EiMKDXJlZnJlc2hfdG9rZW4YASABKAlSDHJlZnJlc2hUb2tlbg==');

@$core.Deprecated('Use logoutResponseDescriptor instead')
const LogoutResponse$json = {
  '1': 'LogoutResponse',
};

/// Descriptor for `LogoutResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logoutResponseDescriptor =
    $convert.base64Decode('Cg5Mb2dvdXRSZXNwb25zZQ==');

@$core.Deprecated('Use activateBusinessProfileRequestDescriptor instead')
const ActivateBusinessProfileRequest$json = {
  '1': 'ActivateBusinessProfileRequest',
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

/// Descriptor for `ActivateBusinessProfileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List activateBusinessProfileRequestDescriptor =
    $convert.base64Decode(
        'Ch5BY3RpdmF0ZUJ1c2luZXNzUHJvZmlsZVJlcXVlc3QSMgoVYnVzaW5lc3NfcHJvZmlsZV91dW'
        'lkGAEgASgJUhNidXNpbmVzc1Byb2ZpbGVVdWlk');

@$core.Deprecated('Use deactivateBusinessProfileRequestDescriptor instead')
const DeactivateBusinessProfileRequest$json = {
  '1': 'DeactivateBusinessProfileRequest',
};

/// Descriptor for `DeactivateBusinessProfileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deactivateBusinessProfileRequestDescriptor =
    $convert.base64Decode('CiBEZWFjdGl2YXRlQnVzaW5lc3NQcm9maWxlUmVxdWVzdA==');

@$core.Deprecated('Use accessTokenDescriptor instead')
const AccessToken$json = {
  '1': 'AccessToken',
  '2': [
    {'1': 'access_token', '3': 1, '4': 1, '5': 9, '10': 'accessToken'},
    {'1': 'token_type', '3': 2, '4': 1, '5': 9, '10': 'tokenType'},
    {'1': 'expire_in', '3': 3, '4': 1, '5': 3, '10': 'expireIn'},
    {
      '1': 'refresh_token',
      '3': 4,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'refreshToken',
      '17': true
    },
    {'1': 'username', '3': 5, '4': 1, '5': 9, '10': 'username'},
    {'1': 'uuid', '3': 6, '4': 1, '5': 9, '10': 'uuid'},
    {'1': 'name', '3': 7, '4': 1, '5': 9, '10': 'name'},
    {'1': 'person_id', '3': 8, '4': 1, '5': 5, '10': 'personId'},
    {'1': 'person_uuid', '3': 9, '4': 1, '5': 9, '10': 'personUuid'},
    {
      '1': 'person_object_key',
      '3': 10,
      '4': 1,
      '5': 9,
      '10': 'personObjectKey'
    },
    {
      '1': 'active_business_profile_id',
      '3': 11,
      '4': 1,
      '5': 5,
      '9': 1,
      '10': 'activeBusinessProfileId',
      '17': true
    },
    {
      '1': 'active_business_profile_uuid',
      '3': 12,
      '4': 1,
      '5': 9,
      '9': 2,
      '10': 'activeBusinessProfileUuid',
      '17': true
    },
    {
      '1': 'pending_account_deletion',
      '3': 13,
      '4': 1,
      '5': 11,
      '6': '.grpc.auth.PendingAccountDeletion',
      '9': 3,
      '10': 'pendingAccountDeletion',
      '17': true
    },
  ],
  '8': [
    {'1': '_refresh_token'},
    {'1': '_active_business_profile_id'},
    {'1': '_active_business_profile_uuid'},
    {'1': '_pending_account_deletion'},
  ],
};

/// Descriptor for `AccessToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accessTokenDescriptor = $convert.base64Decode(
    'CgtBY2Nlc3NUb2tlbhIhCgxhY2Nlc3NfdG9rZW4YASABKAlSC2FjY2Vzc1Rva2VuEh0KCnRva2'
    'VuX3R5cGUYAiABKAlSCXRva2VuVHlwZRIbCglleHBpcmVfaW4YAyABKANSCGV4cGlyZUluEigK'
    'DXJlZnJlc2hfdG9rZW4YBCABKAlIAFIMcmVmcmVzaFRva2VuiAEBEhoKCHVzZXJuYW1lGAUgAS'
    'gJUgh1c2VybmFtZRISCgR1dWlkGAYgASgJUgR1dWlkEhIKBG5hbWUYByABKAlSBG5hbWUSGwoJ'
    'cGVyc29uX2lkGAggASgFUghwZXJzb25JZBIfCgtwZXJzb25fdXVpZBgJIAEoCVIKcGVyc29uVX'
    'VpZBIqChFwZXJzb25fb2JqZWN0X2tleRgKIAEoCVIPcGVyc29uT2JqZWN0S2V5EkAKGmFjdGl2'
    'ZV9idXNpbmVzc19wcm9maWxlX2lkGAsgASgFSAFSF2FjdGl2ZUJ1c2luZXNzUHJvZmlsZUlkiA'
    'EBEkQKHGFjdGl2ZV9idXNpbmVzc19wcm9maWxlX3V1aWQYDCABKAlIAlIZYWN0aXZlQnVzaW5l'
    'c3NQcm9maWxlVXVpZIgBARJgChhwZW5kaW5nX2FjY291bnRfZGVsZXRpb24YDSABKAsyIS5ncn'
    'BjLmF1dGguUGVuZGluZ0FjY291bnREZWxldGlvbkgDUhZwZW5kaW5nQWNjb3VudERlbGV0aW9u'
    'iAEBQhAKDl9yZWZyZXNoX3Rva2VuQh0KG19hY3RpdmVfYnVzaW5lc3NfcHJvZmlsZV9pZEIfCh'
    '1fYWN0aXZlX2J1c2luZXNzX3Byb2ZpbGVfdXVpZEIbChlfcGVuZGluZ19hY2NvdW50X2RlbGV0'
    'aW9u');

@$core.Deprecated('Use pendingAccountDeletionDescriptor instead')
const PendingAccountDeletion$json = {
  '1': 'PendingAccountDeletion',
  '2': [
    {'1': 'requested_at', '3': 1, '4': 1, '5': 9, '10': 'requestedAt'},
    {'1': 'scheduled_at', '3': 2, '4': 1, '5': 9, '10': 'scheduledAt'},
  ],
};

/// Descriptor for `PendingAccountDeletion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pendingAccountDeletionDescriptor =
    $convert.base64Decode(
        'ChZQZW5kaW5nQWNjb3VudERlbGV0aW9uEiEKDHJlcXVlc3RlZF9hdBgBIAEoCVILcmVxdWVzdG'
        'VkQXQSIQoMc2NoZWR1bGVkX2F0GAIgASgJUgtzY2hlZHVsZWRBdA==');
