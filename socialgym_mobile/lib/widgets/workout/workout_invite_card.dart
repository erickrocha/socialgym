import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../models/workout.dart';

/// Card for one workout assignment invite, on the Workout Invites screen.
/// Used for both the Received list (accept/reject in [trailing]) and the Sent
/// list ([statusLabel] chip + optional cancel button in [trailing]).
class WorkoutInviteCard extends StatelessWidget {
  final Workout workout;
  final String? subtitle;
  final String? statusLabel;
  final Widget trailing;

  const WorkoutInviteCard({
    super.key,
    required this.workout,
    required this.trailing,
    this.subtitle,
    this.statusLabel,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 1,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            Container(
              width: 44,
              height: 44,
              decoration: BoxDecoration(
                color: const Color(0xFFF1F1F1),
                borderRadius: BorderRadius.circular(10),
              ),
              alignment: Alignment.center,
              child: Text(
                _muscleEmoji(workout.muscleGroups),
                style: const TextStyle(fontSize: 20),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    workout.name,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                  ),
                  if (subtitle != null) ...[
                    const SizedBox(height: 2),
                    Text(
                      subtitle!,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(fontSize: 13, color: Color(0xFF888888)),
                    ),
                  ],
                  const SizedBox(height: 6),
                  Wrap(
                    spacing: 6,
                    runSpacing: 4,
                    crossAxisAlignment: WrapCrossAlignment.center,
                    children: [
                      _chip(_capitalize(workout.difficulty)),
                      _chip('${workout.exercises.length}'),
                      if (statusLabel != null) _statusChip(statusLabel!, workout.status),
                    ],
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

  Widget _chip(String label) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: const Color(0xFFF1F1F1),
        borderRadius: BorderRadius.circular(10),
      ),
      child: Text(label, style: const TextStyle(fontSize: 11, color: Color(0xFF555555))),
    );
  }

  Widget _statusChip(String label, String status) {
    final Color color = switch (status) {
      'Accepted' => AppColors.success,
      'Rejected' || 'Cancelled' => AppColors.danger,
      _ => AppColors.thirdHover,
    };
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(color: color, borderRadius: BorderRadius.circular(10)),
      child: Text(
        label,
        style: const TextStyle(fontSize: 11, color: Colors.white, fontWeight: FontWeight.w600),
      ),
    );
  }

  static String _capitalize(String s) =>
      s.isEmpty ? s : '${s[0].toUpperCase()}${s.substring(1)}';

  static String _muscleEmoji(List<String> groups) {
    final g = groups.isEmpty ? '' : groups.first.toLowerCase();
    const map = {
      'leg': '🦵',
      'glute': '🦵',
      'chest': '🫀',
      'back': '🔙',
      'arm': '💪',
      'bicep': '💪',
      'tricep': '💪',
      'shoulder': '🏋️',
      'core': '🎯',
      'abs': '🎯',
      'cardio': '🏃',
    };
    for (final entry in map.entries) {
      if (g.contains(entry.key)) {
        return entry.value;
      }
    }
    return '💪';
  }
}
