import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../config/app_colors.dart';
import '../models/exercise.dart';
import '../models/visibility_option.dart';
import '../providers/person_provider.dart';

class ExerciseItemPaginatedWidget extends StatelessWidget {
  final Exercise exercise;
  final bool isSelected;
  final VoidCallback? onTap;

  const ExerciseItemPaginatedWidget({
    super.key,
    required this.exercise,
    required this.isSelected,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final visibility = VisibilityOption.fromApiValue(exercise.visibility);
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

    return GestureDetector(
      onTap: onTap,
      child: Container(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: isSelected
                ? AppColors.primaryFor(businessType)
                : Colors.grey[300] ?? Colors.grey,
            width: isSelected ? 2 : 1,
          ),
          color: isSelected
              ? AppColors.primaryFor(businessType).withValues(alpha: 0.05)
              : Colors.white,
        ),
        child: Padding(
          padding: const EdgeInsets.all(12),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Owner name and checkbox row
              Row(
                children: [
                  const SizedBox(width: 8),
                  Icon(
                    Icons.person,
                    size: 18,
                    color: AppColors.primaryFor(businessType),
                  ),
                  const SizedBox(width: 6),
                  Flexible(
                    child: Text(
                      exercise.ownerName,
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w500,
                        color: Colors.black87,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  const Spacer(),
                  // Category and Visibility
                  Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 8,
                      vertical: 4,
                    ),
                    decoration: BoxDecoration(
                      color: Colors.grey[100],
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: Text(
                      exercise.category,
                      style: TextStyle(
                        fontSize: 12,
                        fontWeight: FontWeight.w600,
                        color: AppColors.primaryFor(businessType),
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Text(visibility.emoji, style: const TextStyle(fontSize: 16)),
                ],
              ),
              const SizedBox(height: 12),
              // Exercise name
              Text(
                exercise.name,
                style: const TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: Colors.black,
                ),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
              const SizedBox(height: 8),
              // Sets and reps
              Row(
                children: [
                  _buildInfoChip(
                    icon: Icons.repeat,
                    label: '${exercise.sets} sets',
                    businessType: businessType,
                  ),
                  const SizedBox(width: 12),
                  _buildInfoChip(
                    icon: Icons.fitness_center,
                    label: '${exercise.repsOrDuration} reps',
                    businessType: businessType,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildInfoChip({
    required IconData icon,
    required String label,
    required String? businessType,
  }) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(icon, size: 16, color: AppColors.primaryFor(businessType)),
        const SizedBox(width: 4),
        Text(
          label,
          style: const TextStyle(
            fontSize: 13,
            color: Colors.black54,
            fontWeight: FontWeight.w500,
          ),
        ),
      ],
    );
  }
}
