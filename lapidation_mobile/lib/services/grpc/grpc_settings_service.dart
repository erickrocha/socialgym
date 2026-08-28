import 'package:grpc/grpc.dart' as grpc;
import 'package:lapidation_mobile/commons/settings_mapper.dart';
import 'package:lapidation_mobile/models/enums.dart';
import 'package:lapidation_mobile/models/settings.dart';
import 'package:lapidation_mobile/services/base_service.dart';
import 'package:lapidation_mobile/services/grpc/grpc_channel_factory.dart';

import '../../config/api_config.dart';
import '../../src/generated/grpc/settings.pbgrpc.dart' as $settings;

/// gRPC client façade for the SettingsService.
///
/// Mirrors [GrpcPersonService]'s shape: a lazily-created generated client,
/// plus domain<->proto mapping kept next to the calls that need it.
class GrpcSettingsService {
  GrpcSettingsService._();

  static $settings.SettingsServiceClient? _client;

  /// Fetches the settings row for the given person.
  ///
  /// Returns `null` if the person has no settings row yet (fresh account) —
  /// callers should treat that as "use defaults", not as an error.
  static Future<Settings?> getByOwnerId({
    required int ownerId,
    String ownerUuid = '',
  }) async {
    try {
      final response = await _ensureClient().getByOwnerIds(
        $settings.SettingOwnerIdRequest(ownerId: ownerId, ownerUuid: ownerUuid),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return SettingsMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      if (e.code == grpc.StatusCode.notFound) return null;
      throw BaseService.handleGrpcError(e, 'Failed to load settings');
    }
  }

  /// Persists (inserts or updates) the given settings and returns the
  /// server's copy. Callers must pass through the previously-loaded
  /// `id`/`uuid` (if any) or the backend will insert a duplicate row.
  static Future<Settings> persistSettings(Settings settings) async {
    try {
      final response = await _ensureClient().persistSettings(
        SettingsMapper().toProto(settings),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return SettingsMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to save settings');
    }
  }

  /// The backend's `Position::from_string`
  /// (workout/business/src/domain/enums.rs) only matches capitalized values
  /// ("Left", "Top", "Right", "Bottom") and silently falls back to `Left`
  /// for anything else. [ContextMenuPosition]'s canonical Dart
  /// representation stays lowercase; this capitalizes only for the wire so
  /// the backend actually stores the chosen value.
  static String positionToWire(ContextMenuPosition position) {
    final name = position.toStringValue();
    return name[0].toUpperCase() + name.substring(1);
  }

  static $settings.SettingsServiceClient _ensureClient() {
    if (_client != null) return _client!;

    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $settings.SettingsServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  /// Closes the underlying gRPC client reference. Call on app shutdown.
  static Future<void> shutdown() async {
    _client = null;
  }
}
