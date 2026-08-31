import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:video_player/video_player.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/feed_post.dart';
import '../../providers/auth_provider.dart';
import '../../providers/feed_provider.dart';
import '../../providers/person_provider.dart';
import '../../widgets/main_layout.dart';
import '../../widgets/full_screen_image_viewer.dart';

class GalleryPage extends StatefulWidget {
  const GalleryPage({super.key});

  @override
  State<GalleryPage> createState() => _GalleryPageState();
}

class _GalleryPageState extends State<GalleryPage> {
  late final PageController _pageController;
  late final ScrollController _scrollController;

  @override
  void initState() {
    super.initState();
    _pageController = PageController();
    _scrollController = ScrollController()..addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _loadPosts();
    });
  }

  @override
  void dispose() {
    _pageController.dispose();
    _scrollController
      ..removeListener(_onScroll)
      ..dispose();
    super.dispose();
  }

  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';

  String? get _businessProfileUuid =>
      context.read<PersonProvider>().activeBusinessProfile?.uuid;

  void _onScroll() {
    if (!_scrollController.hasClients) return;

    final threshold = _scrollController.position.maxScrollExtent - 300;
    if (_scrollController.position.pixels < threshold) return;

    final token = _token;
    if (token.isEmpty) return;

    final feedProvider = context.read<FeedProvider>();
    if (feedProvider.hasMore &&
        !feedProvider.loadingMore &&
        !feedProvider.loading) {
      feedProvider.loadMorePostsForProfile(
        token,
        businessProfileUuid: _businessProfileUuid,
      );
    }
  }

  void _loadPosts() {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    if (token.isNotEmpty) {
      context.read<FeedProvider>().fetchPostsForProfile(
        token,
        businessProfileUuid: _businessProfileUuid,
      );
    }
  }

  void _loadMoreIfNeeded(
    FeedProvider feedProvider,
    int index,
    int totalMediaPosts,
  ) {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    if (token.isEmpty || totalMediaPosts == 0) return;

    final shouldLoadMore = index >= totalMediaPosts - 2;
    if (shouldLoadMore && feedProvider.hasMore && !feedProvider.loadingMore) {
      feedProvider.loadMorePostsForProfile(
        token,
        businessProfileUuid: _businessProfileUuid,
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return MainLayout(
      navSection: NavSection.gallery,
      currentRoute: '/gallery',
      body: Consumer<FeedProvider>(
        builder: (context, feedProvider, _) {
          final businessType = context
              .watch<PersonProvider>()
              .activeBusinessProfile
              ?.businessType;

          if (feedProvider.loading) {
            return Center(
              child: CircularProgressIndicator(
                color: AppColors.primaryFor(businessType),
              ),
            );
          }

          if (feedProvider.error != null) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.error_outline, color: AppColors.danger, size: 48),
                  const SizedBox(height: 16),
                  Text(
                    feedProvider.error!,
                    textAlign: TextAlign.center,
                    style: const TextStyle(color: AppColors.danger),
                  ),
                  const SizedBox(height: 24),
                  ElevatedButton(
                    onPressed: _loadPosts,
                    child: const Text('Retry'),
                  ),
                ],
              ),
            );
          }

          if (feedProvider.posts.isEmpty) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.image_outlined, color: Colors.grey[400], size: 48),
                  const SizedBox(height: 16),
                  Text(
                    AppLocalizations.of(context)!.feedNoPostsYet,
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      color: Colors.grey[600],
                    ),
                  ),
                ],
              ),
            );
          }

          final posts = feedProvider.posts
              .where((p) => p.media.isNotEmpty)
              .toList();

          if (posts.isEmpty &&
              feedProvider.posts.isNotEmpty &&
              feedProvider.hasMore &&
              !feedProvider.loadingMore) {
            WidgetsBinding.instance.addPostFrameCallback((_) {
              if (!mounted) return;
              final token =
                  context.read<AuthProvider>().auth?.accessToken ?? '';
              if (token.isNotEmpty) {
                context.read<FeedProvider>().loadMorePostsForProfile(
                  token,
                  businessProfileUuid: _businessProfileUuid,
                );
              }
            });
          }

          if (posts.isEmpty) {
            return Center(
              child: Text(
                'No media posts yet',
                style: TextStyle(color: Colors.grey[600]),
              ),
            );
          }

          // Vertical feed — one post per page, swipe up/down to navigate
          return RefreshIndicator(
            color: AppColors.primaryFor(businessType),
            onRefresh: () async {
              final token =
                  context.read<AuthProvider>().auth?.accessToken ?? '';
              if (token.isNotEmpty) {
                await context.read<FeedProvider>().fetchPostsForProfile(
                  token,
                  businessProfileUuid: _businessProfileUuid,
                );
              }
            },
            child: Stack(
              children: [
                // Transparent scrollable layer to capture pull-to-refresh gesture
                ListView(
                  controller: _scrollController,
                  physics: const AlwaysScrollableScrollPhysics(),
                  children: const [SizedBox.shrink()],
                ),
                PageView.builder(
                  controller: _pageController,
                  scrollDirection: Axis.vertical,
                  itemCount: posts.length,
                  onPageChanged: (index) =>
                      _loadMoreIfNeeded(feedProvider, index, posts.length),
                  itemBuilder: (context, index) =>
                      _PostFeedCard(post: posts[index]),
                ),
                if (feedProvider.loadingMore)
                  const Positioned(
                    left: 0,
                    right: 0,
                    bottom: 24,
                    child: Center(
                      child: DecoratedBox(
                        decoration: BoxDecoration(
                          color: Colors.black54,
                          borderRadius: BorderRadius.all(Radius.circular(999)),
                        ),
                        child: Padding(
                          padding: EdgeInsets.symmetric(
                            horizontal: 14,
                            vertical: 8,
                          ),
                          child: SizedBox(
                            width: 18,
                            height: 18,
                            child: CircularProgressIndicator(
                              strokeWidth: 2,
                              color: Colors.white,
                            ),
                          ),
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
// Single Post Feed Card (one full "page" in the vertical feed)
// ─────────────────────────────────────────────────────────────────────────────

class _PostFeedCard extends StatefulWidget {
  final FeedPost post;

  const _PostFeedCard({required this.post});

  @override
  State<_PostFeedCard> createState() => _PostFeedCardState();
}

class _PostFeedCardState extends State<_PostFeedCard> {
  int _currentMediaIndex = 0;
  final TextEditingController _commentController = TextEditingController();
  bool _submittingComment = false;

  @override
  void dispose() {
    _commentController.dispose();
    super.dispose();
  }

  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';

  Future<void> _submitComment(BuildContext ctx) async {
    final content = _commentController.text.trim();
    if (content.isEmpty) return;

    setState(() => _submittingComment = true);

    final feedProvider = ctx.read<FeedProvider>();
    final authProvider = ctx.read<AuthProvider>();
    final personProvider = ctx.read<PersonProvider>();
    if (authProvider.auth == null) {
      setState(() => _submittingComment = false);
      return;
    }
    final person = personProvider.person;

    final commentData = {
      'content': content,
      'authorUuid': person?.uuid ?? '',
      'authorName': person?.fullName ?? '',
      'authorAvatar': person?.avatar ?? authProvider.auth?.personAvatar,
      'postUuid': widget.post.uuid,
    };

    final ok = await feedProvider.addComment(
      commentData,
      widget.post.uuid,
      _token,
    );
    if (mounted) {
      setState(() => _submittingComment = false);
      if (ok) _commentController.clear();
    }
  }

  String _timeAgo(DateTime dt) {
    final diff = DateTime.now().difference(dt);
    if (diff.inSeconds < 60) return 'just now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m';
    if (diff.inHours < 24) return '${diff.inHours}h';
    if (diff.inDays < 7) return '${diff.inDays}d';
    return '${dt.day}/${dt.month}/${dt.year}';
  }

  void _openFullScreenViewer(int initialIndex) {
    Navigator.push(
      context,
      PageRouteBuilder(
        opaque: false,
        barrierColor: Colors.black,
        pageBuilder: (_, _, _) => FullScreenImageViewer(
          media: widget.post.media,
          initialIndex: initialIndex,
        ),
        transitionsBuilder: (_, anim, _, child) =>
            FadeTransition(opacity: anim, child: child),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final post = widget.post;
    final clampedMediaIndex = _currentMediaIndex.clamp(
      0,
      post.media.length - 1,
    );
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

    return Column(
      children: [
        // ── Media Carousel ────────────────────────────────────────────
        Expanded(
          child: Stack(
            children: [
              // Horizontal PageView for media items
              PageView.builder(
                onPageChanged: (index) =>
                    setState(() => _currentMediaIndex = index),
                itemCount: post.media.length,
                itemBuilder: (context, index) {
                  return _MediaDisplay(
                    media: post.media[index],
                    businessType: businessType,
                    onTap: () => _openFullScreenViewer(index),
                  );
                },
              ),

              // ── Top bar: author info + media counter ──────────────
              Positioned(
                top: 0,
                left: 0,
                right: 0,
                child: Container(
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      begin: Alignment.topCenter,
                      end: Alignment.bottomCenter,
                      colors: [Colors.black.withAlpha(102), Colors.transparent],
                    ),
                  ),
                  padding: const EdgeInsets.fromLTRB(16, 12, 16, 16),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      // Author info
                      Expanded(
                        child: Row(
                          children: [
                            _AuthorAvatar(
                              avatarUrl: post.authorAvatar,
                              cacheKey: post.authorObjectKey,
                              name: post.authorName ?? '',
                              businessType: businessType,
                            ),
                            const SizedBox(width: 10),
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  Text(
                                    post.authorName ?? 'Unknown',
                                    style: const TextStyle(
                                      color: Colors.white,
                                      fontWeight: FontWeight.w600,
                                      fontSize: 14,
                                    ),
                                    maxLines: 1,
                                    overflow: TextOverflow.ellipsis,
                                  ),
                                  Text(
                                    _timeAgo(post.createdAt),
                                    style: const TextStyle(
                                      color: Colors.white70,
                                      fontSize: 12,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                      // Media counter badge
                      if (post.media.length > 1)
                        Container(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 10,
                            vertical: 4,
                          ),
                          decoration: BoxDecoration(
                            color: Colors.black54,
                            borderRadius: BorderRadius.circular(12),
                          ),
                          child: Text(
                            '${clampedMediaIndex + 1}/${post.media.length}',
                            style: const TextStyle(
                              color: Colors.white,
                              fontSize: 12,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                        ),
                    ],
                  ),
                ),
              ),

              // ── Bottom: media dots indicator ──────────────────────
              if (post.media.length > 1)
                Positioned(
                  bottom: 16,
                  left: 0,
                  right: 0,
                  child: Center(
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: List.generate(
                        post.media.length,
                        (index) => Container(
                          width: 6,
                          height: 6,
                          margin: const EdgeInsets.symmetric(horizontal: 4),
                          decoration: BoxDecoration(
                            shape: BoxShape.circle,
                            color: index == clampedMediaIndex
                                ? Colors.white
                                : Colors.white54,
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
            ],
          ),
        ),

        // ── Bottom Sheet: Caption + Comments + Input ──────────────────
        Container(
          color: Colors.white,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              // Caption and reactions
              Padding(
                padding: const EdgeInsets.fromLTRB(12, 12, 12, 8),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    if (post.totalReactions > 0)
                      Padding(
                        padding: const EdgeInsets.only(bottom: 8),
                        child: _ReactionSummaryRow(post: post),
                      ),
                    if (post.content.isNotEmpty)
                      Text(
                        post.content,
                        style: const TextStyle(fontSize: 14, height: 1.4),
                        maxLines: 3,
                        overflow: TextOverflow.ellipsis,
                      ),
                  ],
                ),
              ),

              const Divider(height: 1, thickness: 0.5),

              // Comments list
              if (post.comments.isNotEmpty)
                SizedBox(
                  height: 120,
                  child: ListView.builder(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 12,
                      vertical: 8,
                    ),
                    itemCount: post.comments.length,
                    itemBuilder: (context, index) => _GalleryCommentBubble(
                      comment: post.comments[index],
                      businessType: businessType,
                    ),
                  ),
                ),

              const Divider(height: 1, thickness: 0.5),

              // Add comment input
              Padding(
                padding: const EdgeInsets.fromLTRB(12, 8, 12, 12),
                child: Row(
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _commentController,
                        decoration: InputDecoration(
                          hintText: AppLocalizations.of(
                            context,
                          )!.feedAddComment,
                          hintStyle: TextStyle(
                            fontSize: 13,
                            color: Colors.grey[500],
                          ),
                          filled: true,
                          fillColor: Colors.grey[100],
                          contentPadding: const EdgeInsets.symmetric(
                            horizontal: 12,
                            vertical: 8,
                          ),
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(20),
                            borderSide: BorderSide.none,
                          ),
                          isDense: true,
                        ),
                        style: const TextStyle(fontSize: 13),
                        maxLines: 2,
                        minLines: 1,
                      ),
                    ),
                    const SizedBox(width: 8),
                    if (_submittingComment)
                      SizedBox(
                        width: 32,
                        height: 32,
                        child: CircularProgressIndicator(
                          strokeWidth: 2,
                          color: AppColors.primaryFor(businessType),
                        ),
                      )
                    else
                      InkWell(
                        onTap: () => _submitComment(context),
                        borderRadius: BorderRadius.circular(16),
                        child: Container(
                          width: 32,
                          height: 32,
                          decoration: BoxDecoration(
                            color: AppColors.primaryFor(businessType),
                            shape: BoxShape.circle,
                          ),
                          child: const Icon(
                            Icons.send,
                            color: Colors.white,
                            size: 16,
                          ),
                        ),
                      ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Media Display Component
// ─────────────────────────────────────────────────────────────────────────────

class _MediaDisplay extends StatefulWidget {
  final FeedMedia media;
  final VoidCallback? onTap;
  final String? businessType;

  const _MediaDisplay({required this.media, this.onTap, this.businessType});

  @override
  State<_MediaDisplay> createState() => _MediaDisplayState();
}

class _MediaDisplayState extends State<_MediaDisplay> {
  late VideoPlayerController _videoController;
  bool _videoInitialized = false;
  bool _videoPlaying = false;

  @override
  void initState() {
    super.initState();
    if (widget.media.isVideo) {
      _initializeVideo();
    }
  }

  void _initializeVideo() {
    _videoController =
        VideoPlayerController.networkUrl(Uri.parse(widget.media.url))
          ..initialize().then((_) {
            if (mounted) setState(() => _videoInitialized = true);
          });
  }

  @override
  void dispose() {
    if (widget.media.isVideo) {
      _videoController.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.media.isVideo) {
      return _buildVideoDisplay();
    }
    return _buildImageDisplay();
  }

  Widget _buildImageDisplay() {
    final imageWidget = CachedNetworkImage(
      imageUrl: widget.media.url,
      cacheKey: widget.media.objectKey,
      fit: BoxFit.cover,
      placeholder: (_, _) => Container(
        color: Colors.grey[200],
        child: Center(
          child: CircularProgressIndicator(
            color: AppColors.primaryFor(widget.businessType),
          ),
        ),
      ),
      errorWidget: (_, _, _) => Container(
        color: Colors.grey[300],
        child: const Center(
          child: Icon(Icons.image_not_supported, color: Colors.grey),
        ),
      ),
    );

    if (widget.onTap == null) return imageWidget;

    return GestureDetector(
      onTap: widget.onTap,
      child: Stack(
        fit: StackFit.expand,
        children: [
          imageWidget,
          Positioned(
            top: 12,
            right: 12,
            child: Container(
              decoration: BoxDecoration(
                color: Colors.black54,
                shape: BoxShape.circle,
              ),
              padding: const EdgeInsets.all(8),
              child: const Icon(
                Icons.fullscreen,
                color: Colors.white,
                size: 20,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildVideoDisplay() {
    if (!_videoInitialized) {
      return Container(
        color: Colors.black,
        child: const Center(
          child: CircularProgressIndicator(color: Colors.white),
        ),
      );
    }

    return GestureDetector(
      onLongPress: widget.onTap,
      onTap: () {
        setState(() {
          _videoPlaying = !_videoPlaying;
          _videoPlaying ? _videoController.play() : _videoController.pause();
        });
      },
      child: Stack(
        alignment: Alignment.center,
        children: [
          AspectRatio(
            aspectRatio: _videoController.value.aspectRatio,
            child: VideoPlayer(_videoController),
          ),
          if (!_videoPlaying)
            Container(
              decoration: const BoxDecoration(
                color: Colors.black38,
                shape: BoxShape.circle,
              ),
              padding: const EdgeInsets.all(12),
              child: const Icon(
                Icons.play_arrow,
                color: Colors.white,
                size: 36,
              ),
            ),
          // Expand hint (top-right)
          if (widget.onTap != null)
            Positioned(
              top: 12,
              right: 12,
              child: Container(
                decoration: BoxDecoration(
                  color: Colors.black54,
                  shape: BoxShape.circle,
                ),
                padding: const EdgeInsets.all(8),
                child: const Icon(
                  Icons.fullscreen,
                  color: Colors.white,
                  size: 20,
                ),
              ),
            ),
        ],
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Author Avatar Component
// ─────────────────────────────────────────────────────────────────────────────

class _AuthorAvatar extends StatelessWidget {
  final String? avatarUrl;
  final String? cacheKey;
  final String name;
  final String? businessType;

  const _AuthorAvatar({
    required this.avatarUrl,
    required this.name,
    this.cacheKey,
    this.businessType,
  });

  @override
  Widget build(BuildContext context) {
    if (avatarUrl != null && avatarUrl!.isNotEmpty) {
      return CircleAvatar(
        radius: 18,
        backgroundColor: Colors.white,
        child: ClipOval(
          child: CachedNetworkImage(
            imageUrl: avatarUrl!,
            cacheKey: cacheKey,
            width: 36,
            height: 36,
            fit: BoxFit.cover,
            placeholder: (_, _) => CircularProgressIndicator(
              strokeWidth: 2,
              color: AppColors.primaryFor(businessType),
            ),
            errorWidget: (_, _, _) => _initials(name),
          ),
        ),
      );
    }

    return CircleAvatar(
      radius: 18,
      backgroundColor: Colors.white,
      child: _initials(name),
    );
  }

  Widget _initials(String name) {
    final parts = name.trim().split(' ');
    final text = parts.length >= 2
        ? '${parts.first[0]}${parts.last[0]}'.toUpperCase()
        : name.isNotEmpty
        ? name[0].toUpperCase()
        : '?';
    return Text(
      text,
      style: TextStyle(
        color: AppColors.primaryFor(businessType),
        fontWeight: FontWeight.bold,
        fontSize: 14,
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reaction Summary Row
// ─────────────────────────────────────────────────────────────────────────────

class _ReactionSummaryRow extends StatelessWidget {
  final FeedPost post;

  const _ReactionSummaryRow({required this.post});

  @override
  Widget build(BuildContext context) {
    final reactionCounts = <String, int>{};
    for (final r in post.reactions) {
      reactionCounts[r.type] = (reactionCounts[r.type] ?? 0) + 1;
    }

    final sortedReactions = reactionCounts.entries.toList()
      ..sort((a, b) => b.value.compareTo(a.value));

    return Row(
      children: [
        ...sortedReactions.take(3).map((entry) {
          final reactionType = ReactionType.fromString(entry.key);
          return Text(reactionType.emoji, style: const TextStyle(fontSize: 14));
        }),
        const SizedBox(width: 4),
        Text(
          '${post.totalReactions}',
          style: TextStyle(fontSize: 12, color: Colors.grey[600]),
        ),
      ],
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Gallery-Optimized Comment Bubble
// ─────────────────────────────────────────────────────────────────────────────

class _GalleryCommentBubble extends StatelessWidget {
  final FeedComment comment;
  final String? businessType;

  const _GalleryCommentBubble({required this.comment, this.businessType});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Avatar
          CircleAvatar(
            radius: 12,
            backgroundImage: comment.authorAvatar != null
                ? CachedNetworkImageProvider(
                    comment.authorAvatar!,
                    cacheKey: comment.authorObjectKey,
                  )
                : null,
            backgroundColor: AppColors.primaryFor(businessType).withAlpha(40),
            child: comment.authorAvatar == null
                ? Text(
                    _getInitials(comment.authorName ?? 'U'),
                    style: TextStyle(
                      fontSize: 8,
                      fontWeight: FontWeight.bold,
                      color: AppColors.primaryFor(businessType),
                    ),
                  )
                : null,
          ),
          const SizedBox(width: 6),
          // Comment bubble
          Expanded(
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
              decoration: BoxDecoration(
                color: Colors.grey[100],
                borderRadius: BorderRadius.circular(10),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    comment.authorName ?? 'User',
                    style: const TextStyle(
                      fontWeight: FontWeight.w600,
                      fontSize: 12,
                    ),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 2),
                  Text(
                    comment.content,
                    style: const TextStyle(fontSize: 12),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 2),
                  Text(
                    _timeAgo(comment.createdAt),
                    style: TextStyle(fontSize: 10, color: Colors.grey[500]),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  String _getInitials(String name) {
    final parts = name.trim().split(' ');
    if (parts.length >= 2) {
      return '${parts.first[0]}${parts.last[0]}'.toUpperCase();
    }
    return name.isNotEmpty ? name[0].toUpperCase() : '?';
  }

  String _timeAgo(DateTime dt) {
    final diff = DateTime.now().difference(dt);
    if (diff.inSeconds < 60) return 'now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m';
    if (diff.inHours < 24) return '${diff.inHours}h';
    return '${diff.inDays}d';
  }
}
