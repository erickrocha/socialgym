import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../providers/feed_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/resource_provider.dart';
import '../../widgets/feed_post_widget.dart';
import '../../widgets/main_layout.dart';
import '../../widgets/person_avatar_widget.dart';
import '../../widgets/post_composer_sheet.dart';

class FeedPage extends StatefulWidget {
  const FeedPage({super.key});

  @override
  State<FeedPage> createState() => _FeedPageState();
}

class _FeedPageState extends State<FeedPage> {
  late final ScrollController _scrollController;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController()..addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _fetchInitialData();
    });
  }

  @override
  void dispose() {
    _scrollController
      ..removeListener(_onScroll)
      ..dispose();
    super.dispose();
  }

  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';

  void _onScroll() {
    if (!_scrollController.hasClients) return;

    final threshold = _scrollController.position.maxScrollExtent - 300;
    if (_scrollController.position.pixels < threshold) return;

    final token = _token;
    if (token.isEmpty) return;

    final feedProvider = context.read<FeedProvider>();
    final businessProfileUuid = context
        .read<PersonProvider>()
        .activeBusinessProfile
        ?.uuid;
    if (feedProvider.hasMore &&
        !feedProvider.loadingMore &&
        !feedProvider.loading) {
      feedProvider.loadMorePostsForProfile(
        token,
        businessProfileUuid: businessProfileUuid,
      );
    }
  }

  void _fetchInitialData() {
    final authProvider = context.read<AuthProvider>();
    final personProvider = context.read<PersonProvider>();
    final resourceProvider = context.read<ResourceProvider>();
    final feedProvider = context.read<FeedProvider>();
    final token = authProvider.auth?.accessToken ?? '';

    if (token.isEmpty) return;

    if (personProvider.person == null) {
      personProvider.fetchMe();
    }
    if (resourceProvider.resources == null) {
      resourceProvider.fetchResources(personProvider.ownerId);
    }
    final businessProfileUuid = personProvider.activeBusinessProfile?.uuid;
    feedProvider.fetchPostsForProfile(
      token,
      businessProfileUuid: businessProfileUuid,
    );
  }

  /// Opens the post composer sheet and refreshes the feed if a post was created.
  Future<void> _openComposer({String? initialContent}) async {
    final created = await PostComposerSheet.show(
      context,
      initialContent: initialContent,
    );
    if (created == true && mounted) {
      final businessProfileUuid = context
          .read<PersonProvider>()
          .activeBusinessProfile
          ?.uuid;
      context.read<FeedProvider>().fetchPostsForProfile(
        _token,
        businessProfileUuid: businessProfileUuid,
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/feed',
      body: Consumer2<FeedProvider, PersonProvider>(
        builder: (context, feedProvider, personProvider, _) {
          final person = personProvider.person;
          return RefreshIndicator(
            color: AppColors.primary,
            onRefresh: () => feedProvider.fetchPostsForProfile(
              _token,
              businessProfileUuid: personProvider.activeBusinessProfile?.uuid,
            ),
            child: CustomScrollView(
              controller: _scrollController,
              slivers: [
                // ── Create Post Card ──────────────────────────────────
                SliverToBoxAdapter(
                  child: _CreatePostCard(
                    person: person,
                    onTap: _openComposer,
                    onPhotoTap: _openComposer,
                    onVideoTap: _openComposer,
                  ),
                ),

                // ── Error banner ──────────────────────────────────────
                if (feedProvider.error != null)
                  SliverToBoxAdapter(
                    child: Container(
                      margin: const EdgeInsets.symmetric(
                        horizontal: 12,
                        vertical: 4,
                      ),
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
                              feedProvider.error!,
                              style: const TextStyle(
                                color: AppColors.danger,
                                fontSize: 13,
                              ),
                            ),
                          ),
                          TextButton(
                            onPressed: () => feedProvider.fetchPostsForProfile(
                              _token,
                              businessProfileUuid:
                                  personProvider.activeBusinessProfile?.uuid,
                            ),
                            child: const Text('Retry'),
                          ),
                        ],
                      ),
                    ),
                  ),

                // ── Loading ───────────────────────────────────────────
                if (feedProvider.loading)
                  const SliverFillRemaining(
                    hasScrollBody: false,
                    child: Center(
                      child: CircularProgressIndicator(
                        color: AppColors.primary,
                      ),
                    ),
                  )
                // ── Empty state ───────────────────────────────────────
                else if (feedProvider.posts.isEmpty &&
                    feedProvider.error == null)
                  SliverFillRemaining(
                    hasScrollBody: false,
                    child: _EmptyFeed(onCreatePost: _openComposer),
                  )
                // ── Post list ─────────────────────────────────────────
                else
                  SliverList(
                    delegate: SliverChildBuilderDelegate(
                      (ctx, i) => FeedPostWidget(post: feedProvider.posts[i]),
                      childCount: feedProvider.posts.length,
                    ),
                  ),

                if (feedProvider.loadingMore)
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

// ─────────────────────────────────────────────────────────────────────────────
// Create Post Card  (tapping any part opens the composer sheet)
// ─────────────────────────────────────────────────────────────────────────────

class _CreatePostCard extends StatelessWidget {
  final Person? person;
  final Future<void> Function({String? initialContent}) onTap;
  final Future<void> Function({String? initialContent}) onPhotoTap;
  final Future<void> Function({String? initialContent}) onVideoTap;

  const _CreatePostCard({
    required this.person,
    required this.onTap,
    required this.onPhotoTap,
    required this.onVideoTap,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Card(
      margin: const EdgeInsets.fromLTRB(0, 0, 0, 4),
      shape: const RoundedRectangleBorder(),
      elevation: 1,
      color: Colors.white,
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // ── "What's on your mind?" prompt ─────────────────────────
            Row(
              children: [
                PersonAvatar(person: person),
                const SizedBox(width: 10),
                Expanded(
                  child: GestureDetector(
                    onTap: () => onTap(),
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 14,
                        vertical: 10,
                      ),
                      decoration: BoxDecoration(
                        color: Colors.grey[100],
                        borderRadius: BorderRadius.circular(24),
                        border: Border.all(color: Colors.grey[300]!),
                      ),
                      child: Text(
                        l10n.feedWriteSomething,
                        style: TextStyle(color: Colors.grey[500], fontSize: 14),
                      ),
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            const Divider(height: 1, thickness: 0.5),
            const SizedBox(height: 8),
            // ── Quick action row ──────────────────────────────────────
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                _QuickAction(
                  icon: Icons.photo_library_outlined,
                  label: l10n.feedAddPhoto,
                  color: AppColors.success,
                  onTap: () => onPhotoTap(),
                ),
                _QuickAction(
                  icon: Icons.videocam_outlined,
                  label: l10n.feedAddVideo,
                  color: AppColors.danger,
                  onTap: () => onVideoTap(),
                ),
                _QuickAction(
                  icon: Icons.emoji_emotions_outlined,
                  label: l10n.feedFeeling,
                  color: AppColors.third,
                  onTap: () => onTap(),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _QuickAction extends StatelessWidget {
  final IconData icon;
  final String label;
  final Color color;
  final VoidCallback onTap;

  const _QuickAction({
    required this.icon,
    required this.label,
    required this.color,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, color: color, size: 20),
            const SizedBox(width: 6),
            Text(
              label,
              style: TextStyle(
                color: Colors.grey[700],
                fontSize: 12,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Empty feed state
// ─────────────────────────────────────────────────────────────────────────────

class _EmptyFeed extends StatelessWidget {
  final VoidCallback onCreatePost;
  const _EmptyFeed({required this.onCreatePost});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.feed_outlined, size: 80, color: Colors.grey[300]),
            const SizedBox(height: 16),
            Text(
              l10n.feedNoPostsYet,
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.w600,
                color: Colors.grey[600],
              ),
            ),
            const SizedBox(height: 8),
            Text(
              l10n.feedStartSharing,
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 14, color: Colors.grey[400]),
            ),
            const SizedBox(height: 24),
            ElevatedButton.icon(
              onPressed: onCreatePost,
              icon: const Icon(Icons.add),
              label: Text(l10n.feedCreatePost),
              style: ElevatedButton.styleFrom(
                backgroundColor: AppColors.primary,
                foregroundColor: Colors.white,
                padding: const EdgeInsets.symmetric(
                  horizontal: 24,
                  vertical: 12,
                ),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(24),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
