import 'package:grpc/grpc.dart' as grpc;

import '../../commons/person_mapper.dart';
import '../../config/api_config.dart';
import '../../models/friends_data.dart';
import '../../models/person.dart';
import '../../src/generated/grpc/friend.pbgrpc.dart' as $friend;
import '../base_service.dart';
import 'grpc_channel_factory.dart';

/// gRPC client for the friend flow, replacing the old REST `FriendsService`.
/// The caller is resolved server-side from the bearer token injected by
/// `GrpcAuthInterceptor`, so requests only carry the other party / filters.
class GrpcFriendService {
  GrpcFriendService._();

  static $friend.FriendServiceClient? _client;

  static final PersonMapper _personMapper = PersonMapper();

  /// Consolidated friends page (suggestions, friends, received/sent requests).
  /// When [latitude]/[longitude] are given, suggestions are centered on that
  /// point instead of the person's saved home address.
  static Future<FriendsData> getFriendPage({
    double? latitude,
    double? longitude,
    double? radiusKm,
  }) async {
    try {
      final request = $friend.FriendPageRequest();
      if (latitude != null) request.latitude = latitude;
      if (longitude != null) request.longitude = longitude;
      if (radiusKm != null) request.radiusKm = radiusKm;

      final response = await _ensureClient().getFriendPage(
        request,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );

      return FriendsData(
        suggestions: _personMapper.fromProtoList(response.suggestions),
        friends: _personMapper.fromProtoList(response.friends),
        receiveRequests: _personMapper.fromProtoList(response.receiveRequests),
        sentRequests: _personMapper.fromProtoList(response.sentRequests),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to fetch friends');
    }
  }

  /// Combined name/location "find friends" search.
  static Future<List<Person>> searchFriends({
    String? query,
    double? latitude,
    double? longitude,
    double? radiusKm,
    int? limit,
  }) async {
    try {
      final request = $friend.SearchFriendsRequest(query: query ?? '');
      if (latitude != null) request.latitude = latitude;
      if (longitude != null) request.longitude = longitude;
      if (radiusKm != null) request.radiusKm = radiusKm;
      if (limit != null) request.limit = limit;

      final response = await _ensureClient().searchFriends(
        request,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );

      return _personMapper.fromProtoList(response.people);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to search friends');
    }
  }

  static Future<void> sendFriendRequest({required int personId}) async {
    try {
      await _ensureClient().sendFriendRequest(
        $friend.FriendRequestRequest(personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to send friend request');
    }
  }

  static Future<void> acceptFriendRequest({required int personId}) async {
    try {
      await _ensureClient().acceptFriendRequest(
        $friend.FriendRequestRequest(personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to accept friend request');
    }
  }

  static Future<void> denyFriendRequest({required int personId}) async {
    try {
      await _ensureClient().denyFriendRequest(
        $friend.FriendRequestRequest(personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to reject friend request');
    }
  }

  static Future<void> cancelFriendRequest({required int personId}) async {
    try {
      await _ensureClient().cancelFriendRequest(
        $friend.FriendRequestRequest(personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to cancel friend request');
    }
  }

  static Future<void> removeFriend({required int personId}) async {
    try {
      await _ensureClient().removeFriend(
        $friend.FriendRequestRequest(personId: personId),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to remove friend');
    }
  }

  static $friend.FriendServiceClient _ensureClient() {
    if (_client != null) return _client!;
    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $friend.FriendServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  static Future<void> shutdown() async {
    _client = null;
  }
}
