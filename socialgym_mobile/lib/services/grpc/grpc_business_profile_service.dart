import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/commons/business_profile_mapper.dart';

import '../../config/api_config.dart';
import '../../models/business_profile.dart';
import '../../models/business_profile_address.dart';
import '../../src/generated/grpc/business_profile.pbgrpc.dart' as $bp;
import '../base_service.dart';
import 'grpc_channel_factory.dart';

class GrpcBusinessProfileService {
  GrpcBusinessProfileService._();

  static $bp.BusinessProfileServiceClient? _client;

  static Future<BusinessProfile> getBusinessProfileById({int? id, String? uuid}) async {
    try {
      final response = await _ensureClient().getBusinessProfileById(
        $bp.BusinessProfileRequestId(id: id ?? 0, uuid: uuid ?? ''),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return BusinessProfileMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load business profile');
    }
  }

  static Future<List<BusinessProfile>> getBusinessProfileByOwnerId({int? ownerId, String? ownerUuid}) async {
    try {
      final response = await _ensureClient().getBusinessProfileByOwnerId(
        $bp.BusinessProfileRequestOwnerId(ownerId: ownerId ?? 0, ownerUuid: ownerUuid ?? ''),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return BusinessProfileMapper().fromProtoList(response.businessProfiles);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load business profiles');
    }
  }

  static Future<BusinessProfile> addBusinessProfile(BusinessProfile profile) async {
    try {
      final response = await _ensureClient().addBusinessProfile(
        BusinessProfileMapper().toProto(profile),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return BusinessProfileMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to create business profile');
    }
  }

  static Future<BusinessProfile> updateBusinessProfile(BusinessProfile profile) async {
    try {
      final response = await _ensureClient().updateBusinessProfile(
          BusinessProfileMapper().toProto(profile),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return BusinessProfileMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to update business profile');
    }
  }

  static Future<BusinessProfileAddress> addBusinessProfileAddress(BusinessProfileAddress address) async {
    try {
      final response = await _ensureClient().addBusinessProfileAddress(
        BusinessProfileAddressMapper().toProto(address),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return BusinessProfileAddressMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to add business profile address');
    }
  }

  static Future<BusinessProfileAddress> updateBusinessProfileAddress(BusinessProfileAddress address) async {
    try {
      final response = await _ensureClient().updateBusinessProfileAddress(
        BusinessProfileAddressMapper().toProto(address),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return BusinessProfileAddressMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to update business profile address');
    }
  }

  static Future<bool> removeBusinessProfileAddress({int? id, String? uuid}) async {
    try {
      final response = await _ensureClient().removeBusinessProfileAddress(
        $bp.RemoveBusinessProfileAddressRequest(id: id ?? 0, uuid: uuid ?? ''),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return response.success;
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to remove business profile address');
    }
  }

  static $bp.BusinessProfileServiceClient _ensureClient() {
    if (_client != null) return _client!;
    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $bp.BusinessProfileServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  static Future<void> shutdown() async {
    _client = null;
  }
}
