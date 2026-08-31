import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';

import 'package:lapidation_mobile/config/app_colors.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/main.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/consent_provider.dart';

class _BlockingConsentProvider extends ConsentProvider {
  @override
  bool get blocking => true;
}

void main() {
  test('profile accents stay inside the Lapidation palette', () {
    expect(AppColors.primaryFor(null), AppColors.primary);
    expect(
      AppColors.primaryFor('Professional'),
      AppColors.professionalSecondary,
    );
    expect(AppColors.primaryFor('Company'), AppColors.secondary);
  });

  testWidgets('consent gate blocks the underlying application', (tester) async {
    final navigatorKey = GlobalKey<NavigatorState>();

    await tester.pumpWidget(
      MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => AuthProvider()),
          ChangeNotifierProvider<ConsentProvider>(
            create: (_) => _BlockingConsentProvider(),
          ),
        ],
        child: MaterialApp(
          navigatorKey: navigatorKey,
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: ConsentGate(
            navigatorKey: navigatorKey,
            child: const Text('underlying application'),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Legal consent required'), findsOneWidget);
    expect(find.text('underlying application'), findsOneWidget);
  });
}
