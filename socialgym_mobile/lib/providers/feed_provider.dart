import 'package:flutter/foundation.dart';

import '../models/feed_post.dart';
import '../services/feed_service.dart';
import '../services/base_service.dart';

class FeedProvider extends ChangeNotifier {
  static const int _pageSize = 20;

  List<FeedPost> _posts = [];
  bool _loading = false;
  bool _loadingMore = false;
  bool _posting = false;
  bool _hasMore = true;
  String? _error;
  int _currentPage = 0;
  int _requestGeneration = 0;

  List<FeedPost> get posts => List.unmodifiable(_posts);
  bool get loading => _loading;
  bool get loadingMore => _loadingMore;
  bool get posting => _posting;
  bool get hasMore => _hasMore;
  String? get error => _error;
  int get currentPage => _currentPage;

  // ── Fetch ────────────────────────────────────────────────────────────────────

  Future<void> fetchPostsForProfile(
    String token, {
    String? businessProfileUuid,
  }) async {
    final generation = ++_requestGeneration;
    _loading = true;
    _error = null;
    _posts = [];
    notifyListeners();

    try {
      final fetchedPosts = businessProfileUuid == null
          ? await FeedService.fetchPosts(token, page: 0)
          : await FeedService.fetchBusinessFeed(
              token,
              businessProfileUuid,
              page: 0,
            );
      if (generation != _requestGeneration) return;
      _posts = fetchedPosts;
      _currentPage = 0;
      _hasMore = fetchedPosts.length == _pageSize;
      _loading = false;
      notifyListeners();
    } on AppException catch (e) {
      if (generation != _requestGeneration) return;
      _error = e.message;
      _loading = false;
      notifyListeners();
    } catch (_) {
      if (generation != _requestGeneration) return;
      _error = 'Failed to load feed. Please try again.';
      _loading = false;
      notifyListeners();
    }
  }

  Future<void> loadMorePostsForProfile(
    String token, {
    String? businessProfileUuid,
  }) async {
    if (_loading || _loadingMore || !_hasMore || token.isEmpty) return;

    final generation = _requestGeneration;
    _loadingMore = true;
    _error = null;
    notifyListeners();

    try {
      final nextPage = _currentPage + 1;
      final fetchedPosts = businessProfileUuid == null
          ? await FeedService.fetchPosts(token, page: nextPage)
          : await FeedService.fetchBusinessFeed(
              token,
              businessProfileUuid,
              page: nextPage,
            );
      if (generation != _requestGeneration) return;
      final existingIds = _posts.map((post) => post.uuid).toSet();
      final uniquePosts = fetchedPosts
          .where((post) => !existingIds.contains(post.uuid))
          .toList();

      _posts = [..._posts, ...uniquePosts];
      _currentPage = nextPage;
      _hasMore = fetchedPosts.length == _pageSize;
      _loadingMore = false;
      notifyListeners();
    } on AppException catch (e) {
      if (generation != _requestGeneration) return;
      _error = e.message;
      _loadingMore = false;
      notifyListeners();
    } catch (_) {
      if (generation != _requestGeneration) return;
      _error = 'Failed to load more posts. Please try again.';
      _loadingMore = false;
      notifyListeners();
    }
  }

  Future<void> fetchPosts(String token) => fetchPostsForProfile(token);

  Future<void> loadMorePosts(String token) => loadMorePostsForProfile(token);

  // ── Create Post ──────────────────────────────────────────────────────────────

  Future<bool> createPost(Map<String, dynamic> data, String token) async {
    _posting = true;
    _error = null;
    notifyListeners();

    try {
      final newPost = await FeedService.createPost(data, token);
      _posts = [newPost, ..._posts];
      _posting = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _posting = false;
      notifyListeners();
      return false;
    } catch (_) {
      _error = 'Failed to create post. Please try again.';
      _posting = false;
      notifyListeners();
      return false;
    }
  }

  // ── Comment ──────────────────────────────────────────────────────────────────

  Future<bool> addComment(
    Map<String, dynamic> data,
    String postId,
    String token,
  ) async {
    try {
      final updatedPost = await FeedService.addComment(data, token);
      _posts = _posts.map((post) {
        if (post.uuid == postId) {
          return updatedPost;
        }
        return post;
      }).toList();
      notifyListeners();
      return true;
    } catch (_) {
      return false;
    }
  }

  // ── Reaction ─────────────────────────────────────────────────────────────────

  Future<bool> addReaction(
    String postId,
    String reactionType,
    String authorId,
    String authorName,
    String token,
  ) async {
    try {
      await FeedService.addReaction(
        postId,
        reactionType,
        authorId,
        authorName,
        token,
      );
      // Optimistically add the reaction locally
      _posts = _posts.map((post) {
        if (post.uuid == postId) {
          final newReaction = FeedReaction(type: reactionType);
          return post.copyWith(reactions: [...post.reactions, newReaction]);
        }
        return post;
      }).toList();
      notifyListeners();
      return true;
    } catch (_) {
      return false;
    }
  }

  // ── Helpers ──────────────────────────────────────────────────────────────────

  void clearError() {
    _error = null;
    notifyListeners();
  }
}
