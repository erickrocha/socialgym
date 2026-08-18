import '../models/evolution_check_in.dart';
import '../models/person.dart';

enum BodyLevel { low, medium, high }

class VitruvianBodyState {
  final String gender;
  final BodyLevel fatBucket;
  final BodyLevel muscleBucket;
  final double scale;
  final bool hasCompositionData;
  final bool hasGenderData;

  const VitruvianBodyState({
    required this.gender,
    required this.fatBucket,
    required this.muscleBucket,
    required this.scale,
    required this.hasCompositionData,
    required this.hasGenderData,
  });

  String get assetPath => 'assets/vitruvian3d/${gender}_${fatBucket.name}_${muscleBucket.name}.glb';
}

const double _minScale = 0.9;
const double _maxScale = 1.15;
const double _bmiLow = 18.5;
const double _bmiHigh = 32.0;

VitruvianBodyState computeVitruvianBodyState({
  required List<EvolutionCheckIn> checkins,
  PersonInfo? personInfo,
  String? gender,
}) {
  final hasGenderData = gender != null && gender.trim().isNotEmpty;
  final normalizedGender = hasGenderData && gender.trim().toLowerCase() == 'female'
      ? 'female'
      : 'male';

  if (checkins.isEmpty) {
    return VitruvianBodyState(
      gender: normalizedGender,
      fatBucket: BodyLevel.medium,
      muscleBucket: BodyLevel.medium,
      scale: 1.0,
      hasCompositionData: false,
      hasGenderData: hasGenderData,
    );
  }

  final ordered = [...checkins]..sort((a, b) => a.createdAt.compareTo(b.createdAt));
  final latest = ordered.last;

  final bodyFatPct = latest.composition?.bodyFatPct;
  final muscleMassPct = latest.composition?.muscleMassPct;
  final hasCompositionData = bodyFatPct != null && muscleMassPct != null;

  final weight = latest.composition?.weight ?? personInfo?.weight;
  final height = personInfo?.height;

  return VitruvianBodyState(
    gender: normalizedGender,
    fatBucket: _fatBucketFor(normalizedGender, bodyFatPct),
    muscleBucket: _muscleBucketFor(normalizedGender, muscleMassPct),
    scale: _scaleFor(weight: weight, heightCm: height),
    hasCompositionData: hasCompositionData,
    hasGenderData: hasGenderData,
  );
}

BodyLevel _fatBucketFor(String gender, double? bodyFatPct) {
  if (bodyFatPct == null) return BodyLevel.medium;
  if (gender == 'female') {
    if (bodyFatPct < 22) return BodyLevel.low;
    if (bodyFatPct <= 32) return BodyLevel.medium;
    return BodyLevel.high;
  }
  if (bodyFatPct < 15) return BodyLevel.low;
  if (bodyFatPct <= 25) return BodyLevel.medium;
  return BodyLevel.high;
}

BodyLevel _muscleBucketFor(String gender, double? muscleMassPct) {
  if (muscleMassPct == null) return BodyLevel.medium;
  if (gender == 'female') {
    if (muscleMassPct < 24) return BodyLevel.low;
    if (muscleMassPct <= 30) return BodyLevel.medium;
    return BodyLevel.high;
  }
  if (muscleMassPct < 33) return BodyLevel.low;
  if (muscleMassPct <= 38) return BodyLevel.medium;
  return BodyLevel.high;
}

double _scaleFor({double? weight, double? heightCm}) {
  if (weight == null || heightCm == null || heightCm <= 0) return 1.0;
  final heightM = heightCm / 100;
  final bmi = weight / (heightM * heightM);
  final t = ((bmi - _bmiLow) / (_bmiHigh - _bmiLow)).clamp(0.0, 1.0);
  return _minScale + t * (_maxScale - _minScale);
}
