import 'package:lapidation_mobile/src/generated/grpc/exercise.pb.dart'
    as $exercise;

class Exercise {
  final int? id;
  final String? uuid;
  final String name;
  final int ownerId;
  final String ownerUuid;
  final String ownerName;
  final String? description;
  final int sets;
  final String category;
  final int repsOrDuration;
  final String visibility;
  final DateTime? createdAt;
  final DateTime? updatedAt;
  final double weight;

  Exercise({
    this.id,
    this.uuid,
    required this.name,
    required this.ownerId,
    required this.ownerUuid,
    required this.ownerName,
    this.description,
    required this.sets,
    required this.category,
    required this.repsOrDuration,
    required this.visibility,
    this.createdAt,
    this.updatedAt,
    this.weight = 0,
  });

  factory Exercise.fromJson(Map<String, dynamic> json) {
    return Exercise(
      id: json['id'],
      name: json['name'] ?? '',
      ownerId: json['ownerId'] ?? 0,
      ownerUuid: json['ownerUuid'] ?? '',
      ownerName: json['ownerName'] ?? '',
      description: json['description'],
      sets: json['sets'] ?? 0,
      category: json['category'] ?? '',
      uuid: json['uuid'] ?? '',
      visibility: json['visibility'] ?? '',
      createdAt: json['createdAt'] != null
          ? DateTime.parse(json['createdAt'])
          : null,
      updatedAt: json['updatedAt'] != null
          ? DateTime.parse(json['updatedAt'])
          : null,
      repsOrDuration: json['repsOrDuration'] ?? 0,
      weight: (json['weight'] ?? 0).toDouble(),
    );
  }

  factory Exercise.fromProto($exercise.Exercise proto) {
    return Exercise(
      id: proto.id,
      name: proto.name,
      ownerId: proto.ownerId,
      ownerUuid: proto.ownerUuid,
      ownerName: proto.ownerName,
      description: proto.description,
      sets: proto.sets,
      category: proto.category,
      uuid: proto.uuid,
      visibility: proto.visibility,
      createdAt: DateTime.parse(proto.createdAt),
      updatedAt: DateTime.parse(proto.updatedAt),
      repsOrDuration: proto.repsOrDuration,
      weight: 0,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      if (id != null) 'id': id,
      'name': name,
      'ownerId': ownerId,
      'ownerUuid': ownerUuid,
      'ownerName': ownerName,
      'description': description,
      'sets': sets,
      'category': category,
      'uuid': uuid,
      'visibility': visibility,
      'createdAt': createdAt?.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
      'repsOrDuration': repsOrDuration,
      'weight': weight,
    };
  }

  Exercise copyWith({
    int? id,
    String? name,
    int? ownerId,
    String? ownerUuid,
    String? ownerName,
    String? description,
    int? sets,
    String? category,
    int? repsOrDuration,
    String? uuid,
    String? visibility,
    DateTime? createdAt,
    DateTime? updatedAt,
    double? weight,
    int? workoutId,
  }) {
    return Exercise(
      id: id ?? this.id,
      uuid: uuid ?? this.uuid,
      ownerId: ownerId ?? this.ownerId,
      ownerUuid: ownerUuid ?? this.ownerUuid,
      ownerName: ownerName ?? this.ownerName,
      name: name ?? this.name,
      description: description ?? this.description,
      sets: sets ?? this.sets,
      repsOrDuration: repsOrDuration ?? this.repsOrDuration,
      category: category ?? this.category,
      visibility: visibility ?? this.visibility,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      weight: weight ?? this.weight,
    );
  }
}
