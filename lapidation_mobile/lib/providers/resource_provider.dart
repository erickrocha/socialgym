import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/models/country.dart';
import 'package:lapidation_mobile/models/settings.dart';

import '../models/resource.dart';
import '../services/resource_service.dart';

class ResourceProvider extends ChangeNotifier {
  AppResources? _resources;
  bool _loading = false;
  String? _error;

  AppResources? get resources => _resources;
  List<Country> get countries => _resources?.countries ?? [];
  bool get loading => _loading;
  String? get error => _error;

  ResourceProvider() {
    _loadFromStorage();
  }

  Future<void> _loadFromStorage() async {
    final prefs = await SharedPreferences.getInstance();
    final resourcesJson = prefs.getString('app_resources');
    if (resourcesJson != null) {
      try {
        final data = jsonDecode(resourcesJson) as Map<String, dynamic>;
        _resources = AppResources.fromJson(data);
        notifyListeners();
      } catch (_) {
        await prefs.remove('app_resources');
      }
    }
  }

  Future<void> _saveToStorage(AppResources resources) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('app_resources', jsonEncode(resources.toJson()));
  }

  /// Fetch application resources from the API.
  /// Optionally apply settings through the provided callback.
  Future<bool> fetchResources(
    String token, {
    Future<void> Function(Settings)? onSettingsReceived,
  }) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _resources = await ResourceService.fetchResources(token);
      await _saveToStorage(_resources!);

      // If settings are present, notify the callback to apply them
      if (_resources!.settings != null && onSettingsReceived != null) {
        await onSettingsReceived(_resources!.settings!);
      }

      _loading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _loading = false;
      _error = e.toString();
      notifyListeners();
      return false;
    }
  }

  /// Get a country by its name.
  Country? getCountryByName(String? name) {
    if (name == null || name.isEmpty) return null;
    try {
      return countries.firstWhere((c) => c.name.toLowerCase() == name.toLowerCase());
    } catch (_) {
      return null;
    }
  }

  /// Get a country by its ID.
  Country? getCountryById(int? id) {
    if (id == null) return null;
    try {
      return countries.firstWhere((c) => c.id == id);
    } catch (_) {
      return null;
    }
  }

  /// Get a country by its acronym (country code).
  Country? getCountryByCode(String? code) {
    if (code == null || code.isEmpty) return null;
    try {
      return countries.firstWhere((c) => c.acronym.toLowerCase() == code.toLowerCase());
    } catch (_) {
      return null;
    }
  }

  /// Clear resources (e.g., on logout).
  Future<void> clear() async {
    _resources = null;
    _error = null;
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove('app_resources');
    notifyListeners();
  }
}
