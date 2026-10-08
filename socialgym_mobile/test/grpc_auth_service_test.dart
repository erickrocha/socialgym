import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/models/sign_up_request.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_auth_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/auth.pbgrpc.dart' as $auth;

import 'utils/fake_grpc_server.dart';

class _Auth extends $auth.AuthServiceBase {
  $auth.SignupRequest? signedUp;

  $auth.AccessToken _token({String access = 'a1', String? profile}) => $auth.AccessToken(
    accessToken: access,
    tokenType: 'Bearer',
    expireIn: Int64(900),
    username: 'ana@example.test',
    uuid: 'u1',
    name: 'Ana',
    personId: 7,
    personUuid: 'p1',
    activeBusinessProfileUuid: profile,
  );

  @override
  Future<$auth.AccessToken> login(grpc.ServiceCall call, $auth.LoginRequest request) async {
    if (request.password != 'right') throw grpc.GrpcError.unauthenticated('bad credentials');
    if (request.email.startsWith('consent')) throw grpc.GrpcError.permissionDenied('terms consent is required');
    return _token()
      ..refreshToken = 'r1'
      ..pendingAccountDeletion = $auth.PendingAccountDeletion(requestedAt: '2026-10-01T10:00:00Z', scheduledAt: '2026-10-31T10:00:00Z');
  }

  @override
  Future<$auth.AccessToken> signup(grpc.ServiceCall call, $auth.SignupRequest request) async {
    signedUp = request;
    if (request.email == 'taken@example.test') throw grpc.GrpcError.alreadyExists('email taken');
    return _token(access: 'a2');
  }

  @override
  Future<$auth.AccessToken> activateBusinessProfile(grpc.ServiceCall call, $auth.ActivateBusinessProfileRequest request) async =>
      _token(access: 'a3', profile: request.businessProfileUuid);

  @override
  Future<$auth.AccessToken> deactivateBusinessProfile(grpc.ServiceCall call, $auth.DeactivateBusinessProfileRequest request) async =>
      _token(access: 'a4');

  @override
  Future<$auth.AccessToken> refresh(grpc.ServiceCall call, $auth.RefreshRequest request) async => _token();

  @override
  Future<$auth.LogoutResponse> logout(grpc.ServiceCall call, $auth.LogoutRequest request) async => $auth.LogoutResponse();
}

void main() {
  late FakeGrpcServer server;
  final auth = _Auth();

  setUpAll(() async {
    server = await FakeGrpcServer.start([auth]);
    GrpcAuthService.useClient($auth.AuthServiceClient(server.channel));
  });
  tearDownAll(() => server.stop());
  tearDown(() => BaseService.onConsentRequired = null);

  test('sign-in maps the token, the refresh token and a pending deletion', () async {
    final response = await GrpcAuthService.signIn(email: 'ana@example.test', password: 'right');
    expect((response.accessToken, response.tokenType, response.expireIn, response.refreshToken), ('a1', 'Bearer', 900, 'r1'));
    expect((response.personId, response.personUuid, response.activeBusinessProfileUuid), (7, 'p1', null));
    expect(response.pendingAccountDeletion!.scheduledAt.difference(response.pendingAccountDeletion!.requestedAt).inDays, 30);
  });

  test('wrong credentials are a 401 AppException', () async {
    await expectLater(
      GrpcAuthService.signIn(email: 'ana@example.test', password: 'wrong'),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 401)),
    );
  });

  test('a missing terms consent raises the consent gate once and is a consent-required 403', () async {
    var raised = 0;
    BaseService.onConsentRequired = () => raised++;
    await expectLater(
      GrpcAuthService.signIn(email: 'consent@example.test', password: 'right'),
      throwsA(isA<AppException>().having((e) => e.isConsentRequired, 'consent required', true).having((e) => e.statusCode, 'status', 403)),
    );
    expect(raised, 1);
  });

  test('sign-up sends every field and a taken email is a 409', () async {
    SignUpRequest request(String email) => SignUpRequest(
      firstname: 'Ana',
      surname: 'Lima',
      email: email,
      password: 'Str0ng!Password',
      dateOfBirth: '1990-01-01',
      gender: 'MALE',
      termsVersion: '1.0.0',
      privacyVersion: '1.0.0',
      termsAccepted: true,
      privacyAccepted: true,
    );
    expect((await GrpcAuthService.signUp(request('new@example.test'))).accessToken, 'a2');
    expect((auth.signedUp!.dateOfBirth, auth.signedUp!.gender, auth.signedUp!.termsAccepted, auth.signedUp!.privacyAccepted), ('1990-01-01', 'MALE', true, true));
    await expectLater(
      GrpcAuthService.signUp(request('taken@example.test')),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 409)),
    );
  });

  test('profile switch returns the re-issued tokens', () async {
    final active = await GrpcAuthService.activateBusinessProfile(businessProfileUuid: 'bp-1');
    expect((active.accessToken, active.activeBusinessProfileUuid), ('a3', 'bp-1'));
    expect((await GrpcAuthService.deactivateBusinessProfile()).activeBusinessProfileUuid, null);
  });
}
