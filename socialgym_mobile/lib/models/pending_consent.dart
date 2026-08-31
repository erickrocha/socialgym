/// A legal document whose current version the signed-in person has not
/// accepted. Returned by `GET /workout/api/people/me/consents/pending`.
class PendingConsent {
  final String document;
  final String currentVersion;

  /// The most recent version the person previously accepted, if any.
  /// `null` means they never accepted this document at all.
  final String? acceptedVersion;

  const PendingConsent({
    required this.document,
    required this.currentVersion,
    this.acceptedVersion,
  });

  factory PendingConsent.fromJson(Map<String, dynamic> json) => PendingConsent(
    document: json['document'] as String,
    currentVersion: json['currentVersion'] as String,
    acceptedVersion: json['acceptedVersion'] as String?,
  );

  bool get isFirstTime => acceptedVersion == null;
}
