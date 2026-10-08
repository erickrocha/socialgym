import 'package:socialgym_mobile/models/address_candidate.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_channel.dart';
import 'package:socialgym_mobile/src/generated/grpc/address.pbgrpc.dart' as $address;

/// gRPC client façade for the AddressSearchService (places autocomplete, needs an access token).
class GrpcAddressSearchService {
  GrpcAddressSearchService._();

  static $address.AddressSearchServiceClient? _override;

  /// Built per call on the shared channel: sign-out shuts the channels down, and a cached client
  /// would keep pointing at the dead one.
  static $address.AddressSearchServiceClient _ensureClient() =>
      _override ?? $address.AddressSearchServiceClient(GrpcWorkoutChannel.channel(), interceptors: GrpcWorkoutChannel.interceptors);

  /// Replaces the client (tests point it at a fake server).
  static void useClient($address.AddressSearchServiceClient? client) => _override = client;

  static Future<List<AddressCandidate>> search({
    required String text,
    double? latitude,
    double? longitude,
  }) => GrpcWorkoutChannel.guard('Failed to search address', () async {
    final response = await _ensureClient().searchAddress(
      $address.SearchAddressRequest(text: text, latitude: latitude, longitude: longitude),
      options: GrpcWorkoutChannel.options,
    );
    return response.candidates
        .map(
          (c) => AddressCandidate(
            placeId: c.placeId,
            formattedAddress: c.formattedAddress,
            addressLine1: c.addressLine1,
            addressLine2: c.hasAddressLine2() ? c.addressLine2 : null,
            locality: c.locality,
            administrativeArea: c.administrativeArea,
            administrativeAreaCode: c.administrativeAreaCode,
            postalCode: c.hasPostalCode() ? c.postalCode : null,
            countryCode: c.countryCode,
            latitude: c.latitude,
            longitude: c.longitude,
          ),
        )
        .toList();
  });
}
