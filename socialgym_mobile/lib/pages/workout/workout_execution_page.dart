import 'dart:async';

import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/workout.dart';
import 'set_detail_page.dart';
import 'workout_complete_dialog.dart';

class WorkoutExecutionPage extends StatefulWidget {
  final Workout workout;

  const WorkoutExecutionPage({super.key, required this.workout});

  @override
  State<WorkoutExecutionPage> createState() => _WorkoutExecutionPageState();
}

class _WorkoutExecutionPageState extends State<WorkoutExecutionPage> {
  int _currentExerciseIndex = 0;
  int _currentSetIndex = 0;
  final List<Map<String, dynamic>> _executedSets = [];
  bool _isEditingWeight = false;
  bool _isEditingReps = false;
  late double _editWeight;
  late int _editRepsOrDuration;
  int _elapsedSeconds = 0;
  Timer? _timer;
  late DateTime _startAt;

  late PageController _pageController;

  @override
  void initState() {
    super.initState();
    _pageController = PageController();
    final exercise = _currentExercise;
    _editWeight = exercise?.weight ?? 0;
    _editRepsOrDuration = exercise?.repsOrDuration ?? 0;
    _startAt = DateTime.now();
    _startTimer();
  }

  @override
  void dispose() {
    _timer?.cancel();
    _pageController.dispose();
    super.dispose();
  }

  void _startTimer() {
    _timer = Timer.periodic(const Duration(seconds: 1), (_) {
      if (mounted) setState(() => _elapsedSeconds++);
    });
  }

  List<dynamic> get _exercises => widget.workout.exercises;
  dynamic get _currentExercise => _exercises.isNotEmpty ? _exercises[_currentExerciseIndex] : null;
  int get _totalSets => _currentExercise?.sets ?? 0;
  bool get _isLastExercise => _currentExerciseIndex == _exercises.length - 1;
  bool get _isLastSet => _currentSetIndex == _totalSets - 1;
  bool get _isFirstExercise => _currentExerciseIndex == 0;

  double get _progress {
    if (_exercises.isEmpty) return 0;
    final totalSetsAll = _exercises.fold<int>(0, (acc, ex) => acc + (ex.sets as int));
    if (totalSetsAll == 0) return 0;
    return (_executedSets.length / totalSetsAll) * 100;
  }

  String _formatTime(int seconds) {
    final mins = seconds ~/ 60;
    final secs = seconds % 60;
    return '${mins.toString().padLeft(2, '0')}:${secs.toString().padLeft(2, '0')}';
  }

  int _completedSetsForExercise(int exerciseIdx) {
    if (exerciseIdx >= _exercises.length) return 0;
    final ex = _exercises[exerciseIdx];
    return _executedSets.where((s) => s['uuid'] == ex.uuid).length;
  }

  void _updateEditValuesForExercise(int exerciseIdx) {
    if (exerciseIdx >= _exercises.length) return;
    final ex = _exercises[exerciseIdx];
    final doneSets = _executedSets.where((s) => s['uuid'] == ex.uuid).toList();
    if (doneSets.isNotEmpty) {
      final last = doneSets.last;
      _editWeight = (last['weight'] as num?)?.toDouble() ?? ex.weight;
      _editRepsOrDuration = (last['repsOrDuration'] as num?)?.toInt() ?? ex.repsOrDuration;
    } else {
      _editWeight = ex.weight;
      _editRepsOrDuration = ex.repsOrDuration;
    }
  }

  void _confirmSet() {
    final ex = _currentExercise;
    _executedSets.add({
      'exerciseId': ex.id,
      'exerciseName': ex.name,
      'ownerId': ex.ownerId,
      'ownerName': ex.ownerName,
      'description': ex.description,
      'sets': ex.sets,
      'category': ex.category,
      'visibility': ex.visibility,
      'createdAt': ex.createdAt.toIso8601String(),
      'updatedAt': ex.updatedAt.toIso8601String(),
      'uuid': ex.uuid,
      'setNumber': _currentSetIndex + 1,
      'weight': _editWeight,
      'repsOrDuration': _editRepsOrDuration,
      'startedAt': _startAt.toIso8601String(),
      'completedAt': DateTime.now().toIso8601String(),
    });
    _isEditingWeight = false;
    _isEditingReps = false;

    if (_isLastSet) {
      if (_isLastExercise) {
        _completeWorkout();
      } else {
        _goToNextExercise();
      }
    } else {
      setState(() => _currentSetIndex++);
    }
  }

  void _skipSet() {
    if (_isLastSet) {
      if (_isLastExercise) {
        _completeWorkout();
      } else {
        _goToNextExercise();
      }
    } else {
      setState(() => _currentSetIndex++);
    }
  }

  void _goToNextExercise() {
    final nextIdx = _currentExerciseIndex + 1;
    setState(() {
      _currentExerciseIndex = nextIdx;
      _currentSetIndex = _completedSetsForExercise(nextIdx);
      _isEditingWeight = false;
      _isEditingReps = false;
      _updateEditValuesForExercise(nextIdx);
    });
    _pageController.animateToPage(
      nextIdx,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  void _goToPreviousExercise() {
    if (_isFirstExercise) return;
    final prevIdx = _currentExerciseIndex - 1;
    final completed = _completedSetsForExercise(prevIdx);
    setState(() {
      _currentExerciseIndex = prevIdx;
      _currentSetIndex = completed > 0 ? completed - 1 : 0;
      _isEditingWeight = false;
      _isEditingReps = false;
      _updateEditValuesForExercise(prevIdx);
    });
    _pageController.animateToPage(
      prevIdx,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  void _navigateToExercise(int idx) {
    final completed = _completedSetsForExercise(idx);
    setState(() {
      _currentExerciseIndex = idx;
      _currentSetIndex = completed > 0 ? completed - 1 : 0;
      _isEditingWeight = false;
      _isEditingReps = false;
      _updateEditValuesForExercise(idx);
    });
    _pageController.animateToPage(
      idx,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  void _completeWorkout() {
    _timer?.cancel();
    final session = {
      'workoutId': widget.workout.id,
      'workoutName': widget.workout.name,
      'duration': _elapsedSeconds,
      'executedSets': _executedSets,
      'startedAt': _startAt.toIso8601String(),
    };
    Navigator.of(context).pushReplacement(
      MaterialPageRoute(builder: (_) => WorkoutCompleteDialog(workoutSession: session)),
    );
  }

  Future<void> _editCompletedSet(int exerciseIdx, int setIdx) async {
    final ex = _exercises[exerciseIdx];
    final globalIdx = _executedSets.indexWhere(
      (s) => s['uuid'] == ex.uuid && s['setNumber'] == setIdx + 1,
    );
    if (globalIdx == -1) return;

    final updated = await Navigator.of(context).push<Map<String, dynamic>>(
      MaterialPageRoute(
        builder: (_) => SetDetailPage(
          set: _executedSets[globalIdx],
          onSave: (updatedSet) {
            setState(() => _executedSets[globalIdx] = updatedSet);
          },
        ),
      ),
    );
    if (updated != null && mounted) {
      setState(() => _executedSets[globalIdx] = updated);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    if (_exercises.isEmpty) {
      return Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Text('🏋️', style: TextStyle(fontSize: 48)),
              const SizedBox(height: 16),
              Text(l10n.workoutNoExercises),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () => Navigator.of(context).pop(),
                child: Text(l10n.buttonCancel),
              ),
            ],
          ),
        ),
      );
    }

    return Scaffold(
      backgroundColor: AppColors.background,
      body: SafeArea(
        child: Column(
          children: [
            _buildHeader(l10n),
            _buildExerciseDots(),
            Expanded(
              child: PageView.builder(
                controller: _pageController,
                physics: const ClampingScrollPhysics(),
                itemCount: _exercises.length,
                onPageChanged: (idx) {
                  final completed = _completedSetsForExercise(idx);
                  setState(() {
                    _currentExerciseIndex = idx;
                    _currentSetIndex = completed > 0 ? completed - 1 : 0;
                    _isEditingWeight = false;
                    _isEditingReps = false;
                    _updateEditValuesForExercise(idx);
                  });
                },
                itemBuilder: (context, pageIdx) {
                  return _buildExercisePage(l10n, pageIdx);
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildHeader(AppLocalizations l10n) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
      decoration: const BoxDecoration(gradient: AppColors.gradient3),
      child: Column(
        children: [
          Row(
            children: [
              IconButton(
                icon: const Icon(Icons.arrow_back, color: Colors.white),
                onPressed: () => _showExitConfirmation(l10n),
              ),
              Expanded(
                child: Column(
                  children: [
                    Text(
                      widget.workout.name,
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    Text(
                      '⏱️ ${_formatTime(_elapsedSeconds)}',
                      style: TextStyle(color: Colors.white.withAlpha(204), fontSize: 14),
                    ),
                  ],
                ),
              ),
              const SizedBox(width: 48),
            ],
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              const SizedBox(width: 8),
              Expanded(
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(4),
                  child: LinearProgressIndicator(
                    value: _progress / 100,
                    backgroundColor: Colors.white.withAlpha(51),
                    valueColor: const AlwaysStoppedAnimation(Colors.white),
                    minHeight: 6,
                  ),
                ),
              ),
              const SizedBox(width: 8),
              Text(
                '${_progress.round()}%',
                style: const TextStyle(color: Colors.white, fontSize: 12),
              ),
              const SizedBox(width: 8),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildExerciseDots() {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 12),
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const SizedBox(width: 16),
            ..._exercises.asMap().entries.map((entry) {
              final idx = entry.key;
              final isCompleted = idx < _currentExerciseIndex;
              final isActive = idx == _currentExerciseIndex;
              return GestureDetector(
                onTap: () => _navigateToExercise(idx),
                child: Container(
                  width: isActive ? 16 : 10,
                  height: isActive ? 16 : 10,
                  margin: const EdgeInsets.symmetric(horizontal: 3),
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: isCompleted
                        ? AppColors.success
                        : isActive
                        ? AppColors.primary
                        : Colors.grey.withAlpha(76),
                    border: isActive ? Border.all(color: AppColors.primary, width: 2) : null,
                  ),
                ),
              );
            }),
            const SizedBox(width: 16),
          ],
        ),
      ),
    );
  }

  Widget _buildExercisePage(AppLocalizations l10n, int pageIdx) {
    final exercise = _exercises[pageIdx];
    final isCurrentPage = pageIdx == _currentExerciseIndex;
    final completedForExercise = _completedSetsForExercise(pageIdx);
    final totalSets = exercise.sets as int;
    final activeSetIndex = isCurrentPage ? _currentSetIndex : -1;
    final isCardio = (exercise.category as String).toLowerCase() == 'cardio';

    return SingleChildScrollView(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 24),
      child: Column(
        children: [
          // Navigation hint row
          if (_exercises.length > 1)
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 6),
              child: Row(
                children: [
                  if (pageIdx > 0)
                    Expanded(
                      child: GestureDetector(
                        onTap: () => _navigateToExercise(pageIdx - 1),
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            const Icon(Icons.chevron_left, size: 16, color: Color(0xFF888888)),
                            Flexible(
                              child: Text(
                                _exercises[pageIdx - 1].name as String,
                                style: const TextStyle(fontSize: 11, color: Color(0xFF888888)),
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                          ],
                        ),
                      ),
                    )
                  else
                    const Expanded(child: SizedBox()),
                  if (pageIdx < _exercises.length - 1)
                    Expanded(
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Flexible(
                            child: Text(
                              _exercises[pageIdx + 1].name as String,
                              style: const TextStyle(fontSize: 11, color: Color(0xFFAAAAAA)),
                              overflow: TextOverflow.ellipsis,
                              textAlign: TextAlign.right,
                            ),
                          ),
                          const Icon(Icons.chevron_right, size: 16, color: Color(0xFFAAAAAA)),
                        ],
                      ),
                    )
                  else
                    const Expanded(child: SizedBox()),
                ],
              ),
            ),

          // Exercise card
          Container(
            width: double.infinity,
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
            child: Column(
              children: [
                Text(
                  '${l10n.executionExercise} ${pageIdx + 1}/${_exercises.length}',
                  style: const TextStyle(
                    fontSize: 12,
                    color: Color(0xFF999999),
                    fontWeight: FontWeight.w500,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  exercise.name as String,
                  style: const TextStyle(
                    fontSize: 22,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF333333),
                  ),
                  textAlign: TextAlign.center,
                ),
                if (exercise.description != null &&
                    (exercise.description as String).isNotEmpty) ...[
                  const SizedBox(height: 4),
                  Text(
                    exercise.description as String,
                    style: const TextStyle(fontSize: 13, color: Color(0xFF888888)),
                    textAlign: TextAlign.center,
                  ),
                ],
              ],
            ),
          ),

          const SizedBox(height: 16),

          // Set dots — completed ones are tappable to edit
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: List.generate(totalSets, (idx) {
              final isCompleted = idx < completedForExercise;
              final isActive = isCurrentPage && idx == activeSetIndex;
              return GestureDetector(
                onTap: isCompleted ? () => _editCompletedSet(pageIdx, idx) : null,
                child: Tooltip(
                  message: isCompleted ? 'Tap to edit' : '',
                  child: Container(
                    width: 32,
                    height: 32,
                    margin: const EdgeInsets.symmetric(horizontal: 4),
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      color: isCompleted
                          ? AppColors.success
                          : isActive
                          ? AppColors.primary
                          : Colors.grey.withAlpha(51),
                      border: isActive
                          ? Border.all(color: AppColors.primary, width: 2)
                          : isCompleted
                          ? Border.all(color: AppColors.success.withAlpha(100), width: 1)
                          : null,
                    ),
                    child: Center(
                      child: isCompleted && !isActive
                          ? const Icon(Icons.check, color: Colors.white, size: 14)
                          : Text(
                              '${idx + 1}',
                              style: TextStyle(
                                fontSize: 13,
                                fontWeight: FontWeight.bold,
                                color: (isCompleted || isActive) ? Colors.white : Colors.grey,
                              ),
                            ),
                    ),
                  ),
                ),
              );
            }),
          ),

          const SizedBox(height: 16),

          // Values card
          if (isCurrentPage)
            _buildValuesCard(l10n, isCardio)
          else
            _buildReadOnlyValues(l10n, exercise, isCardio),

          const SizedBox(height: 16),

          // Actions — only active page
          if (isCurrentPage) ...[
            _buildActions(l10n),
            const SizedBox(height: 16),
            _buildMotivation(l10n),
            const SizedBox(height: 16),
          ],

          // Recent completed sets
          if (_executedSets.isNotEmpty) _buildCompletedSets(l10n),
        ],
      ),
    );
  }

  Widget _buildValuesCard(AppLocalizations l10n, bool isCardio) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(15), blurRadius: 6, offset: const Offset(0, 2)),
        ],
      ),
      child: Column(
        children: [
          Row(
            children: [
              Text(
                '${l10n.executionSet} ${_currentSetIndex + 1} ${l10n.executionOf} $_totalSets',
                style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600),
              ),
              const Spacer(),
            ],
          ),
          const SizedBox(height: 12),
          Row(
            children: [
              Expanded(
                child: _ValueBox(
                  icon: isCardio ? '🏃' : '🏋️',
                  label: isCardio ? l10n.sessionDetailSpeed : l10n.workoutWeight,
                  isEditMode: _isEditingWeight,
                  value: _editWeight,
                  unit: isCardio ? '' : l10n.workoutWeightUnit,
                  onChanged: (v) => setState(() => _editWeight = v),
                  onTap: () => setState(() {
                    _isEditingWeight = true;
                    _isEditingReps = false;
                  }),
                  onEditingComplete: () => setState(() => _isEditingWeight = false),
                  onDone: () => setState(() {
                    _isEditingWeight = false;
                    _isEditingReps = true;
                  }),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _ValueBox(
                  icon: isCardio ? '⏱️' : '🔄',
                  label: isCardio ? l10n.workoutDuration : l10n.workoutReps,
                  isEditMode: _isEditingReps,
                  value: _editRepsOrDuration.toDouble(),
                  unit: '',
                  onChanged: (v) => setState(() => _editRepsOrDuration = v.round()),
                  onTap: () => setState(() {
                    _isEditingReps = true;
                    _isEditingWeight = false;
                  }),
                  onEditingComplete: () => setState(() => _isEditingReps = false),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildReadOnlyValues(AppLocalizations l10n, dynamic exercise, bool isCardio) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white.withAlpha(180),
        borderRadius: BorderRadius.circular(16),
      ),
      child: Row(
        children: [
          Expanded(
            child: _StatDisplay(
              icon: isCardio ? '🏃' : '🏋️',
              label: isCardio ? l10n.sessionDetailSpeed : l10n.workoutWeight,
              value: isCardio
                  ? '${exercise.weight}'
                  : '${exercise.weight} ${l10n.workoutWeightUnit}',
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: _StatDisplay(
              icon: isCardio ? '⏱️' : '🔄',
              label: isCardio ? l10n.workoutDuration : l10n.workoutReps,
              value: '${exercise.repsOrDuration}',
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildActions(AppLocalizations l10n) {
    return Row(
      children: [
        if (!_isFirstExercise) ...[
          IconButton(
            onPressed: _goToPreviousExercise,
            icon: const Icon(Icons.arrow_back_ios_new_rounded),
            color: const Color(0xFF888888),
            tooltip: l10n.tooltipPreviousExercise,
          ),
          const SizedBox(width: 4),
        ],
        Expanded(
          child: OutlinedButton(
            onPressed: _skipSet,
            style: OutlinedButton.styleFrom(
              foregroundColor: const Color(0xFF888888),
              side: const BorderSide(color: Color(0xFFCCCCCC)),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
              padding: const EdgeInsets.symmetric(vertical: 14),
            ),
            child: Text(l10n.executionSkip),
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          flex: 2,
          child: ElevatedButton.icon(
            onPressed: _confirmSet,
            icon: const Icon(Icons.check, size: 20),
            label: Text(l10n.executionConfirmSet),
            style: ElevatedButton.styleFrom(
              backgroundColor: AppColors.primary,
              foregroundColor: Colors.white,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
              padding: const EdgeInsets.symmetric(vertical: 14),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildMotivation(AppLocalizations l10n) {
    final text = _isLastSet && _isLastExercise ? l10n.executionAlmostDone : l10n.executionKeepGoing;
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text('💪', style: TextStyle(fontSize: 20)),
        const SizedBox(width: 8),
        Flexible(
          child: Text(
            text,
            style: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.w600,
              color: AppColors.primary,
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildCompletedSets(AppLocalizations l10n) {
    final recentSets = _executedSets.length > 3
        ? _executedSets.sublist(_executedSets.length - 3)
        : _executedSets;

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(color: Colors.white, borderRadius: BorderRadius.circular(12)),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '${l10n.executionCompletedSets} (${_executedSets.length})',
            style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 8),
          ...recentSets.map(
            (set) => Padding(
              padding: const EdgeInsets.only(bottom: 6),
              child: Row(
                children: [
                  const Icon(Icons.check_circle, color: AppColors.success, size: 16),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      set['exerciseName'] as String,
                      style: const TextStyle(fontSize: 13),
                    ),
                  ),
                  Text(
                    '${l10n.executionSet} ${set['setNumber']} • '
                    '${set['weight']}${l10n.workoutWeightUnit} × ${set['repsOrDuration']}',
                    style: const TextStyle(fontSize: 12, color: Color(0xFF888888)),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  void _showExitConfirmation(AppLocalizations l10n) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text(l10n.executionQuitTitle),
        content: Text(l10n.executionQuitMessage),
        actions: [
          TextButton(onPressed: () => Navigator.of(ctx).pop(), child: Text(l10n.buttonCancel)),
          TextButton(
            onPressed: () {
              Navigator.of(ctx).pop();
              Navigator.of(context).pop();
            },
            child: Text(l10n.executionQuitConfirm, style: const TextStyle(color: AppColors.danger)),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Read-only stat display (for non-active exercise pages)
// ---------------------------------------------------------------------------

class _StatDisplay extends StatelessWidget {
  final String icon;
  final String label;
  final String value;

  const _StatDisplay({required this.icon, required this.label, required this.value});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0xFFF8F8F8),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        children: [
          Text(icon, style: const TextStyle(fontSize: 20)),
          const SizedBox(height: 4),
          Text(label, style: const TextStyle(fontSize: 11, color: Color(0xFF888888))),
          const SizedBox(height: 6),
          Text(
            value,
            style: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Color(0xFF888888),
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Editable value box
// ---------------------------------------------------------------------------

class _ValueBox extends StatefulWidget {
  final String icon;
  final String label;
  final bool isEditMode;
  final double value;
  final String unit;
  final ValueChanged<double> onChanged;
  final VoidCallback onTap;
  final VoidCallback onEditingComplete;
  final VoidCallback? onDone;

  const _ValueBox({
    required this.icon,
    required this.label,
    required this.isEditMode,
    required this.value,
    required this.unit,
    required this.onChanged,
    required this.onTap,
    required this.onEditingComplete,
    this.onDone,
  });

  @override
  State<_ValueBox> createState() => _ValueBoxState();
}

class _ValueBoxState extends State<_ValueBox> {
  late TextEditingController _controller;
  late FocusNode _focusNode;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
    _focusNode = FocusNode();
    _focusNode.addListener(_onFocusChange);
  }

  @override
  void didUpdateWidget(_ValueBox oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.isEditMode != widget.isEditMode && widget.isEditMode) {
      _controller.text = '';
      Future.delayed(const Duration(milliseconds: 100), () {
        if (mounted && _focusNode.canRequestFocus) {
          _focusNode.requestFocus();
        }
      });
    }
  }

  void _onFocusChange() {
    if (!_focusNode.hasFocus && widget.isEditMode) {
      widget.onEditingComplete();
    }
  }

  @override
  void dispose() {
    _focusNode.removeListener(_onFocusChange);
    _focusNode.dispose();
    _controller.dispose();
    super.dispose();
  }

  void _handleDone() {
    final value = double.tryParse(_controller.text) ?? widget.value;
    widget.onChanged(value);
    widget.onEditingComplete();
    widget.onDone?.call();
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: widget.isEditMode ? null : widget.onTap,
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: widget.isEditMode ? AppColors.primary.withAlpha(10) : const Color(0xFFF8F8F8),
          borderRadius: BorderRadius.circular(12),
          border: widget.isEditMode ? Border.all(color: AppColors.primary, width: 1.5) : null,
        ),
        child: Column(
          children: [
            Text(widget.icon, style: const TextStyle(fontSize: 20)),
            const SizedBox(height: 4),
            Text(widget.label, style: const TextStyle(fontSize: 11, color: Color(0xFF888888))),
            const SizedBox(height: 6),
            if (widget.isEditMode)
              SizedBox(
                width: 80,
                height: 36,
                child: TextFormField(
                  controller: _controller,
                  keyboardType: const TextInputType.numberWithOptions(decimal: true),
                  textAlign: TextAlign.center,
                  autofocus: true,
                  focusNode: _focusNode,
                  style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                  decoration: InputDecoration(
                    contentPadding: const EdgeInsets.symmetric(vertical: 4),
                    border: OutlineInputBorder(borderRadius: BorderRadius.circular(8)),
                    isDense: true,
                  ),
                  onChanged: (v) => widget.onChanged(double.tryParse(v) ?? 0),
                  onFieldSubmitted: (_) => _handleDone(),
                  onTapOutside: (_) => widget.onEditingComplete(),
                ),
              )
            else
              Text(
                widget.unit.isNotEmpty
                    ? '${widget.value.toStringAsFixed(widget.value == widget.value.roundToDouble() ? 0 : 1)} ${widget.unit}'
                    : '${widget.value.round()}',
                style: const TextStyle(
                  fontSize: 22,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF333333),
                ),
              ),
          ],
        ),
      ),
    );
  }
}
