// TC-012 step 3: with the timeline stopped the feed flow fails the way the app already handles a
// failed load (an error message, no crash). Run by scripts/e2e-android.sh after stopping the
// timeline.
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:socialgym_mobile/providers/feed_provider.dart';

import 'support.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  test('a stopped timeline is an error message, and the app keeps running', () async {
    await startApp();
    final person = await register('dave');
    final provider = FeedProvider();
    await provider.fetchPostsForProfile(person.auth.accessToken);
    expect(provider.error, isNotNull, reason: 'the screen shows its error state');
    expect(provider.error, isNotEmpty);
    expect(provider.loading, isFalse);
    // Still alive: the app can make another call and fail the same way.
    await provider.fetchPostsForProfile(person.auth.accessToken);
    expect(provider.error, isNotNull);
    // Two failed gRPC calls against a stopped service take ~25s; the default 30s limit is too tight.
  }, timeout: const Timeout(Duration(minutes: 3)));
}
