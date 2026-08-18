import 'dart:async';

import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/services/grpc/grpc_exercise_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_person_service.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/workout.dart';
import '../../models/person.dart';
import '../../models/visibility_option.dart';
import '../../providers/auth_provider.dart';
import '../../providers/exercise_selection_provider.dart';
import '../../providers/person_provider.dart';
import '../../widgets/exercise_item_paginated_widget.dart';
import '../../widgets/main_layout.dart';
import 'add_exercise_dialog.dart';
import 'workout_execution_page.dart';
import 'workout_form_dialog.dart';

class ExercisesPage extends StatefulWidget {
  const ExercisesPage({super.key});

  @override
  State<ExercisesPage> createState() => _ExercisesPageState();
}

class _ExercisesPageState extends State<ExercisesPage> {
  late final PageController _pageController;
  late final ScrollController _scrollController;
  int _currentPage = 0;

  // Applied filter states
  String _selectedCategory = 'Force';
  String _selectedVisibility = VisibilityOption.publicAccess.apiValue;
  String _selectedSort = 'created_at_desc';
  List<Person> _selectedOwnerPersons = [];

  @override
  void initState() {
    super.initState();
    _pageController = PageController();
    _pageController.addListener(_onPageChanged);
    _scrollController = ScrollController();
    _scrollController.addListener(_onScroll);

    WidgetsBinding.instance.addPostFrameCallback((_) {
      _loadExercises();
    });
  }

  void _onPageChanged() {
    setState(() {
      _currentPage = _pageController.page?.toInt() ?? 0;
    });
  }

  @override
  void dispose() {
    _pageController.removeListener(_onPageChanged);
    _pageController.dispose();
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (!_scrollController.hasClients || !mounted) return;

    try {
      if (_scrollController.position.pixels >=
          _scrollController.position.maxScrollExtent * 0.8) {
        final provider = context.read<ExerciseSelectionProvider>();
        if (provider.hasNextPage && !provider.isLoading) {
          provider.loadNextPage();
          _loadExercises();
        }
      }
    } catch (e) {
      debugPrint('Scroll listener error: $e');
    }
  }

  Future<void> _loadExercises() async {
    final provider = context.read<ExerciseSelectionProvider>();
    final ownerUuid = context.read<PersonProvider>().ownerUuid;
    if (ownerUuid.isEmpty) return;

    provider.setLoading(true);

    try {
      final ownerUuids = _selectedOwnerPersons.map((p) => p.uuid).toList();

      final response = await GrpcExerciseService.getPaginatedExercises(
        ownerUuid: ownerUuid,
        category: _selectedCategory,
        publicOwners: ownerUuids,
        visibility: _selectedVisibility,
        pageNumber: provider.currentPage,
        pageSize: 20,
        sortBy: _selectedSort,
      );

      if (mounted) {
        provider.setExercises(
          response.content,
          response.totalCount,
          response.pageNumber,
          response.pageSize,
          response.hasNextPage,
        );
      }
    } catch (e) {
      if (mounted) {
        provider.setLoading(false);
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Error loading exercises: $e')));
      }
    }
  }

  void _applyFilters(
    String category,
    String visibility,
    String sort,
    List<Person> selectedPersons,
  ) {
    setState(() {
      _selectedCategory = category;
      _selectedVisibility = visibility;
      _selectedSort = sort;
      _selectedOwnerPersons = List.from(selectedPersons);
    });

    final provider = context.read<ExerciseSelectionProvider>();
    provider.updateFilters(
      category: category,
      visibility: visibility,
      ownerIds: selectedPersons.map((p) => p.id).toList(),
      sortBy: sort,
    );
    if (_scrollController.hasClients) {
      _scrollController.jumpTo(0);
    }
    _loadExercises();
  }

  void _showFiltersModal() {
    final ownerUuid = context.read<PersonProvider>().ownerUuid;
    final authProvider = context.read<AuthProvider>();
    final token = authProvider.auth?.accessToken ?? '';
    if (ownerUuid.isEmpty && token.isEmpty) return;

    showDialog(
      context: context,
      builder: (dialogContext) => _ExerciseFilterModal(
        initialCategory: _selectedCategory,
        initialVisibility: _selectedVisibility,
        initialSort: _selectedSort,
        initialSelectedPersons: List.from(_selectedOwnerPersons),
        ownerUuid: ownerUuid,
        token: token,
        onApply: (category, visibility, sort, persons) {
          Navigator.pop(dialogContext);
          _applyFilters(category, visibility, sort, persons);
        },
        onCancel: () => Navigator.pop(dialogContext),
      ),
    );
  }

  IconData _getCategoryIcon(String category) {
    switch (category) {
      case 'Force':
        return Icons.fitness_center;
      case 'Cardio':
        return Icons.directions_run;
      default:
        return Icons.fitness_center;
    }
  }

  IconData _getVisibilityIcon(String visibility) {
    return VisibilityOption.fromApiValue(
      visibility,
      fallback: VisibilityOption.publicAccess,
    ).icon;
  }

  String _getVisibilityLabel(AppLocalizations l10n, String visibility) {
    return VisibilityOption.fromApiValue(
      visibility,
      fallback: VisibilityOption.publicAccess,
    ).label(l10n);
  }

  String _getSortLabel(String sortBy, AppLocalizations l10n) {
    switch (sortBy) {
      case 'created_at_desc':
        return l10n.sortCreatedAtDesc;
      case 'created_at_asc':
        return l10n.sortCreatedAtAsc;
      case 'owner_name_asc':
        return l10n.sortOwnerNameAsc;
      case 'name_asc':
        return l10n.sortNameAsc;
      default:
        return sortBy;
    }
  }

  void _showSaveWorkoutDialog() {
    final selectionProvider = context.read<ExerciseSelectionProvider>();
    final l10n = AppLocalizations.of(context)!;
    if (selectionProvider.selectedExercises.isEmpty) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text(l10n.messageNoExercisesSelected)));
      return;
    }

    final personProvider = context.read<PersonProvider>();
    final ownerId = personProvider.activeAuthorId;
    final ownerUuid = personProvider.activeAuthorUuid;

    showDialog(
      context: context,
      builder: (_) => WorkoutFormDialog(
        ownerId: ownerId,
        ownerUuid: ownerUuid,
        initialExercises: selectionProvider.selectedExercises,
      ),
    ).then((result) {
      if (result == true && mounted) {
        selectionProvider.clearSelection();
        selectionProvider.updateFilters(
          category: _selectedCategory,
          visibility: _selectedVisibility,
          ownerIds: _selectedOwnerPersons.map((p) => p.id).toList(),
          sortBy: _selectedSort,
        );
        _loadExercises();

        _pageController.animateToPage(
          0,
          duration: const Duration(milliseconds: 250),
          curve: Curves.easeInOut,
        );

        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(l10n.messageWorkoutSavedSuccessfully)),
        );
      }
    });
  }

  void _showAddExerciseDialog() {
    showDialog(
      context: context,
      builder: (dialogContext) => const AddExerciseDialog(),
    );
  }

  void _showStartSessionDialog() {
    final selectionProvider = context.read<ExerciseSelectionProvider>();
    final l10n = AppLocalizations.of(context)!;
    if (selectionProvider.selectedExercises.isEmpty) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text(l10n.messageNoExercisesSelected)));
      return;
    }

    final personProvider = context.read<PersonProvider>();
    final ownerId = personProvider.activeAuthorId;
    final ownerUuid = personProvider.activeAuthorUuid;

    showDialog<Workout>(
      context: context,
      builder: (_) => WorkoutFormDialog(
        ownerId: ownerId,
        ownerUuid: ownerUuid,
        initialExercises: selectionProvider.selectedExercises,
        mode: WorkoutFormMode.startSession,
      ),
    ).then((workout) {
      if (workout == null || !mounted) return;
      Navigator.of(context).push(
        MaterialPageRoute(
          builder: (_) => WorkoutExecutionPage(workout: workout),
        ),
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return MainLayout(
      navSection: NavSection.workout,
      currentRoute: '/exercises',
      body: Scaffold(
        backgroundColor: AppColors.background,
        body: Column(
          children: [
            _buildFiltersHeader(l10n),
            Expanded(
              child: PageView(
                controller: _pageController,
                children: [_buildExercisesPage(l10n), _buildSelectedPage(l10n)],
              ),
            ),
          ],
        ),
        bottomNavigationBar: _buildBottomBar(l10n),
      ),
    );
  }

  Widget _buildFiltersHeader(AppLocalizations l10n) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: Colors.white,
        boxShadow: [
          BoxShadow(color: Colors.black.withValues(alpha: 0.05), blurRadius: 4),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                l10n.labelFilterExercises,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Color(0xFF666666),
                ),
              ),
              Row(
                children: [
                  IconButton(
                    icon: const Icon(Icons.add),
                    onPressed: _showAddExerciseDialog,
                    tooltip: l10n.tooltipAddExercise,
                  ),
                  IconButton(
                    icon: const Icon(Icons.more_vert),
                    onPressed: _showFiltersModal,
                    tooltip: l10n.labelFilterExercises,
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 8),
          SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: Row(
              spacing: 8,
              children: [
                // Category chip
                _buildFilterChip(
                  icon: _getCategoryIcon(_selectedCategory),
                  label: _selectedCategory,
                  color: AppColors.primary,
                ),
                // Visibility chip
                _buildFilterChip(
                  icon: _getVisibilityIcon(_selectedVisibility),
                  label: _getVisibilityLabel(l10n, _selectedVisibility),
                  color: AppColors.secondary,
                ),
                // Sort chip
                _buildFilterChip(
                  icon: Icons.sort,
                  label: _getSortLabel(_selectedSort, l10n),
                  color: AppColors.third,
                ),
                // Owner name chip (if set)
                if (_selectedOwnerPersons.isNotEmpty)
                  _buildFilterChip(
                    icon: Icons.people,
                    label: _selectedOwnerPersons.length == 1
                        ? _selectedOwnerPersons.first.fullName
                        : '${_selectedOwnerPersons.length} ${l10n.labelSelectedOwners}',
                    color: AppColors.success,
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildFilterChip({
    required IconData icon,
    required String label,
    required Color color,
  }) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: color.withValues(alpha: 0.3)),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        spacing: 6,
        children: [
          Icon(icon, size: 16, color: color),
          Text(
            label,
            style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.w500,
              color: color,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildExercisesPage(AppLocalizations l10n) {
    return Consumer<ExerciseSelectionProvider>(
      builder: (context, provider, _) {
        if (provider.isLoading && provider.allExercises.isEmpty) {
          return const Center(child: CircularProgressIndicator());
        }

        if (provider.allExercises.isEmpty) {
          return Center(
            child: Text(
              l10n.messageNoExercisesFound,
              style: const TextStyle(fontSize: 16, color: Colors.grey),
            ),
          );
        }

        return Stack(
          children: [
            ListView.builder(
              controller: _scrollController,
              padding: const EdgeInsets.only(bottom: 80, top: 10),
              itemCount:
                  provider.allExercises.length +
                  (provider.hasNextPage && provider.isLoading ? 1 : 0),
              itemBuilder: (context, index) {
                if (index == provider.allExercises.length) {
                  return const Center(
                    child: Padding(
                      padding: EdgeInsets.all(16),
                      child: CircularProgressIndicator(),
                    ),
                  );
                }

                final exercise = provider.allExercises[index];

                return Dismissible(
                  key: ValueKey(exercise.id),
                  direction: DismissDirection.startToEnd,
                  onDismissed: (_) {
                    provider.moveToSelected(exercise);
                  },
                  background: Container(
                    color: AppColors.success,
                    alignment: Alignment.centerLeft,
                    padding: const EdgeInsets.only(left: 20),
                    child: Row(
                      children: [
                        const Icon(Icons.arrow_forward, color: Colors.white),
                        const SizedBox(width: 8),
                        Text(
                          l10n.messageAddToSelection,
                          style: const TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ],
                    ),
                  ),
                  child: ExerciseItemPaginatedWidget(
                    exercise: exercise,
                    isSelected: false,
                    onTap: null,
                  ),
                );
              },
            ),
            // Swipe confirmation
            if (!provider.isLoading && provider.selectedExercises.isNotEmpty)
              Positioned(
                bottom: 100,
                left: 0,
                right: 0,
                child: Center(
                  child: Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 16,
                      vertical: 8,
                    ),
                    decoration: BoxDecoration(
                      color: AppColors.success.withValues(alpha: 0.9),
                      borderRadius: BorderRadius.circular(20),
                    ),
                    child: Text(
                      '👉 ${provider.selectionCount} ${l10n.messageExercisesSelected}',
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                        fontSize: 12,
                      ),
                    ),
                  ),
                ),
              ),
          ],
        );
      },
    );
  }

  Widget _buildSelectedPage(AppLocalizations l10n) {
    return Consumer<ExerciseSelectionProvider>(
      builder: (context, provider, _) {
        if (provider.selectedExercises.isEmpty) {
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const Icon(
                  Icons.check_circle_outline,
                  size: 64,
                  color: Colors.grey,
                ),
                const SizedBox(height: 16),
                Text(
                  l10n.messageNoExercisesSelected,
                  style: const TextStyle(fontSize: 16, color: Colors.grey),
                ),
                const SizedBox(height: 8),
                Text(
                  l10n.tooltipSwipeToView,
                  style: const TextStyle(fontSize: 14, color: Colors.grey),
                ),
              ],
            ),
          );
        }

        return Stack(
          children: [
            ListView.builder(
              padding: const EdgeInsets.only(bottom: 80, top: 50),
              itemCount: provider.selectedExercises.length,
              itemBuilder: (context, index) {
                final exercise = provider.selectedExercises[index];

                return Dismissible(
                  key: ValueKey(exercise.id),
                  direction: DismissDirection.endToStart,
                  onDismissed: (_) => provider.moveBackToList(exercise),
                  background: Container(
                    color: Colors.orange,
                    alignment: Alignment.centerRight,
                    padding: const EdgeInsets.only(right: 20),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        Text(
                          l10n.messageRemoveFromSelection,
                          style: const TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(width: 8),
                        const Icon(Icons.arrow_back, color: Colors.white),
                      ],
                    ),
                  ),
                  child: ExerciseItemPaginatedWidget(
                    exercise: exercise,
                    isSelected: true,
                    onTap: null,
                  ),
                );
              },
            ),
          ],
        );
      },
    );
  }

  Widget _buildBottomBar(AppLocalizations l10n) {
    return Consumer<ExerciseSelectionProvider>(
      builder: (context, provider, _) {
        if (_currentPage == 0) {
          return Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.1),
                  blurRadius: 8,
                ),
              ],
            ),
            child: SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: provider.selectedExercises.isEmpty
                    ? null
                    : () {
                        _pageController.animateToPage(
                          1,
                          duration: const Duration(milliseconds: 250),
                          curve: Curves.easeInOut,
                        );
                      },
                icon: const Icon(Icons.chevron_right),
                label: Text(
                  '${provider.selectionCount} ${l10n.messageExercisesSelected}',
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primary,
                  disabledBackgroundColor: Colors.grey[300],
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(vertical: 12),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
              ),
            ),
          );
        }

        // Only show save/start actions on selected page (page 1)
        if (_currentPage != 1) {
          return const SizedBox.shrink();
        }

        return Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white,
            boxShadow: [
              BoxShadow(
                color: Colors.black.withValues(alpha: 0.1),
                blurRadius: 8,
              ),
            ],
          ),
          child: Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: provider.selectedExercises.isEmpty
                      ? null
                      : _showSaveWorkoutDialog,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.primary,
                    disabledBackgroundColor: Colors.grey[300],
                    padding: const EdgeInsets.symmetric(vertical: 12),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                  child: Text(
                    l10n.buttonSaveAsWorkout,
                    style: TextStyle(
                      color: provider.selectedExercises.isEmpty
                          ? Colors.grey
                          : Colors.white,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: ElevatedButton(
                  onPressed: provider.selectedExercises.isEmpty
                      ? null
                      : _showStartSessionDialog,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.success,
                    disabledBackgroundColor: Colors.grey[300],
                    padding: const EdgeInsets.symmetric(vertical: 12),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                  child: Text(
                    l10n.buttonStartWorkout,
                    style: TextStyle(
                      color: provider.selectedExercises.isEmpty
                          ? Colors.grey
                          : Colors.white,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

// ---------------------------------------------------------------------------
// Filter modal extracted as StatefulWidget for clean local state management
// ---------------------------------------------------------------------------

class _ExerciseFilterModal extends StatefulWidget {
  final String initialCategory;
  final String initialVisibility;
  final String initialSort;
  final List<Person> initialSelectedPersons;
  final String ownerUuid;
  final String token;
  final void Function(
    String category,
    String visibility,
    String sort,
    List<Person> selectedPersons,
  )
  onApply;
  final VoidCallback onCancel;

  const _ExerciseFilterModal({
    required this.initialCategory,
    required this.initialVisibility,
    required this.initialSort,
    required this.initialSelectedPersons,
    required this.ownerUuid,
    required this.token,
    required this.onApply,
    required this.onCancel,
  });

  @override
  State<_ExerciseFilterModal> createState() => _ExerciseFilterModalState();
}

class _ExerciseFilterModalState extends State<_ExerciseFilterModal> {
  late String _category;
  late String _visibility;
  late String _sort;
  late List<Person> _selectedPersons;

  final _searchController = TextEditingController();
  List<Person> _searchResults = [];
  bool _isSearching = false;
  Timer? _debounceTimer;

  @override
  void initState() {
    super.initState();
    _category = widget.initialCategory;
    _visibility = widget.initialVisibility;
    _sort = widget.initialSort;
    _selectedPersons = List.from(widget.initialSelectedPersons);
  }

  @override
  void dispose() {
    _searchController.dispose();
    _debounceTimer?.cancel();
    super.dispose();
  }

  void _onSearchChanged(String uuid, String query) {
    _debounceTimer?.cancel();
    if (query.length < 2) {
      setState(() {
        _searchResults = [];
        _isSearching = false;
      });
      return;
    }
    setState(() => _isSearching = true);
    _debounceTimer = Timer(const Duration(milliseconds: 400), () async {
      if (!mounted) return;
      try {
        final results = await GrpcPersonService.searchPersonsByUuid(
          uuid: uuid,
          query: query,
          limit: 20,
        );
        if (mounted) {
          setState(() {
            _searchResults = results;
            _isSearching = false;
          });
        }
      } catch (_) {
        if (mounted) {
          setState(() {
            _searchResults = [];
            _isSearching = false;
          });
        }
      }
    });
  }

  bool _isPersonSelected(Person person) =>
      _selectedPersons.any((p) => p.id == person.id);

  void _togglePerson(Person person) {
    setState(() {
      if (_isPersonSelected(person)) {
        _selectedPersons.removeWhere((p) => p.id == person.id);
      } else {
        _selectedPersons.add(person);
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final personProvider = context.read<PersonProvider>();
    final ownerUuid = personProvider.ownerUuid;

    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // ── Header ──────────────────────────────────────────────────
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    l10n.labelFilterExercises,
                    style: const TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.close),
                    onPressed: widget.onCancel,
                    padding: EdgeInsets.zero,
                    constraints: const BoxConstraints(),
                  ),
                ],
              ),
              const SizedBox(height: 20),

              // ── Owner Search ─────────────────────────────────────────────
              Text(
                l10n.labelSearchOwners,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Color(0xFF666666),
                ),
              ),
              const SizedBox(height: 8),
              TextField(
                controller: _searchController,
                onChanged: (query) => _onSearchChanged(ownerUuid, query),
                decoration: InputDecoration(
                  hintText: l10n.messageSearchHint,
                  prefixIcon: const Icon(
                    Icons.search,
                    color: AppColors.primary,
                  ),
                  suffixIcon: _isSearching
                      ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: Padding(
                            padding: EdgeInsets.all(12),
                            child: CircularProgressIndicator(strokeWidth: 2),
                          ),
                        )
                      : null,
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 16,
                    vertical: 12,
                  ),
                ),
              ),

              // Search results list
              if (_searchResults.isNotEmpty) ...[
                const SizedBox(height: 8),
                Container(
                  constraints: const BoxConstraints(maxHeight: 200),
                  decoration: BoxDecoration(
                    border: Border.all(color: Colors.grey[300]!),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: ListView.builder(
                    shrinkWrap: true,
                    itemCount: _searchResults.length,
                    itemBuilder: (context, index) {
                      final person = _searchResults[index];
                      final selected = _isPersonSelected(person);
                      return ListTile(
                        dense: true,
                        leading: CircleAvatar(
                          radius: 18,
                          backgroundColor: AppColors.primary.withValues(
                            alpha: 0.1,
                          ),
                          backgroundImage: person.avatar != null
                              ? NetworkImage(person.avatar!)
                              : null,
                          child: person.avatar == null
                              ? Text(
                                  person.firstname[0].toUpperCase(),
                                  style: const TextStyle(
                                    color: AppColors.primary,
                                    fontWeight: FontWeight.bold,
                                    fontSize: 12,
                                  ),
                                )
                              : null,
                        ),
                        title: Text(
                          person.fullName,
                          style: const TextStyle(fontSize: 13),
                        ),
                        trailing: selected
                            ? const Icon(
                                Icons.check_circle,
                                color: AppColors.primary,
                                size: 20,
                              )
                            : const Icon(
                                Icons.add_circle_outline,
                                color: Colors.grey,
                                size: 20,
                              ),
                        tileColor: selected
                            ? AppColors.primary.withValues(alpha: 0.05)
                            : null,
                        onTap: () => _togglePerson(person),
                      );
                    },
                  ),
                ),
              ],

              // No results message
              if (!_isSearching &&
                  _searchController.text.length >= 2 &&
                  _searchResults.isEmpty) ...[
                const SizedBox(height: 8),
                Text(
                  l10n.messageNoPersonsFound,
                  style: const TextStyle(fontSize: 12, color: Colors.grey),
                ),
              ],

              // Selected persons chips
              if (_selectedPersons.isNotEmpty) ...[
                const SizedBox(height: 12),
                Text(
                  l10n.labelSelectedOwners,
                  style: const TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.w600,
                    color: Color(0xFF666666),
                  ),
                ),
                const SizedBox(height: 8),
                Wrap(
                  spacing: 8,
                  runSpacing: 4,
                  children: _selectedPersons.map((person) {
                    return Chip(
                      avatar: CircleAvatar(
                        backgroundColor: AppColors.primary.withValues(
                          alpha: 0.1,
                        ),
                        child: Text(
                          person.firstname[0].toUpperCase(),
                          style: const TextStyle(
                            color: AppColors.primary,
                            fontSize: 10,
                          ),
                        ),
                      ),
                      label: Text(
                        person.fullName,
                        style: const TextStyle(fontSize: 12),
                      ),
                      deleteIcon: const Icon(Icons.close, size: 14),
                      onDeleted: () => _togglePerson(person),
                      backgroundColor: AppColors.primary.withValues(
                        alpha: 0.05,
                      ),
                      side: BorderSide(
                        color: AppColors.primary.withValues(alpha: 0.3),
                      ),
                    );
                  }).toList(),
                ),
              ],
              const SizedBox(height: 20),

              // ── Category ─────────────────────────────────────────────────
              Text(
                l10n.labelCategory,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Color(0xFF666666),
                ),
              ),
              const SizedBox(height: 12),
              _buildIconButtonGroup(
                options: const ['Force', 'Cardio'],
                icons: [Icons.fitness_center, Icons.directions_run],
                selectedValue: _category,
                onSelected: (value) => setState(() => _category = value),
              ),
              const SizedBox(height: 20),

              // ── Visibility ───────────────────────────────────────────────
              Text(
                l10n.labelVisibility,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Color(0xFF666666),
                ),
              ),
              const SizedBox(height: 12),
              _buildIconButtonGroup(
                options: [
                  VisibilityOption.publicAccess.apiValueFor(
                    useFriendsOnlyAlias: true,
                  ),
                  VisibilityOption.privateAccess.apiValueFor(
                    useFriendsOnlyAlias: true,
                  ),
                  VisibilityOption.friends.apiValueFor(
                    useFriendsOnlyAlias: true,
                  ),
                ],
                icons: [
                  VisibilityOption.publicAccess.icon,
                  VisibilityOption.privateAccess.icon,
                  VisibilityOption.friends.icon,
                ],
                selectedValue: _visibility,
                onSelected: (value) => setState(() => _visibility = value),
                labelBuilder: (value) => VisibilityOption.fromApiValue(
                  value,
                  fallback: VisibilityOption.publicAccess,
                ).label(l10n),
              ),
              const SizedBox(height: 20),

              // ── Sort ─────────────────────────────────────────────────────
              Text(
                l10n.labelSort,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Color(0xFF666666),
                ),
              ),
              const SizedBox(height: 8),
              DropdownMenu<String>(
                initialSelection: _sort,
                onSelected: (value) {
                  if (value != null) setState(() => _sort = value);
                },
                dropdownMenuEntries: [
                  DropdownMenuEntry(
                    value: 'created_at_desc',
                    label: l10n.sortCreatedAtDesc,
                  ),
                  DropdownMenuEntry(
                    value: 'created_at_asc',
                    label: l10n.sortCreatedAtAsc,
                  ),
                  DropdownMenuEntry(
                    value: 'owner_name_asc',
                    label: l10n.sortOwnerNameAsc,
                  ),
                  DropdownMenuEntry(value: 'name_asc', label: l10n.sortNameAsc),
                ],
                label: Text(l10n.labelSort),
                width: double.infinity,
              ),
              const SizedBox(height: 24),

              // ── Action buttons ───────────────────────────────────────────
              Row(
                spacing: 12,
                children: [
                  Expanded(
                    child: OutlinedButton(
                      onPressed: widget.onCancel,
                      style: OutlinedButton.styleFrom(
                        padding: const EdgeInsets.symmetric(vertical: 12),
                        side: const BorderSide(color: AppColors.primary),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(8),
                        ),
                      ),
                      child: Text(
                        l10n.buttonCancel,
                        style: const TextStyle(
                          color: AppColors.primary,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                  ),
                  Expanded(
                    child: ElevatedButton(
                      onPressed: () => widget.onApply(
                        _category,
                        _visibility,
                        _sort,
                        _selectedPersons,
                      ),
                      style: ElevatedButton.styleFrom(
                        backgroundColor: AppColors.primary,
                        padding: const EdgeInsets.symmetric(vertical: 12),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(8),
                        ),
                      ),
                      child: Text(
                        l10n.buttonApplyFilters,
                        style: const TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildIconButtonGroup({
    required List<String> options,
    required List<IconData> icons,
    required String selectedValue,
    required void Function(String) onSelected,
    String Function(String option)? labelBuilder,
  }) {
    return Row(
      spacing: 12,
      children: [
        for (int i = 0; i < options.length; i++)
          Expanded(
            child: _buildIconButton(
              icon: icons[i],
              label: labelBuilder?.call(options[i]) ?? options[i],
              isSelected: selectedValue == options[i],
              onPressed: () => onSelected(options[i]),
            ),
          ),
      ],
    );
  }

  Widget _buildIconButton({
    required IconData icon,
    required String label,
    required bool isSelected,
    required VoidCallback onPressed,
  }) {
    return InkWell(
      onTap: onPressed,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 12),
        decoration: BoxDecoration(
          color: isSelected
              ? AppColors.primary.withAlpha(25) // ~10%
              : Colors.transparent,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: isSelected ? AppColors.primary : Colors.grey[300]!,
            width: isSelected ? 2 : 1,
          ),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          spacing: 6,
          children: [
            Icon(
              icon,
              color: isSelected ? AppColors.primary : Colors.grey[600],
              size: 24,
            ),
            Text(
              label,
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 12,
                fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
                color: isSelected ? AppColors.primary : Colors.grey[600],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
