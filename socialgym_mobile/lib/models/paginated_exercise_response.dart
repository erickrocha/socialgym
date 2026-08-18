import 'exercise.dart';

class PaginatedExerciseResponse {
  final List<Exercise> content;
  final int totalCount;
  final int pageNumber;
  final int pageSize;
  final bool hasNextPage;

  PaginatedExerciseResponse({
    required this.content,
    required this.totalCount,
    required this.pageNumber,
    required this.pageSize,
    required this.hasNextPage,
  });

  factory PaginatedExerciseResponse.fromJson(Map<String, dynamic> json) {
    return PaginatedExerciseResponse(
      content:
          (json['content'] as List<dynamic>?)
              ?.map((e) => Exercise.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      totalCount: json['totalCount'] as int? ?? 0,
      pageNumber: json['pageNumber'] as int? ?? 1,
      pageSize: json['pageSize'] as int? ?? 20,
      hasNextPage: json['hasNextPage'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'content': content.map((e) => e.toJson()).toList(),
      'totalCount': totalCount,
      'pageNumber': pageNumber,
      'pageSize': pageSize,
      'hasNextPage': hasNextPage,
    };
  }
}
