import '../../models/workout_session.dart';
import '../../src/generated/grpc/timeline/workout_session.pbgrpc.dart' as $session;
import 'grpc_timeline.dart';

/// Workout sessions over gRPC, replacing the REST `WorkoutSessionService`. A session belongs to
/// the person in the token, whatever the payload says.
class GrpcWorkoutSessionService {
  GrpcWorkoutSessionService._();

  static $session.WorkoutSessionServiceClient get _client => $session.WorkoutSessionServiceClient(
        GrpcTimeline.channel,
        interceptors: GrpcTimeline.interceptors,
      );

  static Future<Map<String, dynamic>> saveWorkoutSession(Map<String, dynamic> sessionData) =>
      GrpcTimeline.run(() async {
        final session = GrpcTimeline.fill($session.WorkoutSession(), sessionData);
        final saved = await _client.createWorkoutSession(
          $session.CreateWorkoutSessionRequest(session: session),
          options: GrpcTimeline.options,
        );
        return GrpcTimeline.json(saved);
      }, 'Failed to save workout session');

  static Future<List<WorkoutSession>> fetchWorkoutSessions({
    required DateTime startDate,
    required DateTime endDate,
  }) => GrpcTimeline.run(() async {
        final response = await _client.listWorkoutSessions(
          $session.ListWorkoutSessionsRequest(
            startDate: _toIsoDateTime(startDate, endOfDay: false),
            endDate: _toIsoDateTime(endDate, endOfDay: true),
          ),
          options: GrpcTimeline.options,
        );
        return response.sessions.map((s) => WorkoutSession.fromJson(GrpcTimeline.json(s))).toList();
      }, 'Failed to fetch workout sessions');

  static String _toIsoDateTime(DateTime date, {bool endOfDay = false}) {
    final y = date.year.toString().padLeft(4, '0');
    final m = date.month.toString().padLeft(2, '0');
    final d = date.day.toString().padLeft(2, '0');
    final time = endOfDay ? 'T23:59:59' : 'T00:00:00';
    return '$y-$m-$d$time';
  }
}
