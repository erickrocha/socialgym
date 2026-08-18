class ApiConfig {
  /// Android emulator uses 10.0.2.2 to reach host machine's localhost.
  /// Linux desktop, web, and other platforms connect directly to localhost.
  // static const String baseUrlDefault = 'https://192.168.15.4';
  static const String baseUrlDefault =
      'https://scrutiny-elevator-washstand.ngrok-free.dev';
  // static const String grpcHost = 'https://192.168.15.4';
  static const String grpcHost = 'scrutiny-elevator-washstand.ngrok-free.dev';
  static const int grpcPort = 443;
  static const bool grpcUseTls = true;
  // static const String baseUrlDefault = 'https://pt6mn56fes.us-east-1.awsapprunner.com';
  static const String loginEndpoint = '/login';
  static const String signUpEndpoint = '/signup';
  static String authProfileActivateEndpoint(String uuid) => '/auth/profile/$uuid/activate';
  static const String authProfileDeactivateEndpoint = '/auth/profile/deactivate';

  static const String workoutsEndpoint = '/workout/api/workouts';
  static const String peopleMe = '/workout/api/people/me';
  static const String peopleInfoEndpoint = '/workout/api/people/me/info';
  static const String peopleDeleteAvatarEndpoint =
      '/workout/api/people/me/delete/avatar';
  static const String peopleDeleteCoverEndpoint =
      '/workout/api/people/me/delete/cover';
  static const String peopleAddressEndpoint = '/workout/api/people/me/address';
  static const String peopleSearchEndpoint = '/workout/api/people';
  static const String peopleUploadAvatarEndpoint =
      '/workout/api/people/me/upload/avatar';
  static const String peopleUploadCoverEndpoint =
      '/workout/api/people/me/upload/cover';

  static const String businessProfilesEndpoint =
      '/workout/api/business-profiles';

  static const String friendsEndpoint = '/workout/api/friends';
  static const String friendsRequestEndpoint = '/workout/api/friends/request';
  static const String friendsAcceptEndpoint = '/workout/api/friends/accept';
  static const String friendsRejectEndpoint = '/workout/api/friends/reject';
  static const String friendsCancelEndpoint = '/workout/api/friends/cancel';

  static const String exercisesEndpoint = '/workout/api/exercises';
  static const String exercisesQueryEndpoint = '/workout/api/exercises/query';

  static const String workoutSessionsEndpoint =
      '/timeline/api/workout-sessions';

  static const String feedEndpoint = '/timeline/api/feed';
  static const String businessFeedEndpoint = '/timeline/api/feed';
  static const String postEndpoint = '/timeline/api/posts';
  static const String notificationsEndpoint = '/timeline/api/notifications';

  static const String feedUploadEndpoint = '/workout/api/media/upload';
  static const String resourceEndpoint = '/workout/api/resource';
  static const String evolutionCheckInEndpoint =
      '/timeline/api/evolution-checkin';

  static const Duration timeout = Duration(seconds: 30);

  /// Returns the correct base URL depending on the current platform.
  static String get baseUrl {
    return baseUrlDefault;
  }

  static String? get grpcAuthority => 'latently-bubaline-salome.ngrok-free.dev';
  // static String? get grpcAuthority => '192.168.15.4';
}
