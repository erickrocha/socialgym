import 'package:socialgym_mobile/models/country.dart';
import 'package:socialgym_mobile/models/province.dart';
import 'package:socialgym_mobile/models/settings.dart';

class AppResources {
  final List<Country> countries;
  final Settings? settings;
  final List<Province> provinces;

  AppResources({required this.countries, this.settings, this.provinces = const []});

  factory AppResources.fromJson(Map<String, dynamic> json) {
    return AppResources(
      countries:
          (json['countries'] as List<dynamic>?)
              ?.map((e) => Country.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      settings: json['settings'] != null
          ? Settings.fromJson(json['settings'] as Map<String, dynamic>)
          : null,
      provinces:
          (json['provinces'] as List<dynamic>?)
              ?.map((e) => Province.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'countries': countries.map((e) => e.toJson()).toList(),
      if (settings != null) 'settings': settings!.toJson(),
      'provinces': provinces.map((e) => e.toJson()).toList(),
    };
  }
}
