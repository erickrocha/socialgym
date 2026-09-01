import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../config/app_colors.dart';
import '../config/nav_section.dart';
import '../l10n/app_localizations.dart';
import '../models/person.dart';
import '../providers/auth_provider.dart';
import '../providers/person_provider.dart';
import 'language_selector.dart';

/// The sidebar drawer for mobile — wraps [SidebarContent] in a [Drawer].
class SidebarMenu extends StatefulWidget {
  final NavSection navSection;
  final String currentRoute;

  const SidebarMenu({
    super.key,
    this.navSection = NavSection.home,
    this.currentRoute = '/feed',
  });

  @override
  State<SidebarMenu> createState() => _SidebarMenuState();
}

class _SidebarMenuState extends State<SidebarMenu> {
  bool _isCollapsed = false;

  void _toggleCollapsed() {
    setState(() {
      _isCollapsed = !_isCollapsed;
    });
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      width: _isCollapsed ? 80 : 280,
      child: Drawer(
        width: _isCollapsed ? 80 : 280,
        backgroundColor: Colors.white,
        child: SidebarContent(
          isCollapsed: _isCollapsed,
          onToggleCollapse: _toggleCollapsed,
          navSection: widget.navSection,
          currentRoute: widget.currentRoute,
        ),
      ),
    );
  }
}

/// The sidebar content used both inside a Drawer (mobile)
/// and as a persistent panel (desktop).
class SidebarContent extends StatelessWidget {
  final bool isCollapsed;
  final VoidCallback onToggleCollapse;
  final NavSection navSection;

  /// Current page route — used to highlight the active sidebar item.
  final String currentRoute;

  const SidebarContent({
    super.key,
    required this.isCollapsed,
    required this.onToggleCollapse,
    this.navSection = NavSection.home,
    this.currentRoute = '/feed',
  });

  // ── Navigation helpers ───────────────────────────────────────────────────

  void _go(BuildContext context, String route) {
    _closeDrawer(context);
    Navigator.of(context).pushNamedAndRemoveUntil(route, (r) => false);
  }

  void _showComingSoon(BuildContext context, AppLocalizations l10n) {
    // Capture messenger before closing drawer to avoid context issues.
    final messenger = ScaffoldMessenger.of(context);
    _closeDrawer(context);
    messenger.showSnackBar(
      SnackBar(
        content: Text(l10n.comingSoon),
        duration: const Duration(seconds: 2),
        behavior: SnackBarBehavior.floating,
      ),
    );
  }

  void _closeDrawer(BuildContext context) {
    final scaffold = Scaffold.maybeOf(context);
    if (scaffold != null && scaffold.isDrawerOpen) {
      Navigator.of(context).pop();
    }
  }

  // ── Build ────────────────────────────────────────────────────────────────

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final authProvider = context.watch<AuthProvider>();
    final personProvider = context.watch<PersonProvider>();

    return SafeArea(
      child: SingleChildScrollView(
        child: Column(
          children: [
            // Header
            _buildDrawerHeader(context, l10n, authProvider, personProvider),

            // Section nav items
            ListView(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              padding: const EdgeInsets.only(top: 8),
              children: [
                ..._buildSectionItems(context, l10n, personProvider),
                const Divider(indent: 16, endIndent: 16),
                ..._buildAlwaysItems(
                  context,
                  l10n,
                  personProvider.activeBusinessProfile?.businessType,
                ),
              ],
            ),

            // Logout at the bottom
            const Divider(height: 1),
            _SidebarItem(
              icon: Icons.logout,
              activeIcon: Icons.logout,
              label: l10n.logout,
              textColor: AppColors.danger,
              iconColor: AppColors.danger,
              isCollapsed: isCollapsed,
              onTap: () {
                _closeDrawer(context);
                authProvider.signOut();
                context.read<PersonProvider>().clear();
                Navigator.of(
                  context,
                ).pushNamedAndRemoveUntil('/login', (route) => false);
              },
            ),
            const SizedBox(height: 8),
          ],
        ),
      ),
    );
  }

  // ── Section-specific items ───────────────────────────────────────────────

  List<Widget> _buildSectionItems(
    BuildContext context,
    AppLocalizations l10n,
    PersonProvider personProvider,
  ) {
    switch (navSection) {
      case NavSection.home:
        return _homeItems(context, l10n, personProvider);
      case NavSection.gallery:
        return _homeItems(context, l10n, personProvider);
      case NavSection.workout:
        return _workoutItems(context, l10n, personProvider);
    }
  }

  List<Widget> _homeItems(
    BuildContext context,
    AppLocalizations l10n,
    PersonProvider personProvider,
  ) {
    final isProfessional = personProvider.isProfessional;
    if (isProfessional) {
      return [
        _SidebarItem(
          icon: Icons.feed_outlined,
          activeIcon: Icons.feed,
          label: l10n.menuFeed,
          isActive: currentRoute == '/feed',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/feed'),
        ),
        _SidebarItem(
          icon: Icons.image_outlined,
          activeIcon: Icons.image,
          label: l10n.menuGallery,
          isActive: currentRoute == '/gallery',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/gallery'),
        ),
        _SidebarItem(
          icon: Icons.person_outline,
          activeIcon: Icons.person,
          label: l10n.menuProfile,
          isActive: currentRoute == '/business-profile',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/business-profile'),
        ),
        _SidebarItem(
          icon: Icons.group_outlined,
          activeIcon: Icons.group,
          label: l10n.menuTeam,
          isActive: currentRoute == '/team',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/team'),
        ),
        _SidebarItem(
          icon: Icons.assignment_outlined,
          activeIcon: Icons.assignment,
          label: l10n.workoutInvitesMenu,
          isActive: currentRoute == '/workout-invites',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/workout-invites'),
        ),
        _SidebarItem(
          icon: Icons.people_outline,
          activeIcon: Icons.people,
          label: l10n.menuFollowers,
          isActive: currentRoute == '/followers',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/followers'),
        ),
        _SidebarItem(
          icon: Icons.notifications_outlined,
          activeIcon: Icons.notifications,
          label: l10n.menuNotifications,
          isActive: currentRoute == '/notifications',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/notifications'),
        ),
      ];
    }

    return [
      _SidebarItem(
        icon: Icons.feed_outlined,
        activeIcon: Icons.feed,
        label: l10n.menuFeed,
        isActive: currentRoute == '/feed',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/feed'),
      ),
      _SidebarItem(
        icon: Icons.image_outlined,
        activeIcon: Icons.image,
        label: l10n.menuGallery,
        isActive: currentRoute == '/gallery',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/gallery'),
      ),
      _SidebarItem(
        icon: Icons.person_outline,
        activeIcon: Icons.person,
        label: l10n.menuProfile,
        isActive:
            currentRoute == '/profile' || currentRoute == '/business-profile',
        isCollapsed: isCollapsed,
        onTap: () => _go(
          context,
          personProvider.isProfessional
              ? '/business-profile'
              : '/profile',
        ),
      ),
      _SidebarItem(
        icon: Icons.people_outline,
        activeIcon: Icons.people,
        label: l10n.menuFriends,
        isActive: currentRoute == '/friends',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/friends'),
      ),
      _SidebarItem(
        icon: Icons.group_outlined,
        activeIcon: Icons.group,
        label: l10n.menuTeam,
        isActive: currentRoute == '/team',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/team'),
      ),
      _SidebarItem(
        icon: Icons.assignment_outlined,
        activeIcon: Icons.assignment,
        label: l10n.workoutInvitesMenu,
        isActive: currentRoute == '/workout-invites',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/workout-invites'),
      ),
      _SidebarItem(
        icon: Icons.chat_bubble_outline,
        activeIcon: Icons.chat_bubble,
        label: l10n.menuMessages,
        isActive: false,
        isCollapsed: isCollapsed,
        onTap: () => _showComingSoon(context, l10n),
      ),
      _SidebarItem(
        icon: Icons.notifications_outlined,
        activeIcon: Icons.notifications,
        label: l10n.menuNotifications,
        isActive: currentRoute == '/notifications',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/notifications'),
      ),
    ];
  }

  List<Widget> _workoutItems(
    BuildContext context,
    AppLocalizations l10n,
    PersonProvider personProvider,
  ) {
    return [
      if (!personProvider.isProfessional)
        _SidebarItem(
          icon: Icons.trending_up_outlined,
          activeIcon: Icons.trending_up,
          label: l10n.menuEvolution,
          isActive: currentRoute == '/evolution',
          isCollapsed: isCollapsed,
          onTap: () => _go(context, '/evolution'),
        ),
      _SidebarItem(
        icon: Icons.fitness_center_outlined,
        activeIcon: Icons.fitness_center,
        label: l10n.menuWorkouts,
        isActive: currentRoute == '/workouts',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/workouts'),
      ),
      _SidebarItem(
        icon: Icons.list_outlined,
        activeIcon: Icons.list,
        label: l10n.menuExercises,
        isActive: currentRoute == '/exercises',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/exercises'),
      ),
      _SidebarItem(
        icon: Icons.bar_chart_outlined,
        activeIcon: Icons.bar_chart,
        label: l10n.menuWorkoutSessions,
        isActive: currentRoute == '/workout-sessions',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/workout-sessions'),
      ),
    ];
  }

  // ── Always-visible items (settings + language) ───────────────────────────

  List<Widget> _buildAlwaysItems(
    BuildContext context,
    AppLocalizations l10n,
    String? businessType,
  ) {
    return [
      _SidebarItem(
        icon: Icons.settings_outlined,
        activeIcon: Icons.settings,
        label: l10n.menuSettings,
        isActive: currentRoute == '/settings',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/settings'),
      ),
      if (!isCollapsed)
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          child: Row(
            children: [
              Icon(Icons.language, color: AppColors.primaryFor(businessType), size: 24),
              const SizedBox(width: 16),
              Flexible(
                child: Text(
                  l10n.menuLanguage,
                  style: const TextStyle(
                    fontSize: 15,
                    color: Color(0xFF333333),
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              const SizedBox(width: 8),
              const LanguageSelectorButton(),
            ],
          ),
        )
      else
        Padding(
          padding: const EdgeInsets.symmetric(vertical: 8),
          child: Center(
            child: IconButton(
              icon: Icon(
                Icons.language,
                color: AppColors.primaryFor(businessType),
                size: 24,
              ),
              tooltip: l10n.menuLanguage,
              onPressed: () {
                showDialog(
                  context: context,
                  builder: (_) => AlertDialog(
                    title: Text(l10n.menuLanguage),
                    content: const LanguageSelectorButton(),
                  ),
                );
              },
            ),
          ),
        ),
    ];
  }

  // ── Avatar helper ────────────────────────────────────────────────────────

  Widget _buildAvatar(
    BuildContext context,
    Person? person,
    PersonProvider personProvider, {
    bool isCollapsed = false,
  }) {
    final radius = isCollapsed ? 20.0 : 32.0;
    final size = radius * 2;
    final activeBusinessProfile = personProvider.activeBusinessProfile;
    final businessLogo = activeBusinessProfile?.logo;
    final isFemale = person?.gender?.toLowerCase() == 'female';
    final assetPath = isFemale
        ? 'assets/images/avatar_female.png'
        : 'assets/images/avatar_male.png';
    const businessAssetPath = 'assets/images/avatar_personal_trainer.png';

    if (activeBusinessProfile != null) {
      if (businessLogo != null && businessLogo.isNotEmpty) {
        return ClipOval(
          child: SizedBox(
            width: size,
            height: size,
            child: CachedNetworkImage(
              imageUrl: businessLogo,
              fit: BoxFit.cover,
              placeholder: (_, _) =>
                  Image.asset(businessAssetPath, fit: BoxFit.cover),
              errorWidget: (_, _, _) =>
                  Image.asset(businessAssetPath, fit: BoxFit.cover),
            ),
          ),
        );
      }
      return ClipOval(
        child: SizedBox(
          width: size,
          height: size,
          child: Image.asset(businessAssetPath, fit: BoxFit.cover),
        ),
      );
    }

    if (person?.avatar != null && person!.avatar!.isNotEmpty) {
      return ClipOval(
        child: SizedBox(
          width: size,
          height: size,
          child: CachedNetworkImage(
            imageUrl: person.avatar!,
            fit: BoxFit.cover,
            placeholder: (_, _) => Image.asset(assetPath, fit: BoxFit.cover),
            errorWidget: (_, _, _) => Image.asset(assetPath, fit: BoxFit.cover),
          ),
        ),
      );
    }

    return CircleAvatar(
      radius: radius,
      backgroundColor: Colors.white,
      child: ClipOval(
        child: Image.asset(
          assetPath,
          width: size,
          height: size,
          fit: BoxFit.cover,
        ),
      ),
    );
  }

  // ── Header ───────────────────────────────────────────────────────────────

  Widget _buildDrawerHeader(
    BuildContext context,
    AppLocalizations l10n,
    AuthProvider authProvider,
    PersonProvider personProvider,
  ) {
    final person = personProvider.person;
    final activeBusinessProfile = personProvider.activeBusinessProfile;
    final personName = person != null
        ? '${person.firstname} ${person.surname}'
        : '';
    final headerGradient = personProvider.isProfessional
        ? AppColors.professionalGradient3
        : AppColors.gradient3;

    if (isCollapsed) {
      return Container(
        width: double.infinity,
        padding: const EdgeInsets.symmetric(vertical: 8),
        decoration: BoxDecoration(gradient: headerGradient),
        child: Column(
          children: [
            IconButton(
              icon: const Icon(Icons.chevron_right, color: Colors.white),
              onPressed: onToggleCollapse,
              tooltip: l10n.menuCollapse,
            ),
            const SizedBox(height: 8),
            _buildAvatar(context, person, personProvider, isCollapsed: true),
          ],
        ),
      );
    }

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(gradient: headerGradient),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Align(
            alignment: Alignment.topRight,
            child: IconButton(
              icon: const Icon(Icons.chevron_left, color: Colors.white),
              onPressed: onToggleCollapse,
              tooltip: l10n.menuCollapse,
              padding: EdgeInsets.zero,
              constraints: const BoxConstraints(),
              iconSize: 20,
            ),
          ),
          const SizedBox(height: 4),
          _buildAvatar(context, person, personProvider),
          const SizedBox(height: 8),
          if (personName.isNotEmpty) ...[
            Text(
              personName,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 16,
                fontWeight: FontWeight.bold,
              ),
              overflow: TextOverflow.ellipsis,
              maxLines: 1,
            ),
            const SizedBox(height: 2),
          ],
          if (personProvider.isProfessional &&
              activeBusinessProfile != null) ...[
            Text(
              activeBusinessProfile.businessName,
              style: TextStyle(
                color: Colors.white.withAlpha(230),
                fontSize: 12,
              ),
              overflow: TextOverflow.ellipsis,
              maxLines: 1,
            ),
            const SizedBox(height: 2),
          ],
          Text(
            l10n.appTitle,
            style: TextStyle(
              color: personName.isNotEmpty
                  ? Colors.white.withAlpha(204)
                  : Colors.white,
              fontSize: personName.isNotEmpty ? 12 : 18,
              fontWeight: personName.isNotEmpty
                  ? FontWeight.normal
                  : FontWeight.bold,
            ),
            overflow: TextOverflow.ellipsis,
            maxLines: 1,
          ),
          const SizedBox(height: 2),
          Text(
            authProvider.isAuthenticated ? l10n.menuOnline : l10n.menuOffline,
            style: TextStyle(color: Colors.white.withAlpha(204), fontSize: 12),
            overflow: TextOverflow.ellipsis,
            maxLines: 1,
          ),
        ],
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sidebar Item
// ─────────────────────────────────────────────────────────────────────────────

class _SidebarItem extends StatelessWidget {
  final IconData icon;
  final IconData activeIcon;
  final String label;
  final VoidCallback? onTap;
  final Color? textColor;
  final Color? iconColor;
  final bool isActive;
  final bool isCollapsed;

  const _SidebarItem({
    required this.icon,
    required this.activeIcon,
    required this.label,
    this.onTap,
    this.textColor,
    this.iconColor,
    this.isActive = false,
    this.isCollapsed = false,
  });

  @override
  Widget build(BuildContext context) {
    final businessType = context.watch<PersonProvider>().activeBusinessProfile?.businessType;
    final effectiveIconColor =
        iconColor ?? (isActive ? AppColors.primaryFor(businessType) : const Color(0xFF555555));
    final effectiveTextColor =
        textColor ?? (isActive ? AppColors.primaryFor(businessType) : const Color(0xFF333333));

    if (isCollapsed) {
      return Tooltip(
        message: label,
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(8),
          hoverColor: AppColors.primaryFor(businessType).withAlpha(20),
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 12),
            child: Center(
              child: Icon(
                isActive ? activeIcon : icon,
                color: effectiveIconColor,
                size: 24,
              ),
            ),
          ),
        ),
      );
    }

    return ListTile(
      leading: Icon(
        isActive ? activeIcon : icon,
        color: effectiveIconColor,
        size: 24,
      ),
      title: Text(
        label,
        style: TextStyle(
          color: effectiveTextColor,
          fontSize: 15,
          fontWeight: isActive ? FontWeight.w600 : FontWeight.normal,
        ),
        overflow: TextOverflow.ellipsis,
        maxLines: 1,
      ),
      onTap: onTap,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      contentPadding: const EdgeInsets.symmetric(horizontal: 16),
      horizontalTitleGap: 16,
      hoverColor: AppColors.primaryFor(businessType).withAlpha(20),
      selectedTileColor: AppColors.primaryFor(businessType).withAlpha(25),
      selected: isActive,
      dense: true,
      visualDensity: VisualDensity.compact,
    );
  }
}
