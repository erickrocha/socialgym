import '../../models/feed_post.dart';
import '../../src/generated/grpc/timeline/feed.pbgrpc.dart' as $feed;
import '../../src/generated/grpc/timeline/post.pbgrpc.dart' as $post;
import 'grpc_timeline.dart';

/// Posts and feeds over gRPC, replacing the REST `FeedService`. The acting person is the one in
/// the access token, so no call carries a token or an author.
class GrpcFeedService {
  GrpcFeedService._();

  static $feed.FeedServiceClient get _feed =>
      $feed.FeedServiceClient(GrpcTimeline.channel, interceptors: GrpcTimeline.interceptors);
  static $post.PostServiceClient get _posts =>
      $post.PostServiceClient(GrpcTimeline.channel, interceptors: GrpcTimeline.interceptors);

  static Future<List<FeedPost>> fetchPosts({int page = 0}) => GrpcTimeline.run(() async {
        final response = await _feed.getFeed($feed.GetFeedRequest(page: page), options: GrpcTimeline.options);
        return response.posts.map((p) => FeedPost.fromJson(GrpcTimeline.json(p))).toList();
      }, 'Failed to load feed');

  static Future<List<FeedPost>> fetchBusinessFeed(String businessProfileUuid, {int page = 0}) =>
      GrpcTimeline.run(() async {
        final response = await _feed.getFeedByAuthor(
          $feed.GetFeedByAuthorRequest(authorUuid: businessProfileUuid, page: page),
          options: GrpcTimeline.options,
        );
        return response.posts.map((p) => FeedPost.fromJson(GrpcTimeline.json(p))).toList();
      }, 'Failed to load business feed');

  static Future<FeedPost> createPost(Map<String, dynamic> data) => GrpcTimeline.run(() async {
        final request = GrpcTimeline.fill($post.CreatePostRequest(), data);
        final post = await _posts.createPost(request, options: GrpcTimeline.options);
        return FeedPost.fromJson(GrpcTimeline.json(post));
      }, 'Failed to create post');

  static Future<FeedPost> addComment(Map<String, dynamic> data) => GrpcTimeline.run(() async {
        final request = GrpcTimeline.fill($post.AddCommentRequest(), data);
        final post = await _posts.addComment(request, options: GrpcTimeline.options);
        return FeedPost.fromJson(GrpcTimeline.json(post));
      }, 'Failed to add comment');

  static Future<void> addReaction(String postId, String reactionType) => GrpcTimeline.run(() async {
        await _posts.addReaction(
          $post.AddReactionRequest(postUuid: postId, reactionType: reactionType),
          options: GrpcTimeline.options,
        );
      }, 'Failed to add reaction');
}
