import '../models/person.dart';
import '../models/business_profile.dart';

/// Utility class for consistent display name formatting across the app
class DisplayNameHelper {
  /// Returns person's full name: "firstname surname"
  /// Falls back to firstname only if surname is missing or empty
  static String getPersonFullName(Person person) {
    final first = person.firstname.trim();
    final last = person.surname.trim();
    if (last.isEmpty) return first;
    return '$first $last';
  }

  /// Returns the display name for a business profile
  /// Uses social name if defined and non-empty, otherwise business name
  static String getBusinessProfileDisplayName(BusinessProfile profile) {
    final social = profile.socialName?.trim() ?? '';
    return social.isNotEmpty ? social : profile.businessName;
  }

  /// Returns formatted data for menu display with primary and secondary text
  /// When social name is defined: primary = social name, secondary = business name
  /// When social name is empty/null: primary = business name, secondary = business type
  static Map<String, String> formatBusinessProfileForMenu(
    BusinessProfile profile,
  ) {
    final social = profile.socialName?.trim() ?? '';
    final hasSocialName = social.isNotEmpty;

    return {
      'primary': hasSocialName ? social : profile.businessName,
      'secondary': hasSocialName ? profile.businessName : profile.businessType,
    };
  }
}
