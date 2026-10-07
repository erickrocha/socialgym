// TC-012 step 3: with the timeline stopped the feed flow fails the way the app already handles a
// failed load (an error message, no crash). Run by scripts/e2e-android.sh after stopping the
// timeline.
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:socialgym_mobile/providers/feed_provider.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_feed_service.dart';

import 'support.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  test('a stopped timeline is an error message, and the app keeps running', () async {
    await startApp();
    final person = await register('dave');
    // A stopped service is an outage the app turns into an AppException: UNAVAILABLE (503), the call's
    // deadline running out (504), or the gateway's own 502 (seen by the gRPC client as 500). Which one
    // arrives varies from call to call, as the gateway's 502/503/504 did on REST, and the REST message
    // was the HTTP client's technical text, so what is checked is the type of the error and that the
    // screen shows its message, not the generic fallback used for unexpected exceptions.
    const outage = [500, 503, 504];
    Future<AppException> failure() => GrpcFeedService.fetchPosts().then<AppException>(
          (_) => fail('the call must fail while the timeline is stopped'),
          onError: (Object e) => e is AppException ? e : fail('expected an AppException, got $e'),
        );
    final direct = await failure();
    expect(direct.statusCode, anyOf(outage));
    expect(direct.message, isNotEmpty);
    final provider = FeedProvider();
    for (var attempt = 0; attempt < 2; attempt++) {
      await provider.fetchPostsForProfile(person.auth.accessToken);
      expect(provider.error, isNotNull, reason: 'the screen shows its error state');
      expect(provider.error, isNot('Failed to load feed. Please try again.'), reason: 'an AppException, not the catch-all');
      expect(provider.loading, isFalse);
      // Still alive: the app makes another call and fails the same way.
    }
    // Two failed gRPC calls against a stopped service take ~25s; the default 30s limit is too tight.
  }, timeout: const Timeout(Duration(minutes: 3)));
}
