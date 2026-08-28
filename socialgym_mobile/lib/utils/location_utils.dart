import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';

import '../l10n/app_localizations.dart';

/// Shared GPS-capture flow: checks that location services/permissions are
/// available (prompting for permission, and offering to open app settings if
/// permanently denied), then returns the current position — or `null` if
/// location is unavailable for any reason, so callers can fall back silently
/// to manual/non-GPS behavior.
class LocationUtils {
  LocationUtils._();

  static Future<Position?> getCurrentPosition(BuildContext context) async {
    final l10n = AppLocalizations.of(context)!;
    try {
      final serviceEnabled = await Geolocator.isLocationServiceEnabled();
      if (!serviceEnabled) return null;

      LocationPermission permission = await Geolocator.checkPermission();
      if (permission == LocationPermission.denied) {
        permission = await Geolocator.requestPermission();
        if (permission == LocationPermission.denied) return null;
      }

      if (permission == LocationPermission.deniedForever) {
        if (context.mounted) {
          final shouldOpenSettings = await showDialog<bool>(
            context: context,
            builder: (context) => AlertDialog(
              title: Text(l10n.permissionRequired),
              content: Text(l10n.addressLocationPermissionDeniedForever),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context, false),
                  child: Text(l10n.buttonCancel),
                ),
                TextButton(
                  onPressed: () => Navigator.pop(context, true),
                  child: Text(l10n.openSettings),
                ),
              ],
            ),
          );
          if (shouldOpenSettings == true) {
            await Geolocator.openAppSettings();
          }
        }
        return null;
      }

      return await Geolocator.getCurrentPosition(
        locationSettings: const LocationSettings(
          accuracy: LocationAccuracy.high,
          timeLimit: Duration(seconds: 10),
        ),
      );
    } catch (_) {
      return null;
    }
  }
}
