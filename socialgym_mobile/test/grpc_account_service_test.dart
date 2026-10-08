import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_account_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/account.pbgrpc.dart' as $account;

import 'utils/fake_grpc_server.dart';

class _Account extends $account.AccountServiceBase {
  bool? immediate;
  bool cancelled = false;

  @override
  Future<$account.AccountDeletionStatus> requestAccountDeletion(grpc.ServiceCall call, $account.RequestAccountDeletionRequest request) async {
    immediate = request.immediate;
    return $account.AccountDeletionStatus(requestedAt: '2026-10-01T10:00:00Z', scheduledAt: '2026-10-31T10:00:00Z');
  }

  @override
  Future<$account.CancelAccountDeletionResponse> cancelAccountDeletion(grpc.ServiceCall call, $account.CancelAccountDeletionRequest request) async {
    cancelled = true;
    return $account.CancelAccountDeletionResponse();
  }

  @override
  Future<$account.DataExport> createDataExport(grpc.ServiceCall call, $account.CreateDataExportRequest request) async =>
      $account.DataExport(id: 'e1', status: 'pending', createdAt: '2026-10-01T10:00:00Z');

  @override
  Future<$account.ListDataExportsResponse> listDataExports(grpc.ServiceCall call, $account.ListDataExportsRequest request) async =>
      $account.ListDataExportsResponse(exports: [
        $account.DataExport(id: 'e1', status: 'ready', createdAt: '2026-10-01T10:00:00Z', expiresAt: '2026-10-08T10:00:00Z'),
        $account.DataExport(id: 'e2', status: 'failed', createdAt: '2026-10-02T10:00:00Z'),
      ]);

  @override
  Future<$account.DataExport> getDataExport(grpc.ServiceCall call, $account.GetDataExportRequest request) async =>
      throw grpc.GrpcError.unimplemented('unused');

  @override
  Future<$account.DataExportDownload> getDataExportDownloadUrl(grpc.ServiceCall call, $account.GetDataExportRequest request) async {
    if (request.id != 'e1') throw grpc.GrpcError.failedPrecondition('Export not ready');
    return $account.DataExportDownload(url: 'https://s3/e1.zip', expiresInSeconds: 300);
  }
}

void main() {
  late FakeGrpcServer server;
  final account = _Account();

  setUpAll(() async {
    server = await FakeGrpcServer.start([account]);
    GrpcAccountService.useClient($account.AccountServiceClient(server.channel));
  });
  tearDownAll(() => server.stop());

  test('requests deletion and parses the schedule', () async {
    final status = await GrpcAccountService.requestDeletion(immediate: true);
    expect(account.immediate, isTrue);
    expect(status.scheduledAt.difference(status.requestedAt).inDays, 30);
  });

  test('cancels a pending deletion', () async {
    await GrpcAccountService.cancelDeletion();
    expect(account.cancelled, isTrue);
  });

  test('lists and creates exports, with and without an expiry', () async {
    final jobs = await GrpcAccountService.listDataExports();
    expect(jobs.map((j) => (j.id, j.status, j.expiresAt != null)), [('e1', 'ready', true), ('e2', 'failed', false)]);
    expect((await GrpcAccountService.createDataExport()).status, 'pending');
  });

  test('the download URL comes back for a ready export and a 400 AppException otherwise', () async {
    expect(await GrpcAccountService.dataExportDownloadUrl('e1'), 'https://s3/e1.zip');
    await expectLater(
      GrpcAccountService.dataExportDownloadUrl('e2'),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 400)),
    );
  });
}
