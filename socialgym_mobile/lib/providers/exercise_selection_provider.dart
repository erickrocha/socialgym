import 'package:flutter/foundation.dart';
import '../models/exercise.dart';

class ExerciseSelectionProvider with ChangeNotifier {
  // Selected exercises state, keyed by uuid: exercises loaded from the
  // paginated gRPC endpoint (ExerciseMapper.fromProto) never populate the
  // numeric `id` field, only `uuid`.
  final Set<String> _selectedExerciseIds = {};
  final Map<String, Exercise> _selectedExercisesMap = {};

  // Pagination state
  List<Exercise> _allExercises = [];
  int _currentPage = 1;
  bool _hasNextPage = false;
  bool _isLoading = false;

  // Filter state
  String _filterCategory = 'Force';
  String _filterVisibility = 'Public';
  List<int> _filterOwnerIds = [];
  String _sortBy = 'created_at_desc';

  // Getters
  Set<String> get selectedExerciseIds => _selectedExerciseIds;
  List<Exercise> get selectedExercises => _selectedExercisesMap.values.toList();
  List<Exercise> get allExercises => _allExercises;
  int get selectionCount => _selectedExerciseIds.length;
  bool get hasNextPage => _hasNextPage;
  bool get isLoading => _isLoading;
  int get currentPage => _currentPage;

  // Filter getters
  String get filterCategory => _filterCategory;
  String get filterVisibility => _filterVisibility;
  List<int> get filterOwnerIds => List.unmodifiable(_filterOwnerIds);
  String get sortBy => _sortBy;

  /// Add exercise to selection (toggle)
  void toggleSelection(Exercise exercise) {
    if (_selectedExerciseIds.contains(exercise.uuid)) {
      _selectedExerciseIds.remove(exercise.uuid);
      _selectedExercisesMap.remove(exercise.uuid);
    } else {
      _selectedExerciseIds.add(exercise.uuid!);
      _selectedExercisesMap[exercise.uuid!] = exercise;
    }
    notifyListeners();
  }

  /// Move exercise from the all-exercises list into the selected list.
  /// Called when the user swipes an item in the exercises list (startToEnd).
  void moveToSelected(Exercise exercise) {
    _allExercises.removeWhere((e) => e.uuid == exercise.uuid);
    _selectedExerciseIds.add(exercise.uuid!);
    _selectedExercisesMap[exercise.uuid!] = exercise;
    notifyListeners();
  }

  /// Move exercise back from the selected list into the all-exercises list.
  /// Called when the user swipes an item in the selected list (endToStart).
  void moveBackToList(Exercise exercise) {
    _selectedExerciseIds.remove(exercise.uuid);
    _selectedExercisesMap.remove(exercise.uuid);
    _allExercises.insert(0, exercise);
    notifyListeners();
  }

  /// Check if exercise is selected
  bool isExerciseSelected(String exerciseUuid) {
    return _selectedExerciseIds.contains(exerciseUuid);
  }

  /// Clear all selections
  void clearSelection() {
    _selectedExerciseIds.clear();
    _selectedExercisesMap.clear();
    notifyListeners();
  }

  /// Update filters (resets to page 1)
  void updateFilters({String? category, String? visibility, List<int>? ownerIds, String? sortBy}) {
    _filterCategory = category ?? _filterCategory;
    _filterVisibility = visibility ?? _filterVisibility;
    _filterOwnerIds = ownerIds ?? _filterOwnerIds;
    _sortBy = sortBy ?? _sortBy;
    _currentPage = 1;
    _allExercises.clear();
    notifyListeners();
  }

  /// Set exercises and pagination data
  void setExercises(
    List<Exercise> exercises,
    int totalCount,
    int pageNumber,
    int pageSize,
    bool hasNextPage,
  ) {
    if (pageNumber == 1) {
      _allExercises = exercises;
    } else {
      _allExercises.addAll(exercises);
    }

    _currentPage = pageNumber;
    _hasNextPage = hasNextPage;
    _isLoading = false;
    notifyListeners();
  }

  /// Set loading state
  void setLoading(bool loading) {
    _isLoading = loading;
    notifyListeners();
  }

  /// Load next page
  void loadNextPage() {
    if (!_hasNextPage || _isLoading) return;
    _currentPage++;
  }

  /// Reset to initial state
  void reset() {
    _selectedExerciseIds.clear();
    _selectedExercisesMap.clear();
    _allExercises.clear();
    _currentPage = 1;
    _hasNextPage = false;
    _isLoading = false;
    _filterCategory = 'Force';
    _filterVisibility = 'Public';
    _filterOwnerIds = [];
    _sortBy = 'created_at_desc';
    notifyListeners();
  }

  /// Add new exercise to the list (optimistic UI)
  void addExercise(Exercise exercise) {
    _allExercises.insert(0, exercise);
    notifyListeners();
  }
}
