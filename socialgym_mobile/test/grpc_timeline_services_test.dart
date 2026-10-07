import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_chat_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_chat_stream.dart';
import 'package:socialgym_mobile/services/grpc/grpc_content_report_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_evolution_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_feed_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_notification_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_timeline.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_session_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/timeline/chat.pbgrpc.dart' as $chat;
import 'package:socialgym_mobile/src/generated/grpc/timeline/content_report.pbgrpc.dart' as $report;
import 'package:socialgym_mobile/src/generated/grpc/timeline/evolution.pbgrpc.dart' as $evolution;
import 'package:socialgym_mobile/src/generated/grpc/timeline/feed.pbgrpc.dart' as $feed;
import 'package:socialgym_mobile/src/generated/grpc/timeline/notification.pbgrpc.dart' as $notification;
import 'package:socialgym_mobile/src/generated/grpc/timeline/post.pbgrpc.dart' as $post;
import 'package:socialgym_mobile/src/generated/grpc/timeline/push_device.pbgrpc.dart' as $device;
import 'package:socialgym_mobile/src/generated/grpc/timeline/workout_session.pbgrpc.dart' as $session;

/// What the fake server saw, so each test can assert on what the app actually sent.
class Seen {
  final requests = <String, Object>{};
  final metadata = <String, Map<String, String>>{};
}

class FakePosts extends $post.PostServiceBase {
  FakePosts(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  $post.Post _post(String content) => $post.Post(
    uuid: 'p1',
    authorUuid: 'alice',
    content: content,
    reactions: [$post.Reaction(reactionType: 'Like', authorId: 'bob')],
    comments: [$post.Comment(uuid: 'c1', postUuid: 'p1', content: 'nice')],
  );

  @override
  Future<$post.Post> createPost(grpc.ServiceCall call, $post.CreatePostRequest request) async {
    seen.requests['createPost'] = request;
    return _post(request.content);
  }

  @override
  Future<$post.Post> addComment(grpc.ServiceCall call, $post.AddCommentRequest request) async {
    seen.requests['addComment'] = request;
    return _post('commented');
  }

  @override
  Future<$post.Post> addReaction(grpc.ServiceCall call, $post.AddReactionRequest request) async {
    seen.requests['addReaction'] = request;
    if (request.reactionType == 'dislike') throw grpc.GrpcError.invalidArgument('unknown reaction type');
    return _post('reacted');
  }
}

class FakeFeed extends $feed.FeedServiceBase {
  FakeFeed(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  @override
  Future<$feed.FeedResponse> getFeed(grpc.ServiceCall call, $feed.GetFeedRequest request) async {
    seen.requests['getFeed'] = request;
    return $feed.FeedResponse(posts: [$post.Post(uuid: 'p1', authorUuid: 'alice', content: 'hello')]);
  }

  @override
  Future<$feed.FeedResponse> getFeedByAuthor(grpc.ServiceCall call, $feed.GetFeedByAuthorRequest request) async {
    seen.requests['getFeedByAuthor'] = request;
    if (request.authorUuid == 'a-person') throw grpc.GrpcError.notFound('Business profile not found');
    return $feed.FeedResponse(posts: [$post.Post(uuid: 'b1', authorUuid: request.authorUuid, content: 'promo')]);
  }
}

class FakeNotifications extends $notification.NotificationServiceBase {
  FakeNotifications(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  @override
  Future<$notification.ListNotificationsResponse> listNotifications(
    grpc.ServiceCall call,
    $notification.ListNotificationsRequest request,
  ) async {
    seen.requests['listNotifications'] = request;
    return $notification.ListNotificationsResponse(
      notifications: [
        $notification.Notification(
          uuid: 'n1',
          notificationType: 'Comment',
          recipientPersonUuid: 'alice',
          actorName: 'Bob',
          snippet: 'Someone commented on your post.',
          createdAt: '2026-10-07T10:00:00',
          updatedAt: '2026-10-07T10:00:00',
        ),
      ],
    );
  }

  @override
  Future<$notification.MarkNotificationReadResponse> markNotificationRead(
    grpc.ServiceCall call,
    $notification.MarkNotificationReadRequest request,
  ) async {
    seen.requests['markNotificationRead'] = request;
    return $notification.MarkNotificationReadResponse(read: true);
  }
}

class FakeDevices extends $device.PushDeviceServiceBase {
  FakeDevices(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  @override
  Future<$device.RegisterPushDeviceResponse> registerPushDevice(
    grpc.ServiceCall call,
    $device.RegisterPushDeviceRequest request,
  ) async {
    seen.requests['registerPushDevice'] = request;
    seen.metadata['registerPushDevice'] = Map.of(call.clientMetadata ?? {});
    return $device.RegisterPushDeviceResponse();
  }

  @override
  Future<$device.RemovePushDeviceResponse> removePushDevice(
    grpc.ServiceCall call,
    $device.RemovePushDeviceRequest request,
  ) async {
    seen.requests['removePushDevice'] = request;
    seen.metadata['removePushDevice'] = Map.of(call.clientMetadata ?? {});
    return $device.RemovePushDeviceResponse();
  }
}

class FakeEvolution extends $evolution.EvolutionCheckInServiceBase {
  FakeEvolution(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  @override
  Future<$evolution.EvolutionCheckIn> addEvolutionCheckIn(
    grpc.ServiceCall call,
    $evolution.AddEvolutionCheckInRequest request,
  ) async {
    seen.requests['addEvolutionCheckIn'] = request;
    if (request.visibility == 'needs-consent') {
      throw grpc.GrpcError.permissionDenied('health_data consent is required');
    }
    return $evolution.EvolutionCheckIn(
      uuid: 'e1',
      personUuid: 'alice',
      createdAt: request.createdAt,
      visibility: request.visibility,
      composition: request.composition,
    );
  }

  @override
  Future<$evolution.ListEvolutionCheckInsResponse> listEvolutionCheckIns(
    grpc.ServiceCall call,
    $evolution.ListEvolutionCheckInsRequest request,
  ) async {
    seen.requests['listEvolutionCheckIns'] = request;
    return $evolution.ListEvolutionCheckInsResponse(
      checkIns: [
        $evolution.EvolutionCheckIn(
          uuid: 'e1',
          personUuid: 'alice',
          createdAt: '2026-10-07T10:00:00',
          visibility: 'Private',
          composition: $evolution.BodyComposition(weight: 81.5),
        ),
      ],
    );
  }
}

class FakeSessions extends $session.WorkoutSessionServiceBase {
  FakeSessions(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  @override
  Future<$session.WorkoutSession> createWorkoutSession(
    grpc.ServiceCall call,
    $session.CreateWorkoutSessionRequest request,
  ) async {
    seen.requests['createWorkoutSession'] = request;
    return request.session..personUuid = 'alice'..uuid = 's1';
  }

  @override
  Future<$session.ListWorkoutSessionsResponse> listWorkoutSessions(
    grpc.ServiceCall call,
    $session.ListWorkoutSessionsRequest request,
  ) async {
    seen.requests['listWorkoutSessions'] = request;
    return $session.ListWorkoutSessionsResponse(
      sessions: [
        $session.WorkoutSession(
          uuid: 's1',
          personUuid: 'alice',
          workoutName: 'Legs',
          duration: 30,
          startedAt: '2026-10-07T10:00:00',
          completedAt: '2026-10-07T10:30:00',
        ),
      ],
    );
  }
}

class FakeReports extends $report.ContentReportServiceBase {
  FakeReports(this.seen);
  final Seen seen;

  // The tests exercise only some operations; the rest are unreachable here.
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);

  @override
  Future<$report.ContentReport> createReport(grpc.ServiceCall call, $report.CreateReportRequest request) async {
    seen.requests['createReport'] = request;
    return $report.ContentReport(uuid: 'r1', status: 'open');
  }
}

/// The chat service: unary answers plus a stream that echoes every `send` frame back as
/// `message.new` and answers `ping` with `pong`. [drop] ends the live streams on demand.
class FakeChat extends $chat.ChatServiceBase {
  FakeChat(this.seen);
  final Seen seen;
  final opened = <StreamController<$chat.ServerFrame>>[];
  int streamsOpened = 0;

  void drop() {
    for (final stream in opened) {
      stream.close();
    }
    opened.clear();
  }

  @override
  Future<$chat.ListMessagesResponse> listMessages(grpc.ServiceCall call, $chat.ListMessagesRequest request) async {
    seen.requests['listMessages'] = request;
    return $chat.ListMessagesResponse(
      messages: [
        $chat.Message(
          uuid: 'm1',
          conversationUuid: request.conversationUuid,
          senderPersonUuid: 'bob',
          body: 'oi',
          clientMessageId: 'c1',
          sentAt: '2026-10-07T10:00:00',
        ),
      ],
    );
  }

  @override
  Future<$chat.GetPresenceResponse> getPresence(grpc.ServiceCall call, $chat.GetPresenceRequest request) async {
    seen.requests['getPresence'] = request;
    return $chat.GetPresenceResponse(online: request.personUuids.take(1));
  }

  @override
  Future<$chat.Message> sendMessage(grpc.ServiceCall call, $chat.SendMessageRequest request) async {
    seen.requests['sendMessage'] = request;
    return $chat.Message(uuid: 'm2', conversationUuid: request.conversationUuid, body: request.body, clientMessageId: request.clientMessageId);
  }

  @override
  Stream<$chat.ServerFrame> openStream(grpc.ServiceCall call, Stream<$chat.ClientFrame> request) {
    // Like the real server, answer the open at once so the client knows it is connected.
    call.sendHeaders();
    streamsOpened++;
    final out = StreamController<$chat.ServerFrame>();
    opened.add(out);
    request.listen((frame) {
      if (frame.hasSend()) {
        out.add(
          $chat.ServerFrame(
            messageNew: $chat.MessageNewEvent(
              conversationUuid: frame.send.conversationUuid,
              conversationType: 'Direct',
              message: $chat.Message(uuid: 'm9', conversationUuid: frame.send.conversationUuid, body: frame.send.body),
            ),
          ),
        );
      } else if (frame.hasPing()) {
        out.add($chat.ServerFrame(pong: $chat.PongEvent()));
      } else if (frame.hasTyping()) {
        out.add($chat.ServerFrame(typing: $chat.TypingEvent(conversationUuid: frame.typing.conversationUuid, personUuid: 'bob')));
      }
    }, onDone: () => out.close(), onError: (Object _) => out.close());
    return out.stream;
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

void main() {
  late grpc.Server server;
  late Seen seen;
  late FakeChat chat;

  setUp(() async {
    seen = Seen();
    chat = FakeChat(seen);
    server = grpc.Server.create(
      services: [
        FakePosts(seen),
        FakeFeed(seen),
        FakeNotifications(seen),
        FakeDevices(seen),
        FakeEvolution(seen),
        FakeSessions(seen),
        FakeReports(seen),
        chat,
      ],
    );
    await server.serve(address: 'localhost', port: 0);
    GrpcTimeline.channelOverride = grpc.ClientChannel(
      'localhost',
      port: server.port!,
      options: const grpc.ChannelOptions(credentials: grpc.ChannelCredentials.insecure()),
    );
  });

  tearDown(() async {
    await GrpcTimeline.channelOverride?.shutdown();
    GrpcTimeline.channelOverride = null;
    await server.shutdown();
  });

  group('feed and posts', () {
    test('the main feed parses posts, reactions and comments like the REST body did', () async {
      final posts = await GrpcFeedService.fetchPosts(page: 2);
      expect((seen.requests['getFeed'] as $feed.GetFeedRequest).page, 2);
      expect(posts.single.uuid, 'p1');
      expect(posts.single.content, 'hello');
    });

    test('a business feed asks by author uuid; a person uuid is a not found AppException', () async {
      final posts = await GrpcFeedService.fetchBusinessFeed('biz-1', page: 0);
      expect(posts.single.authorUuid, 'biz-1');
      expect(
        () => GrpcFeedService.fetchBusinessFeed('a-person'),
        throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
      );
    });

    test('create post sends the REST payload fields and ignores the author fields', () async {
      final post = await GrpcFeedService.createPost({
        'authorId': 7,
        'authorName': 'Alice',
        'content': 'hi',
        'media': [
          {'url': 'u', 'mediaType': 'Image', 'objectKey': 'k'},
        ],
        'thirdPartyConsentConfirmed': true,
        'mentions': [
          {'name': 'Bob', 'mentionedUuid': 'bob'},
        ],
      });
      final sent = seen.requests['createPost'] as $post.CreatePostRequest;
      expect(sent.content, 'hi');
      expect(sent.media.single.objectKey, 'k');
      expect(sent.thirdPartyConsentConfirmed, isTrue);
      expect(sent.mentions.single.mentionedUuid, 'bob');
      expect(post.content, 'hi');
      expect(post.comments.single.content, 'nice');
      expect(post.reactions.single.type, 'Like');
    });

    test('comment and reaction carry the post id and the type; a bad type is an error', () async {
      final updated = await GrpcFeedService.addComment({'postUuid': 'p1', 'content': 'x', 'parentUuid': 'c0'});
      final sent = seen.requests['addComment'] as $post.AddCommentRequest;
      expect((sent.postUuid, sent.content, sent.parentUuid), ('p1', 'x', 'c0'));
      expect(updated.uuid, 'p1');
      await GrpcFeedService.addReaction('p1', 'LOVE');
      expect((seen.requests['addReaction'] as $post.AddReactionRequest).reactionType, 'LOVE');
      expect(
        () => GrpcFeedService.addReaction('p1', 'dislike'),
        throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 400)),
      );
    });
  });

  group('notifications and push devices', () {
    test('listing maps the message to the Notification model and sends the filter', () async {
      final items = await GrpcNotificationService.fetchNotifications(unreadOnly: true, limit: 20);
      final sent = seen.requests['listNotifications'] as $notification.ListNotificationsRequest;
      expect((sent.unreadOnly, sent.limit), (true, 20));
      expect(items.single.uuid, 'n1');
      expect(items.single.notificationType, 'Comment');
      expect(items.single.isUnread, isTrue);
    });

    test('mark read sends only the key (the recipient is the token)', () async {
      await GrpcNotificationService.markNotificationAsRead('n1');
      expect((seen.requests['markNotificationRead'] as $notification.MarkNotificationReadRequest).idempotencyKey, 'n1');
    });

    test('device registration and removal carry the given token even after sign-out cleared storage', () async {
      await GrpcNotificationService.registerPushDevice(
        token: 'access-1',
        deviceUuid: 'd1',
        registrationToken: 'fcm',
        platform: 'android',
      );
      await GrpcNotificationService.removePushDevice(token: 'access-1', deviceUuid: 'd1');
      expect(seen.metadata['registerPushDevice']!['authorization'], 'Bearer access-1');
      expect(seen.metadata['removePushDevice']!['authorization'], 'Bearer access-1');
      expect((seen.requests['registerPushDevice'] as $device.RegisterPushDeviceRequest).registrationToken, 'fcm');
    });
  });

  group('check-ins, sessions and reports', () {
    test('create check-in fills the message from the REST payload and parses the answer', () async {
      final created = await GrpcEvolutionService.createEvolutionCheckIn(
        payload: {
          'createdAt': '2026-10-07T10:00:00.000',
          'visibility': 'Private',
          'composition': {'weight': 81.5, 'bodyFatPct': 14.0},
        },
      );
      final sent = seen.requests['addEvolutionCheckIn'] as $evolution.AddEvolutionCheckInRequest;
      expect(sent.composition.weight, 81.5);
      expect(created.composition?.weight, 81.5);
    });

    test('a missing health consent becomes the CONSENT_REQUIRED error the screens react to', () async {
      await expectLater(
        GrpcEvolutionService.createEvolutionCheckIn(payload: {'createdAt': '2026-10-07T10:00:00', 'visibility': 'needs-consent'}),
        throwsA(isA<AppException>().having((e) => e.isConsentRequired, 'consent', isTrue)),
      );
    });

    test('listing check-ins sends the ISO range', () async {
      final items = await GrpcEvolutionService.fetchEvolutionCheckIns(
        startDate: DateTime(2026, 10, 1),
        endDate: DateTime(2026, 10, 8),
      );
      final sent = seen.requests['listEvolutionCheckIns'] as $evolution.ListEvolutionCheckInsRequest;
      expect(sent.startDate, startsWith('2026-10-01T00:00:00'));
      expect(items.single.composition?.weight, 81.5);
    });

    test('a workout session is sent with its executed sets and comes back as a map', () async {
      final saved = await GrpcWorkoutSessionService.saveWorkoutSession({
        'workoutName': 'Legs',
        'duration': 30,
        'startedAt': '2026-10-07T10:00:00',
        'completedAt': '2026-10-07T10:30:00',
        'executedSets': [
          {'exerciseName': 'Squat', 'ownerId': 1, 'ownerName': 'Alice', 'setNumber': 1, 'repsOrDuration': 10, 'weight': 80.0},
        ],
        'totalVolume': 800.0,
        'totalSets': 1,
      });
      final sent = seen.requests['createWorkoutSession'] as $session.CreateWorkoutSessionRequest;
      expect(sent.session.executedSets.single.exerciseName, 'Squat');
      expect(saved['uuid'], 's1');
    });

    test('listing sessions sends whole-day bounds and parses the model', () async {
      final sessions = await GrpcWorkoutSessionService.fetchWorkoutSessions(
        startDate: DateTime(2026, 10, 1),
        endDate: DateTime(2026, 10, 7),
      );
      final sent = seen.requests['listWorkoutSessions'] as $session.ListWorkoutSessionsRequest;
      expect((sent.startDate, sent.endDate), ('2026-10-01T00:00:00', '2026-10-07T23:59:59'));
      expect(sessions.single.workoutName, 'Legs');
    });

    test('a report carries the target and an optional detail', () async {
      await GrpcContentReportService.create(targetType: 'post', targetId: 'p1', postId: 'p1', reason: 'spam', details: '');
      final sent = seen.requests['createReport'] as $report.CreateReportRequest;
      expect((sent.targetType, sent.reason, sent.hasDetails()), ('post', 'spam', false));
    });
  });

  group('chat', () {
    test('unary calls map to the models; since goes as epoch milliseconds', () async {
      final messages = await GrpcChatService.listMessages('conv-1', since: 1700000000000);
      final sent = seen.requests['listMessages'] as $chat.ListMessagesRequest;
      expect(sent.sinceEpochMs, Int64(1700000000000));
      expect(messages.single.body, 'oi');
      expect(await GrpcChatService.presence(['a', 'b']), {'a'});
      expect(await GrpcChatService.presence([]), isEmpty);
      final sentMessage = await GrpcChatService.sendMessage(
        'conv-1',
        body: 'oi',
        media: [
          {'mediaType': 'Image', 'objectKey': 'k'},
        ],
        clientMessageId: 'c9',
      );
      expect(sentMessage.clientMessageId, 'c9');
      expect((seen.requests['sendMessage'] as $chat.SendMessageRequest).media.single.objectKey, 'k');
    });

    test('the stream connects, delivers events in the WebSocket JSON shape and sends frames', () async {
      final stream = GrpcChatStream();
      final events = <Map<String, dynamic>>[];
      final sub = stream.events.listen(events.add);
      stream.connect('signed-in');
      await stream.statusStream.firstWhere((s) => s == ChatSocketStatus.connected).timeout(const Duration(seconds: 5));

      expect(stream.sendMessage(conversationUuid: 'c1', body: 'oi', media: const [], clientMessageId: 'x1'), isTrue);
      expect(stream.sendTyping('c1'), isTrue);
      await Future.doWhile(() async {
        await Future<void>.delayed(const Duration(milliseconds: 50));
        return events.length < 2;
      }).timeout(const Duration(seconds: 5));

      final created = events.firstWhere((e) => e['type'] == 'message.new');
      expect(created['conversationUuid'], 'c1');
      expect(created['message']['body'], 'oi');
      expect(events.firstWhere((e) => e['type'] == 'typing')['personUuid'], 'bob');

      await sub.cancel();
      stream.dispose();
    });

    test('a dropped stream reconnects by itself and sending works again', () async {
      final stream = GrpcChatStream();
      stream.connect('signed-in');
      await stream.statusStream.firstWhere((s) => s == ChatSocketStatus.connected).timeout(const Duration(seconds: 5));
      expect(chat.streamsOpened, 1);

      chat.drop();
      await stream.statusStream.firstWhere((s) => s == ChatSocketStatus.disconnected).timeout(const Duration(seconds: 5));
      expect(stream.sendTyping('c1'), isFalse, reason: 'nothing is sent while disconnected');
      await stream.statusStream.firstWhere((s) => s == ChatSocketStatus.connected).timeout(const Duration(seconds: 10));
      expect(chat.streamsOpened, 2);
      expect(stream.sendTyping('c1'), isTrue);

      stream.dispose();
    });

    test('a closed stream does not reconnect', () async {
      final stream = GrpcChatStream();
      stream.connect('signed-in');
      await stream.statusStream.firstWhere((s) => s == ChatSocketStatus.connected).timeout(const Duration(seconds: 5));
      stream.disconnect();
      await Future<void>.delayed(const Duration(milliseconds: 1500));
      expect(chat.streamsOpened, 1);
      expect(stream.status, ChatSocketStatus.disconnected);
      stream.dispose();
    });
  });

  test('a signed-out person cannot open the stream: connect without a credential does nothing', () async {
    final stream = GrpcChatStream();
    stream.connect('');
    await Future<void>.delayed(const Duration(milliseconds: 200));
    expect(chat.streamsOpened, 0);
    stream.dispose();
  });
}
