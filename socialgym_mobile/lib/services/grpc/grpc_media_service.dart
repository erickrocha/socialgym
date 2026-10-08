import 'package:socialgym_mobile/services/grpc/grpc_workout_channel.dart';
import 'package:socialgym_mobile/src/generated/grpc/media.pbgrpc.dart' as $media;

/// Pre-signed S3 upload URL for a post or chat media file.
class PostMediaUploadUrl {
  final String url;
  final String objectKey;

  PostMediaUploadUrl({required this.url, required this.objectKey});
}

/// gRPC client façade for the MediaService.
class GrpcMediaService {
  GrpcMediaService._();

  static $media.MediaServiceClient? _override;

  /// Built per call on the shared channel: sign-out shuts the channels down, and a cached client
  /// would keep pointing at the dead one.
  static $media.MediaServiceClient _ensureClient() =>
      _override ?? $media.MediaServiceClient(GrpcWorkoutChannel.channel(), interceptors: GrpcWorkoutChannel.interceptors);

  /// Replaces the client (tests point it at a fake server).
  static void useClient($media.MediaServiceClient? client) => _override = client;

  /// [album] is where the file belongs (`post`, `chat`, ...); [format] is the full content type.
  static Future<PostMediaUploadUrl> getPostMediaUploadUrl({
    required String album,
    required String format,
  }) => GrpcWorkoutChannel.guard('Failed to get presigned URL for post media', () async {
    final response = await _ensureClient().getPostMediaUploadUrl(
      $media.MediaUploadRequest(album: album, format: format),
      options: GrpcWorkoutChannel.options,
    );
    return PostMediaUploadUrl(url: response.url, objectKey: response.objectKey);
  });
}
