import 'package:grpc/grpc.dart' as grpc;

/// An in-process gRPC server on a free port plus a client channel to it, for the service façade tests.
class FakeGrpcServer {
  FakeGrpcServer._(this._server, this.channel);

  final grpc.Server _server;
  final grpc.ClientChannel channel;

  static Future<FakeGrpcServer> start(List<grpc.Service> services) async {
    final server = grpc.Server.create(services: services);
    await server.serve(address: 'localhost', port: 0);
    final channel = grpc.ClientChannel(
      'localhost',
      port: server.port!,
      options: const grpc.ChannelOptions(credentials: grpc.ChannelCredentials.insecure()),
    );
    return FakeGrpcServer._(server, channel);
  }

  Future<void> stop() async {
    await channel.shutdown();
    await _server.shutdown();
  }
}
