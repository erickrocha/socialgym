import 'package:socialgym_mobile/models/auth_response.dart';
import 'package:socialgym_mobile/models/sign_up_request.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_channel.dart';
import 'package:socialgym_mobile/src/generated/grpc/auth.pbgrpc.dart' as $auth;

/// gRPC client façade for the AuthService: sign-in, sign-up and the business-profile switch that
/// re-issues the token. Sign-in and sign-up are reachable without an access token.
class GrpcAuthService {
  GrpcAuthService._();

  static $auth.AuthServiceClient? _override;

  /// Built per call on the shared channel: sign-out shuts the channels down, and a cached client
  /// would keep pointing at the dead one.
  static $auth.AuthServiceClient _ensureClient() =>
      _override ?? $auth.AuthServiceClient(GrpcWorkoutChannel.channel(), interceptors: GrpcWorkoutChannel.interceptors);

  /// Replaces the client (tests point it at a fake server).
  static void useClient($auth.AuthServiceClient? client) => _override = client;

  static AuthResponse _response($auth.AccessToken t) => AuthResponse(
    accessToken: t.accessToken,
    tokenType: t.tokenType.isEmpty ? 'Bearer' : t.tokenType,
    expireIn: t.expireIn.toInt(),
    refreshToken: t.hasRefreshToken() ? t.refreshToken : null,
    username: t.username,
    uuid: t.uuid,
    name: t.name,
    personId: t.personId,
    personAvatar: '',
    personUuid: t.personUuid.isEmpty ? null : t.personUuid,
    personObjectKey: t.personObjectKey.isEmpty ? null : t.personObjectKey,
    activeBusinessProfileId: t.hasActiveBusinessProfileId() ? t.activeBusinessProfileId : null,
    activeBusinessProfileUuid: t.hasActiveBusinessProfileUuid() ? t.activeBusinessProfileUuid : null,
    pendingAccountDeletion: t.hasPendingAccountDeletion()
        ? PendingAccountDeletion(
            requestedAt: DateTime.parse(t.pendingAccountDeletion.requestedAt),
            scheduledAt: DateTime.parse(t.pendingAccountDeletion.scheduledAt),
          )
        : null,
  );

  static Future<AuthResponse> signIn({required String email, required String password}) =>
      GrpcWorkoutChannel.guard('Login failed', () async {
        final token = await _ensureClient().login(
          $auth.LoginRequest(email: email, password: password),
          options: GrpcWorkoutChannel.options,
        );
        return _response(token);
      });

  static Future<AuthResponse> signUp(SignUpRequest request) =>
      GrpcWorkoutChannel.guard('Sign up failed', () async {
        final token = await _ensureClient().signup(
          $auth.SignupRequest(
            firstname: request.firstname,
            surname: request.surname,
            dateOfBirth: request.dateOfBirth,
            gender: request.gender,
            email: request.email,
            password: request.password,
            termsVersion: request.termsVersion,
            privacyVersion: request.privacyVersion,
            termsAccepted: request.termsAccepted,
            privacyAccepted: request.privacyAccepted,
          ),
          options: GrpcWorkoutChannel.options,
        );
        return _response(token);
      });

  /// Activates a business profile, returning a new token scoped to it.
  static Future<AuthResponse> activateBusinessProfile({required String businessProfileUuid}) =>
      GrpcWorkoutChannel.guard('Failed to activate business profile', () async {
        final token = await _ensureClient().activateBusinessProfile(
          $auth.ActivateBusinessProfileRequest(businessProfileUuid: businessProfileUuid),
          options: GrpcWorkoutChannel.options,
        );
        return _response(token);
      });

  /// Deactivates the current business profile, returning a new token back in personal context.
  static Future<AuthResponse> deactivateBusinessProfile() =>
      GrpcWorkoutChannel.guard('Failed to deactivate business profile', () async {
        final token = await _ensureClient().deactivateBusinessProfile(
          $auth.DeactivateBusinessProfileRequest(),
          options: GrpcWorkoutChannel.options,
        );
        return _response(token);
      });
}
