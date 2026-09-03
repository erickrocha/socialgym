class ApiConfig {
  /// Base URL do backend. Passe `--dart-define=API_BASE_URL=https://seu.dominio`
  /// no build de release; o default abaixo é só para desenvolvimento local.
  /// Nunca deixe um host de túnel/dev como default de release — foi isso que
  /// causou a rejeição 5.6 da Apple no socialgym_mobile em 03/09/2026.
  static const String baseUrlDefault = String.fromEnvironment(
    'API_BASE_URL',
    defaultValue: 'https://scrutiny-elevator-washstand.ngrok-free.dev',
  );

  /// gRPC usa o mesmo host do REST — uma fonte de verdade só.
  static String get grpcHost => Uri.parse(baseUrlDefault).host;
  static const int grpcPort = int.fromEnvironment(
    'GRPC_PORT',
    defaultValue: 443,
  );
  static const bool grpcUseTls = true;

  /// true apenas em dev contra backend local com certificado autoassinado
  /// (`--dart-define=GRPC_SELF_SIGNED=true`). Em release fica false e o cliente
  /// confia na cadeia TLS pública normal.
  static const bool grpcSelfSignedCert = bool.fromEnvironment(
    'GRPC_SELF_SIGNED',
    defaultValue: false,
  );

  static const String loginEndpoint = '/login';
  static const String signUpEndpoint = '/signup';
  static String authProfileActivateEndpoint(String uuid) =>
      '/auth/profile/$uuid/activate';
  static const String authProfileDeactivateEndpoint =
      '/auth/profile/deactivate';

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
  static const String accountDeleteEndpoint =
      '/workout/api/people/me/account/delete';
  static const String accountCancelDeletionEndpoint =
      '/workout/api/people/me/account/cancel-deletion';

  static const String businessProfilesEndpoint =
      '/workout/api/business-profiles';

  static const String friendsEndpoint = '/workout/api/friends';
  static const String friendsSearchEndpoint = '/workout/api/friends/search';
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
  static const String addressSearchEndpoint = '/workout/api/address/search';
  static const String evolutionCheckInEndpoint =
      '/timeline/api/evolution-checkin';

  static const Duration timeout = Duration(seconds: 30);

  /// Returns the correct base URL depending on the current platform.
  static String get baseUrl {
    return baseUrlDefault;
  }

  static String? get grpcAuthority => grpcHost;
}
