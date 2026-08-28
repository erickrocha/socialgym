import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:cached_network_image/cached_network_image.dart';

import '../config/app_colors.dart';
import '../l10n/app_localizations.dart';
import '../providers/auth_provider.dart';
import '../providers/person_provider.dart';
import '../utils/display_name_helper.dart';

const _businessAvatarPlaceholder = 'assets/images/avatar_personal_trainer.png';

class ProfileMenu extends StatefulWidget {
  final VoidCallback? onProfilePressed;

  const ProfileMenu({super.key, this.onProfilePressed});

  @override
  State<ProfileMenu> createState() => _ProfileMenuState();
}

class _ProfileMenuState extends State<ProfileMenu> {
  Widget _buildAvatarWidget(BuildContext context) {
    final personProvider = context.watch<PersonProvider>();
    final person = personProvider.person;
    final businessLogo = personProvider.activeBusinessProfile?.logo;
    final personalAvatarPlaceholder =
        person?.gender?.trim().toLowerCase() == 'female'
        ? 'assets/images/avatar_female.png'
        : 'assets/images/avatar_male.png';

    if (personProvider.isProfessional) {
      return CircleAvatar(
        radius: 14,
        backgroundColor: Colors.transparent,
        child: ClipOval(
          child: (businessLogo != null && businessLogo.isNotEmpty)
              ? CachedNetworkImage(
                  imageUrl: businessLogo,
                  fit: BoxFit.cover,
                  placeholder: (_, _) => Image.asset(
                    _businessAvatarPlaceholder,
                    fit: BoxFit.cover,
                  ),
                  errorWidget: (_, _, _) => Image.asset(
                    _businessAvatarPlaceholder,
                    fit: BoxFit.cover,
                  ),
                )
              : Image.asset(_businessAvatarPlaceholder, fit: BoxFit.cover),
        ),
      );
    }

    if (person?.avatar != null && person!.avatar!.isNotEmpty) {
      return CircleAvatar(
        radius: 14,
        backgroundColor: Colors.transparent,
        child: ClipOval(
          child: CachedNetworkImage(
            imageUrl: person.avatar!,
            cacheKey: person.objectKey,
            fit: BoxFit.cover,
            placeholder: (_, _) =>
                Image.asset(personalAvatarPlaceholder, fit: BoxFit.cover),
            errorWidget: (_, _, _) =>
                Image.asset(personalAvatarPlaceholder, fit: BoxFit.cover),
          ),
        ),
      );
    }

    return CircleAvatar(
      radius: 14,
      backgroundColor: Colors.transparent,
      child: ClipOval(
        child: Image.asset(
          personalAvatarPlaceholder,
          width: 28,
          height: 28,
          fit: BoxFit.cover,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return PopupMenuButton<String>(
      icon: _buildAvatarWidget(context),
      tooltip: l10n.menuProfile,
      padding: EdgeInsets.zero,
      constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
      offset: const Offset(0, 48),
      onSelected: (value) => _handleMenuAction(value, context),
      itemBuilder: (context) => _buildMenuItems(l10n),
    );
  }

  Future<void> _handleMenuAction(String value, BuildContext context) async {
    if (value.startsWith('profile_')) {
      // Extract profile index and switch profile
      final index = int.parse(value.replaceFirst('profile_', ''));
      final person = context.read<PersonProvider>().person;
      if (person != null && index < person.businessProfiles.length) {
        final token = context.read<AuthProvider>().auth?.accessToken ?? '';
        if (token.isNotEmpty) {
          final authProvider = context.read<AuthProvider>();
          final personProvider = context.read<PersonProvider>();
          final navigator = Navigator.of(context);
          final newAuth = await personProvider.switchProfile(
            index,
            token,
            onTokenIssued: authProvider.applySwitchedToken,
          );
          if (newAuth != null) {
            navigator.pushNamedAndRemoveUntil('/feed', (route) => false);
          } else if (context.mounted && personProvider.error != null) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(personProvider.error!)),
            );
          }
        }
      }
      return;
    }

    switch (value) {
      case 'switch_personal':
        final token = context.read<AuthProvider>().auth?.accessToken ?? '';
        final authProvider = context.read<AuthProvider>();
        final personProvider = context.read<PersonProvider>();
        final navigator = Navigator.of(context);
        final newAuth = await personProvider.switchToPersonal(
          token,
          onTokenIssued: authProvider.applySwitchedToken,
        );
        if (newAuth != null) {
          navigator.pushNamedAndRemoveUntil('/feed', (route) => false);
        } else if (context.mounted && personProvider.error != null) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text(personProvider.error!)),
          );
        }
        break;
      case 'add_profile':
        Navigator.of(
          context,
        ).pushNamedAndRemoveUntil('/add-profile', (route) => false);
        break;
      case 'profile':
        widget.onProfilePressed?.call();
        break;
      case 'friends':
        Navigator.of(
          context,
        ).pushNamedAndRemoveUntil('/friends', (route) => false);
        break;
      case 'settings':
        Navigator.of(
          context,
        ).pushNamedAndRemoveUntil('/settings', (route) => false);
        break;
      case 'logout':
        context.read<AuthProvider>().signOut();
        context.read<PersonProvider>().clear();
        Navigator.of(
          context,
        ).pushNamedAndRemoveUntil('/login', (route) => false);
        break;
    }
  }

  List<PopupMenuEntry<String>> _buildMenuItems(AppLocalizations l10n) {
    final personProvider = Provider.of<PersonProvider>(context, listen: false);
    final person = personProvider.person;
    final businessType = personProvider.activeBusinessProfile?.businessType;
    final profiles = person?.businessProfiles ?? [];

    final items = <PopupMenuEntry<String>>[
      // Profile list
      ...profiles.asMap().entries.map((entry) {
        final businessProfile = entry.value;
        final displayData = DisplayNameHelper.formatBusinessProfileForMenu(
          businessProfile,
        );
        return PopupMenuItem(
          value: 'profile_${entry.key}',
          child: Row(
            children: [
              CircleAvatar(
                radius: 12,
                backgroundImage: (businessProfile.logo?.isNotEmpty ?? false)
                    ? CachedNetworkImageProvider(businessProfile.logo!)
                    : const AssetImage(_businessAvatarPlaceholder)
                          as ImageProvider,
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      displayData['primary']!,
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                      ),
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                    Text(
                      displayData['secondary']!,
                      style: const TextStyle(fontSize: 12, color: Colors.grey),
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      }),

      if (personProvider.isProfessional)
        PopupMenuItem(
          value: 'switch_personal',
          child: Row(
            children: [
              Icon(
                Icons.account_circle_outlined,
                color: AppColors.primaryFor(businessType),
                size: 20,
              ),
              const SizedBox(width: 12),
              Text(
                person != null
                    ? DisplayNameHelper.getPersonFullName(person)
                    : l10n.businessProfileSwitchToPersonal,
              ),
            ],
          ),
        ),

      // Add New Profile
      PopupMenuItem(
        value: 'add_profile',
        child: Row(
          children: [
            Icon(
              Icons.add_circle_outline,
              color: AppColors.primaryFor(businessType),
              size: 20,
            ),
            const SizedBox(width: 12),
            const Text('Add New Profile'),
          ],
        ),
      ),

      const PopupMenuDivider(),

      // Logout
      PopupMenuItem(
        value: 'logout',
        child: Row(
          children: [
            const Icon(Icons.logout, color: AppColors.danger, size: 20),
            const SizedBox(width: 12),
            Text(l10n.logout, style: const TextStyle(color: AppColors.danger)),
          ],
        ),
      ),
    ];

    return items;
  }
}
