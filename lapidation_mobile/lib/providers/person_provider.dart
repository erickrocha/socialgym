import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/models/business_profile.dart';

import '../models/auth_response.dart';
import '../models/person.dart';
import '../services/business_profile_rest_service.dart';
import '../services/person_service.dart';
import '../services/base_service.dart';
import '../services/grpc/grpc_business_profile_service.dart';
import '../utils/jwt_decoder.dart';

class PersonProvider extends ChangeNotifier {
  Person? _person;
  bool _loading = false;
  bool _updating = false;
  String? _error;
  BusinessProfile? _activeBusinessProfile;

  static const String _activeProfileStorageKey = 'active_profile';

  Person? get person => _person;
  bool get loading => _loading;
  bool get updating => _updating;
  String? get error => _error;
  BusinessProfile? get activeBusinessProfile => _activeBusinessProfile;
  bool get isProfessional => _activeBusinessProfile != null;

  /// Returns the person ID or null if not loaded yet.
  int get ownerId => _person?.id ?? 0;
  String get ownerName => _person?.fullName ?? 'Unknow';

  String get ownerUuid => _person?.uuid ?? '';

  /// UUID of whichever account is currently active: the active business
  /// profile if one is set, otherwise the person themself.
  int get activeAuthorId => _activeBusinessProfile?.id ?? _person?.id ?? 0;
  String get activeAuthorUuid => _activeBusinessProfile?.uuid ?? _person?.uuid ?? '';

  /// Avatar/logo image reference of whichever account is currently active.
  String? get activeAuthorObjectKeyOrLogo =>
      _activeBusinessProfile?.logo ?? _person?.objectKey;

  @visibleForTesting
  void setPersonForTest(Person person) {
    _person = person;
    notifyListeners();
  }

  @visibleForTesting
  void setActiveBusinessProfile(BusinessProfile? profile) {
    _activeBusinessProfile = profile;
    notifyListeners();
  }

  PersonProvider() {
    _loadFromStorage();
  }

  Future<void> _loadFromStorage() async {
    final prefs = await SharedPreferences.getInstance();
    final personJson = prefs.getString('person');
    if (personJson != null) {
      try {
        final data = jsonDecode(personJson) as Map<String, dynamic>;
        _person = Person.fromJson(data);
      } catch (_) {
        await prefs.remove('person');
      }
    }
    final activeProfileJson = prefs.getString(_activeProfileStorageKey);
    if (activeProfileJson != null) {
      try {
        _activeBusinessProfile = BusinessProfile.fromJson(
          jsonDecode(activeProfileJson) as Map<String, dynamic>,
        );
      } catch (_) {
        await prefs.remove(_activeProfileStorageKey);
      }
    }
    notifyListeners();
  }

  Future<void> _saveToStorage(Person person) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('person', jsonEncode(person.toJson()));
  }

  Future<void> _clearStorage() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove('person');
  }

  Future<void> _saveActiveBusinessProfileToStorage(BusinessProfile? profile) async {
    final prefs = await SharedPreferences.getInstance();
    if (profile == null) {
      await prefs.remove(_activeProfileStorageKey);
    } else {
      await prefs.setString(_activeProfileStorageKey,jsonEncode(profile.toJson()));
    }
  }

  /// Fetch the current user's person data via gRPC. Identity is derived
  /// server-side from the JWT, so no token needs to be threaded through.
  Future<bool> fetchMe() async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _person = await PersonService.fetchMe();
      await _saveToStorage(_person!);
      _loading = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to load user data. Please try again.';
      _loading = false;
      notifyListeners();
      return false;
    }
  }

  /// Update the person's basic data (firstname, surname, gender, dateOfBirth).
  Future<bool> updatePerson(Map<String, dynamic> data) async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      _person = await PersonService.updatePerson(_person!, data);
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to update profile. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Update the person info (biography, job, relationship, etc.).
  Future<bool> updatePersonInfo(Map<String, dynamic> data) async {
    if (_person?.personInfo == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      await PersonService.updatePersonInfo(_person!.personInfo!, data);
      // Refresh the full person data to get updated info
      _person = await PersonService.fetchMe();
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to update profile info. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Upload avatar image.
  Future<bool> uploadAvatar(XFile imageFile) async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      _person = await PersonService.uploadAvatar(imageFile);
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to upload avatar. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Upload cover image.
  Future<bool> uploadCover(XFile imageFile) async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      _person = await PersonService.uploadCover(imageFile);
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to upload cover. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Remove avatar image.
  Future<bool> removeAvatar() async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      _person = await PersonService.removeAvatar();
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to remove avatar. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Remove cover image.
  Future<bool> removeCover() async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      _person = await PersonService.removeCover();
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to remove cover. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Add a new address.
  Future<bool> addAddress(Map<String, dynamic> data) async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      // Only set current to true if not already specified
      data.putIfAbsent('current', () => true);
      await PersonService.addAddress(data);
      // Refresh person data to get updated addresses
      _person = await PersonService.fetchMe();
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to add address. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Update an existing address.
  Future<bool> updateAddress(
    int addressId,
    Map<String, dynamic> data,
  ) async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      await PersonService.updateAddress(addressId, data);
      // Refresh person data to get updated addresses
      _person = await PersonService.fetchMe();
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to update address. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Delete an address.
  Future<bool> deleteAddress(int addressId) async {
    if (_person == null) return false;

    _updating = true;
    _error = null;
    notifyListeners();

    try {
      await PersonService.deleteAddress(addressId);
      // Refresh person data to get updated addresses
      _person = await PersonService.fetchMe();
      await _saveToStorage(_person!);
      _updating = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Failed to delete address. Please try again.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  /// Switch to a profile by index. Activates the business profile on the
  /// backend (reissuing the JWT scoped to it) and returns the new token, or
  /// null on failure.
  Future<AuthResponse?> switchProfile(int profileIndex, String token) async {
    if (_person == null ||
        profileIndex < 0 ||
        profileIndex >= _person!.businessProfiles.length) {
      _error = 'Invalid profile index';
      notifyListeners();
      return null;
    }
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      final selectedProfile = _person!.businessProfiles[profileIndex];
      if (selectedProfile.uuid == null || selectedProfile.uuid!.isEmpty) {
        throw Exception('Business profile has no uuid');
      }
      final newAuth = await BusinessProfileRestService.activate(businessProfileUuid: selectedProfile.uuid!,token: token);
      _activeBusinessProfile = await GrpcBusinessProfileService.getBusinessProfileById(uuid: selectedProfile.uuid!);
      await _saveActiveBusinessProfileToStorage(_activeBusinessProfile);
      _updating = false;
      notifyListeners();
      return newAuth;
    } on AppException catch (e) {
      _error = e.message;
      _updating = false;
      notifyListeners();
      return null;
    } catch (e) {
      _error = 'Failed to switch profile.';
      _updating = false;
      notifyListeners();
      return null;
    }
  }

  /// Deactivate the current business profile on the backend (reissuing the
  /// JWT back in personal context) and returns the new token, or null on
  /// failure.
  Future<AuthResponse?> switchToPersonal(String token) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      final newAuth = await BusinessProfileRestService.deactivate(token: token);
      _activeBusinessProfile = null;
      await _saveActiveBusinessProfileToStorage(null);
      _updating = false;
      notifyListeners();
      return newAuth;
    } catch (_) {
      _error = 'Failed to switch to personal profile.';
      _updating = false;
      notifyListeners();
      return null;
    }
  }

  Future<void> restoreActiveBusinessProfileFromToken(String token) async {
    final claims = JwtDecoder.decode(token);
    if (claims == null || !claims.isBusinessProfile || _person == null) {
      return;
    }
    final index = _person!.businessProfiles.indexWhere(
      (p) => p.uuid == claims.activeBusinessProfileUuid,
    );
    if (index == -1) return;
    _activeBusinessProfile = _person!.businessProfiles[index];
    await _saveActiveBusinessProfileToStorage(_activeBusinessProfile);
    notifyListeners();
  }

  /// Clear the person data (on logout).
  void clear() {
    _person = null;
    _activeBusinessProfile = null;
    _error = null;
    _clearStorage();
    _saveActiveBusinessProfileToStorage(null);
    notifyListeners();
  }

  void clearError() {
    _error = null;
    notifyListeners();
  }
}
