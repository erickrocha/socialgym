import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/config/app_colors.dart';
import 'package:socialgym_mobile/models/notification.dart' as app_notification;
import 'package:socialgym_mobile/providers/person_provider.dart';

class NotificationItemWidget extends StatelessWidget {
  final app_notification.Notification notification;
  final VoidCallback? onTap;

  const NotificationItemWidget({
    super.key,
    required this.notification,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final actorName = notification.actorName.isNotEmpty
        ? notification.actorName
        : 'Someone';
    final subtitle = notification.snippet.isNotEmpty
        ? notification.snippet
        : notification.notificationType;
    final businessType =
        context.watch<PersonProvider>().activeBusinessProfile?.businessType;

    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
      color: notification.isUnread
          ? AppColors.primaryFor(businessType).withAlpha(10)
          : Colors.white,
      child: ListTile(
        onTap: onTap,
        leading: CircleAvatar(
          backgroundColor: notification.isUnread
              ? AppColors.primaryFor(businessType)
              : Colors.grey.shade300,
          child: Icon(
            _iconForType(notification.notificationType),
            color: notification.isUnread ? Colors.white : Colors.grey.shade700,
          ),
        ),
        title: Text(
          actorName,
          maxLines: 1,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(
            fontWeight: notification.isUnread
                ? FontWeight.w700
                : FontWeight.w500,
          ),
        ),
        subtitle: Text(subtitle, maxLines: 2, overflow: TextOverflow.ellipsis),
        trailing: notification.isUnread
            ? Icon(Icons.circle, color: AppColors.primaryFor(businessType), size: 10)
            : null,
      ),
    );
  }

  IconData _iconForType(String type) {
    switch (type.toLowerCase()) {
      case 'comment':
        return Icons.comment_outlined;
      case 'reaction':
      case 'like':
        return Icons.favorite_border;
      case 'friend':
      case 'follow':
        return Icons.person_add_alt_1_outlined;
      default:
        return Icons.notifications_outlined;
    }
  }
}
