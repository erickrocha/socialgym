import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/l10n/app_localizations.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/pages/home/post_composer_page.dart';
import 'package:socialgym_mobile/providers/auth_provider.dart';
import 'package:socialgym_mobile/providers/feed_provider.dart';
import 'package:socialgym_mobile/providers/person_provider.dart';

/// Regression tests for the iOS bug where the composer's media toolbar was
/// hidden behind the software keyboard: the composer used to be a
/// `showModalBottomSheet` whose bottom toolbar got no `viewInsets`
/// compensation, and iOS offers no back gesture to dismiss the keyboard with.
void main() {
  const keyboardHeight = 336.0;

  late PersonProvider personProvider;
  late AuthProvider authProvider;
  late FeedProvider feedProvider;

  setUp(() {
    SharedPreferences.setMockInitialValues({});
    personProvider = PersonProvider()
      ..setPersonForTest(
        Person(
          id: 1,
          uuid: 'person-uuid-1',
          firstname: 'John',
          surname: 'Doe',
          gender: 'male',
        ),
      );
    authProvider = AuthProvider();
    feedProvider = FeedProvider();
  });

  Future<void> pumpComposer(
    WidgetTester tester, {
    required double bottomInset,
  }) {
    return tester.pumpWidget(
      MaterialApp(
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        home: MediaQuery(
          data: MediaQueryData(
            size: tester.view.physicalSize / tester.view.devicePixelRatio,
            viewInsets: EdgeInsets.only(bottom: bottomInset),
          ),
          child: MultiProvider(
            providers: [
              ChangeNotifierProvider<PersonProvider>.value(
                value: personProvider,
              ),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
              ChangeNotifierProvider<FeedProvider>.value(value: feedProvider),
            ],
            child: const PostComposerPage(),
          ),
        ),
      ),
    );
  }

  testWidgets('media button stays above the keyboard', (tester) async {
    await pumpComposer(tester, bottomInset: keyboardHeight);
    await tester.pumpAndSettle();

    final mediaButton = find.text('Media');
    expect(mediaButton, findsOneWidget);

    final screenHeight =
        tester.view.physicalSize.height / tester.view.devicePixelRatio;
    final keyboardTop = screenHeight - keyboardHeight;

    // The whole toolbar must sit in the region the keyboard leaves free.
    expect(
      tester.getBottomRight(mediaButton).dy,
      lessThanOrEqualTo(keyboardTop),
      reason: 'the media button must not be covered by the keyboard',
    );
  });

  testWidgets('offers a hide-keyboard action only while the keyboard is up', (
    tester,
  ) async {
    await pumpComposer(tester, bottomInset: keyboardHeight);
    await tester.pumpAndSettle();
    expect(find.byIcon(Icons.keyboard_hide_outlined), findsOneWidget);

    await pumpComposer(tester, bottomInset: 0);
    await tester.pumpAndSettle();
    expect(find.byIcon(Icons.keyboard_hide_outlined), findsNothing);
  });
}
