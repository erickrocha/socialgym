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

  /// Assignment consent state: `Pending` / `Accepted` / `Rejected` / `Cancelled`.
  /// Self-created workouts are `Accepted`.
  final String status;

  /// UUID of the business profile that assigned this workout (only set while it
  /// is a team-member assignment); `null` for self-created workouts.
  final String? assignedByProfileUuid;

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
    this.status = 'Accepted',
    this.assignedByProfileUuid,
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
      status: json['status'] ?? 'Accepted',
      assignedByProfileUuid: json['assignedByProfileUuid'],
      createdAt: json['createdAt'] != null
          ? DateTime.parse(json['createdAt'])
          : null,
      updatedAt: json['updatedAt'] != null
          ? DateTime.parse(json['updatedAt'])
          : null,
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
      'status': status,
      if (assignedByProfileUuid != null)
        'assignedByProfileUuid': assignedByProfileUuid,
      'createdAt': createdAt?.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
    };
  }

  List<String> get muscleGroups =>
      muscleGroup.isEmpty ? [] : muscleGroup.split('|');

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
    String? status,
    String? assignedByProfileUuid,
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
      status: status ?? this.status,
      assignedByProfileUuid:
          assignedByProfileUuid ?? this.assignedByProfileUuid,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}
