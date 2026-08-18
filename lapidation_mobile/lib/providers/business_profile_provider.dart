import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';

import '../models/business_profile.dart';
import '../models/business_profile_address.dart';
import '../services/grpc/grpc_business_profile_service.dart';
import '../services/upload_service.dart';

class BusinessProfileProvider extends ChangeNotifier {
  BusinessProfile? _current;
  bool _loading = false;
  bool _updating = false;
  String? _error;

  BusinessProfile? get current => _current;
  bool get loading => _loading;
  bool get updating => _updating;
  String? get error => _error;

  Future<bool> load({int? id, String? uuid}) async {
    _loading = true;
    _error = null;
    notifyListeners();
    try {
      _current = await GrpcBusinessProfileService.getBusinessProfileById(id: id, uuid: uuid);
      _loading = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to load business profile.';
      _loading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> update(BusinessProfile profile) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      _current = await GrpcBusinessProfileService.updateBusinessProfile(profile);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to update business profile.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> addAddress(BusinessProfileAddress address) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await GrpcBusinessProfileService.addBusinessProfileAddress(address);
      final loadSuccess = await load(id: _current?.id, uuid: _current?.uuid);
      _updating = false;
      notifyListeners();
      return loadSuccess;
    } catch (_) {
      _error = 'Failed to add address.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> updateAddress(BusinessProfileAddress address) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await GrpcBusinessProfileService.updateBusinessProfileAddress(address);
      final loadSuccess = await load(id: _current?.id, uuid: _current?.uuid);
      _updating = false;
      notifyListeners();
      return loadSuccess;
    } catch (_) {
      _error = 'Failed to update address.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> removeAddress({int? id, String? uuid}) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      final removed = await GrpcBusinessProfileService.removeBusinessProfileAddress(id: id, uuid: uuid);
      if (!removed) {
        _error = 'Failed to remove address.';
        _updating = false;
        notifyListeners();
        return false;
      }
      final loadSuccess = await load(id: _current?.id, uuid: _current?.uuid);
      _updating = false;
      notifyListeners();
      return loadSuccess;
    } catch (_) {
      _error = 'Failed to remove address.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> uploadLogo(String token, XFile file) async {
    final businessProfileId = _current?.id;
    if (businessProfileId == null) {
      _error = 'Business profile not loaded.';
      notifyListeners();
      return false;
    }
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await UploadService.uploadBusinessProfileLogo(token, businessProfileId, file);
      final loadSuccess = await load(id: businessProfileId);
      _updating = false;
      notifyListeners();
      return loadSuccess;
    } catch (_) {
      _error = 'Failed to upload logo.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> uploadCover(String token, XFile file) async {
    final businessProfileId = _current?.id;
    if (businessProfileId == null) {
      _error = 'Business profile not loaded.';
      notifyListeners();
      return false;
    }
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await UploadService.uploadBusinessProfileCover(token, file);
      final loadSuccess = await load(id: businessProfileId);
      _updating = false;
      notifyListeners();
      return loadSuccess;
    } catch (_) {
      _error = 'Failed to upload cover.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  void clear() {
    _current = null;
    _error = null;
    notifyListeners();
  }
}
