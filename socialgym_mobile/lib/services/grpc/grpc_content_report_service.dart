import '../../src/generated/grpc/timeline/content_report.pbgrpc.dart' as $report;
import 'grpc_timeline.dart';

/// Content reports over gRPC, replacing the REST `ContentReportService`.
class GrpcContentReportService {
  GrpcContentReportService._();

  static $report.ContentReportServiceClient get _client => $report.ContentReportServiceClient(
        GrpcTimeline.channel,
        interceptors: GrpcTimeline.interceptors,
      );

  static Future<void> create({
    required String targetType,
    required String targetId,
    required String postId,
    required String reason,
    String? details,
  }) => GrpcTimeline.run(() async {
        await _client.createReport(
          $report.CreateReportRequest(
            targetType: targetType,
            targetId: targetId,
            postId: postId,
            reason: reason,
            details: details?.isNotEmpty == true ? details : null,
          ),
          options: GrpcTimeline.options,
        );
      }, 'Could not submit report');
}
