
class BusinessProfileAddress {
  final int? id;
  final String? uuid;
  final int businessProfileId;
  final String addressLine1;
  final String? addressLine2;
  final String locality;
  final String administrativeArea;
  final String postalCode;
  final String countryCode;
  final double? latitude;
  final double? longitude;
  final String? createdAt;
  final String? updatedAt;

  const BusinessProfileAddress({
    this.id,
    this.uuid,
    required this.businessProfileId,
    required this.addressLine1,
    this.addressLine2,
    required this.locality,
    required this.administrativeArea,
    required this.postalCode,
    required this.countryCode,
    this.latitude,
    this.longitude,
    this.createdAt,
    this.updatedAt,
  });

  factory BusinessProfileAddress.fromJson(Map<String, dynamic> json) {
    return BusinessProfileAddress(
      id: json['id'] as int?,
      uuid: json['uuid'] as String?,
      businessProfileId: json['businessProfileId'] as int,
      addressLine1: json['addressLine1'] as String,
      addressLine2: json['addressLine2'] as String?,
      locality: json['locality'] as String,
      administrativeArea: json['administrativeArea'] as String,
      postalCode: json['postalCode'] as String,
      countryCode: json['countryCode'] as String,
      latitude: (json['latitude'] as num?)?.toDouble(),
      longitude: (json['longitude'] as num?)?.toDouble(),
      createdAt: json['createdAt'] as String?,
      updatedAt: json['updatedAt'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'uuid': uuid,
      'businessProfileId': businessProfileId,
      'addressLine1': addressLine1,
      'addressLine2': addressLine2,
      'locality': locality,
      'administrativeArea': administrativeArea,
      'postalCode': postalCode,
      'countryCode': countryCode,
      'latitude': latitude,
      'longitude': longitude,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}