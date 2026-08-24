import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../config/app_colors.dart';
import '../config/nav_section.dart';
import '../l10n/app_localizations.dart';
import '../providers/person_provider.dart';
import 'profile_menu.dart';

class AppHeader extends StatelessWidget implements PreferredSizeWidget {
  final VoidCallback? onMenuPressed;
  final VoidCallback? onSearchPressed;
  final VoidCallback? onNotificationsPressed;
  final VoidCallback? onProfilePressed;
  final int notificationCount;

  /// Which top-level section is currently active.
  /// Drives the highlighted tab in the center of the header.
  final NavSection currentSection;

  /// Shows the hamburger menu button even on desktop layouts —
  /// used when the sidebar has no persistent desktop panel (sheet style).
  final bool forceMenuButton;

  const AppHeader({
    super.key,
    this.onMenuPressed,
    this.onSearchPressed,
    this.onNotificationsPressed,
    this.onProfilePressed,
    this.notificationCount = 0,
    this.currentSection = NavSection.home,
    this.forceMenuButton = false,
  });

  @override
  Size get preferredSize => const Size.fromHeight(56);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final screenWidth = MediaQuery.of(context).size.width;
    final isDesktop = screenWidth >= 768;
    final isProfessional = context.watch<PersonProvider>().isProfessional;

    return Container(
      decoration: BoxDecoration(
        gradient: isProfessional ? AppColors.professionalGradient3 : AppColors.gradient3,
        boxShadow: [
          BoxShadow(
            color: Color(0x29000000),
            blurRadius: 6,
            offset: Offset(0, 2),
          ),
        ],
      ),
      child: SafeArea(
        bottom: false,
        child: SizedBox(
          height: 56,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8),
            child: Row(
              children: [
                // ── Left: hamburger (mobile, or forced sheet-style sidebar) + logo ──
                if (!isDesktop || forceMenuButton)
                  SizedBox(
                    width: 36,
                    height: 36,
                    child: IconButton(
                      icon: const Icon(Icons.menu, color: Colors.white),
                      tooltip: l10n.menuOpen,
                      onPressed: onMenuPressed,
                      padding: EdgeInsets.zero,
                      constraints: const BoxConstraints(),
                    ),
                  ),
                GestureDetector(
                  onTap: () => Navigator.of(context).pushNamedAndRemoveUntil(
                    '/feed',
                    (route) => false,
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Image.asset(
                        'assets/images/logo.png',
                        width: 32,
                        height: 32,
                      ),
                      if (isDesktop) ...[
                        const SizedBox(width: 8),
                        Text(
                          l10n.appTitle,
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ],
                    ],
                  ),
                ),

                const SizedBox(width: 8),

                // ── Center: scrollable section tabs ───────────────────
                // Each NavSection value becomes a tab automatically.
                // To add a new section, just add a value to NavSection.
                Expanded(
                  child: Center(
                    child: SingleChildScrollView(
                      scrollDirection: Axis.horizontal,
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: NavSection.values
                            .map(
                              (s) => _SectionTab(
                                section: s,
                                isActive: s == currentSection,
                                showLabel: isDesktop,
                                label: _sectionLabel(s, l10n),
                                isProfessional: isProfessional,
                              ),
                            )
                            .toList(),
                      ),
                    ),
                  ),
                ),

                const SizedBox(width: 4),

                // ── Right: search + notifications + profile ───────────
                _NavIconCompact(
                  icon: Icons.search,
                  tooltip: l10n.searchPlaceholder,
                  onPressed: onSearchPressed,
                ),
                _NotificationIcon(
                  count: notificationCount,
                  onPressed: onNotificationsPressed,
                  tooltip: l10n.menuNotifications,
                ),
                ProfileMenu(onProfilePressed: onProfilePressed),
              ],
            ),
          ),
        ),
      ),
    );
  }

  String _sectionLabel(NavSection section, AppLocalizations l10n) {
    switch (section) {
      case NavSection.home:
        return l10n.menuHome;
      case NavSection.gallery:
        return 'Gallery';
      case NavSection.workout:
        return l10n.menuWorkouts;
    }
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Section Tab — one per NavSection, rendered in the center of the header.
// ─────────────────────────────────────────────────────────────────────────────

class _SectionTab extends StatelessWidget {
  final NavSection section;
  final bool isActive;
  final bool showLabel; // true on desktop
  final String label;
  final bool isProfessional;

  const _SectionTab({
    required this.section,
    required this.isActive,
    required this.showLabel,
    required this.label,
    required this.isProfessional,
  });

  @override
  Widget build(BuildContext context) {
    final iconColor =
        isActive ? Colors.white : Colors.white.withAlpha(153);
    final textColor =
        isActive ? Colors.white : Colors.white.withAlpha(153);

    return InkWell(
      onTap: () => Navigator.of(context).pushNamedAndRemoveUntil(
        section.rootRouteFor(isProfessional),
        (route) => false,
      ),
      borderRadius: BorderRadius.circular(4),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 4),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              isActive ? section.activeIcon : section.icon,
              color: iconColor,
              size: 22,
            ),
            if (showLabel) ...[
              const SizedBox(height: 1),
              Text(
                label,
                style: TextStyle(
                  color: textColor,
                  fontSize: 11,
                  fontWeight:
                      isActive ? FontWeight.w600 : FontWeight.normal,
                ),
              ),
            ],
            const SizedBox(height: 2),
            // Active indicator bar — animates in/out
            AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              height: 3,
              width: isActive ? 24 : 0,
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Notification icon with badge
// ─────────────────────────────────────────────────────────────────────────────

class _NotificationIcon extends StatelessWidget {
  final int count;
  final VoidCallback? onPressed;
  final String tooltip;

  const _NotificationIcon({
    required this.count,
    this.onPressed,
    required this.tooltip,
  });

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        _NavIconCompact(
          icon: Icons.notifications_outlined,
          tooltip: tooltip,
          onPressed: onPressed,
        ),
        if (count > 0)
          Positioned(
            right: 2,
            top: 2,
            child: Container(
              padding: const EdgeInsets.all(2),
              decoration: BoxDecoration(
                color: AppColors.danger,
                borderRadius: BorderRadius.circular(8),
              ),
              constraints:
                  const BoxConstraints(minWidth: 14, minHeight: 14),
              child: Text(
                count > 99 ? '99+' : '$count',
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 9,
                  fontWeight: FontWeight.bold,
                ),
                textAlign: TextAlign.center,
              ),
            ),
          ),
      ],
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Compact icon button helper
// ─────────────────────────────────────────────────────────────────────────────

class _NavIconCompact extends StatelessWidget {
  final IconData icon;
  final String tooltip;
  final VoidCallback? onPressed;

  const _NavIconCompact({
    required this.icon,
    required this.tooltip,
    this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 36,
      height: 36,
      child: IconButton(
        icon: Icon(icon, color: Colors.white, size: 22),
        tooltip: tooltip,
        padding: EdgeInsets.zero,
        constraints: const BoxConstraints(),
        onPressed: onPressed,
      ),
    );
  }
}
