import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/models/enums.dart';
import 'package:socialgym_mobile/models/settings.dart';

import '../services/grpc/grpc_settings_service.dart';

class SettingsProvider extends ChangeNotifier {
  static const String _settingsStorageKey = 'user_settings';

  Settings? _settings;
  bool _isLoaded = false;
  bool _loading = false;
  bool _saving = false;
  String? _error;

  Settings? get settings => _settings;
  Pages get homePage => _settings?.homePage ?? Pages.feed;
  ContextMenuPosition get contextMenuPosition =>
      _settings?.contextMenuPosition ?? ContextMenuPosition.left;
  String? get userLanguage => _settings?.language;
  bool get isLoaded => _isLoaded;
  bool get loading => _loading;
  bool get saving => _saving;
  String? get error => _error;

  SettingsProvider() {
    _loadFromStorage();
  }

  /// Load settings from local storage.
  Future<void> _loadFromStorage() async {
    final prefs = await SharedPreferences.getInstance();
    final settingsJson = prefs.getString(_settingsStorageKey);
    if (settingsJson != null) {
      try {
        final data = jsonDecode(settingsJson) as Map<String, dynamic>;
        _settings = Settings.fromJson(data);
      } catch (_) {
        await prefs.remove(_settingsStorageKey);
      }
    }
    _isLoaded = true;
    notifyListeners();
  }

  /// Save settings to local storage.
  Future<void> _saveToStorage(Settings settings) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_settingsStorageKey, jsonEncode(settings.toJson()));
  }

  /// Apply settings received from the API.
  /// This updates the local settings and persists them.
  Future<void> applySettings(Settings newSettings) async {
    _settings = newSettings;
    await _saveToStorage(newSettings);
    notifyListeners();
  }

  /// Check if settings are cached and should not be fetched again.
  bool hasSettingsCached() => _settings != null;

  /// Update the user's language setting.
  Future<void> setLanguage(String language) async {
    final updated = (_settings ?? Settings()).copyWith(language: language);
    await applySettings(updated);
  }

  /// Update the user's home page preference.
  Future<void> setHomePage(Pages homePage) async {
    final updated = (_settings ?? Settings()).copyWith(homePage: homePage);
    await applySettings(updated);
  }

  /// Update the context menu position preference.
  Future<void> setContextMenuPosition(ContextMenuPosition contextMenuPosition) async {
    final updated = (_settings ?? Settings()).copyWith(contextMenuPosition: contextMenuPosition);
    await applySettings(updated);
  }

  /// Clear all settings (e.g., on logout).
  Future<void> clear() async {
    _settings = null;
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_settingsStorageKey);
    notifyListeners();
  }

  /// Refreshes settings from the backend over gRPC.
  ///
  /// If the person has no settings row yet, the current cached/default
  /// settings are left untouched (not an error — just nothing to sync yet).
  /// Any other failure sets [error] but does not clear the current settings,
  /// since the page already has cached/default values to show.
  Future<void> fetchFromServer({required int personId, String personUuid = ''}) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final fetched = await GrpcSettingsService.getByOwnerId(
        ownerId: personId,
        ownerUuid: personUuid,
      );
      if (fetched != null) {
        await applySettings(fetched);
      }
    } catch (e) {
      _error = 'Failed to refresh settings.';
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  /// Persists [updated] to the backend over gRPC. Returns `true` on success
  /// (and applies the server's response as the new local settings), `false`
  /// on failure (leaving the caller's form values untouched so nothing is
  /// lost).
  Future<bool> persistToServer(Settings updated) async {
    _saving = true;
    _error = null;
    notifyListeners();

    try {
      final persisted = await GrpcSettingsService.persistSettings(updated);
      await applySettings(persisted);
      return true;
    } catch (e) {
      _error = 'Failed to save settings.';
      return false;
    } finally {
      _saving = false;
      notifyListeners();
    }
  }
}
