import 'package:grpc/grpc.dart' as grpc;
import 'package:protobuf/protobuf.dart' show GeneratedMessage;
import 'package:socialgym_mobile/config/api_config.dart';

import '../base_service.dart';
import 'grpc_channel_factory.dart';

/// Shared plumbing of the timeline gRPC services: the channel, the call options, the error
/// translation and the bridge to the JSON maps the models already parse.
class GrpcTimeline {
  GrpcTimeline._();

  /// Replaced by tests to point the services at a fake server.
  static grpc.ClientChannel? channelOverride;

  static grpc.ClientChannel get channel =>
      channelOverride ??
      GrpcChannelFactory.channelFor(
        host: ApiConfig.timelineGrpcHost,
        port: ApiConfig.timelineGrpcPort,
        authority: ApiConfig.timelineGrpcAuthority,
      );

  static List<grpc.ClientInterceptor> get interceptors =>
      channelOverride != null ? const [] : GrpcChannelFactory.interceptors;

  static grpc.CallOptions get options => grpc.CallOptions(timeout: ApiConfig.timeout);

  /// Options that carry [token] explicitly. The auth interceptor reads the stored sign-in, which a
  /// sign-out may already have cleared; the push-device removal that follows a sign-out must still
  /// be authenticated as the person who is leaving.
  static grpc.CallOptions withToken(String token) =>
      grpc.CallOptions(timeout: ApiConfig.timeout, metadata: {'authorization': 'Bearer $token'});

  /// Runs [call] and turns a gRPC failure into the [AppException] the screens already handle.
  static Future<T> run<T>(Future<T> Function() call, String fallback) async {
    try {
      return await call();
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, fallback);
    }
  }

  /// A message as the camelCase JSON map the REST API used to return (`fromJson` of the models).
  static Map<String, dynamic> json(GeneratedMessage message) =>
      Map<String, dynamic>.from(message.toProto3Json() as Map);

  /// Fills [message] from a REST-style camelCase payload; fields the contract does not carry
  /// (for example the author, which the server takes from the token) are ignored.
  static T fill<T extends GeneratedMessage>(T message, Map<String, dynamic> payload) {
    message.mergeFromProto3Json(payload, ignoreUnknownFields: true);
    return message;
  }
}
