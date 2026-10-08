import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:protobuf/well_known_types/google/protobuf/empty.pb.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_business_profile_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_exercise_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_media_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/business_profile.pbgrpc.dart' as $bp;
import 'package:socialgym_mobile/src/generated/grpc/exercise.pbgrpc.dart' as $exercise;
import 'package:socialgym_mobile/src/generated/grpc/media.pbgrpc.dart' as $media;

import 'utils/fake_grpc_server.dart';

class _Media extends $media.MediaServiceBase {
  $media.MediaUploadRequest? last;

  @override
  Future<$media.MediaUploadResponse> getPostMediaUploadUrl(grpc.ServiceCall call, $media.MediaUploadRequest request) async {
    last = request;
    if (request.album == 'bad') throw grpc.GrpcError.invalidArgument('album');
    return $media.MediaUploadResponse(url: 'https://s3/${request.album}', objectKey: 'k/${request.album}');
  }
}

/// Only the upload-URL method is implemented; the others are never called.
class _Profiles extends $bp.BusinessProfileServiceBase {
  $bp.BusinessProfileImageUploadRequest? last;

  @override
  Future<$bp.BusinessProfileImageUploadResponse> getBusinessProfileImageUploadUrl(
    grpc.ServiceCall call,
    $bp.BusinessProfileImageUploadRequest request,
  ) async {
    last = request;
    return $bp.BusinessProfileImageUploadResponse(url: 'https://s3/${request.imageType}', objectKey: 'bp/${request.imageType}');
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _Exercises extends $exercise.ExerciseServiceBase {
  int? deleted;

  @override
  Future<Empty> deleteExercise(grpc.ServiceCall call, $exercise.ExerciseRequest request) async {
    if (request.id == 404) throw grpc.GrpcError.notFound('exercise');
    deleted = request.id;
    return Empty();
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

void main() {
  late FakeGrpcServer server;
  final media = _Media();
  final profiles = _Profiles();
  final exercises = _Exercises();

  setUpAll(() async {
    server = await FakeGrpcServer.start([media, profiles, exercises]);
    GrpcMediaService.useClient($media.MediaServiceClient(server.channel));
    GrpcBusinessProfileService.useClient($bp.BusinessProfileServiceClient(server.channel));
    GrpcExerciseService.useClient($exercise.ExerciseServiceClient(server.channel));
  });
  tearDownAll(() => server.stop());

  test('post media: album and content type go out, URL and key come back', () async {
    final url = await GrpcMediaService.getPostMediaUploadUrl(album: 'chat', format: 'image/png');
    expect((url.url, url.objectKey), ('https://s3/chat', 'k/chat'));
    expect((media.last!.album, media.last!.format), ('chat', 'image/png'));
  });

  test('post media: a refused album is a 400 AppException', () async {
    await expectLater(
      GrpcMediaService.getPostMediaUploadUrl(album: 'bad', format: 'image/png'),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 400)),
    );
  });

  test('business profile image upload URL carries the image type', () async {
    final url = await GrpcBusinessProfileService.getBusinessProfileImageUploadUrl(imageType: 'logo', format: 'image/jpeg');
    expect((url.url, url.objectKey), ('https://s3/logo', 'bp/logo'));
    expect((profiles.last!.imageType, profiles.last!.format), ('logo', 'image/jpeg'));
  });

  test('delete exercise sends the id; a missing one is a 404 AppException', () async {
    await GrpcExerciseService.deleteExercise(id: 12);
    expect(exercises.deleted, 12);
    await expectLater(
      GrpcExerciseService.deleteExercise(id: 404),
      throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
    );
  });
}
