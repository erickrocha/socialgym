import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/config/app_colors.dart';
import 'package:socialgym_mobile/config/nav_section.dart';
import 'package:socialgym_mobile/models/notification.dart' as app_notification;
import 'package:socialgym_mobile/providers/auth_provider.dart';
import 'package:socialgym_mobile/providers/notifications_provider.dart';
import 'package:socialgym_mobile/providers/person_provider.dart';
import 'package:socialgym_mobile/widgets/main_layout.dart';
import 'package:socialgym_mobile/widgets/notification_item_widget.dart';

class NotificationsPage extends StatefulWidget {
  const NotificationsPage({super.key});

  @override
  State<NotificationsPage> createState() => _NotificationsPageState();
}

class _NotificationsPageState extends State<NotificationsPage> {
  late final ScrollController _scrollController;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController()..addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) => _fetchNotifications());
  }

  @override
  void dispose() {
    _scrollController
      ..removeListener(_onScroll)
      ..dispose();
    super.dispose();
  }

  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';
  String get _mentionedUuid => context.read<PersonProvider>().activeAuthorUuid;

  void _fetchNotifications() {
    final token = _token;
    final mentionedUuid = _mentionedUuid;
    if (token.isEmpty || mentionedUuid.isEmpty) return;
    context.read<NotificationsProvider>().fetchNotifications(token, mentionedUuid);
  }

  void _onScroll() {
    if (!_scrollController.hasClients) return;

    final threshold = _scrollController.position.maxScrollExtent - 200;
    if (_scrollController.position.pixels < threshold) return;

    final token = _token;
    final mentionedUuid = _mentionedUuid;
    if (token.isEmpty || mentionedUuid.isEmpty) return;

    final notificationsProvider = context.read<NotificationsProvider>();
    if (!notificationsProvider.loadingMore && !notificationsProvider.loading) {
      notificationsProvider.loadMoreNotifications(token, mentionedUuid);
    }
  }

  Future<void> _handleNotificationTap(NotificationsProvider provider,app_notification.Notification notification) async {
    // Mark as read
    await provider.markAsRead(_token, _mentionedUuid, notification.uuid);

    // Navigate to the post
    if (mounted && context.mounted) {
      Navigator.of(
        context,
      ).pushNamed('/feed', arguments: {'postUuid': notification.postUuid});
    }
  }

  @override
  Widget build(BuildContext context) {
    final personProvider = context.read<PersonProvider>();
    final mentionedUuid = personProvider.activeAuthorUuid;
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/notifications',
      body: Consumer<NotificationsProvider>(
        builder: (context, notificationsProvider, _) {
          return RefreshIndicator(
            color: AppColors.primary,
            onRefresh: () => notificationsProvider.fetchNotifications(_token, mentionedUuid),
            child: CustomScrollView(
              controller: _scrollController,
              slivers: [
                if (notificationsProvider.error != null)
                  SliverToBoxAdapter(
                    child: Container(
                      margin: const EdgeInsets.all(12),
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: AppColors.danger.withAlpha(20),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Row(
                        children: [
                          const Icon(
                            Icons.error_outline,
                            color: AppColors.danger,
                            size: 18,
                          ),
                          const SizedBox(width: 8),
                          Expanded(
                            child: Text(
                              notificationsProvider.error!,
                              style: const TextStyle(
                                color: AppColors.danger,
                                fontSize: 13,
                              ),
                            ),
                          ),
                          TextButton(
                            onPressed: _fetchNotifications,
                            child: const Text('Retry'),
                          ),
                        ],
                      ),
                    ),
                  ),
                if (notificationsProvider.loading &&
                    notificationsProvider.notifications.isEmpty)
                  const SliverFillRemaining(
                    hasScrollBody: false,
                    child: Center(
                      child: CircularProgressIndicator(
                        color: AppColors.primary,
                      ),
                    ),
                  )
                else if (notificationsProvider.notifications.isEmpty)
                  const SliverFillRemaining(
                    hasScrollBody: false,
                    child: Center(
                      child: Text(
                        'No notifications yet.',
                        style: TextStyle(fontSize: 16, color: Colors.black54),
                      ),
                    ),
                  )
                else
                  SliverList(
                    delegate: SliverChildBuilderDelegate((context, index) {
                      final notification =
                          notificationsProvider.notifications[index];
                      return NotificationItemWidget(
                        notification: notification,
                        onTap: () => _handleNotificationTap(
                          notificationsProvider,
                          notification,
                        ),
                      );
                    }, childCount: notificationsProvider.notifications.length),
                  ),
                if (notificationsProvider.loadingMore)
                  const SliverToBoxAdapter(
                    child: Padding(
                      padding: EdgeInsets.symmetric(vertical: 16),
                      child: Center(
                        child: CircularProgressIndicator(
                          color: AppColors.primary,
                        ),
                      ),
                    ),
                  ),
              ],
            ),
          );
        },
      ),
    );
  }
}
