// TC-012 steps 1 and 4: the app's timeline flows on a device against the infra/test stack, with
// every timeline call going over gRPC. Run by scripts/e2e-android.sh, which also starts the stack,
// reads the gateway log (step 2) and restarts the timeline when this test asks (step 4).
import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/providers/feed_provider.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_business_profile_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_chat_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_chat_stream.dart';
import 'package:socialgym_mobile/services/grpc/grpc_content_report_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_evolution_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_feed_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_notification_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_session_service.dart';
import 'package:socialgym_mobile/services/push_registration_service.dart';

import 'support.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  late Account alice;
  late Account bob;
  late Account carol;

  setUpAll(() async {
    await startApp();
    alice = await register('alice');
    bob = await register('bob');
    carol = await register('carol');
    await befriend(alice, bob);
    // Whoever registered last is signed in; the flows below start as Alice.
    await asUser(alice, () async {}, back: alice);
  });

  test('feed: create a post, a friend comments and reacts, the feed shows it', () async {
    final text = 'e2e post ${DateTime.now().microsecondsSinceEpoch}';
    final post = await GrpcFeedService.createPost({'content': text});
    expect(post.content, text);
    expect(post.authorUuid, alice.personUuid, reason: 'the author is the person in the token');

    final commented = await asUser(bob, () => GrpcFeedService.addComment({'postUuid': post.uuid, 'content': 'nice'}), back: alice);
    expect(commented.comments.map((c) => c.content), contains('nice'));
    await asUser(bob, () => GrpcFeedService.addReaction(post.uuid, 'Love'), back: alice);

    final feed = await GrpcFeedService.fetchPosts();
    final mine = feed.firstWhere((p) => p.uuid == post.uuid);
    expect(mine.comments, isNotEmpty);
    expect(mine.reactions.map((r) => r.type.toLowerCase()), contains('love'));

    final friendsFeed = await asUser(bob, () => GrpcFeedService.fetchPosts(), back: alice);
    expect(friendsFeed.any((p) => p.uuid == post.uuid), isTrue, reason: 'a friend sees the post');
    final strangersFeed = await asUser(carol, () => GrpcFeedService.fetchPosts(), back: alice);
    expect(strangersFeed.any((p) => p.uuid == post.uuid), isFalse);
    await expectLater(
      asUser(carol, () => GrpcFeedService.addComment({'postUuid': post.uuid, 'content': 'hi'}), back: alice),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
    );
  });

  test('business profile feed: a profile is listable, a person is not', () async {
    final profile = await GrpcBusinessProfileService.addBusinessProfile(
      BusinessProfile(
        ownerId: alice.personId,
        ownerUuid: alice.personUuid,
        taxId: '12345678000199',
        businessName: 'E2E Gym ${DateTime.now().microsecondsSinceEpoch}',
        businessType: 'Company',
      ),
    );
    expect(await GrpcFeedService.fetchBusinessFeed(profile.uuid!), isEmpty);
    await expectLater(
      GrpcFeedService.fetchBusinessFeed(bob.personUuid),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
    );
  });

  test('notifications: the comment reaches the author, who marks it read', () async {
    final post = await GrpcFeedService.createPost({'content': 'notify me'});
    await asUser(bob, () => GrpcFeedService.addComment({'postUuid': post.uuid, 'content': 'ping'}), back: alice);
    final unread = await GrpcNotificationService.fetchNotifications(unreadOnly: true);
    final comment = unread.firstWhere((n) => n.postUuid == post.uuid && n.notificationType == 'Comment');
    expect(comment.isUnread, isTrue);
    await GrpcNotificationService.markNotificationAsRead(comment.uuid);
    final after = await GrpcNotificationService.fetchNotifications(unreadOnly: true);
    expect(after.any((n) => n.uuid == comment.uuid), isFalse);
  });

  test('check-in and workout session: create, then list', () async {
    final now = DateTime.now();
    final created = await GrpcEvolutionService.createEvolutionCheckIn(
      payload: {
        'createdAt': now.toUtc().toIso8601String().replaceFirst('Z', ''),
        'visibility': 'Private',
        'composition': {'weight': 82.5, 'bodyFatPct': 14.0},
      },
    );
    expect(created.composition?.weight, 82.5);
    final listed = await GrpcEvolutionService.fetchEvolutionCheckIns(
      startDate: now.subtract(const Duration(days: 1)),
      endDate: now.add(const Duration(days: 1)),
    );
    expect(listed.any((c) => c.uuid == created.uuid), isTrue);

    final stamp = now.toUtc().toIso8601String().replaceFirst('Z', '');
    await GrpcWorkoutSessionService.saveWorkoutSession({
      'workoutName': 'E2E legs',
      'duration': 30,
      'startedAt': stamp,
      'completedAt': stamp,
      'executedSets': [
        {'exerciseName': 'Squat', 'ownerId': alice.personId, 'ownerName': 'Alice', 'setNumber': 1, 'repsOrDuration': 10, 'weight': 80.0, 'startedAt': stamp, 'completedAt': stamp},
      ],
      'totalVolume': 800.0,
      'totalSets': 1,
    });
    final sessions = await GrpcWorkoutSessionService.fetchWorkoutSessions(
      startDate: now.subtract(const Duration(days: 1)),
      endDate: now.add(const Duration(days: 1)),
    );
    expect(sessions.any((s) => s.workoutName == 'E2E legs'), isTrue);
  });

  test('content report: a friend reports the post; a stranger cannot', () async {
    final post = await GrpcFeedService.createPost({'content': 'reportable'});
    await asUser(
      bob,
      () => GrpcContentReportService.create(targetType: 'post', targetId: post.uuid, postId: post.uuid, reason: 'spam'),
      back: alice,
    );
    await expectLater(
      asUser(carol, () => GrpcContentReportService.create(targetType: 'post', targetId: post.uuid, postId: post.uuid, reason: 'spam'), back: alice),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
    );
  });

  test('chat: messages arrive in real time and the stream survives a timeline restart', () async {
    final conversation = await GrpcChatService.createDirect(bob.personUuid);
    final stream = GrpcChatStream();
    final events = <Map<String, dynamic>>[];
    final subscription = stream.events.listen(events.add);
    stream.connect('signed-in');
    expect(await eventually(() => stream.status == ChatSocketStatus.connected), isTrue, reason: 'the stream opens');

    Future<void> bobSays(String text) => asUser(
      bob,
      () => GrpcChatService.sendMessage(conversation.uuid, body: text, clientMessageId: 'c-${DateTime.now().microsecondsSinceEpoch}'),
      back: alice,
    );
    bool arrived(String text) => events.any((e) => e['type'] == 'message.new' && e['message']?['body'] == text);

    await bobSays('hello alice');
    expect(await eventually(() => arrived('hello alice')), isTrue, reason: 'delivered in real time');

    // Alice sends through the stream and Bob reads it back.
    expect(stream.sendMessage(conversationUuid: conversation.uuid, body: 'hello bob', media: const [], clientMessageId: 'from-stream'), isTrue);
    expect(await eventually(() async {
      final seen = await asUser(bob, () => GrpcChatService.listMessages(conversation.uuid), back: alice);
      return seen.any((m) => m.body == 'hello bob');
    }), isTrue);

    // The host script restarts the timeline when it sees this line (TC-012 step 4).
    marker('restart-timeline');
    expect(
      await eventually(() => stream.status == ChatSocketStatus.disconnected, timeout: const Duration(seconds: 60)),
      isTrue,
      reason: 'the restart drops the stream',
    );
    expect(
      await eventually(() => stream.status == ChatSocketStatus.connected, timeout: const Duration(seconds: 60)),
      isTrue,
      reason: 'the app reopens the stream within 60 seconds',
    );
    await bobSays('after the restart');
    expect(await eventually(() => arrived('after the restart')), isTrue, reason: 'real time again, with no manual refresh');

    await subscription.cancel();
    stream.dispose();
  });

  test('push device: the app registers a real FCM token over gRPC and the device can be removed', () async {
    // Needs the FIREBASE_* defines and the notification permission, which e2e-android.sh provides.
    await PushRegistrationService.initialize(GlobalKey<NavigatorState>());
    await PushRegistrationService.bindAuthenticatedUser(alice.auth);
    final preferences = await SharedPreferences.getInstance();
    final deviceUuid = preferences.getString('push_device_uuid');
    expect(deviceUuid, isNotNull, reason: 'the app did not reach the registration: Firebase options or permission missing');
    // The timeline answers NOT_FOUND for a device it does not hold for the person, so a successful
    // removal proves the app's registration reached it.
    await expectLater(
      GrpcNotificationService.removePushDevice(token: alice.auth.accessToken, deviceUuid: '00000000-0000-4000-8000-000000000000'),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'statusCode', 404)),
    );
    await GrpcNotificationService.removePushDevice(token: alice.auth.accessToken, deviceUuid: deviceUuid!);
  }, timeout: const Timeout(Duration(minutes: 2)));

  test('the feed provider shows the posts of a profile it loaded', () async {
    final provider = FeedProvider();
    await provider.fetchPostsForProfile(alice.auth.accessToken);
    expect(provider.error, isNull);
    expect(provider.posts, isNotEmpty);
  });
}
