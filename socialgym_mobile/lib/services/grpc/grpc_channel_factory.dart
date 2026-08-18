import 'package:flutter/services.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/config/api_config.dart';

import 'grpc_auth_interceptor.dart';

class GrpcChannelFactory {
  GrpcChannelFactory._();

  static Uint8List? _trustedCertBytes;
  static final Map<String, grpc.ClientChannel> _channels = {};
  // 1. Guardamos a referência do Interceptor global aqui
  static grpc.ClientInterceptor? _authInterceptor;

  // AQUI: Expõe de forma centralizada os interceptores para os stubs utilizarem
  static List<grpc.ClientInterceptor> get interceptors =>
      _authInterceptor != null ? [_authInterceptor!] : [];

  /// Call once at app startup (or lazy-call on first channel creation).
  static Future<void> initialize({String certAssetPath = 'assets/certs/server.crt'}) async {
    _authInterceptor = GrpcAuthInterceptor();
    if (!ApiConfig.grpcUseTls) return;
    if (_trustedCertBytes != null) return; // Already initialized.

    final data = await rootBundle.load(certAssetPath);
    _trustedCertBytes = data.buffer.asUint8List();
  }

  static grpc.ClientChannel channelFor({required String host,required int port,String? authority}) {
    final key = '$host:$port:${ApiConfig.grpcUseTls ? "tls" : "insecure"}';
    final existing = _channels[key];
    if (existing != null) return existing;
    // Identifica se o host atual é o Ngrok
    final bool isNgrok = host.contains('ngrok-free.dev');

    final options = grpc.ChannelOptions(
      credentials: ApiConfig.grpcUseTls
          ? (isNgrok
                ? const grpc.ChannelCredentials.secure()
                // ^ SE FOR NGROK: Usa a credencial padrão limpa. Ele vai confiar no TLS oficial do Ngrok automaticamente.
                : grpc.ChannelCredentials.secure(
                    certificates: _trustedCertBytes,
                    authority: authority, // Precisa ser apenas '192.168.15.4' (sem https)
                  ))
          // ^ SE FOR IP LOCAL: Usa os bytes do seu server.crt autoassinado.
          : const grpc.ChannelCredentials.insecure(),
    );
    final channel = grpc.ClientChannel(host, port: port, options: options);
    _channels[key] = channel;
    return channel;
  }

  static Future<void> shutdownAll() async {
    final channels = _channels.values.toList(growable: false);
    _channels.clear();
    for (final ch in channels) {
      await ch.shutdown();
    }
  }
}
