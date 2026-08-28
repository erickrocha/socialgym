import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/auth_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/workout_session_provider.dart';
import '../../widgets/post_composer_sheet.dart';

class WorkoutCompleteDialog extends StatefulWidget {
  final Map<String, dynamic> workoutSession;

  const WorkoutCompleteDialog({super.key, required this.workoutSession});

  @override
  State<WorkoutCompleteDialog> createState() => _WorkoutCompleteDialogState();
}

class _WorkoutCompleteDialogState extends State<WorkoutCompleteDialog> {
  bool _sessionSaved = false;
  String _formatDuration(int seconds) {
    final mins = seconds ~/ 60;
    final secs = seconds % 60;
    return '$mins:${secs.toString().padLeft(2, '0')}';
  }

  int _getCompletedSets() {
    return (widget.workoutSession['executedSets'] as List?)?.length ?? 0;
  }

  double _getTotalVolume() {
    final sets = widget.workoutSession['executedSets'] as List? ?? [];
    return sets.fold<double>(0, (acc, set) {
      final weight = (set['weight'] as num?)?.toDouble() ?? 0;
      final reps = (set['repsOrDuration'] as num?)?.toInt() ?? 0;
      return acc + (weight * reps);
    });
  }

  String _getDayOfWeek(int index) {
    final l10n = AppLocalizations.of(context)!;
    final daysOfWeek = [
      '', // 0 index not used
      l10n.dayMonday,
      l10n.dayTuesday,
      l10n.dayWednesday,
      l10n.dayThursday,
      l10n.dayFriday,
      l10n.daySaturday,
      l10n.daySunday,
    ];
    return daysOfWeek[index];
  }

  Future<void> _saveSession(WorkoutSessionProvider provider) async {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    final person = context.read<PersonProvider>().person;

    final now = DateTime.now();

    final sessionData = {
      'personUuid': person?.uuid,
      'workoutName': widget.workoutSession['workoutName'],
      'duration': widget.workoutSession['duration'],
      'startedAt': widget.workoutSession['startedAt'],
      'dayOfWeek': _getDayOfWeek(now.weekday),
      'completedAt': now.toIso8601String(),
      'executedSets': get(widget.workoutSession['executedSets']),
      'totalVolume': _getTotalVolume(),
      'totalSets': _getCompletedSets(),
    };

    final success = await provider.saveSession(sessionData, token);
    if (success && mounted) {
      setState(() => _sessionSaved = true);
    }
  }

  List<Map<String, dynamic>> get(dynamic executedSets) {
    // Defensive: handle both List<Map> and List<dynamic>
    final sets = <Map<String, dynamic>>[];
    if (executedSets is List) {
      for (var set in executedSets) {
        if (set is Map) {
          sets.add({
            'uuid': set['uuid'],
            'exerciseName': set['exerciseName'],
            'ownerId': set['ownerId'],
            'ownerName': set['ownerName'],
            'category': set['category'],
            'visibility': set['visibility'],
            'setNumber': set['setNumber'],
            'repsOrDuration': set['repsOrDuration'],
            'weight': set['weight'],
            'startedAt': set['startedAt'],
            'completedAt': set['completedAt'],
          });
        }
      }
    }
    return sets;
  }

  void _navigateBack() {
    Navigator.of(context).pushNamedAndRemoveUntil('/workouts', (route) => false);
  }

  void _navigateToSessions() {
    Navigator.of(context).pushNamedAndRemoveUntil('/workout-sessions', (route) => false);
  }

  /// Builds the pre-filled workout summary to share as a post.
  String _buildShareContent(AppLocalizations l10n) {
    final name = widget.workoutSession['workoutName'] as String? ?? '';
    final duration = widget.workoutSession['duration'] as int? ?? 0;
    final sets = _getCompletedSets();
    final volume = _getTotalVolume().toStringAsFixed(3);
    return '💪 ${l10n.feedShareWorkoutSummary}\n'
        '🏋️ $name\n'
        '⏱️ ${l10n.executionDuration}: ${_formatDuration(duration)}\n'
        '✅ ${l10n.executionSetsCompleted}: $sets\n'
        '📊 ${l10n.executionTotalVolume}: $volume ${l10n.workoutWeightUnit}\n'
        '#SocialGym #Workout #Fitness';
  }

  Future<void> _shareWorkout() async {
    final l10n = AppLocalizations.of(context)!;
    await PostComposerSheet.show(context, initialContent: _buildShareContent(l10n));
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final duration = widget.workoutSession['duration'] as int? ?? 0;
    final businessType = context.watch<PersonProvider>().activeBusinessProfile?.businessType;

    return Consumer<WorkoutSessionProvider>(
      builder: (context, provider, _) {
        return Scaffold(
          backgroundColor: AppColors.background,
          body: SafeArea(
            child: Center(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(24),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    // Celebration
                    const Text('🏆', style: TextStyle(fontSize: 64)),
                    const SizedBox(height: 8),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: const [
                        Text('🎉', style: TextStyle(fontSize: 28)),
                        SizedBox(width: 8),
                        Text('💪', style: TextStyle(fontSize: 28)),
                        SizedBox(width: 8),
                        Text('⭐', style: TextStyle(fontSize: 28)),
                        SizedBox(width: 8),
                        Text('🔥', style: TextStyle(fontSize: 28)),
                      ],
                    ),
                    const SizedBox(height: 24),

                    // Title
                    Text(
                      l10n.executionWorkoutComplete,
                      style: const TextStyle(
                        fontSize: 28,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF333333),
                      ),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 8),
                    Text(
                      widget.workoutSession['workoutName'] as String? ?? '',
                      style: TextStyle(
                        fontSize: 18,
                        color: AppColors.primaryFor(businessType),
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    const SizedBox(height: 32),

                    // Stats
                    Container(
                      padding: const EdgeInsets.all(20),
                      decoration: BoxDecoration(
                        color: Colors.white,
                        borderRadius: BorderRadius.circular(16),
                        boxShadow: [
                          BoxShadow(
                            color: Colors.black.withAlpha(15),
                            blurRadius: 6,
                            offset: const Offset(0, 2),
                          ),
                        ],
                      ),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                        children: [
                          _StatItem(
                            icon: '⏱️',
                            value: _formatDuration(duration),
                            label: l10n.executionDuration,
                          ),
                          Container(width: 1, height: 48, color: Colors.grey.withAlpha(50)),
                          _StatItem(
                            icon: '✓',
                            value: '${_getCompletedSets()}',
                            label: l10n.executionSetsCompleted,
                          ),
                          Container(width: 1, height: 48, color: Colors.grey.withAlpha(50)),
                          _StatItem(
                            icon: '🏋️',
                            value: _getTotalVolume().toStringAsFixed(3),
                            label: '${l10n.executionTotalVolume} (${l10n.workoutWeightUnit})',
                          ),
                        ],
                      ),
                    ),
                    const SizedBox(height: 24),

                    // Motivational message
                    Container(
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        color: AppColors.primaryFor(businessType).withAlpha(20),
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Text(
                        l10n.executionGreatJob,
                        style: TextStyle(
                          fontSize: 16,
                          fontWeight: FontWeight.w600,
                          color: AppColors.primaryFor(businessType),
                        ),
                        textAlign: TextAlign.center,
                      ),
                    ),
                    const SizedBox(height: 32),

                    // Error message
                    if (provider.error != null) ...[
                      Container(
                        padding: const EdgeInsets.all(12),
                        decoration: BoxDecoration(
                          color: Colors.red.withAlpha(20),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: Row(
                          children: [
                            const Icon(Icons.error_outline, color: Colors.red, size: 20),
                            const SizedBox(width: 8),
                            Expanded(
                              child: Text(
                                provider.error!,
                                style: const TextStyle(color: Colors.red, fontSize: 14),
                              ),
                            ),
                          ],
                        ),
                      ),
                      const SizedBox(height: 16),
                    ],

                    // Actions
                    if (_sessionSaved) ...[
                      // Session saved — offer to view progress or share
                      SizedBox(
                        width: double.infinity,
                        child: ElevatedButton.icon(
                          onPressed: _navigateToSessions,
                          icon: const Icon(Icons.bar_chart),
                          label: Text(l10n.sessionsViewProgress),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.primaryFor(businessType),
                            foregroundColor: Colors.white,
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                            padding: const EdgeInsets.symmetric(vertical: 14),
                          ),
                        ),
                      ),
                      const SizedBox(height: 8),
                      SizedBox(
                        width: double.infinity,
                        child: ElevatedButton.icon(
                          onPressed: _shareWorkout,
                          icon: const Icon(Icons.share_outlined),
                          label: Text(l10n.feedShareWorkout),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.secondary,
                            foregroundColor: Colors.white,
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                            padding: const EdgeInsets.symmetric(vertical: 14),
                          ),
                        ),
                      ),
                      const SizedBox(height: 8),
                      SizedBox(
                        width: double.infinity,
                        child: OutlinedButton(
                          onPressed: _navigateBack,
                          style: OutlinedButton.styleFrom(
                            foregroundColor: AppColors.primaryFor(businessType),
                            side: BorderSide(color: AppColors.primaryFor(businessType)),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                            padding: const EdgeInsets.symmetric(vertical: 14),
                          ),
                          child: Text(l10n.executionClose),
                        ),
                      ),
                    ] else ...[
                      Row(
                        children: [
                          Expanded(
                            child: OutlinedButton(
                              onPressed: provider.saving ? null : _navigateBack,
                              style: OutlinedButton.styleFrom(
                                foregroundColor: AppColors.primaryFor(businessType),
                                side: BorderSide(color: AppColors.primaryFor(businessType)),
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(12),
                                ),
                                padding: const EdgeInsets.symmetric(vertical: 14),
                              ),
                              child: Text(l10n.executionClose),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: ElevatedButton.icon(
                              onPressed: provider.saving ? null : () => _saveSession(provider),
                              icon: provider.saving
                                  ? const SizedBox(
                                      width: 16,
                                      height: 16,
                                      child: CircularProgressIndicator(
                                        strokeWidth: 2,
                                        color: Colors.white,
                                      ),
                                    )
                                  : const Text('💾', style: TextStyle(fontSize: 16)),
                              label: Text(
                                provider.saving ? l10n.executionSaving : l10n.executionSaveSession,
                              ),
                              style: ElevatedButton.styleFrom(
                                backgroundColor: AppColors.primaryFor(businessType),
                                foregroundColor: Colors.white,
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(12),
                                ),
                                padding: const EdgeInsets.symmetric(vertical: 14),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ],
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

class _StatItem extends StatelessWidget {
  final String icon;
  final String value;
  final String label;

  const _StatItem({required this.icon, required this.value, required this.label});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text(icon, style: const TextStyle(fontSize: 20)),
        const SizedBox(height: 4),
        Text(
          value,
          style: const TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold,
            color: Color(0xFF333333),
          ),
        ),
        const SizedBox(height: 2),
        Text(
          label,
          style: const TextStyle(fontSize: 11, color: Color(0xFF888888)),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }
}
