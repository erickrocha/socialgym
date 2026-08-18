import 'dart:convert';

import 'package:flutter/material.dart';

import '../models/evolution_check_in.dart';
import '../services/evolution_service.dart';
import '../services/base_service.dart';

class EvolutionProvider extends ChangeNotifier {
  // ── State ────────────────────────────────────────────────────────────────
  bool _loading = false;
  bool _submitting = false;
  String? _error;
  List<EvolutionCheckIn> _checkins = [];

  bool get loading => _loading;
  bool get submitting => _submitting;
  String? get error => _error;
  List<EvolutionCheckIn> get checkins => List.unmodifiable(_checkins);

  // ── Fetch ────────────────────────────────────────────────────────────────

  Future<void> fetchCheckIns({
    required DateTime startDate,
    required DateTime endDate,
    required String token,
  }) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _checkins = await EvolutionService.fetchEvolutionCheckIns(
        startDate: startDate,
        endDate: endDate,
        token: token,
      );
      _loading = false;
      notifyListeners();
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
    } catch (_) {
      _error = 'Failed to load evolution data. Please try again.';
      _loading = false;
      notifyListeners();
    }
  }

  Future<bool> addCheckIn({required Map<String, dynamic> payload, required String token}) async {
    _submitting = true;
    _error = null;
    notifyListeners();

    try {
      jsonEncode(payload);
      final created = await EvolutionService.createEvolutionCheckIn(payload: payload, token: token);
      _checkins = [created, ..._checkins];
      _submitting = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _submitting = false;
      notifyListeners();
      return false;
    } catch (_) {
      _error = 'Failed to create evolution check-in. Please try again.';
      _submitting = false;
      notifyListeners();
      return false;
    }
  }

  // ── Helpers ──────────────────────────────────────────────────────────────

  void clearError() {
    _error = null;
    notifyListeners();
  }
}
