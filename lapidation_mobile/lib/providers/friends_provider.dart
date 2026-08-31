import 'package:flutter/foundation.dart';

import '../models/friends_data.dart';
import '../models/person.dart';
import '../services/friends_service.dart';

class FriendsProvider extends ChangeNotifier {
  FriendsData? _friendsData;
  bool _loading = false;
  bool _actionLoading = false;
  String? _error;

  FriendsData? get friendsData => _friendsData;
  bool get loading => _loading;
  bool get actionLoading => _actionLoading;
  String? get error => _error;

  List<Person> get suggestions => _friendsData?.suggestions ?? [];
  List<Person> get friends => _friendsData?.friends ?? [];
  List<Person> get receiveRequests => _friendsData?.receiveRequests ?? [];
  List<Person> get sentRequests => _friendsData?.sentRequests ?? [];

  int get friendsCount => friends.length;
  int get pendingRequestsCount => _friendsData?.pendingRequestsCount ?? 0;

  /// Fetch all friends data from the API.
  ///
  /// When [latitude]/[longitude] are given, suggestions are centered on that
  /// point (e.g. the device's current GPS position) instead of the person's
  /// saved home address.
  Future<void> fetchFriends(
    String token, {
    double? latitude,
    double? longitude,
  }) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _friendsData = await FriendsService.fetchFriends(
        token,
        latitude: latitude,
        longitude: longitude,
      );
      _error = null;
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  /// Send a friend request.
  Future<bool> sendFriendRequest(int personId, String token) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await FriendsService.sendFriendRequest(personId, token);
      // Refresh the data after action
      await fetchFriends(token);
      _actionLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  /// Accept a friend request.
  Future<bool> acceptFriendRequest(int personId, String token) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await FriendsService.acceptFriendRequest(personId, token);
      // Refresh the data after action
      await fetchFriends(token);
      _actionLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  /// Reject a friend request.
  Future<bool> rejectFriendRequest(int personId, String token) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await FriendsService.rejectFriendRequest(personId, token);
      // Refresh the data after action
      await fetchFriends(token);
      _actionLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  /// Cancel a sent friend request.
  Future<bool> cancelFriendRequest(int personId, String token) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await FriendsService.cancelFriendRequest(personId, token);
      // Refresh the data after action
      await fetchFriends(token);
      _actionLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  /// Remove a friend.
  Future<bool> removeFriend(int personId, String token) async {
    _actionLoading = true;
    notifyListeners();

    try {
      await FriendsService.removeFriend(personId, token);
      // Refresh the data after action
      await fetchFriends(token);
      _actionLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _actionLoading = false;
      notifyListeners();
      return false;
    }
  }

  /// Clear the friends data.
  void clear() {
    _friendsData = null;
    _loading = false;
    _actionLoading = false;
    _error = null;
    notifyListeners();
  }
}
