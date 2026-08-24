import 'package:device_preview/device_preview.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/pages/workout/evolution_page.dart';
import 'package:socialgym_mobile/pages/notifications/notifications_page.dart';
import 'package:socialgym_mobile/providers/notifications_provider.dart';

import 'config/app_colors.dart';
import 'l10n/app_localizations.dart';
import 'pages/friends/friends_page.dart';
import 'pages/gallery/gallery_page.dart';
import 'pages/home/home_page.dart';
import 'pages/profile/profile_page.dart';
import 'pages/settings/settings_page.dart';
import 'pages/sign_in/sign_in_page.dart';
import 'pages/workout/exercises_page.dart';
import 'pages/workout/workout_page.dart';
import 'pages/workout/workout_sessions_page.dart';
import 'pages/team/team_page.dart';
import 'pages/followers/followers_page.dart';
import 'providers/auth_provider.dart';
import 'providers/exercise_selection_provider.dart';
import 'providers/evolution_provider.dart';
import 'providers/feed_provider.dart';
import 'providers/friends_provider.dart';
import 'providers/locale_provider.dart';
import 'providers/person_provider.dart';
import 'providers/resource_provider.dart';
import 'providers/settings_provider.dart';
import 'providers/team_member_provider.dart';
import 'providers/workout_provider.dart';
import 'providers/workout_session_provider.dart';
import 'services/grpc/grpc_channel_factory.dart';
import 'pages/business_profile/add_profile_page.dart';
import 'pages/business_profile/business_profile_page.dart';
import 'pages/business_profile/business_profile_sign_up_page.dart';
import 'providers/business_profile_provider.dart';

const List<String> _fontFamilyFallbacks = <String>[
  'Noto Sans',
  'Noto Sans CJK SC',
  'Noto Sans CJK JP',
  'Noto Sans CJK KR',
  'Noto Sans Symbols 2',
  'Noto Color Emoji',
  'Apple Color Emoji',
  'Segoe UI Emoji',
];

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await GrpcChannelFactory.initialize(certAssetPath: 'assets/certs/server.crt');
  runApp(
    DevicePreview(
      enabled: !kReleaseMode,
      builder: (context) => const SocialGymApp(),
    ),
  );
}

class SocialGymApp extends StatelessWidget {
  const SocialGymApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => AuthProvider()),
        ChangeNotifierProvider(create: (_) => LocaleProvider()),
        ChangeNotifierProvider(create: (_) => SettingsProvider()),
        ChangeNotifierProvider(create: (_) => PersonProvider()),
        ChangeNotifierProvider(create: (_) => BusinessProfileProvider()),
        ChangeNotifierProvider(create: (_) => WorkoutProvider()),
        ChangeNotifierProvider(create: (_) => ExerciseSelectionProvider()),
        ChangeNotifierProvider(create: (_) => FriendsProvider()),
        ChangeNotifierProvider(create: (_) => TeamMemberProvider()),
        ChangeNotifierProvider(create: (_) => ResourceProvider()),
        ChangeNotifierProvider(create: (_) => WorkoutSessionProvider()),
        ChangeNotifierProvider(create: (_) => EvolutionProvider()),
        ChangeNotifierProvider(create: (_) => FeedProvider()),
        ChangeNotifierProvider(create: (_) => NotificationsProvider()),
      ],
      child: Consumer2<LocaleProvider, PersonProvider>(
        builder: (context, localeProvider, personProvider, _) {
          return MaterialApp(
            title: 'Social Gym',
            debugShowCheckedModeBanner: false,
            // DevicePreview configuration
            locale: DevicePreview.locale(context) ?? localeProvider.locale,
            builder: DevicePreview.appBuilder,
            // Localization
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            supportedLocales: AppLocalizations.supportedLocales,
            theme: ThemeData(
              fontFamilyFallback: _fontFamilyFallbacks,
              colorScheme: ColorScheme.fromSeed(
                seedColor: personProvider.isProfessional
                    ? AppColors.professionalSecondary
                    : AppColors.primary,
                surface: AppColors.background,
              ),
              useMaterial3: true,
            ),
            initialRoute: '/login',
            routes: {
              '/login': (context) => const SignInPage(),
              '/feed': (context) => const FeedPage(),
              '/gallery': (context) => const GalleryPage(),
              '/profile': (context) => const ProfilePage(),
              '/add-profile': (context) => const AddProfilePage(),
              '/add-profile/form': (context) =>
                  const BusinessProfileSignUpPage(),
              '/business-profile': (context) => const BusinessProfilePage(),
              '/workouts': (context) => const WorkoutPage(),
              '/exercises': (context) => const ExercisesPage(),
              '/workout-sessions': (context) => const WorkoutSessionsPage(),
              '/evolution': (context) =>
                  context.read<PersonProvider>().isProfessional
                      ? const WorkoutPage()
                      : const EvolutionPage(),
              '/friends': (context) => const FriendsPage(),
              '/team': (context) => const TeamPage(),
              '/followers': (context) => const FollowersPage(),
              '/notifications': (context) => const NotificationsPage(),
              '/settings': (context) => const SettingsPage(),
            },
          );
        },
      ),
    );
  }
}
