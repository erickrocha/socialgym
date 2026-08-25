import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../models/person.dart';
import '../../pages/profile/person_profile_page.dart';
import '../../providers/person_provider.dart';

/// Displays a [Person] row (team member or invitee), navigating to their
/// profile on tap, mirroring FriendsPage's `_buildPersonCard`.
class TeamMemberCard extends StatelessWidget {
  final Person person;
  final Widget trailing;

  const TeamMemberCard({super.key, required this.person, required this.trailing});

  @override
  Widget build(BuildContext context) {
    final businessType = context.watch<PersonProvider>().activeBusinessProfile?.businessType;
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 1,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        onTap: () => Navigator.of(
          context,
        ).push(MaterialPageRoute(builder: (_) => PersonProfilePage(person: person))),
        child: Padding(
          padding: const EdgeInsets.all(12),
          child: Row(
            children: [
              _buildAvatar(businessType),
              const SizedBox(width: 12),
              Expanded(
                child: Text(
                  person.fullName,
                  style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                ),
              ),
              trailing,
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildAvatar(String? businessType) {
    return Container(
      width: 50,
      height: 50,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        border: Border.all(color: AppColors.primaryFor(businessType).withAlpha(51), width: 2),
      ),
      child: ClipOval(
        child: person.avatar != null
            ? CachedNetworkImage(
                imageUrl: person.avatar!,
                fit: BoxFit.cover,
                placeholder: (context, url) => Container(
                  color: AppColors.primaryFor(businessType).withAlpha(51),
                  child: const Center(child: CircularProgressIndicator(strokeWidth: 2)),
                ),
                errorWidget: (context, url, error) => _buildDefaultAvatar(),
              )
            : _buildDefaultAvatar(),
      ),
    );
  }

  Widget _buildDefaultAvatar() {
    return Image.asset(
      person.gender?.toLowerCase() == 'female'
          ? 'assets/images/avatar_female.png'
          : 'assets/images/avatar_male.png',
      fit: BoxFit.cover,
    );
  }
}
