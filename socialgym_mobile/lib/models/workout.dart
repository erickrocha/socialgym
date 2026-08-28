import 'exercise.dart';

class Workout {
  final int? id;
  final String? uuid;
  final int ownerId;
  final String ownerUuid;
  final String name;
  final String? description;
  final String difficulty;
  final String muscleGroup;
  final String visibility;
  final List<Exercise> exercises;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  Workout({
    this.id,
    this.uuid,
    required this.ownerId,
    required this.ownerUuid,
    required this.name,
    this.description,
    this.difficulty = 'soft',
    this.muscleGroup = '',
    this.visibility = 'Private',
    this.exercises = const [],
    this.createdAt,
    this.updatedAt,
  });

  factory Workout.fromJson(Map<String, dynamic> json) {
    return Workout(
      id: json['id'],
      uuid: json['uuid'],
      ownerId: json['ownerId'],
      ownerUuid: json['ownerUuid'],
      name: json['name'] ?? '',
      description: json['description'],
      difficulty: json['difficulty'] ?? 'soft',
      muscleGroup: json['muscleGroup'] ?? '',
      visibility: json['visibility'] ?? 'Private',
      exercises:
          (json['exercises'] as List<dynamic>?)
              ?.map((e) => Exercise.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      createdAt: json['createdAt'] != null ? DateTime.parse(json['createdAt']) : null,
      updatedAt: json['updatedAt'] != null ? DateTime.parse(json['updatedAt']) : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      if (id != null) 'id': id,
      'uuid': uuid,
      'ownerId': ownerId,
      'ownerUuid': ownerUuid,
      'name': name,
      'description': description,
      'difficulty': difficulty,
      'muscleGroup': muscleGroup,
      'visibility': visibility,
      'exercises': exercises.map((e) => e.toJson()).toList(),
      'createdAt': createdAt?.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
    };
  }

  List<String> get muscleGroups => muscleGroup.isEmpty ? [] : muscleGroup.split('|');

  Workout copyWith({
    int? id,
    String? uuid,
    String? name,
    String? description,
    String? difficulty,
    String? muscleGroup,
    String? visibility,
    int? ownerId,
    String? ownerUuid,
    List<Exercise>? exercises,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return Workout(
      id: id ?? this.id,
      uuid: uuid ?? this.uuid,
      name: name ?? this.name,
      description: description ?? this.description,
      difficulty: difficulty ?? this.difficulty,
      muscleGroup: muscleGroup ?? this.muscleGroup,
      visibility: visibility ?? this.visibility,
      ownerId: ownerId ?? this.ownerId,
      ownerUuid: ownerUuid ?? this.ownerUuid,
      exercises: exercises ?? this.exercises,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}
