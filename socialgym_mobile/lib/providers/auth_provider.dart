import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/auth_response.dart';
import '../models/sign_up_request.dart';
import '../services/auth_service.dart';
import '../services/base_service.dart';
import '../services/grpc/grpc_business_profile_service.dart';
import '../services/grpc/grpc_channel_factory.dart';
import '../services/grpc/grpc_exercise_service.dart';
import '../services/grpc/grpc_person_service.dart';
import '../services/grpc/grpc_resource_service.dart';
import '../services/grpc/grpc_settings_service.dart';
import '../services/grpc/grpc_team_member_service.dart';
import '../services/grpc/grpc_workout_service.dart';

class AuthProvider extends ChangeNotifier {
  AuthResponse? _auth;
  bool _loading = false;
  bool _initialized = false;
  String? _error;

  AuthResponse? get auth => _auth;
  bool get loading => _loading;
  bool get initialized => _initialized;
  String? get error => _error;
  bool get isAuthenticated => _auth != null;

  AuthProvider() {
    _loadFromStorage();
  }

  Future<void> _loadFromStorage() async {
    final prefs = await SharedPreferences.getInstance();
    final authJson = prefs.getString('auth');
    if (authJson != null) {
      try {
        final data = jsonDecode(authJson) as Map<String, dynamic>;
        _auth = AuthResponse.fromJson(data);
      } catch (_) {
        await prefs.remove('auth');
      }
    }
    _initialized = true;
    notifyListeners();
  }

  Future<void> _saveToStorage(AuthResponse auth) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('auth', jsonEncode(auth.toJson()));
  }

  Future<void> _clearStorage() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove('auth');
  }

  Future<bool> signIn({required String email, required String password}) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _auth = await AuthService.signIn(email: email, password: password);
      await _saveToStorage(_auth!);
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

  Future<bool> signUp(SignUpRequest request) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _auth = await AuthService.signUp(request);
      await _saveToStorage(_auth!);
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

  /// Apply a token reissued by a business-profile activate/deactivate call.
  /// Those endpoints never reissue a refresh token, so the previous one is
  /// carried forward.
  Future<void> applySwitchedToken(AuthResponse newAuth) async {
    _auth = newAuth.copyWith(refreshToken: newAuth.refreshToken ?? _auth?.refreshToken);
    await _saveToStorage(_auth!);
    notifyListeners();
  }

  Future<void> signOut() async {
    _auth = null;
    await _clearStorage();
    // Drop each service's cached client *before* tearing down the shared
    // channels: otherwise a client left pointing at an already-shut-down
    // channel gets reused on the next sign-in and every call fails locally
    // with UNAVAILABLE ("Channel shutting down") before reaching the server.
    await Future.wait([
      GrpcPersonService.shutdown(),
      GrpcBusinessProfileService.shutdown(),
      GrpcSettingsService.shutdown(),
      GrpcTeamMemberService.shutdown(),
      GrpcResourceService.shutdown(),
      GrpcWorkoutService.shutdown(),
      GrpcExerciseService.shutdown(),
    ]);
    await GrpcChannelFactory.shutdownAll();
    notifyListeners();
  }

  void clearError() {
    _error = null;
    notifyListeners();
  }
}
