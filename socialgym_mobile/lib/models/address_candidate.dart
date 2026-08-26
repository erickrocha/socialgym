class AddressCandidate {
  final String placeId;
  final String formattedAddress;
  final String addressLine1;
  final String? addressLine2;
  final String locality;
  final String administrativeArea;
  final String administrativeAreaCode;
  final String? postalCode;
  final String countryCode;
  final double latitude;
  final double longitude;

  AddressCandidate({
    required this.placeId,
    required this.formattedAddress,
    required this.addressLine1,
    this.addressLine2,
    required this.locality,
    required this.administrativeArea,
    required this.administrativeAreaCode,
    this.postalCode,
    required this.countryCode,
    required this.latitude,
    required this.longitude,
  });

  factory AddressCandidate.fromJson(Map<String, dynamic> json) {
    return AddressCandidate(
      placeId: json['placeId'] as String,
      formattedAddress: json['formattedAddress'] as String,
      addressLine1: json['addressLine1'] as String,
      addressLine2: json['addressLine2'] as String?,
      locality: json['locality'] as String,
      administrativeArea: json['administrativeArea'] as String,
      administrativeAreaCode: json['administrativeAreaCode'] as String,
      postalCode: json['postalCode'] as String?,
      countryCode: json['countryCode'] as String,
      latitude: (json['latitude'] as num).toDouble(),
      longitude: (json['longitude'] as num).toDouble(),
    );
  }

  @override
  String toString() => formattedAddress;
}
