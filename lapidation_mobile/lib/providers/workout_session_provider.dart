import 'package:flutter/material.dart';

import '../models/workout_session.dart';
import '../services/workout_session_service.dart';
import '../services/base_service.dart';

class WorkoutSessionProvider extends ChangeNotifier {
  // ── Save state ──────────────────────────────────────────────────────────────
  bool _saving = false;
  String? _saveError;

  bool get saving => _saving;
  String? get error => _saveError;

  // ── Fetch state ─────────────────────────────────────────────────────────────
  bool _fetching = false;
  String? _fetchError;
  List<WorkoutSession> _sessions = [];

  bool get fetching => _fetching;
  String? get fetchError => _fetchError;
  List<WorkoutSession> get sessions => List.unmodifiable(_sessions);

  // ── Save ────────────────────────────────────────────────────────────────────

  Future<bool> saveSession(
    Map<String, dynamic> sessionData,
    String token,
  ) async {
    _saving = true;
    _saveError = null;
    notifyListeners();

    try {
      await WorkoutSessionService.saveWorkoutSession(sessionData, token);
      _saving = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _saveError = e.message;
      _saving = false;
      notifyListeners();
      return false;
    } catch (_) {
      _saveError = 'Failed to save session. Please try again.';
      _saving = false;
      notifyListeners();
      return false;
    }
  }

  // ── Fetch ───────────────────────────────────────────────────────────────────

  Future<void> fetchSessions({
    required String personUuid,
    required DateTime startDate,
    required DateTime endDate,
    required String token,
  }) async {
    _fetching = true;
    _fetchError = null;
    notifyListeners();

    try {
      _sessions = await WorkoutSessionService.fetchWorkoutSessions(
        personUuid: personUuid,
        startDate: startDate,
        endDate: endDate,
        token: token,
      );
      _fetching = false;
      notifyListeners();
    } on AppException catch (e) {
      _fetchError = e.message;
      _fetching = false;
      notifyListeners();
    } catch (_) {
      _fetchError = 'Failed to load sessions. Please try again.';
      _fetching = false;
      notifyListeners();
    }
  }

  // ── Helpers ─────────────────────────────────────────────────────────────────

  void clearError() {
    _saveError = null;
    _fetchError = null;
    notifyListeners();
  }

  void setError(String error) {
    _saveError = error;
    notifyListeners();
  }
}
