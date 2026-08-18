import 'package:image_picker/image_picker.dart';
import 'package:socialgym_mobile/services/grpc/grpc_person_service.dart';
import '../models/person.dart';
import 'upload_service.dart';

class PersonService {
  static Future<Person> fetchMe() async {
    return await GrpcPersonService.getMe();
  }

  static Future<Person> fetchPersonByUuid(String uuid) async {
    return await GrpcPersonService.getPerson(uuid: uuid);
  }

  static Future<Person> updatePerson(Person current, Map<String, dynamic> data) async {
    final domain = Person(
      id: current.id,
      uuid: current.uuid,
      firstname: data['firstname'] as String? ?? current.firstname,
      surname: data['surname'] as String? ?? current.surname,
      dateOfBirth: data['dateOfBirth'] != null
          ? DateTime.tryParse(data['dateOfBirth'] as String)
          : current.dateOfBirth,
      gender: data['gender'] as String? ?? current.gender,
      avatar: current.avatar,
      objectKey: current.objectKey,
      cover: current.cover,
      personInfo: current.personInfo,
      addresses: current.addresses,
      hasBusinessProfiles: current.hasBusinessProfiles,
      createdAt: current.createdAt,
      updatedAt: current.updatedAt,
      businessProfiles: current.businessProfiles,
    );
    return await GrpcPersonService.updatePerson(domain);
  }

  static Future<PersonInfo> updatePersonInfo(PersonInfo current, Map<String, dynamic> data) async {
    final domain = PersonInfo(
      id: current.id,
      uuid: current.uuid,
      personId: current.personId,
      biography: data.containsKey('biography') ? data['biography'] as String? : current.biography,
      relationship: data.containsKey('relationship') ? data['relationship'] as String? : current.relationship,
      job: data.containsKey('job') ? data['job'] as String? : current.job,
      homeTown: data.containsKey('homeTown') ? data['homeTown'] as String? : current.homeTown,
      currentCity: data.containsKey('currentCity') ? data['currentCity'] as String? : current.currentCity,
      weight: data.containsKey('weight') ? data['weight'] as double? : current.weight,
      height: data.containsKey('height') ? data['height'] as double? : current.height,
      createdAt: current.createdAt,
      updatedAt: current.updatedAt,
    );
    return await GrpcPersonService.updatePersonInfo(domain);
  }

  static Future<Person> uploadAvatar(XFile imageFile) async {
    await UploadService.uploadAvatar(imageFile);
    return await fetchMe();
  }

  static Future<Person> uploadCover(XFile imageFile) async {
    await UploadService.uploadCover(imageFile);
    return await fetchMe();
  }

  static Future<Person> removeAvatar() async {
    await GrpcPersonService.deletePersonImage(imageType: 'avatar');
    return await fetchMe();
  }

  static Future<Person> removeCover() async {
    await GrpcPersonService.deletePersonImage(imageType: 'cover');
    return await fetchMe();
  }

  static Future<PersonAddress> addAddress(Map<String, dynamic> data) async {
    final domain = PersonAddress(
      addressLine1: data['addressLine1'] as String?,
      addressLine2: data['addressLine2'] as String?,
      locality: data['locality'] as String?,
      administrativeArea: data['administrativeArea'] as String?,
      countryCode: data['countryCode'] as String?,
      postalCode: data['postalCode'] as String?,
      latitude: (data['latitude'] as num?)?.toDouble(),
      longitude: (data['longitude'] as num?)?.toDouble(),
      current: data['current'] as bool? ?? true,
    );
    return await GrpcPersonService.addPersonAddress(domain);
  }

  static Future<PersonAddress> updateAddress(int addressId, Map<String, dynamic> data) async {
    final domain = PersonAddress(
      id: addressId,
      addressLine1: data['addressLine1'] as String?,
      addressLine2: data['addressLine2'] as String?,
      locality: data['locality'] as String?,
      administrativeArea: data['administrativeArea'] as String?,
      countryCode: data['countryCode'] as String?,
      postalCode: data['postalCode'] as String?,
      latitude: (data['latitude'] as num?)?.toDouble(),
      longitude: (data['longitude'] as num?)?.toDouble(),
      current: data['current'] as bool? ?? true,
    );
    return await GrpcPersonService.updatePersonAddress(domain);
  }

  static Future<void> deleteAddress(int addressId) async {
    await GrpcPersonService.removePersonAddress(id: addressId);
  }
}
