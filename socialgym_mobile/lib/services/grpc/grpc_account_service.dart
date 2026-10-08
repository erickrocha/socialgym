import 'package:socialgym_mobile/models/account_deletion_status.dart';
import 'package:socialgym_mobile/models/data_export_job.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_channel.dart';
import 'package:socialgym_mobile/src/generated/grpc/account.pbgrpc.dart' as $account;

/// gRPC client façade for the AccountService: account deletion (App Store / Play Store requirement)
/// and the personal-data exports.
class GrpcAccountService {
  GrpcAccountService._();

  static $account.AccountServiceClient? _override;

  /// Built per call on the shared channel: sign-out shuts the channels down, and a cached client
  /// would keep pointing at the dead one.
  static $account.AccountServiceClient _ensureClient() =>
      _override ?? $account.AccountServiceClient(GrpcWorkoutChannel.channel(), interceptors: GrpcWorkoutChannel.interceptors);

  /// Replaces the client (tests point it at a fake server).
  static void useClient($account.AccountServiceClient? client) => _override = client;

  /// Requests deletion, immediate or after the server's grace period.
  static Future<AccountDeletionStatus> requestDeletion({required bool immediate}) =>
      GrpcWorkoutChannel.guard('Failed to delete account', () async {
        final status = await _ensureClient().requestAccountDeletion(
          $account.RequestAccountDeletionRequest(immediate: immediate),
          options: GrpcWorkoutChannel.options,
        );
        return AccountDeletionStatus(
          requestedAt: DateTime.parse(status.requestedAt),
          scheduledAt: DateTime.parse(status.scheduledAt),
        );
      });

  static Future<void> cancelDeletion() => GrpcWorkoutChannel.guard('Failed to cancel account deletion', () async {
    await _ensureClient().cancelAccountDeletion(
      $account.CancelAccountDeletionRequest(),
      options: GrpcWorkoutChannel.options,
    );
  });

  static DataExportJob _job($account.DataExport e) => DataExportJob(
    id: e.id,
    status: e.status,
    createdAt: DateTime.parse(e.createdAt),
    expiresAt: e.hasExpiresAt() ? DateTime.parse(e.expiresAt) : null,
  );

  static Future<List<DataExportJob>> listDataExports() => GrpcWorkoutChannel.guard('Could not load exports', () async {
    final response = await _ensureClient().listDataExports(
      $account.ListDataExportsRequest(),
      options: GrpcWorkoutChannel.options,
    );
    return response.exports.map(_job).toList();
  });

  static Future<DataExportJob> createDataExport() => GrpcWorkoutChannel.guard('Could not create export', () async {
    final response = await _ensureClient().createDataExport(
      $account.CreateDataExportRequest(),
      options: GrpcWorkoutChannel.options,
    );
    return _job(response);
  });

  static Future<String> dataExportDownloadUrl(String id) => GrpcWorkoutChannel.guard('Export not ready', () async {
    final response = await _ensureClient().getDataExportDownloadUrl(
      $account.GetDataExportRequest(id: id),
      options: GrpcWorkoutChannel.options,
    );
    return response.url;
  });
}
