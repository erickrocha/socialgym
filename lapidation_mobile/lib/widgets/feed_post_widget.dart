import 'dart:async';

import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:video_player/video_player.dart';

import '../providers/person_provider.dart';
import 'full_screen_image_viewer.dart';
import 'mention_text.dart';

import '../config/app_colors.dart';
import '../l10n/app_localizations.dart';
import '../models/feed_post.dart';
import '../models/person.dart';
import '../models/mentionable_friend.dart';
import '../providers/auth_provider.dart';
import '../providers/feed_provider.dart';
import '../services/grpc/grpc_person_service.dart';
import '../pages/profile/person_profile_page.dart';
import '../utils/mention_text_utils.dart';

// ─────────────────────────────────────────────────────────────────────────────
// Main Post Card
// ─────────────────────────────────────────────────────────────────────────────

class FeedPostWidget extends StatefulWidget {
  final FeedPost post;

  const FeedPostWidget({super.key, required this.post});

  @override
  State<FeedPostWidget> createState() => _FeedPostWidgetState();
}

class _FeedPostWidgetState extends State<FeedPostWidget> with SingleTickerProviderStateMixin {
  bool _showReactionPicker = false;
  bool _submittingComment = false;
  String? _replyToCommentId;
  String? _replyToCommentAuthor;
  final TextEditingController _commentController = TextEditingController();
  Timer? _mentionDebounce;
  MentionQuery? _activeMention;
  List<MentionableFriend> _mentionSuggestions = const [];
  bool _isSearchingMentions = false;
  int _mentionRequestId = 0;
  late AnimationController _reactionAnim;
  late Animation<double> _reactionFade;

  @override
  void initState() {
    super.initState();
    _commentController.addListener(_onCommentTextChanged);
    _reactionAnim = AnimationController(vsync: this, duration: const Duration(milliseconds: 200));
    _reactionFade = CurvedAnimation(parent: _reactionAnim, curve: Curves.easeInOut);
  }

  @override
  void dispose() {
    _mentionDebounce?.cancel();
    _commentController.removeListener(_onCommentTextChanged);
    _commentController.dispose();
    _reactionAnim.dispose();
    super.dispose();
  }

  String _token(BuildContext ctx) => ctx.read<AuthProvider>().auth?.accessToken ?? '';

  int _personId(BuildContext ctx) => ctx.read<AuthProvider>().auth?.personId ?? 0;

  void _toggleReactionPicker() {
    setState(() => _showReactionPicker = !_showReactionPicker);
    if (_showReactionPicker) {
      _reactionAnim.forward();
    } else {
      _reactionAnim.reverse();
    }
  }

  void _onCommentTextChanged() {
    final selection = _commentController.selection;
    if (!selection.isValid || !selection.isCollapsed) {
      _clearMentionSuggestions();
      return;
    }

    final query = findActiveMentionQuery(_commentController.text, selection.baseOffset);
    if (query == null || query.token.length < 2) {
      _clearMentionSuggestions();
      return;
    }

    _activeMention = query;
    _mentionDebounce?.cancel();
    _mentionDebounce = Timer(const Duration(milliseconds: 300), () {
      _searchMentions(context, query.token);
    });
  }

  Future<void> _searchMentions(BuildContext ctx, String rawQuery) async {
    final personId = _personId(ctx);
    if (personId <= 0) {
      _clearMentionSuggestions();
      return;
    }

    final requestId = ++_mentionRequestId;
    setState(() => _isSearchingMentions = true);
    try {
      final suggestions = await GrpcPersonService.searchMentionableFriends(
        personId: personId,
        query: rawQuery,
        limit: 20,
      );
      if (!mounted || requestId != _mentionRequestId) return;
      setState(() {
        _mentionSuggestions = suggestions;
        _isSearchingMentions = false;
      });
    } catch (_) {
      if (!mounted || requestId != _mentionRequestId) return;
      setState(() {
        _mentionSuggestions = const [];
        _isSearchingMentions = false;
      });
    }
  }

  void _clearMentionSuggestions() {
    _mentionDebounce?.cancel();
    _activeMention = null;
    _mentionRequestId++;
    if (_mentionSuggestions.isEmpty && !_isSearchingMentions) return;
    setState(() {
      _mentionSuggestions = const [];
      _isSearchingMentions = false;
    });
  }

  void _selectMentionFromComment(MentionableFriend friend) {
    final mention = _activeMention;
    if (mention == null) return;

    final updated = replaceMentionQuery(
      text: _commentController.text,
      query: mention,
      mentionDisplay: friend.fullName,
    );
    final cursor = mention.start + friend.fullName.length + 2;
    _commentController.value = TextEditingValue(
      text: updated,
      selection: TextSelection.collapsed(offset: cursor),
    );
    _clearMentionSuggestions();
  }

  Future<void> _react(BuildContext ctx, ReactionType type) async {
    final feedProvider = ctx.read<FeedProvider>();
    final personProvider = ctx.read<PersonProvider>();
    final person = personProvider.person;
    await feedProvider.addReaction(
      widget.post.uuid,
      type.name,
      person?.uuid ?? '',
      person?.fullName ?? '',
      _token(ctx),
    );
    if (mounted) {
      setState(() {
        _showReactionPicker = false;
      });
      _reactionAnim.reverse();
    }
  }

  Future<void> _submitComment(BuildContext ctx) async {
    final content = _commentController.text.trim();
    if (content.isEmpty) return;

    setState(() => _submittingComment = true);
    final feedProvider = ctx.read<FeedProvider>();
    final authProvider = ctx.read<AuthProvider>();
    final personProvider = ctx.read<PersonProvider>();
    if (authProvider.auth == null) return;
    final postId = widget.post.uuid;
    final person = personProvider.person;
    final commentData = {
      'content': content,
      'authorUuid': person?.uuid ?? '',
      'authorName': person?.fullName ?? '',
      'authorAvatar': person?.avatar,
      'authorObjectKey': person?.objectKey,
      'postUuid': postId,
      if (_replyToCommentId != null) 'parentUuid': _replyToCommentId,
    };
    final ok = await feedProvider.addComment(commentData, postId, _token(ctx));
    if (mounted) {
      setState(() {
        _submittingComment = false;
        _replyToCommentId = null;
        _replyToCommentAuthor = null;
      });
      if (ok) {
        _commentController.clear();
        _clearMentionSuggestions();
      }
    }
  }

  void _onReplyLongPress(FeedComment comment) {
    setState(() {
      _replyToCommentId = comment.uuid;
      _replyToCommentAuthor = comment.authorName ?? 'User';
    });
  }

  void _cancelReply() {
    setState(() {
      _replyToCommentId = null;
      _replyToCommentAuthor = null;
    });
  }

  Future<void> _navigateToMentionedProfile(BuildContext context, Mention mention) async {
    try {
      // Create a minimal Person object with the UUID we already have
      // PersonProfilePage will load the full profile data
      final minimalPerson = Person(
        id: 0, // Placeholder - will be loaded from server
        firstname: mention.name,
        surname: '',
        uuid: mention.mentionedUuid,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );

      // Navigate to person profile page
      if (!context.mounted) return;
      Navigator.push(
        context,
        MaterialPageRoute(
          builder: (ctx) => PersonProfilePage(person: minimalPerson),
        ),
      );
    } catch (e) {
      if (!context.mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Could not navigate to profile: ${e.toString()}'),
          backgroundColor: Colors.red,
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    // Use Selector so card only rebuilds when this specific post changes
    return Selector<FeedProvider, FeedPost?>(
      selector: (_, p) {
        try {
          return p.posts.firstWhere((e) => e.uuid == widget.post.uuid);
        } catch (_) {
          return widget.post;
        }
      },
      builder: (context, post, _) {
        final currentPost = post ?? widget.post;
        return Card(
          margin: const EdgeInsets.symmetric(horizontal: 0, vertical: 4),
          shape: const RoundedRectangleBorder(),
          elevation: 1,
          color: Colors.white,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // ── Header ──
              _PostHeader(post: currentPost),

              // ── Content ──
              if (currentPost.content.isNotEmpty)
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                  child: MentionText(
                    currentPost.content,
                    mentions: currentPost.mentions,
                    style: const TextStyle(fontSize: 15, height: 1.4),
                    onMentionTapped: (mention) => _navigateToMentionedProfile(context, mention),
                  ),
                ),

              // ── Media ──
              if (currentPost.media.isNotEmpty) _MediaSection(media: currentPost.media),

              // ── Reaction / Comment summary ──
              if (currentPost.totalReactions > 0 || currentPost.comments.isNotEmpty)
                _ReactionSummaryBar(post: currentPost),

              const Divider(height: 1, thickness: 0.5),

              // ── Action buttons ──
              _ActionBar(
                post: currentPost,
                showReactionPicker: _showReactionPicker,
                onReactionToggle: _toggleReactionPicker,
                onCommentTap: () {}, // Comments are always visible
              ),

              // ── Reaction picker ──
              FadeTransition(
                opacity: _reactionFade,
                child: _showReactionPicker
                    ? _ReactionPicker(onReact: (t) => _react(context, t))
                    : const SizedBox.shrink(),
              ),

              // ── Comments section ──
              if (currentPost.comments.isNotEmpty) ...[
                const Divider(height: 1, thickness: 0.5),
                _CommentsSection(post: currentPost, onLongPress: _onReplyLongPress),
                const Divider(height: 1, thickness: 0.5),
              ],

              // ── Add comment ──
              _AddCommentRow(
                controller: _commentController,
                submitting: _submittingComment,
                replyToAuthor: _replyToCommentAuthor,
                onCancelReply: _cancelReply,
                onSubmit: () => _submitComment(context),
                mentionSuggestions: _mentionSuggestions,
                searchingMentions: _isSearchingMentions,
                onMentionSelected: _selectMentionFromComment,
              ),
            ],
          ),
        );
      },
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Post Header
// ─────────────────────────────────────────────────────────────────────────────

class _PostHeader extends StatelessWidget {
  final FeedPost post;

  const _PostHeader({required this.post});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(12, 12, 12, 4),
      child: Row(
        children: [
          _AuthorAvatar(
            avatarUrl: post.authorAvatar,
            cacheKey: post.authorObjectKey,
            name: post.authorName ?? '',
          ),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  post.authorName ?? 'Unknown',
                  style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14),
                ),
                const SizedBox(height: 2),
                Text(
                  _timeAgo(post.createdAt),
                  style: TextStyle(fontSize: 12, color: Colors.grey[600]),
                ),
              ],
            ),
          ),
          IconButton(
            icon: Icon(Icons.more_horiz, color: Colors.grey[600]),
            onPressed: () {},
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints(),
          ),
        ],
      ),
    );
  }

  String _timeAgo(DateTime dt) {
    final diff = DateTime.now().difference(dt);
    if (diff.inSeconds < 60) return 'just now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m';
    if (diff.inHours < 24) return '${diff.inHours}h';
    if (diff.inDays < 7) return '${diff.inDays}d';
    return '${dt.day}/${dt.month}/${dt.year}';
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Author avatar with fallback initials
// ─────────────────────────────────────────────────────────────────────────────

class _AuthorAvatar extends StatelessWidget {
  final String? avatarUrl;
  final String? cacheKey;
  final String name;

  const _AuthorAvatar({required this.avatarUrl, required this.name, this.cacheKey});

  @override
  Widget build(BuildContext context) {
    if (avatarUrl != null && avatarUrl!.isNotEmpty) {
      return CircleAvatar(
        radius: 20,
        child: ClipOval(
          child: CachedNetworkImage(
            imageUrl: avatarUrl!,
            cacheKey: cacheKey,
            width: 40,
            height: 40,
            fit: BoxFit.cover,
            placeholder: (_, _) =>
                const CircularProgressIndicator(strokeWidth: 2, color: AppColors.primary),
            errorWidget: (_, _, _) => _initials(name),
          ),
        ),
      );
    }
    return CircleAvatar(
      radius: 20,
      backgroundColor: AppColors.primary.withAlpha(40),
      child: _initials(name),
    );
  }

  Widget _initials(String name) {
    final parts = name.trim().split(' ');
    final initials = parts.length >= 2
        ? '${parts.first[0]}${parts.last[0]}'.toUpperCase()
        : name.isNotEmpty
        ? name[0].toUpperCase()
        : '?';
    return Center(
      child: Text(
        initials,
        textAlign: TextAlign.center,
        style: const TextStyle(color: AppColors.primary, fontWeight: FontWeight.bold, fontSize: 14),
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Media section
// ─────────────────────────────────────────────────────────────────────────────

class _MediaSection extends StatelessWidget {
  final List<FeedMedia> media;

  const _MediaSection({required this.media});

  // Images-only list, used to build the viewer index space
  List<FeedMedia> get _images => media.where((m) => m.isImage).toList();

  void _openViewer(BuildContext context, FeedMedia tapped) {
    final images = _images;
    final idx = images.indexWhere((m) => m.url == tapped.url);
    if (idx < 0) return;
    Navigator.push(
      context,
      PageRouteBuilder(
        opaque: false,
        barrierColor: Colors.black,
        pageBuilder: (_, _, _) => FullScreenImageViewer(media: images, initialIndex: idx),
        transitionsBuilder: (_, anim, _, child) => FadeTransition(opacity: anim, child: child),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (media.length == 1) {
      return _mediaItem(context, media.first, fullWidth: true);
    }
    // Grid for multiple media (max 4 thumbnails shown)
    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 2,
        mainAxisSpacing: 2,
        crossAxisSpacing: 2,
      ),
      itemCount: media.length > 4 ? 4 : media.length,
      itemBuilder: (ctx, i) {
        if (i == 3 && media.length > 4) {
          return Stack(
            fit: StackFit.expand,
            children: [
              _mediaItem(ctx, media[i]),
              GestureDetector(
                onTap: () => _openViewer(ctx, media[i]),
                child: Container(
                  color: Colors.black54,
                  child: Center(
                    child: Text(
                      '+${media.length - 4}',
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                ),
              ),
            ],
          );
        }
        return _mediaItem(ctx, media[i]);
      },
    );
  }

  Widget _mediaItem(BuildContext context, FeedMedia item, {bool fullWidth = false}) {
    // Videos stay inline
    if (item.isVideo) {
      return _VideoItem(url: item.url, fullWidth: fullWidth);
    }

    // ── Image widget ─────────────────────────────────────────────────────────
    // Single (fullWidth): fill card width, grow vertically to natural ratio
    //   → BoxFit.fitWidth — no cropping, no letter-box bars
    // Grid thumbnail: square crop is acceptable (they are small previews)
    Widget imageWidget = fullWidth
        ? Hero(
            tag: 'feed_img_${item.url}',
            child: CachedNetworkImage(
              imageUrl: item.url,
              cacheKey: item.objectKey,
              width: double.infinity,
              fit: BoxFit.fitWidth,
              placeholder: (_, _) => Container(
                height: 220,
                color: Colors.grey[200],
                child: const Center(child: CircularProgressIndicator()),
              ),
              errorWidget: (_, _, _) => Container(height: 220, color: Colors.grey[300]),
            ),
          )
        : Hero(
            tag: 'feed_img_${item.url}',
            child: CachedNetworkImage(
              imageUrl: item.url,
              cacheKey: item.objectKey,
              fit: BoxFit.cover,
              placeholder: (_, _) => Container(color: Colors.grey[200]),
              errorWidget: (_, _, _) => Container(color: Colors.grey[300]),
            ),
          );

    // Wrap with tap → full-screen viewer
    return GestureDetector(onTap: () => _openViewer(context, item), child: imageWidget);
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Video player item
// ─────────────────────────────────────────────────────────────────────────────

class _VideoItem extends StatefulWidget {
  final String url;
  final bool fullWidth;

  const _VideoItem({required this.url, this.fullWidth = false});

  @override
  State<_VideoItem> createState() => _VideoItemState();
}

class _VideoItemState extends State<_VideoItem> {
  late VideoPlayerController _controller;
  bool _initialized = false;
  bool _playing = false;

  @override
  void initState() {
    super.initState();
    _controller = VideoPlayerController.networkUrl(Uri.parse(widget.url))
      ..initialize().then((_) {
        if (mounted) setState(() => _initialized = true);
      });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final content = _initialized
        ? AspectRatio(
            aspectRatio: widget.fullWidth ? 16 / 9 : _controller.value.aspectRatio,
            child: VideoPlayer(_controller),
          )
        : AspectRatio(
            aspectRatio: 16 / 9,
            child: Container(
              color: Colors.black,
              child: const Center(child: CircularProgressIndicator(color: Colors.white)),
            ),
          );

    return GestureDetector(
      onTap: () {
        setState(() {
          _playing = !_playing;
          _playing ? _controller.play() : _controller.pause();
        });
      },
      child: Stack(
        alignment: Alignment.center,
        children: [
          content,
          if (!_playing)
            Container(
              decoration: const BoxDecoration(color: Colors.black38, shape: BoxShape.circle),
              padding: const EdgeInsets.all(12),
              child: const Icon(Icons.play_arrow, color: Colors.white, size: 36),
            ),
        ],
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reaction summary bar (👍5 ❤️2 ... · N comments)
// ─────────────────────────────────────────────────────────────────────────────

class _ReactionSummaryBar extends StatelessWidget {
  final FeedPost post;

  const _ReactionSummaryBar({required this.post});

  @override
  Widget build(BuildContext context) {
    final reactionCounts = <String, int>{};
    for (final r in post.reactions) {
      reactionCounts[r.type] = (reactionCounts[r.type] ?? 0) + 1;
    }

    final sortedReactions = reactionCounts.entries.toList()
      ..sort((a, b) => b.value.compareTo(a.value));

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      child: Row(
        children: [
          if (post.totalReactions > 0) ...[
            ...sortedReactions.take(3).map((entry) {
              final reactionType = ReactionType.fromString(entry.key);
              return Text(reactionType.emoji, style: const TextStyle(fontSize: 14));
            }),
            const SizedBox(width: 4),
            Text('${post.totalReactions}', style: TextStyle(fontSize: 13, color: Colors.grey[600])),
          ],
          const Spacer(),
          if (post.comments.isNotEmpty)
            Text(
              '${post.comments.length} ${AppLocalizations.of(context)!.feedComments}',
              style: TextStyle(fontSize: 13, color: Colors.grey[600]),
            ),
        ],
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Action bar (Like | Comment)
// ─────────────────────────────────────────────────────────────────────────────

class _ActionBar extends StatelessWidget {
  final FeedPost post;
  final bool showReactionPicker;
  final VoidCallback onReactionToggle;
  final VoidCallback onCommentTap;

  const _ActionBar({
    required this.post,
    required this.showReactionPicker,
    required this.onReactionToggle,
    required this.onCommentTap,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Row(
      children: [
        Expanded(
          child: TextButton.icon(
            onPressed: onReactionToggle,
            icon: Icon(
              showReactionPicker ? Icons.thumb_up : Icons.thumb_up_outlined,
              size: 18,
              color: showReactionPicker ? AppColors.primary : Colors.grey[700],
            ),
            label: Text(
              l10n.feedLike,
              style: TextStyle(
                color: showReactionPicker ? AppColors.primary : Colors.grey[700],
                fontSize: 13,
                fontWeight: FontWeight.w500,
              ),
            ),
            style: TextButton.styleFrom(padding: const EdgeInsets.all(8)),
          ),
        ),
        Expanded(
          child: TextButton.icon(
            onPressed: onCommentTap,
            icon: Icon(Icons.chat_bubble_outline, size: 18, color: Colors.grey[700]),
            label: Text(
              l10n.feedComment,
              style: TextStyle(color: Colors.grey[700], fontSize: 13, fontWeight: FontWeight.w500),
            ),
            style: TextButton.styleFrom(padding: const EdgeInsets.all(8)),
          ),
        ),
      ],
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reaction picker row
// ─────────────────────────────────────────────────────────────────────────────

class _ReactionPicker extends StatelessWidget {
  final void Function(ReactionType) onReact;

  const _ReactionPicker({required this.onReact});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      color: Colors.grey[50],
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: ReactionType.values.map((type) {
          return GestureDetector(
            onTap: () => onReact(type),
            child: Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: Colors.white,
                shape: BoxShape.circle,
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withAlpha(26),
                    blurRadius: 4,
                    offset: const Offset(0, 2),
                  ),
                ],
              ),
              child: Text(type.emoji, style: const TextStyle(fontSize: 24)),
            ),
          );
        }).toList(),
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Comments list
// ─────────────────────────────────────────────────────────────────────────────

class _CommentsSection extends StatelessWidget {
  final FeedPost post;
  final void Function(FeedComment)? onLongPress;

  const _CommentsSection({required this.post, this.onLongPress});

  @override
  Widget build(BuildContext context) {
    if (post.comments.isEmpty) {
      return Padding(
        padding: const EdgeInsets.all(12),
        child: Text(
          AppLocalizations.of(context)!.feedNoComments,
          style: TextStyle(color: Colors.grey[600], fontSize: 13),
        ),
      );
    }
    return Column(
      children: post.comments
          .map((c) => _CommentItem(comment: c, onLongPress: onLongPress))
          .toList(),
    );
  }
}

class _CommentItem extends StatelessWidget {
  final FeedComment comment;
  final void Function(FeedComment)? onLongPress;

  const _CommentItem({required this.comment, this.onLongPress});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onLongPress: () => onLongPress?.call(comment),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(12, 8, 12, 0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _AuthorAvatar(
              avatarUrl: comment.authorAvatar,
              cacheKey: comment.authorObjectKey,
              name: comment.authorName ?? comment.authorUuid,
            ),
            const SizedBox(width: 8),
            Expanded(
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                decoration: BoxDecoration(
                  color: Colors.grey[100],
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      comment.authorName ?? 'User',
                      style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13),
                    ),
                    const SizedBox(height: 2),
                    Text(comment.content, style: const TextStyle(fontSize: 13)),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Add comment row at the bottom
// ─────────────────────────────────────────────────────────────────────────────

class _AddCommentRow extends StatelessWidget {
  final TextEditingController controller;
  final bool submitting;
  final String? replyToAuthor;
  final VoidCallback onCancelReply;
  final VoidCallback onSubmit;
  final List<MentionableFriend> mentionSuggestions;
  final bool searchingMentions;
  final ValueChanged<MentionableFriend> onMentionSelected;

  const _AddCommentRow({
    required this.controller,
    required this.submitting,
    this.replyToAuthor,
    required this.onCancelReply,
    required this.onSubmit,
    required this.mentionSuggestions,
    required this.searchingMentions,
    required this.onMentionSelected,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Padding(
      padding: const EdgeInsets.fromLTRB(12, 8, 12, 12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (replyToAuthor != null)
            Padding(
              padding: const EdgeInsets.only(bottom: 8.0, left: 12.0),
              child: Row(
                children: [
                  Text(
                    'Replying to $replyToAuthor',
                    style: const TextStyle(fontSize: 12, color: Colors.grey),
                  ),
                  const Spacer(),
                  GestureDetector(
                    onTap: onCancelReply,
                    child: const Icon(Icons.close, size: 16, color: Colors.grey),
                  ),
                ],
              ),
            ),
          Row(
            children: [
              Expanded(
                child: TextField(
                  controller: controller,
                  decoration: InputDecoration(
                    hintText: l10n.feedAddComment,
                    hintStyle: TextStyle(fontSize: 13, color: Colors.grey[500]),
                    filled: true,
                    fillColor: Colors.grey[100],
                    contentPadding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(24),
                      borderSide: BorderSide.none,
                    ),
                    isDense: true,
                  ),
                  style: const TextStyle(fontSize: 13),
                  maxLines: 3,
                  minLines: 1,
                  textInputAction: TextInputAction.send,
                  onSubmitted: (_) => onSubmit(),
                ),
              ),
              const SizedBox(width: 8),
              if (submitting)
                const SizedBox(
                  width: 32,
                  height: 32,
                  child: CircularProgressIndicator(strokeWidth: 2, color: AppColors.primary),
                )
              else
                InkWell(
                  onTap: onSubmit,
                  borderRadius: BorderRadius.circular(20),
                  child: Container(
                    width: 36,
                    height: 36,
                    decoration: const BoxDecoration(
                      color: AppColors.primary,
                      shape: BoxShape.circle,
                    ),
                    child: const Icon(Icons.send, color: Colors.white, size: 18),
                  ),
                ),
            ],
          ),
          if (searchingMentions || mentionSuggestions.isNotEmpty) ...[
            const SizedBox(height: 6),
            _CommentMentionSuggestions(
              searching: searchingMentions,
              suggestions: mentionSuggestions,
              onSelected: onMentionSelected,
            ),
          ],
        ],
      ),
    );
  }
}

class _CommentMentionSuggestions extends StatelessWidget {
  final bool searching;
  final List<MentionableFriend> suggestions;
  final ValueChanged<MentionableFriend> onSelected;

  const _CommentMentionSuggestions({
    required this.searching,
    required this.suggestions,
    required this.onSelected,
  });

  @override
  Widget build(BuildContext context) {
    if (searching) {
      return const Padding(
        padding: EdgeInsets.symmetric(vertical: 8),
        child: Center(
          child: SizedBox(width: 18, height: 18, child: CircularProgressIndicator(strokeWidth: 2)),
        ),
      );
    }
    if (suggestions.isEmpty) {
      return const SizedBox.shrink();
    }

    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: Colors.grey.shade300),
        color: Colors.white,
      ),
      constraints: const BoxConstraints(maxHeight: 220),
      child: ListView.separated(
        shrinkWrap: true,
        itemCount: suggestions.length,
        separatorBuilder: (_, _) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final item = suggestions[index];
          return ListTile(
            dense: true,
            leading: CircleAvatar(
              radius: 16,
              backgroundColor: AppColors.primary.withAlpha(25),
              backgroundImage: item.avatar != null && item.avatar!.isNotEmpty
                  ? NetworkImage(item.avatar!)
                  : null,
              child: item.avatar == null || item.avatar!.isEmpty
                  ? Text(
                      item.fullName.isNotEmpty ? item.fullName[0].toUpperCase() : '?',
                      style: const TextStyle(color: AppColors.primary, fontWeight: FontWeight.w700),
                    )
                  : null,
            ),
            title: Text(item.fullName, maxLines: 1, overflow: TextOverflow.ellipsis),
            onTap: () => onSelected(item),
          );
        },
      ),
    );
  }
}
