import 'package:flutter/material.dart';

import '../l10n/app_localizations.dart';

enum VisibilityOption {
  privateAccess,
  friends,
  publicAccess,
  professionalAccess;

  String get apiValue {
    switch (this) {
      case VisibilityOption.privateAccess:
        return 'Private';
      case VisibilityOption.friends:
        return 'Friends';
      case VisibilityOption.publicAccess:
        return 'Public';
      case VisibilityOption.professionalAccess:
        return 'Professional';
    }
  }

  String apiValueFor({bool useFriendsOnlyAlias = false}) {
    if (useFriendsOnlyAlias && this == VisibilityOption.friends) {
      return 'FriendsOnly';
    }

    return apiValue;
  }

  static VisibilityOption fromApiValue(
    String? value, {
    VisibilityOption fallback = VisibilityOption.privateAccess,
  }) {
    final normalized =
        value?.trim().toLowerCase().replaceAll(RegExp(r'[^a-z]'), '') ?? '';

    switch (normalized) {
      case 'private':
        return VisibilityOption.privateAccess;
      case 'friends':
        return VisibilityOption.friends;
      case 'public':
        return VisibilityOption.publicAccess;
      case 'professional':
        return VisibilityOption.professionalAccess;
      default:
        return fallback;
    }
  }

  String label(AppLocalizations l10n) {
    switch (this) {
      case VisibilityOption.privateAccess:
        return l10n.visibilityPrivate;
      case VisibilityOption.friends:
        return l10n.visibilityFriends;
      case VisibilityOption.publicAccess:
        return l10n.visibilityPublic;
      case VisibilityOption.professionalAccess:
        return l10n.visibilityProfessional;
    }
  }

  IconData get icon {
    switch (this) {
      case VisibilityOption.privateAccess:
        return Icons.lock;
      case VisibilityOption.friends:
        return Icons.people;
      case VisibilityOption.publicAccess:
        return Icons.public;
      case VisibilityOption.professionalAccess:
        return Icons.work;
    }
  }

  String get emoji {
    switch (this) {
      case VisibilityOption.privateAccess:
        return '🔒';
      case VisibilityOption.friends:
        return '👥';
      case VisibilityOption.publicAccess:
        return '🌍';
      case VisibilityOption.professionalAccess:
        return '💼';
    }
  }
}
