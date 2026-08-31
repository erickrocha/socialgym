import 'dart:convert';
import 'package:grpc/grpc.dart' as grpc;
import 'package:shared_preferences/shared_preferences.dart';
import '../../models/auth_response.dart';

class GrpcAuthInterceptor implements grpc.ClientInterceptor {
  GrpcAuthInterceptor();

  @override
  grpc.ResponseStream<R> interceptStreaming<Q, R>(
    grpc.ClientMethod<Q, R> method,
    Stream<Q> requests,
    grpc.CallOptions options,
    grpc.ClientStreamingInvoker<Q, R> invoker,
  ) {
    // Injeta o provedor de metadados assíncrono também nos Streamings
    final novasOptions = options.mergedWith(
      grpc.CallOptions(providers: [_tokenProvider]),
    );
    return invoker(method, requests, novasOptions);
  }

  @override
  grpc.ResponseFuture<R> interceptUnary<Q, R>(
    grpc.ClientMethod<Q, R> method,
    Q request,
    grpc.CallOptions options,
    grpc.ClientUnaryInvoker<Q, R> invoker,
  ) {
    // Injeta o provedor de metadados assíncrono nas chamadas Unary
    final novasOptions = options.mergedWith(
      grpc.CallOptions(providers: [_tokenProvider]),
    );

    // Agora o invoker retorna o tipo exato que o gRPC espera, sem hacks ou casts!
    return invoker(method, request, novasOptions);
  }

  // O SEGREDO: Uma função assíncrona que o gRPC chama internamente antes do envio.
  // Ela recebe o mapa de metadados atual e permite que você o modifique.
  Future<void> _tokenProvider(Map<String, String> metadata, String uri) async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final authJson = prefs.getString('auth');

      if (authJson != null) {
        final data = jsonDecode(authJson) as Map<String, dynamic>;
        final auth = AuthResponse.fromJson(data);

        // Modifica o mapa diretamente. O gRPC vai ler esse valor na hora do envio.
        metadata['authorization'] = '${auth.tokenType} ${auth.accessToken}';
      }
    } catch (_) {
      // Silencia falhas de decode para não quebrar o fluxo da requisição deslogada
    }
  }
}
