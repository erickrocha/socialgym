import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../services/grpc/grpc_team_member_service.dart';

/// Sentinel popped by the picker sheet when the user explicitly chooses
/// "Myself" — distinct from `null`, which means the sheet was dismissed
/// without a choice (so the previous selection should be kept).
const Object _myself = Object();

/// Tap target (styled like a form field) that opens a bottom sheet to pick
/// one of the business profile's Accepted team members, or "Myself" (null)
/// to keep the workout owned by the business profile itself.
class TeamMemberPickerField extends StatelessWidget {
  final int businessProfileId;
  final Person? selected;
  final ValueChanged<Person?> onChanged;
  final InputDecoration? decoration;

  const TeamMemberPickerField({
    super.key,
    required this.businessProfileId,
    required this.selected,
    required this.onChanged,
    this.decoration,
  });

  Future<void> _openPicker(BuildContext context) async {
    final result = await showModalBottomSheet<Object?>(
      context: context,
      isScrollControlled: true,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (_) => _TeamMemberPickerSheet(
        businessProfileId: businessProfileId,
        selected: selected,
      ),
    );
    if (result == null) return; // dismissed without choosing
    onChanged(identical(result, _myself) ? null : result as Person);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return InputDecorator(
      decoration: decoration ?? InputDecoration(labelText: l10n.workoutAssignToTeamMember),
      child: InkWell(
        onTap: () => _openPicker(context),
        child: Row(
          children: [
            Expanded(
              child: Text(
                selected?.fullName ?? l10n.workoutAssignToMyself,
                overflow: TextOverflow.ellipsis,
              ),
            ),
            const Icon(Icons.arrow_drop_down),
          ],
        ),
      ),
    );
  }
}

class _TeamMemberPickerSheet extends StatefulWidget {
  final int businessProfileId;
  final Person? selected;

  const _TeamMemberPickerSheet({required this.businessProfileId, required this.selected});

  @override
  State<_TeamMemberPickerSheet> createState() => _TeamMemberPickerSheetState();
}

class _TeamMemberPickerSheetState extends State<_TeamMemberPickerSheet> {
  late Future<List<Person>> _membersFuture;

  @override
  void initState() {
    super.initState();
    _membersFuture = GrpcTeamMemberService.getTeamMemberPage(
      businessProfileId: widget.businessProfileId,
    ).then((data) => data.members);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Text(
                l10n.workoutSelectTeamMember,
                style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
              ),
            ),
            const SizedBox(height: 12),
            ListTile(
              leading: const CircleIcon(icon: Icons.person),
              title: Text(l10n.workoutAssignToMyself),
              selected: widget.selected == null,
              onTap: () => Navigator.of(context).pop(_myself),
            ),
            const Divider(height: 1),
            Flexible(
              child: FutureBuilder<List<Person>>(
                future: _membersFuture,
                builder: (context, snapshot) {
                  if (snapshot.connectionState != ConnectionState.done) {
                    return const Padding(
                      padding: EdgeInsets.symmetric(vertical: 24),
                      child: Center(child: CircularProgressIndicator()),
                    );
                  }
                  final members = snapshot.data ?? const [];
                  if (members.isEmpty) {
                    return Padding(
                      padding: const EdgeInsets.symmetric(vertical: 24, horizontal: 16),
                      child: Text(
                        l10n.workoutNoTeamMembers,
                        textAlign: TextAlign.center,
                        style: TextStyle(color: Colors.grey[600]),
                      ),
                    );
                  }
                  return ListView.builder(
                    shrinkWrap: true,
                    itemCount: members.length,
                    itemBuilder: (context, index) {
                      final person = members[index];
                      return ListTile(
                        leading: _PersonAvatar(person: person),
                        title: Text(person.fullName),
                        selected: widget.selected?.uuid == person.uuid,
                        onTap: () => Navigator.of(context).pop(person),
                      );
                    },
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class CircleIcon extends StatelessWidget {
  final IconData icon;
  const CircleIcon({super.key, required this.icon});

  @override
  Widget build(BuildContext context) {
    return CircleAvatar(
      backgroundColor: AppColors.primary.withAlpha(40),
      child: Icon(icon, color: AppColors.primary),
    );
  }
}

class _PersonAvatar extends StatelessWidget {
  final Person person;
  const _PersonAvatar({required this.person});

  @override
  Widget build(BuildContext context) {
    return CircleAvatar(
      backgroundColor: AppColors.primary.withAlpha(40),
      backgroundImage: person.avatar != null ? CachedNetworkImageProvider(person.avatar!) : null,
      child: person.avatar == null ? const Icon(Icons.person, color: AppColors.primary) : null,
    );
  }
}
