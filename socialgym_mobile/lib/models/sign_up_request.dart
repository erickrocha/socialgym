class SignUpRequest {
  final String firstname;
  final String surname;
  final String email;
  final String password;
  final String dateOfBirth;
  final String gender;
  final String termsVersion;
  final String privacyVersion;
  final bool termsAccepted;
  final bool privacyAccepted;

  SignUpRequest({
    required this.firstname,
    required this.surname,
    required this.email,
    required this.password,
    required this.dateOfBirth,
    required this.gender,
    required this.termsVersion,
    required this.privacyVersion,
    required this.termsAccepted,
    required this.privacyAccepted,
  });

  Map<String, dynamic> toJson() {
    return {
      'firstname': firstname,
      'surname': surname,
      'email': email,
      'password': password,
      'dateOfBirth': dateOfBirth,
      'gender': gender,
      'termsVersion': termsVersion,
      'privacyVersion': privacyVersion,
      'termsAccepted': termsAccepted,
      'privacyAccepted': privacyAccepted,
    };
  }
}
