import 'dart:developer' as developer;

import 'package:flutter/foundation.dart';
import 'package:permission_handler/permission_handler.dart';

class PermissionsUtils {
  static bool get _requiresRuntimePermission {
    return defaultTargetPlatform == TargetPlatform.android ||
        defaultTargetPlatform == TargetPlatform.iOS;
  }

  /// Request camera and photo permissions
  /// Returns true only if both are granted
  static Future<bool> requestCameraAndPhotosPermission() async {
    if (!_requiresRuntimePermission) {
      return true;
    }

    try {
      final permissions = [Permission.camera, Permission.photos];
      final statuses = await permissions.request();

      bool cameraGranted = statuses[Permission.camera]?.isGranted ?? false;
      bool photosGranted = statuses[Permission.photos]?.isGranted ?? false;

      return cameraGranted && photosGranted;
    } catch (e) {
      developer.log(
        'Error requesting permissions',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Deny access on error for security
    }
  }

  /// Request camera permission only
  /// Returns true if permission is granted
  static Future<bool> requestCameraPermission() async {
    if (!_requiresRuntimePermission) {
      return true; // Desktop/Web doesn't need permissions
    }

    try {
      final status = await Permission.camera.request();
      return status.isGranted;
    } catch (e) {
      developer.log(
        'Error requesting camera permission',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Deny access on error for security
    }
  }

  /// Request photos/media permission only
  /// Returns true if permission is granted
  static Future<bool> requestPhotosPermission() async {
    if (!_requiresRuntimePermission) {
      return true; // Desktop/Web doesn't need permissions
    }

    try {
      final status = await Permission.photos.request();
      return status.isGranted;
    } catch (e) {
      developer.log(
        'Error requesting photos permission',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Deny access on error for security
    }
  }

  /// Check if camera permission is granted (without requesting)
  static Future<bool> isCameraPermissionGranted() async {
    if (!_requiresRuntimePermission) {
      return true;
    }

    try {
      final status = await Permission.camera.status;
      return status.isGranted;
    } catch (e) {
      developer.log(
        'Error checking camera permission',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Deny access on error for security
    }
  }

  /// Check if photos permission is granted (without requesting)
  static Future<bool> isPhotosPermissionGranted() async {
    if (!_requiresRuntimePermission) {
      return true;
    }

    try {
      final status = await Permission.photos.status;
      return status.isGranted;
    } catch (e) {
      developer.log(
        'Error checking photos permission',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Deny access on error for security
    }
  }

  /// Check if camera permission is permanently denied
  static Future<bool> isCameraPermissionPermanentlyDenied() async {
    if (!_requiresRuntimePermission) {
      return false;
    }

    try {
      final status = await Permission.camera.status;
      return status.isPermanentlyDenied;
    } catch (e) {
      developer.log(
        'Error checking camera permanent denial',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Assume not permanently denied on error
    }
  }

  /// Check if photos permission is permanently denied
  static Future<bool> isPhotosPermissionPermanentlyDenied() async {
    if (!_requiresRuntimePermission) {
      return false;
    }

    try {
      final status = await Permission.photos.status;
      return status.isPermanentlyDenied;
    } catch (e) {
      developer.log(
        'Error checking photos permanent denial',
        error: e,
        name: 'PermissionsUtils',
      );
      return false; // Assume not permanently denied on error
    }
  }

  /// Open app settings to allow user to enable permissions
  static Future<void> openAppSettingsDialog() async {
    if (!_requiresRuntimePermission) {
      return; // Not needed on desktop/web
    }

    try {
      await openAppSettings();
    } catch (e) {
      developer.log(
        'Error opening app settings',
        error: e,
        name: 'PermissionsUtils',
      );
    }
  }
}
