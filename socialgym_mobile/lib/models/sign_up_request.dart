class SignUpRequest {
  final String firstname;
  final String surname;
  final String email;
  final String password;
  final String dateOfBirth;
  final String gender;

  SignUpRequest({
    required this.firstname,
    required this.surname,
    required this.email,
    required this.password,
    required this.dateOfBirth,
    required this.gender,
  });

  Map<String, dynamic> toJson() {
    return {
      'firstname': firstname,
      'surname': surname,
      'email': email,
      'password': password,
      'dateOfBirth': dateOfBirth,
      'gender': gender,
    };
  }
}
