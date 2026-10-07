import '../../models/notification.dart';
import '../../src/generated/grpc/timeline/notification.pbgrpc.dart' as $notification;
import '../../src/generated/grpc/timeline/push_device.pbgrpc.dart' as $device;
import 'grpc_timeline.dart';

/// Notifications and push devices over gRPC, replacing the REST `NotificationService`. The
/// recipient is the person in the token, so there is no owner argument.
class GrpcNotificationService {
  GrpcNotificationService._();

  static $notification.NotificationServiceClient get _notifications => $notification.NotificationServiceClient(
        GrpcTimeline.channel,
        interceptors: GrpcTimeline.interceptors,
      );
  static $device.PushDeviceServiceClient get _devices =>
      $device.PushDeviceServiceClient(GrpcTimeline.channel, interceptors: GrpcTimeline.interceptors);

  static Future<void> registerPushDevice({
    required String token,
    required String deviceUuid,
    required String registrationToken,
    required String platform,
  }) => GrpcTimeline.run(() async {
        await _devices.registerPushDevice(
          $device.RegisterPushDeviceRequest(
            deviceUuid: deviceUuid,
            platform: platform,
            registrationToken: registrationToken,
          ),
          options: GrpcTimeline.withToken(token),
        );
      }, 'Failed to register push device');

  static Future<void> removePushDevice({required String token, required String deviceUuid}) =>
      GrpcTimeline.run(() async {
        await _devices.removePushDevice(
          $device.RemovePushDeviceRequest(deviceUuid: deviceUuid),
          options: GrpcTimeline.withToken(token),
        );
      }, 'Failed to remove push device');

  static Future<List<Notification>> fetchNotifications({bool unreadOnly = false, int limit = 50}) =>
      GrpcTimeline.run(() async {
        final response = await _notifications.listNotifications(
          $notification.ListNotificationsRequest(unreadOnly: unreadOnly, limit: limit),
          options: GrpcTimeline.options,
        );
        return response.notifications.map((n) => Notification.fromJson(GrpcTimeline.json(n))).toList();
      }, 'Failed to load notifications');

  /// Marks the notification read; the server answers only `read: true`, so the caller keeps the
  /// notification it already holds and flips its flag.
  static Future<void> markNotificationAsRead(String idempotencyKey) => GrpcTimeline.run(() async {
        await _notifications.markNotificationRead(
          $notification.MarkNotificationReadRequest(idempotencyKey: idempotencyKey),
          options: GrpcTimeline.options,
        );
      }, 'Failed to mark notification as read');
}
