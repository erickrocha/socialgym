import '../models/evolution_check_in.dart';
import '../models/person.dart';

enum VitruvianRegion {
  // Front view regions
  shoulders,
  chest,
  arms,
  abdomen,
  waist,
  hips,
  thighs,
  neck,
  // Back view regions
  shouldersBack,
  chestBack,
  armsBack,
  back,
  triceps,
  glutes,
  calves,
}

enum VitruvianViewMode { front, back }

enum BaselineSource { profileThenCheckinFallback, checkinOnly }

class VitruvianRegionProgress {
  final double intensity;
  final bool interpolated;
  final int metricCount;

  const VitruvianRegionProgress({
    required this.intensity,
    required this.interpolated,
    required this.metricCount,
  });
}

class VitruvianProgressData {
  final Map<VitruvianRegion, VitruvianRegionProgress> progressByRegion;
  final DateTime? baselineDate;
  final DateTime? latestDate;
  final BaselineSource baselineSource;
  final bool usedInterpolation;

  const VitruvianProgressData({
    required this.progressByRegion,
    required this.baselineDate,
    required this.latestDate,
    required this.baselineSource,
    required this.usedInterpolation,
  });

  bool get hasRenderableData => progressByRegion.isNotEmpty;
}

class _MetricReading {
  final double value;
  final bool interpolated;

  const _MetricReading(this.value, {this.interpolated = false});
}

const Map<String, double> _metricScale = {
  'weight': 10,
  'bodyFatPct': 8,
  'muscleMassPct': 6,
  'visceralFat': 5,
  'neck': 5,
  'chest': 12,
  'waist': 12,
  'abdomen': 12,
  'hip': 12,
  'bicepsRight': 6,
  'bicepsLeft': 6,
  'thighRight': 8,
  'thighLeft': 8,
};

const List<String> _allMetrics = [
  'weight',
  'bodyFatPct',
  'muscleMassPct',
  'visceralFat',
  'neck',
  'chest',
  'waist',
  'abdomen',
  'hip',
  'bicepsRight',
  'bicepsLeft',
  'thighRight',
  'thighLeft',
];

VitruvianProgressData computeVitruvianProgress({
  required List<EvolutionCheckIn> checkins,
  PersonInfo? personInfo,
  VitruvianViewMode viewMode = VitruvianViewMode.front,
}) {
  if (checkins.isEmpty) {
    return const VitruvianProgressData(
      progressByRegion: {},
      baselineDate: null,
      latestDate: null,
      baselineSource: BaselineSource.profileThenCheckinFallback,
      usedInterpolation: false,
    );
  }

  final ordered = [...checkins]..sort((a, b) => a.createdAt.compareTo(b.createdAt));
  final latest = ordered.last;

  final baselineReadings = <String, _MetricReading>{};
  if (personInfo?.weight != null) {
    baselineReadings['weight'] = _MetricReading(personInfo!.weight!);
  }

  for (final metric in _allMetrics) {
    if (baselineReadings.containsKey(metric)) continue;
    final firstKnown = _interpolateAtTarget(
      ordered: ordered,
      metric: metric,
      targetDate: ordered.first.createdAt,
    );
    if (firstKnown != null) {
      baselineReadings[metric] = firstKnown;
    }
  }

  final latestReadings = <String, _MetricReading>{};
  for (final metric in _allMetrics) {
    final latestReading = _interpolateAtTarget(
      ordered: ordered,
      metric: metric,
      targetDate: latest.createdAt,
    );
    if (latestReading != null) {
      latestReadings[metric] = latestReading;
    }
  }

  final progressMap = <VitruvianRegion, VitruvianRegionProgress>{};
  var usedInterpolation = false;

  VitruvianRegionProgress? buildRegion(List<String> metrics) {
    var count = 0;
    var sumIntensity = 0.0;
    var interpolated = false;

    for (final key in metrics) {
      final baseline = baselineReadings[key];
      final latestValue = latestReadings[key];
      if (baseline == null || latestValue == null) continue;
      final scale = _metricScale[key] ?? 10;
      final delta = (latestValue.value - baseline.value).abs();
      sumIntensity += (delta / scale).clamp(0.0, 1.0);
      count += 1;
      interpolated = interpolated || latestValue.interpolated;
    }

    if (count == 0) return null;
    return VitruvianRegionProgress(
      intensity: (sumIntensity / count).clamp(0.0, 1.0),
      interpolated: interpolated,
      metricCount: count,
    );
  }

  final definitions = _getRegionDefinitions(viewMode);

  definitions.forEach((region, metrics) {
    final progress = buildRegion(metrics);
    if (progress != null) {
      progressMap[region] = progress;
      usedInterpolation = usedInterpolation || progress.interpolated;
    }
  });

  return VitruvianProgressData(
    progressByRegion: progressMap,
    baselineDate: ordered.first.createdAt,
    latestDate: latest.createdAt,
    baselineSource: personInfo?.weight != null
        ? BaselineSource.profileThenCheckinFallback
        : BaselineSource.checkinOnly,
    usedInterpolation: usedInterpolation,
  );
}

/// Get region definitions based on view mode
Map<VitruvianRegion, List<String>> _getRegionDefinitions(VitruvianViewMode viewMode) {
  if (viewMode == VitruvianViewMode.back) {
    return <VitruvianRegion, List<String>>{
      VitruvianRegion.shouldersBack: const ['neck', 'chest', 'muscleMassPct'],
      VitruvianRegion.chestBack: const ['chest', 'muscleMassPct'],
      VitruvianRegion.back: const ['waist', 'abdomen', 'bodyFatPct', 'weight'],
      VitruvianRegion.triceps: const ['bicepsRight', 'bicepsLeft', 'muscleMassPct'],
      VitruvianRegion.armsBack: const ['bicepsRight', 'bicepsLeft', 'weight'],
      VitruvianRegion.glutes: const ['hip', 'thighRight', 'thighLeft', 'weight'],
      VitruvianRegion.calves: const ['thighRight', 'thighLeft', 'weight'],
    };
  }

  // Front view (default)
  return <VitruvianRegion, List<String>>{
    VitruvianRegion.shoulders: const ['neck', 'chest', 'muscleMassPct'],
    VitruvianRegion.chest: const ['chest', 'muscleMassPct', 'bodyFatPct'],
    VitruvianRegion.arms: const ['bicepsRight', 'bicepsLeft', 'muscleMassPct'],
    VitruvianRegion.abdomen: const ['abdomen', 'bodyFatPct', 'visceralFat'],
    VitruvianRegion.waist: const ['waist', 'abdomen', 'bodyFatPct', 'weight'],
    VitruvianRegion.hips: const ['hip', 'weight'],
    VitruvianRegion.thighs: const ['thighRight', 'thighLeft', 'weight'],
    VitruvianRegion.neck: const ['neck', 'bodyFatPct'],
  };
}

_MetricReading? _interpolateAtTarget({
  required List<EvolutionCheckIn> ordered,
  required String metric,
  required DateTime targetDate,
}) {
  final known = <(DateTime date, double value)>[];
  for (final checkin in ordered) {
    final value = _metricValue(checkin, metric);
    if (value != null) {
      known.add((checkin.createdAt, value));
    }
  }

  if (known.isEmpty) return null;

  (DateTime date, double value)? exact;
  (DateTime date, double value)? previous;
  (DateTime date, double value)? next;

  for (final sample in known) {
    if (sample.$1.isAtSameMomentAs(targetDate)) {
      exact = sample;
      break;
    }
    if (sample.$1.isBefore(targetDate)) {
      previous = sample;
      continue;
    }
    next = sample;
    break;
  }

  if (exact != null) return _MetricReading(exact.$2);

  if (previous != null && next != null) {
    final total = next.$1.difference(previous.$1).inMilliseconds;
    if (total <= 0) return _MetricReading(previous.$2, interpolated: true);
    final elapsed = targetDate.difference(previous.$1).inMilliseconds;
    final t = (elapsed / total).clamp(0.0, 1.0);
    final value = previous.$2 + (next.$2 - previous.$2) * t;
    return _MetricReading(value, interpolated: true);
  }

  if (previous != null) return _MetricReading(previous.$2, interpolated: true);
  if (next != null) return _MetricReading(next.$2, interpolated: true);

  return null;
}

double? _metricValue(EvolutionCheckIn checkin, String metric) {
  switch (metric) {
    case 'weight':
      return checkin.composition?.weight;
    case 'bodyFatPct':
      return checkin.composition?.bodyFatPct;
    case 'muscleMassPct':
      return checkin.composition?.muscleMassPct;
    case 'visceralFat':
      return checkin.composition?.visceralFat;
    case 'neck':
      return checkin.circumferences?.neck;
    case 'chest':
      return checkin.circumferences?.chest;
    case 'waist':
      return checkin.circumferences?.waist;
    case 'abdomen':
      return checkin.circumferences?.abdomen;
    case 'hip':
      return checkin.circumferences?.hip;
    case 'bicepsRight':
      return checkin.circumferences?.bicepsRight;
    case 'bicepsLeft':
      return checkin.circumferences?.bicepsLeft;
    case 'thighRight':
      return checkin.circumferences?.thighRight;
    case 'thighLeft':
      return checkin.circumferences?.thighLeft;
    default:
      return null;
  }
}
