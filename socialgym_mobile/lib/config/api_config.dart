class ApiConfig {
  /// Base URL do backend. Passe `--dart-define=API_BASE_URL=https://seu.dominio`
  /// no build de release; o default abaixo é só para desenvolvimento local.
  /// Nunca deixe um host de túnel/dev como default de release — foi isso que
  /// causou a rejeição 5.6 da Apple em 03/09/2026.
  static const String baseUrlDefault = String.fromEnvironment(
    'API_BASE_URL',
    defaultValue: 'https://scrutiny-elevator-washstand.ngrok-free.dev',
  );

  /// gRPC usa o mesmo host do REST — uma fonte de verdade só.
  static String get grpcHost => Uri.parse(baseUrlDefault).host;
  static const int grpcPort = int.fromEnvironment('GRPC_PORT', defaultValue: 443);
  static const bool grpcUseTls = true;

  /// true apenas em dev contra backend local com certificado autoassinado
  /// (`--dart-define=GRPC_SELF_SIGNED=true`). Em release fica false e o cliente
  /// confia na cadeia TLS pública normal.
  static const bool grpcSelfSignedCert =
      bool.fromEnvironment('GRPC_SELF_SIGNED', defaultValue: false);

  // The workout and timeline APIs are gRPC only; REST stays for the web client.
 
  static const Duration timeout = Duration(seconds: 30);

  /// Returns the correct base URL depending on the current platform.
  static String get baseUrl {
    return baseUrlDefault;
  }

  static String? get grpcAuthority => grpcHost;

  /// Where the timeline gRPC services answer. Behind the gateway they share the workout gRPC host
  /// and port (the gateway routes `/grpc.timeline.*` to the timeline); against a stack that
  /// publishes the timeline directly, override with `--dart-define=TIMELINE_GRPC_HOST=...` and
  /// `--dart-define=TIMELINE_GRPC_PORT=...`.
  static const String _timelineGrpcHostOverride = String.fromEnvironment('TIMELINE_GRPC_HOST');
  static String get timelineGrpcHost =>
      _timelineGrpcHostOverride.isEmpty ? grpcHost : _timelineGrpcHostOverride;
  static const int timelineGrpcPort = int.fromEnvironment('TIMELINE_GRPC_PORT', defaultValue: grpcPort);

  /// TLS server name to verify; the certificate is issued for the gateway host even when the
  /// connection goes straight to the timeline (`--dart-define=TIMELINE_GRPC_AUTHORITY=...`).
  static const String _timelineGrpcAuthorityOverride = String.fromEnvironment('TIMELINE_GRPC_AUTHORITY');
  static String? get timelineGrpcAuthority =>
      _timelineGrpcAuthorityOverride.isEmpty ? grpcAuthority : _timelineGrpcAuthorityOverride;
}
