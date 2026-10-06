import 'dart:async';
import 'dart:convert';

import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/config/firebase_runtime_options.dart';
import 'package:socialgym_mobile/models/auth_response.dart';
import 'package:socialgym_mobile/services/notification_service.dart';
import 'package:uuid/uuid.dart';

class PushRegistrationService {
  static const String _deviceUuidStorageKey = 'push_device_uuid';
  static const AndroidNotificationChannel _channel = AndroidNotificationChannel(
    'socialgym_social_updates',
    'Social updates',
    description: 'Friendship and social activity notifications',
    importance: Importance.high,
  );

  static final FlutterLocalNotificationsPlugin _localNotifications =
      FlutterLocalNotificationsPlugin();
  static final Uuid _uuid = Uuid();
  static GlobalKey<NavigatorState>? _navigatorKey;
  static StreamSubscription<String>? _tokenRefreshSubscription;
  static AuthResponse? _auth;
  static Map<String, dynamic>? _pendingTap;
  static bool _initialized = false;
  static bool _navigationScheduled = false;

  static Future<void> initialize(
    GlobalKey<NavigatorState> navigatorKey,
  ) async {
    _navigatorKey = navigatorKey;
    if (kIsWeb ||
        (defaultTargetPlatform != TargetPlatform.android &&
            defaultTargetPlatform != TargetPlatform.iOS)) {
      return;
    }

    final options = FirebaseRuntimeOptions.currentPlatform;
    if (options == null) {
      debugPrint('Push disabled: Firebase runtime options are not configured.');
      return;
    }

    try {
      if (Firebase.apps.isEmpty) {
        await Firebase.initializeApp(options: options);
      }
      await _initializeLocalNotifications();
      await FirebaseMessaging.instance
          .setForegroundNotificationPresentationOptions(
            alert: true,
            badge: true,
            sound: true,
          );
      FirebaseMessaging.onMessage.listen(_showForegroundNotification);
      FirebaseMessaging.onMessageOpenedApp.listen(_handleRemoteTap);
      _initialized = true;

      final initialMessage = await FirebaseMessaging.instance.getInitialMessage();
      if (initialMessage != null) _queueTap(initialMessage.data);
      _navigatePendingTap();
    } catch (error) {
      debugPrint('Push initialization failed: $error');
    }
  }

  static Future<void> _initializeLocalNotifications() async {
    await _localNotifications.initialize(
      settings: const InitializationSettings(
        android: AndroidInitializationSettings('@mipmap/launcher_icon'),
        iOS: DarwinInitializationSettings(
          requestAlertPermission: false,
          requestBadgePermission: false,
          requestSoundPermission: false,
        ),
      ),
      onDidReceiveNotificationResponse: (response) {
        final payload = response.payload;
        if (payload == null || payload.isEmpty) return;
        try {
          final data = jsonDecode(payload) as Map<String, dynamic>;
          _queueTap(data);
        } catch (_) {}
      },
    );
    await _localNotifications
        .resolvePlatformSpecificImplementation<
          AndroidFlutterLocalNotificationsPlugin
        >()
        ?.createNotificationChannel(_channel);
  }

  /// Remembers the signed-in user. It does NOT open a pending push tap: the
  /// sign-in page replaces its own route right after login, which would swap the
  /// pushed target away. The page calls [openPendingTap] once it has navigated.
  static Future<void> bindAuthenticatedUser(AuthResponse auth) async {
    _auth = auth;
    if (!_initialized) return;

    try {
      await _attachTokenRefresh();
      final settings = await FirebaseMessaging.instance.getNotificationSettings();
      if (settings.authorizationStatus == AuthorizationStatus.authorized ||
          settings.authorizationStatus == AuthorizationStatus.provisional) {
        await _registerCurrentToken(auth);
      }
    } catch (error) {
      debugPrint('Push permission/token registration failed: $error');
    }
  }

  static Future<void> requestPermissionAndRegister(AuthResponse auth) async {
    if (!_initialized) return;
    _auth = auth;
    try {
      final permission = await FirebaseMessaging.instance.requestPermission(
        alert: true,
        badge: true,
        sound: true,
      );
      final granted =
          permission.authorizationStatus == AuthorizationStatus.authorized ||
          permission.authorizationStatus == AuthorizationStatus.provisional;
      if (granted) {
        await _attachTokenRefresh();
        await _registerCurrentToken(auth);
      } else {
        await _removeDevice(auth);
      }
    } catch (error) {
      debugPrint('Push permission/token registration failed: $error');
    }
  }

  static Future<void> _attachTokenRefresh() async {
    _tokenRefreshSubscription ??= FirebaseMessaging.instance.onTokenRefresh.listen(
      (registrationToken) {
        final currentAuth = _auth;
        if (currentAuth != null) {
          unawaited(_registerToken(currentAuth, registrationToken));
        }
      },
    );
  }

  static Future<void> _registerCurrentToken(AuthResponse auth) async {
    final registrationToken = await FirebaseMessaging.instance.getToken();
    if (registrationToken != null && registrationToken.isNotEmpty) {
      await _registerToken(auth, registrationToken);
    }
  }

  static Future<void> _registerToken(
    AuthResponse auth,
    String registrationToken,
  ) async {
    try {
      await NotificationService.registerPushDevice(
        token: auth.accessToken,
        deviceUuid: await _deviceUuid(),
        registrationToken: registrationToken,
        platform: defaultTargetPlatform == TargetPlatform.iOS ? 'ios' : 'android',
      );
    } catch (error) {
      debugPrint('Push device registration failed: $error');
    }
  }

  static Future<void> unregister(AuthResponse auth) async {
    _auth = null;
    await _tokenRefreshSubscription?.cancel();
    _tokenRefreshSubscription = null;
    await _removeDevice(auth);
  }

  static Future<void> _removeDevice(AuthResponse auth) async {
    if (!_initialized) return;
    try {
      await NotificationService.removePushDevice(
        token: auth.accessToken,
        deviceUuid: await _deviceUuid(),
      );
    } catch (error) {
      debugPrint('Push device removal failed: $error');
    }
  }

  static Future<String> _deviceUuid() async {
    final preferences = await SharedPreferences.getInstance();
    final stored = preferences.getString(_deviceUuidStorageKey);
    if (stored != null && stored.isNotEmpty) return stored;
    final generated = _uuid.v4();
    await preferences.setString(_deviceUuidStorageKey, generated);
    return generated;
  }

  static Future<void> _showForegroundNotification(RemoteMessage message) async {
    if (defaultTargetPlatform == TargetPlatform.iOS) return;
    final payload = jsonEncode(message.data);
    await _localNotifications.show(
      id: message.messageId.hashCode,
      title: message.notification?.title ?? 'Social Gym',
      body: message.notification?.body ?? 'You have a new notification.',
      notificationDetails: NotificationDetails(
        android: AndroidNotificationDetails(
          _channel.id,
          _channel.name,
          channelDescription: _channel.description,
          importance: Importance.high,
          priority: Priority.high,
        ),
        iOS: const DarwinNotificationDetails(),
      ),
      payload: payload,
    );
  }

  /// Opens a push tap that arrived while the user was signed out (or before the
  /// session was restored). Call it after the post-login navigation has run.
  static void openPendingTap() => _navigatePendingTap();

  static void _handleRemoteTap(RemoteMessage message) => _queueTap(message.data);

  @visibleForTesting
  static void handleTapForTest(Map<String, dynamic> data) => _queueTap(data);

  @visibleForTesting
  static void resetForTest() {
    _navigatorKey = null;
    _auth = null;
    _pendingTap = null;
    _initialized = false;
    _navigationScheduled = false;
  }

  static void _queueTap(Map<String, dynamic> data) {
    _pendingTap = Map<String, dynamic>.from(data);
    _navigatePendingTap();
  }

  static void _navigatePendingTap() {
    if (_auth == null || _pendingTap == null) return;
    final navigator = _navigatorKey?.currentState;
    if (navigator == null) {
      if (!_navigationScheduled) {
        _navigationScheduled = true;
        WidgetsBinding.instance.addPostFrameCallback((_) {
          _navigationScheduled = false;
          _navigatePendingTap();
        });
      }
      return;
    }

    final data = _pendingTap!;
    _pendingTap = null;
    final target = routeForTap(data);
    navigator.pushNamed(target.route, arguments: target.arguments);
  }

  /// Maps a push `data` payload to the route it opens. Unknown or missing
  /// targets fall back to `/notifications`.
  @visibleForTesting
  static ({String route, Map<String, String>? arguments}) routeForTap(
    Map<String, dynamic> data,
  ) {
    final targetType = data['targetType'] as String?;
    final targetUuid = data['targetUuid'] as String?;
    final hasTarget = targetUuid != null && targetUuid.isNotEmpty;
    if (targetType == 'post' && hasTarget) {
      return (route: '/feed', arguments: {'postUuid': targetUuid});
    }
    if (targetType == 'friendship_request' && hasTarget) {
      return (route: '/friends', arguments: {'friendshipUuid': targetUuid});
    }
    return (route: '/notifications', arguments: null);
  }
}