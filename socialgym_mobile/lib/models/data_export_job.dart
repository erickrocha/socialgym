class DataExportJob {
  final String id;
  final String status;
  final DateTime createdAt;
  final DateTime? expiresAt;

  const DataExportJob({
    required this.id,
    required this.status,
    required this.createdAt,
    this.expiresAt,
  });
}
