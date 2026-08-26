import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/commons/country_mapper.dart';
import 'package:socialgym_mobile/commons/province_mapper.dart';
import 'package:socialgym_mobile/commons/settings_mapper.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:socialgym_mobile/src/generated/grpc/resource.pbgrpc.dart'
    as $resource;

import '../../config/api_config.dart';
import '../../models/resource.dart';

class GrpcResourceService {
  GrpcResourceService._();

  static $resource.ResourceServiceClient? _client;

  static $resource.ResourceServiceClient _ensureClient() {
    if (_client != null) return _client!;

    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $resource.ResourceServiceClient(
      channel,
      interceptors: GrpcChannelFactory.interceptors,
    );
    return _client!;
  }

  /// Closes the underlying gRPC channel. Call this when the app is shutting
  /// down or when the channel is no longer needed.
  static Future<void> shutdown() async {
    _client = null;
  }

  static Future<AppResources> fetchResources({required int id}) async {
    try {
      final response = await _ensureClient().getResource(
        $resource.ResourceRequest()..userId = id,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      final countries = CountryMapper().fromProtoList(response.countries);
      final settings = SettingsMapper().fromProto(response.setting);
      final provinces = ProvinceMapper().fromProtoList(response.provinces);
      return AppResources(countries: countries, settings: settings, provinces: provinces);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load resources');
    }
  }
}
