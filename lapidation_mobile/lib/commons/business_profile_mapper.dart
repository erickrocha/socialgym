
import 'package:lapidation_mobile/commons/mapper.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/models/business_profile_address.dart';

import '../src/generated/grpc/business_profile.pb.dart' as $business_profile;
import '../src/generated/grpc/business_profile_address.pb.dart' as $business_profile_address;

class BusinessProfileMapper implements Mapper<BusinessProfile,$business_profile.BusinessProfile>{
  @override
  BusinessProfile fromProto($business_profile.BusinessProfile proto) {
    return BusinessProfile(
      id: proto.id != 0 ? proto.id : null,
      uuid: proto.uuid.isNotEmpty ? proto.uuid : null,
      ownerId: proto.ownerId,
      ownerUuid: proto.ownerUuid,
      taxId: proto.taxId,
      businessName: proto.businessName,
      businessType: proto.businessType,
      socialName: proto.socialName.isNotEmpty ? proto.socialName : null,
      logo: proto.logo.isNotEmpty ? proto.logo : null,
      coverImage: proto.coverImage.isNotEmpty ? proto.coverImage : null,
      createdAt: proto.createdAt.isNotEmpty ? proto.createdAt : null,
      updatedAt: proto.updatedAt.isNotEmpty ? proto.updatedAt : null,
      addresses: BusinessProfileAddressMapper().fromProtoList(proto.addresses),
    );
  }

  @override
  List<BusinessProfile> fromProtoList(List<$business_profile.BusinessProfile> protoList) {
    return protoList.map(fromProto).toList();
  }

  @override
  $business_profile.BusinessProfile toProto(BusinessProfile domain) {
    return $business_profile.BusinessProfile(
      id: domain.id ?? 0,
      uuid: domain.uuid ?? '',
      ownerId: domain.ownerId,
      ownerUuid: domain.ownerUuid,
      taxId: domain.taxId,
      businessName: domain.businessName,
      businessType: domain.businessType,
      socialName: domain.socialName ?? '',
      logo: domain.logo ?? '',
      coverImage: domain.coverImage ?? '',
      createdAt: domain.createdAt ?? '',
      updatedAt: domain.updatedAt ?? '',
      addresses: BusinessProfileAddressMapper().toProtoList(domain.addresses),
    );
  }

  @override
  List<$business_profile.BusinessProfile> toProtoList(List<BusinessProfile> domainList) {
    return domainList.map(toProto).toList();
  }
  
}

class BusinessProfileAddressMapper implements Mapper<BusinessProfileAddress,$business_profile_address.BusinessProfileAddress> {
  @override
  BusinessProfileAddress fromProto($business_profile_address.BusinessProfileAddress proto) {
    return BusinessProfileAddress(
      id: proto.id != 0 ? proto.id : null,
      uuid: proto.uuid.isNotEmpty ? proto.uuid : null,
      businessProfileId: proto.businessProfileId,
      addressLine1: proto.addressLine1,
      addressLine2: proto.addressLine2.isNotEmpty ? proto.addressLine2 : null,
      locality: proto.locality,
      administrativeArea: proto.administrativeArea,
      postalCode: proto.postalCode,
      countryCode: proto.countryCode,
      latitude: proto.latitude != 0 ? proto.latitude : null,
      longitude: proto.longitude != 0 ? proto.longitude : null,
      createdAt: proto.createdAt.isNotEmpty ? proto.createdAt : null,
      updatedAt: proto.updatedAt.isNotEmpty ? proto.updatedAt : null,
    );
  }

  @override
  List<BusinessProfileAddress> fromProtoList(List<$business_profile_address.BusinessProfileAddress> protoList) {
    return protoList.map(fromProto).toList();
  }

  @override
  $business_profile_address.BusinessProfileAddress toProto(BusinessProfileAddress domain) {
    return $business_profile_address.BusinessProfileAddress(
      id: domain.id ?? 0,
      uuid: domain.uuid ?? '',
      businessProfileId: domain.businessProfileId,
      addressLine1: domain.addressLine1,
      addressLine2: domain.addressLine2 ?? '',
      locality: domain.locality,
      administrativeArea: domain.administrativeArea,
      postalCode: domain.postalCode,
      countryCode: domain.countryCode,
      latitude: domain.latitude ?? 0,
      longitude: domain.longitude ?? 0,
      createdAt: domain.createdAt ?? '',
      updatedAt: domain.updatedAt ?? '',
    );
  }

  @override
  List<$business_profile_address.BusinessProfileAddress> toProtoList(List<BusinessProfileAddress> domainList) {
    return domainList.map(toProto).toList();
  }
  
}