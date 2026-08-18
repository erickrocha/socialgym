import 'package:flutter/material.dart';

import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../widgets/main_layout.dart';

class FollowersPage extends StatelessWidget {
  const FollowersPage({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/followers',
      body: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.people_outline, size: 64, color: Colors.grey[500]),
            const SizedBox(height: 12),
            Text(
              l10n.menuFollowers,
              style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
            ),
            const SizedBox(height: 8),
            Text(l10n.comingSoon, style: TextStyle(color: Colors.grey[600])),
          ],
        ),
      ),
    );
  }
}
