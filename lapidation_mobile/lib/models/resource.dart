import 'package:lapidation_mobile/models/country.dart';
import 'package:lapidation_mobile/models/settings.dart';

class AppResources {
  final List<Country> countries;
  final Settings? settings;

  AppResources({required this.countries, this.settings});

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
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'countries': countries.map((e) => e.toJson()).toList(),
      if (settings != null) 'settings': settings!.toJson(),
    };
  }
}
