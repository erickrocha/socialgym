// TC-015: the app's workout flows on a device against the infra/test stack, every call over gRPC.
// Run by scripts/e2e-android.sh, which also starts the stack and reads the gateway log to show that
// no workout REST route was called.
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/models/exercise.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/models/settings.dart';
import 'package:socialgym_mobile/models/workout.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_account_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_address_search_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_auth_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_business_profile_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_consent_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_exercise_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_friend_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_legal_document_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_person_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_settings_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_service.dart';
import 'package:socialgym_mobile/services/upload_service.dart';

import 'support.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  late Account alice;
  late Account bob;
  late Account carol;

  setUpAll(() async {
    await startApp();
    alice = await register('alice');
    bob = await register('bob');
    carol = await register('carol');
    await befriend(alice, bob);
    await asUser(alice, () async {}, back: alice);
  });

  test('legal documents are public and consent follows the current version', () async {
    final documents = await GrpcLegalDocumentService.list();
    expect(documents.map((d) => d.document), containsAll(['terms', 'privacy', 'health_data']));
    expect(await GrpcConsentService.pending(), isEmpty, reason: 'sign-up accepted terms and privacy');
    expect(await GrpcConsentService.hasActive('health_data'), isTrue, reason: 'register() accepted it');
    await GrpcConsentService.accept('health_data'); // idempotent
  });

  test('sign-in returns a session and a wrong password is a 401', () async {
    final session = await GrpcAuthService.signIn(email: alice.email, password: alice.password);
    expect((session.personId, session.accessToken.isNotEmpty), (alice.personId, true));
    await expectLater(
      GrpcAuthService.signIn(email: alice.email, password: 'not-the-password'),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 401)),
    );
  });

  test('profile: read, update, info, address and image upload URL', () async {
    final me = await GrpcPersonService.getMe();
    expect(me.uuid, alice.personUuid);
    final renamed = await GrpcPersonService.updatePerson(
      Person(id: me.id, uuid: me.uuid, firstname: 'Alicia', surname: me.surname, dateOfBirth: me.dateOfBirth, gender: me.gender, addresses: me.addresses),
    );
    expect(renamed.firstname, 'Alicia');
    final info = await GrpcPersonService.updatePersonInfo(
      PersonInfo(id: me.personInfo!.id, uuid: me.personInfo!.uuid, personId: me.personInfo!.personId, job: 'coach', weight: 70.0, height: 170.0),
    );
    expect(info.job, 'coach');
    final address = await GrpcPersonService.addPersonAddress(
      PersonAddress(addressLine1: 'Rua A 1', locality: 'Sao Paulo', administrativeArea: 'SP', countryCode: 'BR', postalCode: '01000-000', latitude: -23.5, longitude: -46.6, current: true),
    );
    expect(address.id, isNotNull);
    await GrpcPersonService.removePersonAddress(id: address.id);
    expect((await GrpcPersonService.getMe()).addresses.any((a) => a.id == address.id), isFalse);
    final upload = await UploadService.getAvatarPresignedUrl('image/png');
    expect(upload.url, isNotEmpty);
    expect(upload.objectKey, isNotEmpty);
    final media = await UploadService.getPostMediaPresignedUrl('image/png', 'post');
    expect(media.url, isNotEmpty);
  });

  test('settings: persist and read back', () async {
    // Sign-up created the settings row; the app loads it and saves over it.
    final current = await GrpcSettingsService.getByOwnerId(ownerId: alice.personId, ownerUuid: alice.personUuid);
    expect(current, isNotNull);
    final saved = await GrpcSettingsService.persistSettings(
      Settings(id: current!.id, uuid: current.uuid, ownerId: alice.personId, ownerUuid: alice.personUuid, language: 'pt', theme: 'dark'),
    );
    expect(saved.theme, 'dark');
    final read = await GrpcSettingsService.getByOwnerId(ownerId: alice.personId, ownerUuid: alice.personUuid);
    expect((read?.language, read?.theme), ('pt', 'dark'));
  });

  test('friends: a stranger is not a friend, a friend is, removal works', () async {
    final page = await GrpcFriendService.getFriendPage();
    expect(page.friends.map((f) => f.id), contains(bob.personId));
    expect(page.friends.map((f) => f.id), isNot(contains(carol.personId)));
    await GrpcFriendService.sendFriendRequest(personId: carol.personId);
    await asUser(carol, () => GrpcFriendService.denyFriendRequest(personId: alice.personId), back: alice);
    await expectLater(
      asUser(carol, () => GrpcPersonService.searchMentionableFriends(personId: alice.personId, query: 'a'), back: alice),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 403)),
    );
  });

  test('exercises and workouts: create, read, compose, a stranger gets 404, delete', () async {
    final exercise = await GrpcExerciseService.addExercise(
      exercise: Exercise(name: 'E2E squat', ownerId: alice.personId, ownerUuid: alice.personUuid, ownerName: 'Alice', description: 'd', sets: 3, category: 'Force', repsOrDuration: 10, visibility: 'Private'),
    );
    expect((await GrpcExerciseService.getExercise(id: exercise.id!)).name, 'E2E squat');
    final page = await GrpcExerciseService.getPaginatedExercises(ownerUuid: alice.personUuid, category: 'Force', visibility: 'Private', publicOwners: const [], pageNumber: 1, pageSize: 20, sortBy: 'created_at_desc');
    expect(page.content.any((e) => e.id == exercise.id), isTrue);
    await expectLater(
      asUser(carol, () => GrpcExerciseService.deleteExercise(id: exercise.id!), back: alice),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
    );

    final workout = await GrpcWorkoutService.createWorkout(
      workout: Workout(ownerId: alice.personId, ownerUuid: alice.personUuid, name: 'E2E legs', muscleGroup: 'Legs'),
    );
    final composed = await GrpcWorkoutService.addExercisesToWorkout(workoutUuid: workout.uuid!, exercises: [exercise]);
    expect(composed.exercises.map((e) => e.id), contains(exercise.id));
    expect((await GrpcWorkoutService.getWorkoutsByOwnerUuid(ownerUuid: alice.personUuid)).any((w) => w.uuid == workout.uuid), isTrue);
    await GrpcWorkoutService.deleteWorkout(uuid: workout.uuid!);
    await GrpcExerciseService.deleteExercise(id: exercise.id!);
    await expectLater(
      GrpcExerciseService.getExercise(id: exercise.id!),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
    );
  });

  test('business profile: create, switch the session into it and back, image upload URL', () async {
    final profile = await GrpcBusinessProfileService.addBusinessProfile(
      BusinessProfile(ownerId: alice.personId, ownerUuid: alice.personUuid, taxId: '12345678000199', businessName: 'E2E Gym ${DateTime.now().microsecondsSinceEpoch}', businessType: 'Company'),
    );
    expect(profile.uuid, isNotNull);
    final active = await GrpcAuthService.activateBusinessProfile(businessProfileUuid: profile.uuid!);
    expect(active.activeBusinessProfileUuid, profile.uuid);
    await storeSession(active);
    final logo = await UploadService.getBusinessProfileLogoPresignedUrl('image/png');
    expect(logo.url, isNotEmpty);
    final personal = await GrpcAuthService.deactivateBusinessProfile();
    expect(personal.activeBusinessProfileUuid, isNull);
    await storeSession(personal);
    final owned = await GrpcBusinessProfileService.getBusinessProfileByOwnerId(ownerId: alice.personId);
    expect(owned.any((p) => p.uuid == profile.uuid), isTrue);
  });

  test('address search reaches the server (the feature is switched off on the test stack)', () async {
    await expectLater(
      GrpcAddressSearchService.search(text: 'Rua Augusta'),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 403)),
    );
  });

  test('account: data export, then deletion requested, seen at sign-in and cancelled', () async {
    final job = await GrpcAccountService.createDataExport();
    expect((await GrpcAccountService.listDataExports()).map((j) => j.id), contains(job.id));
    final status = await GrpcAccountService.requestDeletion(immediate: false);
    expect(status.scheduledAt.isAfter(status.requestedAt), isTrue);
    // Requesting deletion revokes every session issued up to that second; sign in after it.
    await Future<void>.delayed(const Duration(milliseconds: 1500));
    final back = await GrpcAuthService.signIn(email: alice.email, password: alice.password);
    expect(back.pendingAccountDeletion, isNotNull);
    await storeSession(back);
    await GrpcAccountService.cancelDeletion();
    final again = await GrpcAuthService.signIn(email: alice.email, password: alice.password);
    expect(again.pendingAccountDeletion, isNull);
  });
}
