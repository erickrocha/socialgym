import '../../models/evolution_check_in.dart';
import '../../src/generated/grpc/timeline/evolution.pbgrpc.dart' as $evolution;
import 'grpc_timeline.dart';

/// Evolution check-ins over gRPC, replacing the REST `EvolutionService`. The check-in always
/// belongs to the person in the token.
class GrpcEvolutionService {
  GrpcEvolutionService._();

  static $evolution.EvolutionCheckInServiceClient get _client => $evolution.EvolutionCheckInServiceClient(
        GrpcTimeline.channel,
        interceptors: GrpcTimeline.interceptors,
      );

  /// Check-ins between [startDate] and [endDate] (ISO 8601 on the wire).
  static Future<List<EvolutionCheckIn>> fetchEvolutionCheckIns({
    required DateTime startDate,
    required DateTime endDate,
  }) => GrpcTimeline.run(() async {
        final response = await _client.listEvolutionCheckIns(
          $evolution.ListEvolutionCheckInsRequest(
            startDate: startDate.toIso8601String(),
            endDate: endDate.toIso8601String(),
          ),
          options: GrpcTimeline.options,
        );
        return response.checkIns.map((c) => EvolutionCheckIn.fromJson(GrpcTimeline.json(c))).toList();
      }, 'Failed to load evolution data');

  static Future<EvolutionCheckIn> createEvolutionCheckIn({required Map<String, dynamic> payload}) =>
      GrpcTimeline.run(() async {
        final request = GrpcTimeline.fill($evolution.AddEvolutionCheckInRequest(), payload);
        final saved = await _client.addEvolutionCheckIn(request, options: GrpcTimeline.options);
        return EvolutionCheckIn.fromJson(GrpcTimeline.json(saved));
      }, 'Failed to create evolution check-in');
}
