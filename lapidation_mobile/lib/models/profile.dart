

import '../src/generated/grpc/profile.pb.dart' as $profile;

class Profile {
  final int id;
  final String uuid;
  final int personId;
  final String personUuid;
  final int businessProfileId;
  final String businessProfileUuid;

  Profile({
    required this.id,
    required this.uuid,
    required this.personId,
    required this.personUuid,
    required this.businessProfileId,
    required this.businessProfileUuid,

  });

  factory Profile.fromProto($profile.Profile proto) {
    return Profile(
      id: proto.id,
      uuid: proto.uuid,
      personId: proto.personId,
      personUuid: proto.personUuid,
      businessProfileId: proto.businessProfileId,
      businessProfileUuid: proto.businessProfileUuid,
    );
  }

  factory Profile.fromJson(Map<String, dynamic> json) {
    return Profile(
      id: json['id'] as int,
      uuid: json['uuid'] as String,
      personId: json['personId'] as int,
      personUuid: json['personUuid'] as String,
      businessProfileId: json['businessProfileId'] as int,
      businessProfileUuid: json['businessProfileUuid'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'uuid': uuid,
      'personId': personId,
      'personUuid': personUuid,
      'businessProfileId': businessProfileId,
      'businessProfileUuid': businessProfileUuid,
    };
  }
}
