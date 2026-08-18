import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/commons/person_mapper.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:socialgym_mobile/src/generated/grpc/person.pbgrpc.dart' as $person;

import '../../config/api_config.dart';
import '../../models/mentionable_friend.dart';

// Re-export so existing consumers (e.g. tests) can still import
// SearchMentionableFriendsRequest from this file.
export '../../src/generated/grpc/person.pb.dart' show SearchMentionableFriendsRequest;

/// Pre-signed S3 upload URL for a person's avatar/cover image, returned by
/// [GrpcPersonService.getPersonImageUploadUrl].
class PersonImageUploadUrl {
  final String url;
  final String objectKey;

  PersonImageUploadUrl({required this.url, required this.objectKey});
}

/// gRPC client façade for the PersonService.
///
/// Uses the code-generated [PersonServiceClient] and proto messages from
/// `lib/src/generated/grpc/person.pb(grpc).dart` rather than maintaining
/// hand-written protobuf definitions.
class GrpcPersonService {
  GrpcPersonService._();

  static $person.PersonServiceClient? _client;

  /// Returns up to [limit] friends whose name matches [query], mapped to
  /// [MentionableFriend] domain objects.
  ///
  /// [MentionableFriend.uuid] is populated from the proto `uuid` field when
  /// present and non-empty; otherwise it falls back to the numeric `id` as a
  /// string for backward compatibility.
  static Future<List<MentionableFriend>> searchMentionableFriends({
    required int personId,
    required String query,
    int limit = 20,
  }) async {
    final normalized = query.trim();
    if (personId <= 0 || normalized.isEmpty) {
      return const <MentionableFriend>[];
    }

    final response = await _ensureClient().searchMentionableFriends(
      $person.SearchMentionableFriendsRequest(personId: personId, query: normalized, limit: limit),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );

    return response.people
        .map(
          (p) => MentionableFriend(
            id: p.id,
            uuid: p.uuid,
            firstname: p.firstname,
            surname: p.surname,
            avatar: p.hasAvatar() && p.avatar.isNotEmpty ? p.avatar : null,
          ),
        )
        .toList(growable: false);
  }

  static Future<Person> getPerson({int? id,String? uuid}) async {
    final response = await _ensureClient().getPerson(
      $person.PersonIdRequest(id: id, uuid: uuid),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );

    return PersonMapper().fromProto(response.person);
  }

  /// Fetches the authenticated caller's own profile. Identity is derived
  /// server-side from the JWT, mirroring REST's `GET /me`.
  static Future<Person> getMe() async {
    final response = await _ensureClient().getMe(
      $person.GetMeRequest(),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonMapper().fromProto(response.person);
  }

  static Future<Person> updatePerson(Person domain) async {
    final response = await _ensureClient().updatePerson(
      PersonMapper().toProto(domain),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonMapper().fromProto(response.person);
  }

  static Future<PersonInfo> updatePersonInfo(PersonInfo domain) async {
    final response = await _ensureClient().updatePersonInfo(
      PersonInfoMapper().toProto(domain),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonInfoMapper().fromProto(response);
  }

  static Future<PersonAddress> addPersonAddress(PersonAddress domain) async {
    final response = await _ensureClient().addPersonAddress(
      PersonAddressMapper().toProto(domain),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonAddressMapper().fromProto(response);
  }

  static Future<PersonAddress> updatePersonAddress(PersonAddress domain) async {
    final response = await _ensureClient().updatePersonAddress(
      PersonAddressMapper().toProto(domain),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonAddressMapper().fromProto(response);
  }

  static Future<bool> removePersonAddress({int? id, String? uuid}) async {
    final response = await _ensureClient().removePersonAddress(
      $person.RemovePersonAddressRequest(id: id ?? 0, uuid: uuid ?? ''),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return response.success;
  }

  static Future<PersonImageUploadUrl> getPersonImageUploadUrl({
    required String imageType,
    required String format,
  }) async {
    final response = await _ensureClient().getPersonImageUploadUrl(
      $person.PersonImageUploadRequest(imageType: imageType, format: format),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonImageUploadUrl(url: response.url, objectKey: response.objectKey);
  }

  static Future<bool> deletePersonImage({required String imageType}) async {
    final response = await _ensureClient().deletePersonImage(
      $person.PersonImageRequest(imageType: imageType),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return response.success;
  }

  static $person.PersonServiceClient _ensureClient() {
    if (_client != null) return _client!;

    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $person.PersonServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  /// Closes the underlying gRPC channel. Call this when the app is shutting
  /// down or when the channel is no longer needed.
  static Future<void> shutdown() async {
    _client = null;
  }

  static Future<List<Person>> searchPersonsByUuid({required String uuid, required String query, required int limit}) async {
    final response = await _ensureClient().searchPersons(
      $person.PersonParams()..uuid = uuid
        ..query = query
        ..limit = limit,
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return PersonMapper().fromProtoList(response.people);
  }
}
