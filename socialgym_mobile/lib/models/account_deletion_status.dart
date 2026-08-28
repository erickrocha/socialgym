class AccountDeletionStatus {
  final DateTime requestedAt;
  final DateTime scheduledAt;

  AccountDeletionStatus({required this.requestedAt, required this.scheduledAt});

  factory AccountDeletionStatus.fromJson(Map<String, dynamic> json) {
    return AccountDeletionStatus(
      requestedAt: DateTime.parse(json['requestedAt'] as String),
      scheduledAt: DateTime.parse(json['scheduledAt'] as String),
    );
  }
}
