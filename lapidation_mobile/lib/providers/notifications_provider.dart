import 'package:flutter/foundation.dart';
import 'package:lapidation_mobile/models/notification.dart';
import 'package:lapidation_mobile/services/base_service.dart';
import 'package:lapidation_mobile/services/notification_service.dart';

class NotificationsProvider extends ChangeNotifier {
  static const int _pageSize = 50;

  List<Notification> _notifications = [];
  bool _loading = false;
  bool _loadingMore = false;
  String? _error;
  int _currentPage = 0;
  int _unreadCount = 0;
  bool _hasMore = true;

  List<Notification> get notifications => List.unmodifiable(_notifications);
  bool get loading => _loading;
  bool get loadingMore => _loadingMore;
  String? get error => _error;
  int get currentPage => _currentPage;
  int get unreadCount => _unreadCount;
  bool get hasMore => _hasMore;

  Future<void> fetchNotifications(String token, String ownerUUid) async {
    if (token.isEmpty) return;

    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final fetchedNotifications = await NotificationService.fetchNotifications(
        token,
        ownerUUid,
        limit: _pageLimitForPage(0),
      );
      _notifications = fetchedNotifications;
      _currentPage = 0;
      _unreadCount = _countUnread(fetchedNotifications);
      _hasMore = fetchedNotifications.length >= _pageSize;
      _loading = false;
      notifyListeners();
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
    } catch (_) {
      _error = 'Failed to load notifications. Please try again.';
      _loading = false;
      notifyListeners();
    }
  }

  Future<void> loadMoreNotifications(String token, String ownerUuid) async {
    if (_loading || _loadingMore || !_hasMore || token.isEmpty) return;

    _loadingMore = true;
    _error = null;
    notifyListeners();

    try {
      final nextPage = _currentPage + 1;
      final fetchedNotifications = await NotificationService.fetchNotifications(
        token,
        ownerUuid,
        limit: _pageLimitForPage(nextPage),
      );

      _notifications = _mergeNotifications(fetchedNotifications);
      _currentPage = nextPage;
      _unreadCount = _countUnread(_notifications);
      _hasMore = fetchedNotifications.length >= _pageLimitForPage(nextPage);
      _loadingMore = false;
      notifyListeners();
    } on AppException catch (e) {
      _error = e.message;
      _loadingMore = false;
      notifyListeners();
    } catch (_) {
      _error = 'Failed to load more notifications. Please try again.';
      _loadingMore = false;
      notifyListeners();
    }
  }

  Future<void> markAsRead(String token, String ownerUuid, String uuid) async {
    if (token.isEmpty) return;

    final index = _notifications.indexWhere(
      (notification) => notification.uuid == uuid,
    );
    if (index == -1) return;

    final currentNotification = _notifications[index];
    if (!currentNotification.isUnread) return;

    try {
      final updatedNotification =
          await NotificationService.markNotificationAsRead(
            token,
            ownerUuid,
            uuid,
          );
      final replacement = updatedNotification.uuid.isNotEmpty
          ? updatedNotification
          : currentNotification.copyWith(read: true, updatedAt: DateTime.now());

      _notifications = _notifications.map((notification) {
        if (notification.uuid == uuid) {
          return replacement;
        }
        return notification;
      }).toList();
      _unreadCount = _countUnread(_notifications);
      notifyListeners();
    } on AppException catch (e) {
      _error = e.message;
      notifyListeners();
    } catch (_) {
      _error = 'Failed to update notification. Please try again.';
      notifyListeners();
    }
  }

  void clearError() {
    _error = null;
    notifyListeners();
  }

  int _pageLimitForPage(int page) => (page + 1) * _pageSize;

  int _countUnread(List<Notification> notifications) {
    return notifications.where((notification) => notification.isUnread).length;
  }

  List<Notification> _mergeNotifications(
    List<Notification> latestNotifications,
  ) {
    final notificationsById = <String, Notification>{};

    for (final notification in _notifications) {
      notificationsById[notification.uuid] = notification;
    }

    for (final notification in latestNotifications) {
      notificationsById[notification.uuid] = notification;
    }

    return latestNotifications
        .map(
          (notification) =>
              notificationsById[notification.uuid] ?? notification,
        )
        .toList();
  }
}
