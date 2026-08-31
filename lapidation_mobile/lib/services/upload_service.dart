import 'package:dio/dio.dart';
import 'package:image_picker/image_picker.dart';
import '../config/api_config.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';
import 'grpc/grpc_person_service.dart';

class PresignedUrlResponse {
  final String url;
  final String objectKey;
  final int? ownerId;
  PresignedUrlResponse({
    required this.url,
    required this.objectKey,
    this.ownerId,
  });
  factory PresignedUrlResponse.fromJson(Map<String, dynamic> json) {
    return PresignedUrlResponse(
      url: json['url'] as String,
      objectKey: json['objectKey'] as String,
      ownerId: json['ownerId'] as int?,
    );
  }
}

class UploadService {
  static final _dio = DioClient().dio;
  static String _getImageFormat(String filePath) {
    final extension = filePath.split('.').last.toLowerCase();
    switch (extension) {
      case 'jpg':
      case 'jpeg':
        return 'jpeg';
      case 'png':
        return 'png';
      case 'gif':
        return 'gif';
      case 'webp':
        return 'webp';
      default:
        return 'jpeg';
    }
  }

  static Future<PresignedUrlResponse> getAvatarPresignedUrl(
    String format,
  ) async {
    final result = await GrpcPersonService.getPersonImageUploadUrl(
      imageType: 'avatar',
      format: format,
    );
    return PresignedUrlResponse(url: result.url, objectKey: result.objectKey);
  }

  static Future<PresignedUrlResponse> getCoverPresignedUrl(
    String format,
  ) async {
    final result = await GrpcPersonService.getPersonImageUploadUrl(
      imageType: 'cover',
      format: format,
    );
    return PresignedUrlResponse(url: result.url, objectKey: result.objectKey);
  }

  static Future<PresignedUrlResponse> getBusinessProfileLogoPresignedUrl(
    String token,
    int businessProfileId,
    String format,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '${ApiConfig.businessProfilesEndpoint}/upload/avatar?format=$format',
      );
      if (response.statusCode == 200) {
        return PresignedUrlResponse.fromJson(
          response.data as Map<String, dynamic>,
        );
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to get presigned URL',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to get presigned URL');
    }
  }

  static Future<PresignedUrlResponse> getBusinessProfileCoverPresignedUrl(
    String token,
    String format,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '${ApiConfig.businessProfilesEndpoint}/upload/cover?format=$format',
      );
      if (response.statusCode == 200) {
        return PresignedUrlResponse.fromJson(
          response.data as Map<String, dynamic>,
        );
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to get presigned URL',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to get presigned URL');
    }
  }

  static bool _isVideoExtension(String ext) =>
      ['mp4', 'mov', 'avi', 'mkv', 'webm', '3gp'].contains(ext);
  static String _getVideoFormat(String filePath) {
    final ext = filePath.split('.').last.toLowerCase();
    return (ext == 'mov') ? 'mp4' : ext;
  }

  static (String mediaType, String format, String contentType) _mediaInfo(
    String filePath,
  ) {
    final ext = filePath.split('.').last.toLowerCase();
    if (_isVideoExtension(ext)) {
      final fmt = _getVideoFormat(filePath);
      return ('Video', fmt, 'video/$fmt');
    }
    final fmt = _getImageFormat(filePath);
    return ('Image', fmt, 'image/$fmt');
  }

  static Future<PresignedUrlResponse> getPostMediaPresignedUrl(
    String token,
    String mediaType,
    String format,
    String album,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.feedUploadEndpoint,
        queryParameters: {
          'mediaType': mediaType,
          'format': format,
          'album': album,
        },
      );
      if (response.statusCode == 200) {
        return PresignedUrlResponse.fromJson(
          response.data as Map<String, dynamic>,
        );
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message:
              response.data?['message'] ??
              'Failed to get presigned URL for post media',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(
        e,
        'Failed to get presigned URL for post media',
      );
    }
  }

  static Future<PresignedUrlResponse> uploadPostMedia(
    String token,
    XFile file,
    String album,
  ) async {
    final (mediaType, format, contentType) = _mediaInfo(
      file.name.isNotEmpty ? file.name : file.path,
    );
    final presigned = await getPostMediaPresignedUrl(
      token,
      mediaType,
      contentType,
      album,
    );
    await uploadToS3(presigned.url, file, format, contentType: contentType);
    return presigned;
  }

  static Future<void> uploadToS3(
    String presignedUrl,
    XFile file,
    String format, {
    String? contentType,
  }) async {
    final dio = Dio();
    final bytes = await file.readAsBytes();
    final resolvedContentType = contentType ?? 'image/$format';
    try {
      final response = await dio.put(
        presignedUrl,
        data: bytes,
        options: Options(
          headers: {"Content-Type": resolvedContentType},
          followRedirects: false,
          validateStatus: (status) => status != null && status < 500,
        ),
      );
      if (response.statusCode != 200) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: 'Failed to upload file to S3',
        );
      }
    } on DioException catch (e) {
      throw AppException(
        statusCode: e.response?.statusCode ?? 500,
        message: e.message ?? 'Failed to upload file to S3',
      );
    }
  }

  static Future<bool> uploadAvatar(XFile imageFile) async {
    final (mediaType, format, contentType) = _mediaInfo(
      imageFile.name.isNotEmpty ? imageFile.name : imageFile.path,
    );
    final presignedResponse = await getAvatarPresignedUrl(contentType);
    await uploadToS3(presignedResponse.url, imageFile, format);
    return true;
  }

  static Future<bool> uploadCover(XFile imageFile) async {
    final (mediaType, format, contentType) = _mediaInfo(
      imageFile.name.isNotEmpty ? imageFile.name : imageFile.path,
    );
    final presignedResponse = await getCoverPresignedUrl(contentType);
    await uploadToS3(presignedResponse.url, imageFile, format);
    return true;
  }

  static Future<bool> uploadBusinessProfileLogo(
    String token,
    int businessProfileId,
    XFile imageFile,
  ) async {
    final (mediaType, format, contentType) = _mediaInfo(
      imageFile.name.isNotEmpty ? imageFile.name : imageFile.path,
    );
    final presignedResponse = await getBusinessProfileLogoPresignedUrl(
      token,
      businessProfileId,
      contentType,
    );
    await uploadToS3(presignedResponse.url, imageFile, format);
    return true;
  }

  static Future<bool> uploadBusinessProfileCover(
    String token,
    XFile imageFile,
  ) async {
    final (mediaType, format, contentType) = _mediaInfo(
      imageFile.name.isNotEmpty ? imageFile.name : imageFile.path,
    );
    final presignedResponse = await getBusinessProfileCoverPresignedUrl(
      token,
      contentType,
    );
    await uploadToS3(presignedResponse.url, imageFile, format);
    return true;
  }
}
