import 'package:socialgym_mobile/models/business_profile.dart';

class PersonInfo {
  final int id;
  final int personId;
  final String? biography;
  final String? relationship;
  final String? job;
  final String? homeTown;
  final String? currentCity;
  final double? weight;
  final double? height;
  final String uuid;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  PersonInfo({
    required this.id,
    required this.uuid,
    required this.personId,
    this.biography,
    this.relationship,
    this.job,
    this.homeTown,
    this.currentCity,
    this.weight,
    this.height,
    this.createdAt,
    this.updatedAt,
  });

  factory PersonInfo.fromJson(Map<String, dynamic> json) {
    return PersonInfo(
      id: json['id'] as int,
      uuid: json['uuid'] as String,
      personId: json['personId'] as int,
      biography: json['biography'] as String?,
      relationship: json['relationship'] as String?,
      job: json['job'] as String?,
      homeTown: json['homeTown'] as String?,
      currentCity: json['currentCity'] as String?,
      weight: json['weight'] != null
          ? (json['weight'] as num).toDouble()
          : null,
      height: json['height'] != null
          ? (json['height'] as num).toDouble()
          : null,
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: DateTime.parse(json['updatedAt'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'personId': personId,
      'biography': biography,
      'relationship': relationship,
      'job': job,
      'homeTown': homeTown,
      'currentCity': currentCity,
      'weight': weight,
      'height': height,
      'uuid': uuid,
      'createdAt': createdAt?.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
    };
  }
}

class PersonAddress {
  final int? id;
  final int? personId;
  final String? addressLine1;
  final String? addressLine2;
  final String? locality;
  final String? administrativeArea;
  final String? countryCode;
  final String? postalCode;
  final double? latitude;
  final double? longitude;
  final bool current;
  final String? uuid;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  PersonAddress({
    this.id,
    this.uuid,
    this.personId,
    this.addressLine1,
    this.addressLine2,
    this.locality,
    this.administrativeArea,
    this.countryCode,
    this.postalCode,
    this.latitude,
    this.longitude,
    this.current = false,
    this.createdAt,
    this.updatedAt,
  });

  factory PersonAddress.fromJson(Map<String, dynamic> json) {
    return PersonAddress(
      id: json['id'] as int?,
      personId: json['personId'] as int?,
      addressLine1: json['addressLine1'] as String?,
      addressLine2: json['addressLine2'] as String?,
      locality: json['locality'] as String?,
      administrativeArea: json['administrativeArea'] as String?,
      countryCode: json['countryCode'] as String?,
      postalCode: json['postalCode'] as String?,
      latitude: json['latitude'] != null
          ? (json['latitude'] as num).toDouble()
          : null,
      longitude: json['longitude'] != null
          ? (json['longitude'] as num).toDouble()
          : null,
      current: json['current'] as bool? ?? false,
      uuid: json['uuid'] as String?,
      createdAt: json['createdAt'] != null
          ? DateTime.parse(json['createdAt'] as String)
          : null,
      updatedAt: json['updatedAt'] != null
          ? DateTime.parse(json['updatedAt'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'personId': personId,
      'addressLine1': addressLine1,
      'addressLine2': addressLine2,
      'locality': locality,
      'administrativeArea': administrativeArea,
      'countryCode': countryCode,
      'postalCode': postalCode,
      'latitude': latitude,
      'longitude': longitude,
      'current': current,
      'uuid': uuid,
      'createdAt': createdAt?.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
    };
  }

  PersonAddress copyWith({
    int? id,
    int? personId,
    String? addressLine1,
    String? addressLine2,
    String? locality,
    String? administrativeArea,
    String? countryCode,
    String? postalCode,
    double? latitude,
    double? longitude,
    bool? current,
    String? uuid,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return PersonAddress(
      id: id ?? this.id,
      personId: personId ?? this.personId,
      addressLine1: addressLine1 ?? this.addressLine1,
      addressLine2: addressLine2 ?? this.addressLine2,
      locality: locality ?? this.locality,
      administrativeArea: administrativeArea ?? this.administrativeArea,
      countryCode: countryCode ?? this.countryCode,
      postalCode: postalCode ?? this.postalCode,
      latitude: latitude ?? this.latitude,
      longitude: longitude ?? this.longitude,
      current: current ?? this.current,
      uuid: uuid ?? this.uuid,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }

  String get formattedAddress {
    final parts = <String>[];

    // Address Line 1 (primary address)
    if (addressLine1 != null && addressLine1!.isNotEmpty) {
      parts.add(addressLine1!);
    }

    // Address Line 2 (secondary address info)
    if (addressLine2 != null && addressLine2!.isNotEmpty) {
      parts.add(addressLine2!);
    }

    // Locality (city)
    if (locality != null && locality!.isNotEmpty) {
      var localityPart = locality!;
      // Add administrative area (state/province) if available
      if (administrativeArea != null && administrativeArea!.isNotEmpty) {
        localityPart += ' - $administrativeArea';
      }
      parts.add(localityPart);
    }

    // Country code
    if (countryCode != null && countryCode!.isNotEmpty) {
      parts.add(countryCode!);
    }

    // Postal code
    if (postalCode != null && postalCode!.isNotEmpty) {
      parts.add(postalCode!);
    }

    return parts.join(', ');
  }
}

class Person {
  final int id;
  final String firstname;
  final String surname;
  final DateTime? dateOfBirth;
  final String? gender;
  final String? avatar;
  final String? objectKey;
  final String? cover;
  final PersonInfo? personInfo;
  final List<PersonAddress> addresses;
  final bool hasBusinessProfiles;
  final String uuid;
  final DateTime? createdAt;
  final DateTime? updatedAt;
  final List<BusinessProfile> businessProfiles;

  Person({
    required this.id,
    required this.uuid,
    required this.firstname,
    required this.surname,
    this.dateOfBirth,
    this.gender,
    this.avatar,
    this.objectKey,
    this.cover,
    this.personInfo,
    this.addresses = const [],
    this.hasBusinessProfiles = false,
    this.createdAt,
    this.updatedAt,
    this.businessProfiles = const [],
  });

  String get fullName => '$firstname $surname';

  factory Person.fromJson(Map<String, dynamic> json) {
    return Person(
      id: json['id'] as int,
      firstname: json['firstname'] as String,
      surname: json['surname'] as String,
      dateOfBirth: json['dateOfBirth'] != null
          ? DateTime.parse(json['dateOfBirth'] as String)
          : null,
      gender: json['gender'] as String?,
      avatar: json['avatar'] as String?,
      objectKey: json['objectKey'] as String?,
      cover: json['cover'] as String?,
      personInfo: json['personInfo'] != null
          ? PersonInfo.fromJson(json['personInfo'] as Map<String, dynamic>)
          : null,
      addresses:
          (json['addresses'] as List<dynamic>?)
              ?.map((e) => PersonAddress.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      hasBusinessProfiles: json['hasBusinessProfiles'] as bool? ?? false,
      uuid: json['uuid'] as String,
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: DateTime.parse(json['updatedAt'] as String),
      businessProfiles:
          (json['businessProfiles'] as List<dynamic>?)
              ?.map((e) => BusinessProfile.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'firstname': firstname,
      'surname': surname,
      'dateOfBirth': dateOfBirth?.toIso8601String().split('T').first,
      'gender': gender,
      'avatar': avatar,
      'objectKey': objectKey,
      'cover': cover,
      'personInfo': personInfo?.toJson(),
      'addresses': addresses.map((e) => e.toJson()).toList(),
      'hasBusinessProfiles': hasBusinessProfiles,
      'uuid': uuid,
      'createdAt': createdAt?.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
      'businessProfiles': businessProfiles.map((e) => e.toJson()).toList(),
    };
  }
}
