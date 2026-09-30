/// The unit a weight value (exercise load or bodyweight) is displayed and
/// entered in. All weight is always stored as kilograms — this only governs
/// the client-side display/input boundary.
enum WeightUnit {
  kilograms,
  pounds;

  static const double _kgPerLb = 0.45359237;

  /// The unit conventionally associated with a language, used only until the
  /// person makes an explicit choice in Settings.
  static WeightUnit defaultForLanguageCode(String? languageCode) {
    return languageCode?.toLowerCase().startsWith('en') == true
        ? WeightUnit.pounds
        : WeightUnit.kilograms;
  }

  static WeightUnit fromString(String? value) {
    return WeightUnit.values.firstWhere(
      (e) => e.wireValue.toLowerCase() == value?.toLowerCase(),
      orElse: () => WeightUnit.kilograms,
    );
  }

  /// Backend wire value (`WeightUnit::from_string`/`Display` in
  /// workout/business/src/domain/enums.rs).
  String get wireValue =>
      this == WeightUnit.pounds ? 'Pounds' : 'Kilograms';

  /// Short unit label for display next to a value.
  String get label => this == WeightUnit.pounds ? 'lbs' : 'kg';

  /// Converts a canonical kilogram value to this unit for display.
  double fromKg(double kg) =>
      this == WeightUnit.pounds ? kg / _kgPerLb : kg;

  /// Converts a value entered in this unit back to canonical kilograms for
  /// storage/transmission.
  double toKg(double value) =>
      this == WeightUnit.pounds ? value * _kgPerLb : value;
}
