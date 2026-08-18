import 'package:flutter/material.dart';
import 'package:flutter_3d_controller/flutter_3d_controller.dart';

import '../../l10n/app_localizations.dart';
import '../../models/evolution_check_in.dart';
import '../../models/person.dart';
import '../../utils/evolution_body_progress_mapper.dart';
import '../../utils/evolution_body_state_mapper.dart';

class VitruvianBodyCard extends StatefulWidget {
  final PersonInfo? personInfo;
  final String? gender;
  final List<EvolutionCheckIn> checkins;

  const VitruvianBodyCard({
    super.key,
    required this.personInfo,
    required this.gender,
    required this.checkins,
  });

  @override
  State<VitruvianBodyCard> createState() => _VitruvianBodyCardState();
}

class _VitruvianBodyCardState extends State<VitruvianBodyCard> {
  final Flutter3DController _controller = Flutter3DController();

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final progress = computeVitruvianProgress(
      checkins: widget.checkins,
      personInfo: widget.personInfo,
      viewMode: VitruvianViewMode.front,
    );

    if (!progress.hasRenderableData) {
      return const SizedBox.shrink();
    }

    final bodyState = computeVitruvianBodyState(
      checkins: widget.checkins,
      personInfo: widget.personInfo,
      gender: widget.gender,
    );

    final entries = progress.progressByRegion.entries.toList()
      ..sort((a, b) => b.value.intensity.compareTo(a.value.intensity));

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 2)),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            l10n.evolutionVitruvianTitle,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Color(0xFF333333),
            ),
          ),
          const SizedBox(height: 4),
          Text(
            _subtitle(progress, l10n),
            style: const TextStyle(fontSize: 12, color: Color(0xFF6B7280)),
          ),
          if (!bodyState.hasGenderData) ...[
            const SizedBox(height: 4),
            Text(
              l10n.evolutionVitruvianSetGenderHint,
              style: const TextStyle(fontSize: 11, color: Color(0xFF9CA3AF)),
            ),
          ],
          const SizedBox(height: 10),
          _buildModel(bodyState),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: entries
                .map(
                  (entry) => _RegionBadge(
                    label: _regionLabel(entry.key, l10n),
                    value: (entry.value.intensity * 100).round(),
                    color: _displayColorForIntensity(entry.value.intensity),
                  ),
                )
                .toList(),
          ),
          if (progress.usedInterpolation) ...[
            const SizedBox(height: 8),
            Text(
              l10n.evolutionVitruvianInterpolationNote,
              style: const TextStyle(fontSize: 11, color: Color(0xFF2FA15F)),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildModel(VitruvianBodyState bodyState) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: SizedBox(
        height: 340,
        width: double.infinity,
        child: Transform.scale(
          scale: bodyState.scale,
          child: Flutter3DViewer(
            key: ValueKey(bodyState.assetPath),
            controller: _controller,
            src: bodyState.assetPath,
            enableTouch: true,
            activeGestureInterceptor: true,
          ),
        ),
      ),
    );
  }

  static String _regionLabel(VitruvianRegion region, AppLocalizations l10n) {
    switch (region) {
      case VitruvianRegion.shoulders:
        return l10n.evolutionVitruvianShoulders;
      case VitruvianRegion.chest:
        return l10n.evolutionCircumferenceChest;
      case VitruvianRegion.arms:
        return l10n.evolutionVitruvianArms;
      case VitruvianRegion.abdomen:
        return l10n.evolutionVitruvianAbdomen;
      case VitruvianRegion.waist:
        return l10n.evolutionCircumferenceWaist;
      case VitruvianRegion.hips:
        return l10n.evolutionVitruvianHips;
      case VitruvianRegion.thighs:
        return l10n.evolutionVitruvianThighs;
      case VitruvianRegion.neck:
        return l10n.evolutionCircumferenceNeck;
      default:
        return region.name;
    }
  }

  static Color _displayColorForIntensity(double intensity) {
    return Color.lerp(
          const Color(0xFFD9C5A3),
          const Color(0xFF2FA15F),
          intensity.clamp(0.0, 1.0),
        ) ??
        const Color(0xFFD9C5A3);
  }

  static String _subtitle(VitruvianProgressData progress, AppLocalizations l10n) {
    final baseline = progress.baselineDate;
    final latest = progress.latestDate;
    final source = progress.baselineSource == BaselineSource.profileThenCheckinFallback
        ? l10n.evolutionVitruvianBaselineProfileCheckin
        : l10n.evolutionVitruvianBaselineCheckinOnly;

    if (baseline == null || latest == null) {
      return source;
    }

    return '$source | ${_formatDate(baseline)} -> ${_formatDate(latest)}';
  }

  static String _formatDate(DateTime date) {
    final y = date.year.toString().padLeft(4, '0');
    final m = date.month.toString().padLeft(2, '0');
    final d = date.day.toString().padLeft(2, '0');
    return '$y-$m-$d';
  }
}

class _RegionBadge extends StatelessWidget {
  final String label;
  final int value;
  final Color color;

  const _RegionBadge({required this.label, required this.value, required this.color});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      decoration: BoxDecoration(
        color: color.withAlpha(45),
        borderRadius: BorderRadius.circular(999),
        border: Border.all(color: color.withAlpha(160)),
      ),
      child: Text(
        '$label $value%',
        style: const TextStyle(fontSize: 11, fontWeight: FontWeight.w600, color: Color(0xFF1F2937)),
      ),
    );
  }
}
