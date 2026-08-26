

import 'package:socialgym_mobile/commons/business_profile_mapper.dart';
import 'package:socialgym_mobile/commons/mapper.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/src/generated/grpc/person.pbgrpc.dart' as $person;
import 'package:socialgym_mobile/src/generated/grpc/person_info.pb.dart' as $person_info;
import 'package:socialgym_mobile/src/generated/grpc/person_address.pb.dart' as $person_address;

class PersonMapper implements Mapper<Person,$person.Person>{
  @override
  Person fromProto($person.Person proto) {
    return Person(
      id: proto.id,
      uuid: proto.uuid,
      firstname: proto.firstname,
      surname: proto.surname,
      dateOfBirth: proto.hasDateOfBirth() && proto.dateOfBirth.isNotEmpty
          ? DateTime.tryParse(proto.dateOfBirth)
          : null,
      avatar: proto.hasAvatar() && proto.avatar.isNotEmpty
          ? proto.avatar
          : null,
      objectKey: proto.hasObjectKey() && proto.objectKey.isNotEmpty
          ? proto.objectKey
          : null,
      cover: proto.hasCover() && proto.cover.isNotEmpty ? proto.cover : null,
      gender: proto.gender.isNotEmpty ? proto.gender : null,
      personInfo: proto.hasPersonInfo()
          ? PersonInfoMapper().fromProto(proto.personInfo)
          : null,
      addresses: proto.addresses
          .map((a) => PersonAddressMapper().fromProto(a))
          .toList(growable: false),
      createdAt: proto.createdAt.isNotEmpty
          ? DateTime.tryParse(proto.createdAt)
          : null,
      updatedAt: proto.updatedAt.isNotEmpty
          ? DateTime.tryParse(proto.updatedAt)
          : null,
      businessProfiles: proto.businessProfiles
          .map((p) => BusinessProfileMapper().fromProto(p))
          .toList(growable: false),
    );
  }

  @override
  List<Person> fromProtoList(List<$person.Person> protoList) {
    return protoList.map(fromProto).toList();
  }

  @override
  $person.Person toProto(Person domain) {
    return $person.Person(
      id: domain.id,
      uuid: domain.uuid,
      firstname: domain.firstname,
      surname: domain.surname,
      dateOfBirth: domain.dateOfBirth?.toIso8601String() ?? '',
      avatar: domain.avatar ?? '',
      objectKey: domain.objectKey ?? '',
      cover: domain.cover ?? '',
      gender: domain.gender ?? '',
      personInfo: domain.personInfo != null
          ? PersonInfoMapper().toProto(domain.personInfo!)
          : null,
      addresses: PersonAddressMapper().toProtoList(domain.addresses),
      createdAt: domain.createdAt?.toIso8601String() ?? '',
      updatedAt: domain.updatedAt?.toIso8601String() ?? '',
      businessProfiles: BusinessProfileMapper().toProtoList(domain.businessProfiles),
    );
  }

  @override
  List<$person.Person> toProtoList(List<Person> domainList) {
    return domainList.map(toProto).toList();
  }
}

class PersonInfoMapper implements Mapper<PersonInfo,$person_info.PersonInfo>{
  @override
  PersonInfo fromProto($person_info.PersonInfo proto) {
    return PersonInfo(
      id: proto.id,
      uuid: proto.uuid,
      personId: proto.personId,
      biography: proto.biography,
      relationship: proto.relationship,
      job: proto.job,
      homeTown: proto.homeTown,
      currentCity: proto.currentCity,
      weight: proto.weight,
      height: proto.height,
      createdAt: proto.hasCreatedAt() ? DateTime.tryParse(proto.createdAt) : null,
      updatedAt: proto.hasUpdatedAt() ? DateTime.tryParse(proto.updatedAt) : null,
    );
  }

  @override
  List<PersonInfo> fromProtoList(List<$person_info.PersonInfo> protoList) {
    return protoList.map(fromProto).toList();
  }

  @override
  $person_info.PersonInfo toProto(PersonInfo domain) {
    return $person_info.PersonInfo(
      id: domain.id,
      uuid: domain.uuid,
      personId: domain.personId,
      biography: domain.biography,
      relationship: domain.relationship,
      job: domain.job,
      homeTown: domain.homeTown,
      currentCity: domain.currentCity,
      weight: domain.weight,
      height: domain.height,
      createdAt: domain.createdAt?.toIso8601String(),
      updatedAt: domain.updatedAt?.toIso8601String(),
    );
  }

  @override
  List<$person_info.PersonInfo> toProtoList(List<PersonInfo> domainList) {
    return domainList.map(toProto).toList();
  }
}

class PersonAddressMapper implements Mapper<PersonAddress,$person_address.PersonAddress>{
  @override
  PersonAddress fromProto($person_address.PersonAddress proto) {
    return PersonAddress(
      id: proto.id,
      personId: proto.personId,
      addressLine1: proto.addressLine1,
      addressLine2: proto.addressLine2,
      locality: proto.locality,
      administrativeArea: proto.administrativeArea,
      countryCode: proto.countryCode,
      postalCode: proto.postalCode,
      latitude: proto.hasLatitude() ? proto.latitude : null,
      longitude: proto.hasLongitude() ? proto.longitude : null,
      current: proto.current,
      uuid: proto.uuid,
      createdAt: proto.hasCreatedAt()
          ? DateTime.tryParse(proto.createdAt)
          : null,
      updatedAt: proto.hasUpdatedAt()
          ? DateTime.tryParse(proto.updatedAt)
          : null,
    );
  }

  @override
  List<PersonAddress> fromProtoList(List<$person_address.PersonAddress> protoList) {
    return protoList.map(fromProto).toList();
  }

  @override
  $person_address.PersonAddress toProto(PersonAddress domain) {
    return $person_address.PersonAddress(
      id: domain.id,
      uuid: domain.uuid,
      personId: domain.personId,
      addressLine1: domain.addressLine1,
      addressLine2: domain.addressLine2,
      locality: domain.locality,
      administrativeArea: domain.administrativeArea,
      countryCode: domain.countryCode,
      postalCode: domain.postalCode,
      latitude: domain.latitude,
      longitude: domain.longitude,
      current: domain.current,
      createdAt: domain.createdAt?.toIso8601String(),
      updatedAt: domain.updatedAt?.toIso8601String(),
    );
  }

  @override
  List<$person_address.PersonAddress> toProtoList(List<PersonAddress> domainList) {
    return domainList.map(toProto).toList();
  }
}
