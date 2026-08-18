class MentionableFriend {
  final int id;
  final String uuid;
  final String firstname;
  final String surname;
  final String? avatar;

  const MentionableFriend({
    required this.id,
    required this.uuid,
    required this.firstname,
    required this.surname,
    this.avatar,
  });

  String get fullName {
    final full = '$firstname $surname'.trim();
    return full.isEmpty ? firstname : full;
  }
}
