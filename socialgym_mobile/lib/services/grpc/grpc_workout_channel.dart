import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/config/api_config.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_channel_factory.dart';

/// The channel to the workout service and the call plumbing every workout gRPC façade shares: the
/// timeout, and the translation of a [grpc.GrpcError] into the [AppException] the screens already handle.
class GrpcWorkoutChannel {
  GrpcWorkoutChannel._();

  static grpc.ClientChannel channel() => GrpcChannelFactory.channelFor(
    host: ApiConfig.grpcHost,
    port: ApiConfig.grpcPort,
    authority: ApiConfig.grpcAuthority,
  );

  static grpc.CallOptions get options => grpc.CallOptions(timeout: ApiConfig.timeout);

  static List<grpc.ClientInterceptor> get interceptors => GrpcChannelFactory.interceptors;

  static AppException toAppException(grpc.GrpcError e, String fallback) =>
      BaseService.handleGrpcError(e, fallback);

  /// Runs [call]; a gRPC failure becomes an [AppException] carrying the HTTP-equivalent status, the
  /// localized message and the `error-key` trailer, like the REST responses did.
  static Future<T> guard<T>(String fallback, Future<T> Function() call) async {
    try {
      return await call();
    } on grpc.GrpcError catch (e) {
      throw toAppException(e, fallback);
    }
  }
}
