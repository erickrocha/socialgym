class EvolutionCheckIn {
  final String uuid;
  final String personUuid;
  final DateTime createdAt;
  final String note;
  final String visibility;
  final EvolutionComposition? composition;
  final EvolutionCircumferences? circumferences;

  EvolutionCheckIn({
    required this.uuid,
    required this.personUuid,
    required this.createdAt,
    required this.note,
    required this.visibility,
    this.composition,
    this.circumferences,
  });

  factory EvolutionCheckIn.fromJson(Map<String, dynamic> json) {
    return EvolutionCheckIn(
      uuid: json['uuid'] ?? '',
      personUuid: json['personUuid'] ?? '',
      createdAt: json['createdAt'] != null
          ? DateTime.parse(json['createdAt'])
          : DateTime.now(),
      note: json['note'] ?? '',
      visibility: json['visibility'] ?? 'Private',
      composition: json['composition'] != null
          ? EvolutionComposition.fromJson(json['composition'])
          : null,
      circumferences: json['circumferences'] != null
          ? EvolutionCircumferences.fromJson(json['circumferences'])
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'uuid': uuid,
      'personUuid': personUuid,
      'createdAt': createdAt.toIso8601String(),
      'note': note,
      'visibility': visibility,
      'composition': composition?.toJson(),
      'circumferences': circumferences?.toJson(),
    };
  }
}

class EvolutionComposition {
  final double? weight;
  final double? bodyFatPct;
  final double? muscleMassPct;
  final double? visceralFat;

  EvolutionComposition({
    this.weight,
    this.bodyFatPct,
    this.muscleMassPct,
    this.visceralFat,
  });

  factory EvolutionComposition.fromJson(Map<String, dynamic> json) {
    return EvolutionComposition(
      weight: _parseDouble(json['weight']),
      bodyFatPct: _parseDouble(json['bodyFatPct']),
      muscleMassPct: _parseDouble(json['muscleMassPct']),
      visceralFat: _parseDouble(json['visceralFat']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'weight': weight,
      'bodyFatPct': bodyFatPct,
      'muscleMassPct': muscleMassPct,
      'visceralFat': visceralFat,
    };
  }
}

class EvolutionCircumferences {
  final double? neck;
  final double? chest;
  final double? waist;
  final double? abdomen;
  final double? hip;
  final double? bicepsRight;
  final double? bicepsLeft;
  final double? thighRight;
  final double? thighLeft;

  EvolutionCircumferences({
    this.neck,
    this.chest,
    this.waist,
    this.abdomen,
    this.hip,
    this.bicepsRight,
    this.bicepsLeft,
    this.thighRight,
    this.thighLeft,
  });

  factory EvolutionCircumferences.fromJson(Map<String, dynamic> json) {
    return EvolutionCircumferences(
      neck: _parseDouble(json['neck']),
      chest: _parseDouble(json['chest']),
      waist: _parseDouble(json['waist']),
      abdomen: _parseDouble(json['abdomen']),
      hip: _parseDouble(json['hip']),
      bicepsRight: _parseDouble(json['bicepsRight']),
      bicepsLeft: _parseDouble(json['bicepsLeft']),
      thighRight: _parseDouble(json['thighRight']),
      thighLeft: _parseDouble(json['thighLeft']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'neck': neck,
      'chest': chest,
      'waist': waist,
      'abdomen': abdomen,
      'hip': hip,
      'bicepsRight': bicepsRight,
      'bicepsLeft': bicepsLeft,
      'thighRight': thighRight,
      'thighLeft': thighLeft,
    };
  }
}

double? _parseDouble(dynamic value) {
  if (value == null) return null;
  if (value is double) return value;
  if (value is int) return value.toDouble();
  if (value is String) return double.tryParse(value);
  return null;
}
