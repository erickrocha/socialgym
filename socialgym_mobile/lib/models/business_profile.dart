import 'business_profile_address.dart';

class BusinessProfile {
  final int? id;
  final String? uuid;
  final int ownerId;
  final String ownerUuid;
  final String taxId;
  final String businessName;
  final String businessType;
  final String? socialName;
  final String? logo;
  final String? coverImage;
  final String? createdAt;
  final String? updatedAt;
  final List<BusinessProfileAddress> addresses;

  const BusinessProfile({
    this.id,
    this.uuid,
    required this.ownerId,
    required this.ownerUuid,
    required this.taxId,
    required this.businessName,
    required this.businessType,
    this.socialName,
    this.logo,
    this.coverImage,
    this.createdAt,
    this.updatedAt,
    this.addresses = const [],
  });

  factory BusinessProfile.fromJson(Map<String, dynamic> json) {
    return BusinessProfile(
      id: json['id'] as int?,
      uuid: json['uuid'] as String?,
      ownerId: json['ownerId'] as int,
      ownerUuid: json['ownerUuid'] as String,
      taxId: json['taxId'] as String,
      businessName: json['businessName'] as String,
      businessType: json['businessType'] as String,
      socialName: json['socialName'] as String?,
      logo: json['logo'] as String?,
      coverImage: json['coverImage'] as String?,
      createdAt: json['createdAt'] as String?,
      updatedAt: json['updatedAt'] as String?,
      addresses:
          (json['addresses'] as List<dynamic>?)
              ?.map(
                (e) =>
                    BusinessProfileAddress.fromJson(e as Map<String, dynamic>),
              )
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'uuid': uuid,
      'ownerId': ownerId,
      'ownerUuid': ownerUuid,
      'taxId': taxId,
      'businessName': businessName,
      'businessType': businessType,
      'socialName': socialName,
      'logo': logo,
      'coverImage': coverImage,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
      'addresses': addresses.map((e) => e.toJson()).toList(),
    };
  }
}
