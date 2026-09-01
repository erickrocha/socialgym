import 'package:flutter/foundation.dart';

import '../models/business_profile.dart';
import '../models/person.dart';
import '../models/workout.dart';
import '../services/base_service.dart';
import '../services/grpc/grpc_business_profile_service.dart';
import '../services/grpc/grpc_person_service.dart';
import '../services/grpc/grpc_workout_service.dart';

/// Workout-assignment invites, mirroring [TeamMemberProvider]:
/// - [received]: Pending workouts a business profile assigned to me.
/// - [sent]: workouts I assigned (only when acting as a business profile).
class WorkoutInviteProvider extends ChangeNotifier {
  List<Workout> _received = [];
  List<Workout> _sent = [];
  final Map<String, BusinessProfile> _assignerByUuid = {};
  final Map<String, Person> _recipientByUuid = {};

  bool _loading = false;
  bool _actionLoading = false;
  String? _error;

  String? _personUuid;
  int? _businessProfileId;

  List<Workout> get received => _received;
  List<Workout> get sent => _sent;
  bool get loading => _loading;
  bool get actionLoading => _actionLoading;
  String? get error => _error;

  int get pendingSentCount => _sent.where((w) => w.status == 'Pending').length;

  BusinessProfile? assignerFor(String? profileUuid) =>
      profileUuid == null ? null : _assignerByUuid[profileUuid];

  Person? recipientFor(String personUuid) => _recipientByUuid[personUuid];

  /// [businessProfileId] is passed only when the caller currently has an active
  /// business profile; it populates [sent].
  Future<void> fetch({
    required String personUuid,
    int? businessProfileId,
  }) async {
    _personUuid = personUuid;
    _businessProfileId = businessProfileId;

    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final owned = await GrpcWorkoutService.getWorkoutsByOwnerUuid(
        ownerUuid: personUuid,
      );
      _received = owned
          .where(
            (w) => w.status == 'Pending' && w.assignedByProfileUuid != null,
          )
          .toList();

      _sent = businessProfileId == null
          ? []
          : await GrpcWorkoutService.getWorkoutsAssignedByProfile(
              businessProfileId: businessProfileId,
            );

      await _resolveNames();
    } on AppException catch (e) {
      _error = e.message;
    } catch (e) {
      _error = 'Connection error. Please try again.';
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  Future<void> _resolveNames() async {
    final profileUuids = _received
        .map((w) => w.assignedByProfileUuid)
        .whereType<String>()
        .toSet()
        .where((uuid) => !_assignerByUuid.containsKey(uuid));
    for (final uuid in profileUuids) {
      try {
        _assignerByUuid[uuid] =
            await GrpcBusinessProfileService.getBusinessProfileById(uuid: uuid);
      } catch (_) {
        // best-effort — the card falls back to a generic label
      }
    }

    final personUuids = _sent
        .map((w) => w.ownerUuid)
        .toSet()
        .where(
          (uuid) => uuid.isNotEmpty && !_recipientByUuid.containsKey(uuid),
        );
    for (final uuid in personUuids) {
      try {
        _recipientByUuid[uuid] = await GrpcPersonService.getPerson(uuid: uuid);
      } catch (_) {
        // best-effort
      }
    }
  }

  Future<void> _refresh() async {
    final personUuid = _personUuid;
    if (personUuid == null) return;
    await fetch(personUuid: personUuid, businessProfileId: _businessProfileId);
  }

  /// Assigned person: accept a pending workout invite.
  Future<bool> accept(String uuid) =>
      _mutate(() => GrpcWorkoutService.acceptWorkout(uuid: uuid));

  /// Assigned person: reject a pending workout invite.
  Future<bool> reject(String uuid) =>
      _mutate(() => GrpcWorkoutService.rejectWorkout(uuid: uuid));

  /// Assigning business profile: cancel a pending assignment it sent.
  Future<bool> cancel(String uuid) =>
      _mutate(() => GrpcWorkoutService.cancelWorkout(uuid: uuid));

  Future<bool> _mutate(Future<Workout> Function() action) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await action();
      await _refresh();
      _actionLoading = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _actionLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Connection error. Please try again.';
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  void clear() {
    _received = [];
    _sent = [];
    _assignerByUuid.clear();
    _recipientByUuid.clear();
    _loading = false;
    _actionLoading = false;
    _error = null;
    _personUuid = null;
    _businessProfileId = null;
    notifyListeners();
  }
}
