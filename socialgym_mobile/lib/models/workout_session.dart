class WorkoutSession {
  final String uuid;
  final String personUuid;
  final String workoutName;
  final int duration;
  final String startedAt;
  final String completedAt;
  final String dayOfWeek;
  final double totalVolume;
  final int totalSets;
  final List<Map<String, dynamic>> executedSets;

  const WorkoutSession({
    required this.uuid,
    required this.personUuid,
    required this.workoutName,
    required this.duration,
    required this.startedAt,
    required this.completedAt,
    required this.dayOfWeek,
    required this.totalVolume,
    required this.totalSets,
    required this.executedSets,
  });

  factory WorkoutSession.fromJson(Map<String, dynamic> json) {
    final rawSets = json['executedSets'];
    final sets = <Map<String, dynamic>>[];
    if (rawSets is List) {
      for (final s in rawSets) {
        if (s is Map) sets.add(Map<String, dynamic>.from(s));
      }
    }

    return WorkoutSession(
      uuid: json['uuid'] as String? ?? '',
      personUuid: json['personUuid'] as String? ?? '',
      workoutName: json['workoutName'] as String? ?? '',
      duration: (json['duration'] as num?)?.toInt() ?? 0,
      startedAt: json['startedAt'] as String? ?? '',
      completedAt: json['completedAt'] as String? ?? '',
      dayOfWeek: json['dayOfWeek'] as String? ?? '',
      totalVolume: (json['totalVolume'] as num?)?.toDouble() ?? 0.0,
      totalSets: (json['totalSets'] as num?)?.toInt() ?? 0,
      executedSets: sets,
    );
  }

  Map<String, dynamic> toJson() => {
    'uuid': uuid,
    'personUuid': personUuid,
    'workoutName': workoutName,
    'duration': duration,
    'startedAt': startedAt,
    'completedAt': completedAt,
    'dayOfWeek': dayOfWeek,
    'totalVolume': totalVolume,
    'totalSets': totalSets,
    'executedSets': executedSets,
  };

  /// Duration formatted as mm:ss
  String get formattedDuration {
    final mins = duration ~/ 60;
    final secs = duration % 60;
    return '$mins:${secs.toString().padLeft(2, '0')}';
  }

  /// Completion date parsed from completedAt ISO string, null if invalid.
  DateTime? get completedAtDate {
    try {
      return DateTime.parse(completedAt);
    } catch (_) {
      return null;
    }
  }
}
