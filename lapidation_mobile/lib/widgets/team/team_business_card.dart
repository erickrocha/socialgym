import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../models/business_profile.dart';

/// Displays a [BusinessProfile] row (a team the person joined, or a pending
/// invite received from a business). Read-only card, no per-profile detail
/// page exists yet to navigate to.
class TeamBusinessCard extends StatelessWidget {
  final BusinessProfile business;
  final Widget trailing;

  const TeamBusinessCard({super.key, required this.business, required this.trailing});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 1,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      clipBehavior: Clip.antiAlias,
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            _buildLogo(),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    business.businessName,
                    style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                  ),
                  Text(
                    business.businessType,
                    style: TextStyle(fontSize: 13, color: Colors.grey[600]),
                  ),
                ],
              ),
            ),
            trailing,
          ],
        ),
      ),
    );
  }

  Widget _buildLogo() {
    return Container(
      width: 50,
      height: 50,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        border: Border.all(color: AppColors.professionalSecondary.withAlpha(51), width: 2),
      ),
      child: ClipOval(
        child: business.logo != null
            ? CachedNetworkImage(
                imageUrl: business.logo!,
                fit: BoxFit.cover,
                placeholder: (context, url) => Container(
                  color: AppColors.professionalSecondary.withAlpha(51),
                  child: const Center(child: CircularProgressIndicator(strokeWidth: 2)),
                ),
                errorWidget: (context, url, error) => _buildDefaultLogo(),
              )
            : _buildDefaultLogo(),
      ),
    );
  }

  Widget _buildDefaultLogo() {
    return Image.asset('assets/images/avatar_personal_trainer.png', fit: BoxFit.cover);
  }
}
