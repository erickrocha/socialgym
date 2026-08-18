import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

class LocaleProvider extends ChangeNotifier {
  static const String _storageKey = 'locale';
  static const String _userSettingsLanguageKey = 'user_settings_language';

  Locale? _locale;

  Locale? get locale => _locale;

  static const List<Locale> supportedLocales = [
    Locale('en'),
    Locale('pt', 'BR'),
    Locale('es'),
    Locale('fr'),
    Locale('nl'),
  ];

  static const Map<String, String> localeNames = {
    'en': 'English',
    'pt_BR': 'Português (BR)',
    'es': 'Español',
    'fr': 'Français',
    'nl': 'Nederlands',
  };

  LocaleProvider() {
    _loadFromStorage();
  }

  Future<void> _loadFromStorage() async {
    final prefs = await SharedPreferences.getInstance();

    // First, check if there's a user settings language override
    final userLanguage = prefs.getString(_userSettingsLanguageKey);
    if (userLanguage != null) {
      _parseAndSetLocale(userLanguage);
      return;
    }

    // Fall back to manually saved locale preference
    final localeCode = prefs.getString(_storageKey);
    if (localeCode != null) {
      _parseAndSetLocale(localeCode);
      notifyListeners();
    }
  }

  void _parseAndSetLocale(String localeCode) {
    if (localeCode.contains('_')) {
      final parts = localeCode.split('_');
      _locale = Locale(parts[0], parts[1]);
    } else {
      _locale = Locale(localeCode);
    }
    notifyListeners();
  }

  String get currentLocaleKey {
    if (_locale == null) return 'en';
    if (_locale!.countryCode != null && _locale!.countryCode!.isNotEmpty) {
      return '${_locale!.languageCode}_${_locale!.countryCode}';
    }
    return _locale!.languageCode;
  }

  Future<void> setLocale(Locale locale) async {
    _locale = locale;
    final prefs = await SharedPreferences.getInstance();
    String key = locale.languageCode;
    if (locale.countryCode != null && locale.countryCode!.isNotEmpty) {
      key = '${locale.languageCode}_${locale.countryCode}';
    }
    await prefs.setString(_storageKey, key);
    notifyListeners();
  }

  /// Apply user settings language override (from API settings).
  /// This takes precedence over manually set locale.
  Future<void> applyUserSettingsLanguage(String? language) async {
    if (language == null || language.isEmpty) {
      // Clear the override if no language is provided
      final prefs = await SharedPreferences.getInstance();
      await prefs.remove(_userSettingsLanguageKey);
    } else {
      // Save and apply the language
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString(_userSettingsLanguageKey, language);
      _parseAndSetLocale(language);
    }
  }
}
