import 'package:flutter/material.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_service.dart';

import '../models/exercise.dart';
import '../models/workout.dart';
import '../services/base_service.dart';

class WorkoutProvider extends ChangeNotifier {
  List<Workout> _workouts = [];
  Workout? _selectedWorkout;
  bool _loading = false;
  String? _error;

  List<Workout> get workouts => _workouts;
  Workout? get selectedWorkout => _selectedWorkout;
  bool get loading => _loading;
  String? get error => _error;

  void selectWorkout(Workout? workout) {
    if (_selectedWorkout?.id == workout?.id) {
      _selectedWorkout = null;
    } else {
      _selectedWorkout = workout;
    }
    notifyListeners();
  }

  void clearError() {
    _error = null;
    notifyListeners();
  }

  Future<void> fetchWorkouts(String ownerUuid, String token) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _workouts = await GrpcWorkoutService.getWorkoutsByOwnerUuid(ownerUuid: ownerUuid);
      // Update selected workout if it exists
      if (_selectedWorkout != null) {
        final updated = _workouts.where((w) => w.id == _selectedWorkout!.id).firstOrNull;
        _selectedWorkout = updated;
      }
      _loading = false;
      notifyListeners();
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
    } catch (e) {
      _error = 'Connection error. Please try again.';
      _loading = false;
      notifyListeners();
    }
  }

  Future<bool> addWorkout(Map<String, dynamic> workoutData, String token) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final workout = await GrpcWorkoutService.createWorkout(workout: Workout.fromJson(workoutData));
      _workouts = [..._workouts, workout];
      _loading = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Connection error. Please try again.';
      _loading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> addExercisesToWorkout(String workoutUuid,List<Exercise> exercises) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final workout = await GrpcWorkoutService.addExercisesToWorkout(workoutUuid: workoutUuid, exercises: exercises);

      // Update the workout in the list
      _workouts = _workouts.map((w) {
        if (w.uuid == workoutUuid) {
          return w.copyWith(exercises: [...w.exercises, ...workout.exercises]);
        }
        return w;
      }).toList();

      // Update selected workout
      if (_selectedWorkout?.uuid == workoutUuid) {
        _selectedWorkout = _selectedWorkout!.copyWith(
          exercises: [..._selectedWorkout!.exercises, ...workout.exercises],
        );
      }

      _loading = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Connection error. Please try again.';
      _loading = false;
      notifyListeners();
      return false;
    }
  }

  void reorderExercises(int workoutId, int oldIndex, int newIndex) {
    // Update the workout in the list
    _workouts = _workouts.map((w) {
      if (w.id == workoutId) {
        final exercises = List.of(w.exercises);
        if (newIndex > oldIndex) {
          newIndex -= 1;
        }
        final exercise = exercises.removeAt(oldIndex);
        exercises.insert(newIndex, exercise);
        return w.copyWith(exercises: exercises);
      }
      return w;
    }).toList();

    // Update selected workout
    if (_selectedWorkout?.id == workoutId) {
      final exercises = List.of(_selectedWorkout!.exercises);
      int adjustedNewIndex = newIndex;
      if (adjustedNewIndex > oldIndex) {
        adjustedNewIndex -= 1;
      }
      final exercise = exercises.removeAt(oldIndex);
      exercises.insert(adjustedNewIndex, exercise);
      _selectedWorkout = _selectedWorkout!.copyWith(exercises: exercises);
    }

    notifyListeners();
  }
}
