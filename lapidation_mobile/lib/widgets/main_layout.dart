import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/models/enums.dart';
import 'package:lapidation_mobile/providers/notifications_provider.dart';

import '../config/nav_section.dart';
import '../providers/person_provider.dart';
import '../providers/settings_provider.dart';
import 'app_header.dart';
import 'sidebar_menu.dart';

/// A wrapper layout that provides the AppHeader and SidebarMenu (drawer)
/// to any page that needs the standard authenticated layout.
class MainLayout extends StatefulWidget {
  final Widget body;
  final int notificationCount;
  final FloatingActionButton? floatingActionButton;

  /// Which top-level section this page belongs to.
  /// Drives the active tab in the header and the items shown in the sidebar.
  final NavSection navSection;

  /// Named route of the current page (e.g. '/feed', '/profile').
  /// Used by the sidebar to highlight the active nav item.
  final String currentRoute;

  const MainLayout({
    super.key,
    required this.body,
    this.notificationCount = 0,
    this.floatingActionButton,
    this.navSection = NavSection.home,
    this.currentRoute = '/feed',
  });

  @override
  State<MainLayout> createState() => _MainLayoutState();
}

class _MainLayoutState extends State<MainLayout> {
  final GlobalKey<ScaffoldState> _scaffoldKey = GlobalKey<ScaffoldState>();
  bool _isCollapsed = false;

  void _toggleCollapsed() {
    setState(() {
      _isCollapsed = !_isCollapsed;
    });
  }

  void _openSidebarSheet(BuildContext context, ContextMenuPosition position) {
    final isTop = position == ContextMenuPosition.top;
    showGeneralDialog(
      context: context,
      barrierLabel: 'Sidebar',
      barrierColor: Colors.black.withAlpha(102),
      barrierDismissible: true,
      pageBuilder: (dialogContext, _, _) {
        return Align(
          alignment: isTop ? Alignment.topCenter : Alignment.bottomCenter,
          child: Material(
            color: Colors.white,
            child: SafeArea(
              top: isTop,
              bottom: !isTop,
              child: SizedBox(
                width: double.infinity,
                height: MediaQuery.of(dialogContext).size.height * 0.75,
                child: SidebarContent(
                  isCollapsed: false,
                  onToggleCollapse: () {},
                  navSection: widget.navSection,
                  currentRoute: widget.currentRoute,
                ),
              ),
            ),
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final isDesktop = screenWidth >= 768;
    final position = context.watch<SettingsProvider>().contextMenuPosition;
    final isSheetStyle =
        position == ContextMenuPosition.top ||
        position == ContextMenuPosition.bottom;
    final isRightSide = position == ContextMenuPosition.right;
    final notificationCount = widget.notificationCount == 0
        ? context.select<NotificationsProvider, int>(
            (provider) => provider.unreadCount,
          )
        : widget.notificationCount;

    final sidebarDrawer = SidebarMenu(
      navSection: widget.navSection,
      currentRoute: widget.currentRoute,
    );

    return Scaffold(
      key: _scaffoldKey,
      appBar: AppHeader(
        notificationCount: notificationCount,
        currentSection: widget.navSection,
        forceMenuButton: isSheetStyle,
        onMenuPressed: () {
          if (isSheetStyle) {
            _openSidebarSheet(context, position);
          } else if (isRightSide) {
            _scaffoldKey.currentState?.openEndDrawer();
          } else {
            _scaffoldKey.currentState?.openDrawer();
          }
        },
        onSearchPressed: () {
          // TODO: Implement search
        },
        onNotificationsPressed: () {
          Navigator.of(context).pushNamed('/notifications');
        },
        onProfilePressed: () {
          final destination = context.read<PersonProvider>().isProfessional ? '/business-profile' : '/profile';
          Navigator.of(context).pushNamedAndRemoveUntil(destination, (route) => false);
        },
      ),
      drawer: (isDesktop || isSheetStyle || isRightSide) ? null : sidebarDrawer,
      endDrawer: (isDesktop || isSheetStyle || !isRightSide) ? null : sidebarDrawer,
      body: (isDesktop && !isSheetStyle)
          ? Row(
              children: isRightSide
                  ? [
                      // Main content
                      Expanded(child: widget.body),
                      // Vertical divider
                      const VerticalDivider(width: 1, thickness: 1),
                      // Persistent sidebar on desktop
                      AnimatedContainer(
                        duration: const Duration(milliseconds: 200),
                        width: _isCollapsed ? 80 : 260,
                        child: ClipRect(
                          child: Material(
                            color: Colors.white,
                            child: SidebarContent(
                              isCollapsed: _isCollapsed,
                              onToggleCollapse: _toggleCollapsed,
                              navSection: widget.navSection,
                              currentRoute: widget.currentRoute,
                            ),
                          ),
                        ),
                      ),
                    ]
                  : [
                      // Persistent sidebar on desktop
                      AnimatedContainer(
                        duration: const Duration(milliseconds: 200),
                        width: _isCollapsed ? 80 : 260,
                        child: ClipRect(
                          child: Material(
                            color: Colors.white,
                            child: SidebarContent(
                              isCollapsed: _isCollapsed,
                              onToggleCollapse: _toggleCollapsed,
                              navSection: widget.navSection,
                              currentRoute: widget.currentRoute,
                            ),
                          ),
                        ),
                      ),
                      // Vertical divider
                      const VerticalDivider(width: 1, thickness: 1),
                      // Main content
                      Expanded(child: widget.body),
                    ],
            )
          : widget.body,
      floatingActionButton: widget.floatingActionButton,
    );
  }
}
