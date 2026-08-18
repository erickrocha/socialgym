// This is a generated file - do not edit.
//
// Generated from person.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'person.pb.dart' as $0;
import 'person_address.pb.dart' as $2;
import 'person_info.pb.dart' as $1;

export 'person.pb.dart';

@$pb.GrpcServiceName('grpc.person.PersonService')
class PersonServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  PersonServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.PersonResponse> getPerson(
    $0.PersonIdRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPerson, request, options: options);
  }

  $grpc.ResponseFuture<$0.PersonResponse> getMe(
    $0.GetMeRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getMe, request, options: options);
  }

  $grpc.ResponseFuture<$0.PeopleResponse> searchMentionableFriends(
    $0.SearchMentionableFriendsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$searchMentionableFriends, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.PersonResponse> updatePerson(
    $0.Person request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePerson, request, options: options);
  }

  $grpc.ResponseFuture<$0.PeopleResponse> searchPersons(
    $0.PersonParams request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$searchPersons, request, options: options);
  }

  $grpc.ResponseFuture<$1.PersonInfo> updatePersonInfo(
    $1.PersonInfo request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePersonInfo, request, options: options);
  }

  $grpc.ResponseFuture<$2.PersonAddress> addPersonAddress(
    $2.PersonAddress request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addPersonAddress, request, options: options);
  }

  $grpc.ResponseFuture<$2.PersonAddress> updatePersonAddress(
    $2.PersonAddress request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updatePersonAddress, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemovePersonAddressResponse> removePersonAddress(
    $0.RemovePersonAddressRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removePersonAddress, request, options: options);
  }

  $grpc.ResponseFuture<$0.PersonImageUploadResponse> getPersonImageUploadUrl(
    $0.PersonImageUploadRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPersonImageUploadUrl, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.DeletePersonImageResponse> deletePersonImage(
    $0.PersonImageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePersonImage, request, options: options);
  }

  // method descriptors

  static final _$getPerson =
      $grpc.ClientMethod<$0.PersonIdRequest, $0.PersonResponse>(
          '/grpc.person.PersonService/GetPerson',
          ($0.PersonIdRequest value) => value.writeToBuffer(),
          $0.PersonResponse.fromBuffer);
  static final _$getMe = $grpc.ClientMethod<$0.GetMeRequest, $0.PersonResponse>(
      '/grpc.person.PersonService/GetMe',
      ($0.GetMeRequest value) => value.writeToBuffer(),
      $0.PersonResponse.fromBuffer);
  static final _$searchMentionableFriends =
      $grpc.ClientMethod<$0.SearchMentionableFriendsRequest, $0.PeopleResponse>(
          '/grpc.person.PersonService/SearchMentionableFriends',
          ($0.SearchMentionableFriendsRequest value) => value.writeToBuffer(),
          $0.PeopleResponse.fromBuffer);
  static final _$updatePerson =
      $grpc.ClientMethod<$0.Person, $0.PersonResponse>(
          '/grpc.person.PersonService/updatePerson',
          ($0.Person value) => value.writeToBuffer(),
          $0.PersonResponse.fromBuffer);
  static final _$searchPersons =
      $grpc.ClientMethod<$0.PersonParams, $0.PeopleResponse>(
          '/grpc.person.PersonService/SearchPersons',
          ($0.PersonParams value) => value.writeToBuffer(),
          $0.PeopleResponse.fromBuffer);
  static final _$updatePersonInfo =
      $grpc.ClientMethod<$1.PersonInfo, $1.PersonInfo>(
          '/grpc.person.PersonService/UpdatePersonInfo',
          ($1.PersonInfo value) => value.writeToBuffer(),
          $1.PersonInfo.fromBuffer);
  static final _$addPersonAddress =
      $grpc.ClientMethod<$2.PersonAddress, $2.PersonAddress>(
          '/grpc.person.PersonService/AddPersonAddress',
          ($2.PersonAddress value) => value.writeToBuffer(),
          $2.PersonAddress.fromBuffer);
  static final _$updatePersonAddress =
      $grpc.ClientMethod<$2.PersonAddress, $2.PersonAddress>(
          '/grpc.person.PersonService/UpdatePersonAddress',
          ($2.PersonAddress value) => value.writeToBuffer(),
          $2.PersonAddress.fromBuffer);
  static final _$removePersonAddress = $grpc.ClientMethod<
          $0.RemovePersonAddressRequest, $0.RemovePersonAddressResponse>(
      '/grpc.person.PersonService/RemovePersonAddress',
      ($0.RemovePersonAddressRequest value) => value.writeToBuffer(),
      $0.RemovePersonAddressResponse.fromBuffer);
  static final _$getPersonImageUploadUrl = $grpc.ClientMethod<
          $0.PersonImageUploadRequest, $0.PersonImageUploadResponse>(
      '/grpc.person.PersonService/GetPersonImageUploadUrl',
      ($0.PersonImageUploadRequest value) => value.writeToBuffer(),
      $0.PersonImageUploadResponse.fromBuffer);
  static final _$deletePersonImage =
      $grpc.ClientMethod<$0.PersonImageRequest, $0.DeletePersonImageResponse>(
          '/grpc.person.PersonService/DeletePersonImage',
          ($0.PersonImageRequest value) => value.writeToBuffer(),
          $0.DeletePersonImageResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.person.PersonService')
abstract class PersonServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.person.PersonService';

  PersonServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.PersonIdRequest, $0.PersonResponse>(
        'GetPerson',
        getPerson_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.PersonIdRequest.fromBuffer(value),
        ($0.PersonResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetMeRequest, $0.PersonResponse>(
        'GetMe',
        getMe_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetMeRequest.fromBuffer(value),
        ($0.PersonResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SearchMentionableFriendsRequest,
            $0.PeopleResponse>(
        'SearchMentionableFriends',
        searchMentionableFriends_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SearchMentionableFriendsRequest.fromBuffer(value),
        ($0.PeopleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Person, $0.PersonResponse>(
        'updatePerson',
        updatePerson_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Person.fromBuffer(value),
        ($0.PersonResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PersonParams, $0.PeopleResponse>(
        'SearchPersons',
        searchPersons_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.PersonParams.fromBuffer(value),
        ($0.PeopleResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.PersonInfo, $1.PersonInfo>(
        'UpdatePersonInfo',
        updatePersonInfo_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.PersonInfo.fromBuffer(value),
        ($1.PersonInfo value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$2.PersonAddress, $2.PersonAddress>(
        'AddPersonAddress',
        addPersonAddress_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $2.PersonAddress.fromBuffer(value),
        ($2.PersonAddress value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$2.PersonAddress, $2.PersonAddress>(
        'UpdatePersonAddress',
        updatePersonAddress_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $2.PersonAddress.fromBuffer(value),
        ($2.PersonAddress value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemovePersonAddressRequest,
            $0.RemovePersonAddressResponse>(
        'RemovePersonAddress',
        removePersonAddress_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemovePersonAddressRequest.fromBuffer(value),
        ($0.RemovePersonAddressResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PersonImageUploadRequest,
            $0.PersonImageUploadResponse>(
        'GetPersonImageUploadUrl',
        getPersonImageUploadUrl_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PersonImageUploadRequest.fromBuffer(value),
        ($0.PersonImageUploadResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PersonImageRequest,
            $0.DeletePersonImageResponse>(
        'DeletePersonImage',
        deletePersonImage_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.PersonImageRequest.fromBuffer(value),
        ($0.DeletePersonImageResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.PersonResponse> getPerson_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PersonIdRequest> $request) async {
    return getPerson($call, await $request);
  }

  $async.Future<$0.PersonResponse> getPerson(
      $grpc.ServiceCall call, $0.PersonIdRequest request);

  $async.Future<$0.PersonResponse> getMe_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.GetMeRequest> $request) async {
    return getMe($call, await $request);
  }

  $async.Future<$0.PersonResponse> getMe(
      $grpc.ServiceCall call, $0.GetMeRequest request);

  $async.Future<$0.PeopleResponse> searchMentionableFriends_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SearchMentionableFriendsRequest> $request) async {
    return searchMentionableFriends($call, await $request);
  }

  $async.Future<$0.PeopleResponse> searchMentionableFriends(
      $grpc.ServiceCall call, $0.SearchMentionableFriendsRequest request);

  $async.Future<$0.PersonResponse> updatePerson_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Person> $request) async {
    return updatePerson($call, await $request);
  }

  $async.Future<$0.PersonResponse> updatePerson(
      $grpc.ServiceCall call, $0.Person request);

  $async.Future<$0.PeopleResponse> searchPersons_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.PersonParams> $request) async {
    return searchPersons($call, await $request);
  }

  $async.Future<$0.PeopleResponse> searchPersons(
      $grpc.ServiceCall call, $0.PersonParams request);

  $async.Future<$1.PersonInfo> updatePersonInfo_Pre(
      $grpc.ServiceCall $call, $async.Future<$1.PersonInfo> $request) async {
    return updatePersonInfo($call, await $request);
  }

  $async.Future<$1.PersonInfo> updatePersonInfo(
      $grpc.ServiceCall call, $1.PersonInfo request);

  $async.Future<$2.PersonAddress> addPersonAddress_Pre(
      $grpc.ServiceCall $call, $async.Future<$2.PersonAddress> $request) async {
    return addPersonAddress($call, await $request);
  }

  $async.Future<$2.PersonAddress> addPersonAddress(
      $grpc.ServiceCall call, $2.PersonAddress request);

  $async.Future<$2.PersonAddress> updatePersonAddress_Pre(
      $grpc.ServiceCall $call, $async.Future<$2.PersonAddress> $request) async {
    return updatePersonAddress($call, await $request);
  }

  $async.Future<$2.PersonAddress> updatePersonAddress(
      $grpc.ServiceCall call, $2.PersonAddress request);

  $async.Future<$0.RemovePersonAddressResponse> removePersonAddress_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemovePersonAddressRequest> $request) async {
    return removePersonAddress($call, await $request);
  }

  $async.Future<$0.RemovePersonAddressResponse> removePersonAddress(
      $grpc.ServiceCall call, $0.RemovePersonAddressRequest request);

  $async.Future<$0.PersonImageUploadResponse> getPersonImageUploadUrl_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PersonImageUploadRequest> $request) async {
    return getPersonImageUploadUrl($call, await $request);
  }

  $async.Future<$0.PersonImageUploadResponse> getPersonImageUploadUrl(
      $grpc.ServiceCall call, $0.PersonImageUploadRequest request);

  $async.Future<$0.DeletePersonImageResponse> deletePersonImage_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.PersonImageRequest> $request) async {
    return deletePersonImage($call, await $request);
  }

  $async.Future<$0.DeletePersonImageResponse> deletePersonImage(
      $grpc.ServiceCall call, $0.PersonImageRequest request);
}
