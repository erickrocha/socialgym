import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/evolution_check_in.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../providers/evolution_provider.dart';
import '../../providers/person_provider.dart';
import '../../config/nav_section.dart';
import '../../widgets/main_layout.dart';
import '../../widgets/evolution/vitruvian_body_card.dart';
import 'evolution_checkin_form_dialog.dart';

// ---------------------------------------------------------------------------
// Filter period enum
// ---------------------------------------------------------------------------

enum _FilterPeriod { lastWeek, lastMonth, last6Months, custom }

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

class EvolutionPage extends StatefulWidget {
  const EvolutionPage({super.key});

  @override
  State<EvolutionPage> createState() => _EvolutionPageState();
}

class _EvolutionPageState extends State<EvolutionPage> {
  _FilterPeriod _selectedPeriod = _FilterPeriod.lastWeek;
  DateTime? _customStart;
  DateTime? _customEnd;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _load());
  }

  // ── Data loading ────────────────────────────────────────────────────────

  void _load() {
    final (start, end) = _dateRange();
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';

    context.read<EvolutionProvider>().fetchCheckIns(startDate: start, endDate: end, token: token);
  }

  (DateTime, DateTime) _dateRange() {
    final now = DateTime.now();
    final endOfToday = DateTime(now.year, now.month, now.day, 23, 59, 59);
    switch (_selectedPeriod) {
      case _FilterPeriod.lastWeek:
        final start = now.subtract(const Duration(days: 7));
        return (DateTime(start.year, start.month, start.day), endOfToday);
      case _FilterPeriod.lastMonth:
        final start = DateTime(now.year, now.month - 1, now.day);
        return (DateTime(start.year, start.month, start.day), endOfToday);
      case _FilterPeriod.last6Months:
        final start = DateTime(now.year, now.month - 6, now.day);
        return (DateTime(start.year, start.month, start.day), endOfToday);
      case _FilterPeriod.custom:
        final s = _customStart ?? now.subtract(const Duration(days: 7));
        final e = _customEnd ?? now;
        return (DateTime(s.year, s.month, s.day), DateTime(e.year, e.month, e.day, 23, 59, 59));
    }
  }

  void _selectPeriod(_FilterPeriod period) {
    setState(() => _selectedPeriod = period);
    _load();
  }

  Future<void> _pickCustomRange() async {
    final now = DateTime.now();
    final businessType = context.read<PersonProvider>().activeBusinessProfile?.businessType;
    final picked = await showDateRangePicker(
      context: context,
      firstDate: DateTime(2020),
      lastDate: now,
      initialDateRange: DateTimeRange(
        start: _customStart ?? now.subtract(const Duration(days: 30)),
        end: _customEnd ?? now,
      ),
      builder: (context, child) => Theme(
        data: Theme.of(
          context,
        ).copyWith(colorScheme: ColorScheme.light(primary: AppColors.primaryFor(businessType))),
        child: child!,
      ),
    );

    if (picked != null) {
      setState(() {
        _customStart = picked.start;
        _customEnd = picked.end;
        _selectedPeriod = _FilterPeriod.custom;
      });
      _load();
    }
  }

  Future<void> _showAddCheckInDialog() async {
    final l10n = AppLocalizations.of(context)!;
    final personUuid = context.read<PersonProvider>().person?.uuid;

    if (personUuid == null || personUuid.isEmpty) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text(l10n.messageUserDataNotLoaded)));
      return;
    }

    context.read<EvolutionProvider>().clearError();

    final created = await showDialog<bool>(
      context: context,
      builder: (_) => EvolutionCheckInFormDialog(personUuid: personUuid, l10n: l10n),
    );

    if (created == true && mounted) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text(l10n.evolutionCreatedSuccessfully)));
      _load();
    }
  }

  // ── Build ────────────────────────────────────────────────────────────────

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final personProvider = context.watch<PersonProvider>();
    final person = personProvider.person;
    final personInfo = person?.personInfo;
    final gender = person?.gender;
    final businessType = personProvider.activeBusinessProfile?.businessType;

    return MainLayout(
      navSection: NavSection.workout,
      currentRoute: '/evolution',
      floatingActionButton: FloatingActionButton(
        onPressed: _showAddCheckInDialog,
        backgroundColor: AppColors.primaryFor(businessType),
        foregroundColor: Colors.white,
        tooltip: l10n.evolutionAddCheckin,
        child: const Icon(Icons.add),
      ),
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildHeader(l10n),
          _buildFilterRow(l10n, businessType),
          Expanded(
            child: Consumer<EvolutionProvider>(
              builder: (context, provider, _) {
                if (provider.loading) {
                  return const Center(child: CircularProgressIndicator());
                }
                if (provider.error != null) {
                  return _buildError(provider.error!, businessType);
                }
                if (provider.checkins.isEmpty) {
                  return _buildEmpty(l10n);
                }
                return _buildContent(l10n, provider.checkins, personInfo, gender, businessType);
              },
            ),
          ),
        ],
      ),
    );
  }

  // ── Header ──────────────────────────────────────────────────────────────

  Widget _buildHeader(AppLocalizations l10n) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(16, 20, 16, 8),
      decoration: const BoxDecoration(gradient: AppColors.gradient3),
      child: Text(
        l10n.evolutionTitle,
        style: const TextStyle(color: Colors.white, fontSize: 22, fontWeight: FontWeight.bold),
      ),
    );
  }

  // ── Filter row ──────────────────────────────────────────────────────────

  Widget _buildFilterRow(AppLocalizations l10n, String? businessType) {
    final customLabel =
        _selectedPeriod == _FilterPeriod.custom && _customStart != null && _customEnd != null
        ? '${_fmtDate(_customStart!)} – ${_fmtDate(_customEnd!)}'
        : l10n.evolutionFilterCustom;

    return Container(
      color: Colors.white,
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          children: [
            _PeriodChip(
              label: l10n.evolutionFilterLastWeek,
              selected: _selectedPeriod == _FilterPeriod.lastWeek,
              onTap: () => _selectPeriod(_FilterPeriod.lastWeek),
              businessType: businessType,
            ),
            const SizedBox(width: 8),
            _PeriodChip(
              label: l10n.evolutionFilterLastMonth,
              selected: _selectedPeriod == _FilterPeriod.lastMonth,
              onTap: () => _selectPeriod(_FilterPeriod.lastMonth),
              businessType: businessType,
            ),
            const SizedBox(width: 8),
            _PeriodChip(
              label: l10n.evolutionFilterLast6Months,
              selected: _selectedPeriod == _FilterPeriod.last6Months,
              onTap: () => _selectPeriod(_FilterPeriod.last6Months),
              businessType: businessType,
            ),
            const SizedBox(width: 8),
            _PeriodChip(
              label: customLabel,
              selected: _selectedPeriod == _FilterPeriod.custom,
              icon: Icons.calendar_month_outlined,
              onTap: _pickCustomRange,
              businessType: businessType,
            ),
          ],
        ),
      ),
    );
  }

  // ── Main content ────────────────────────────────────────────────────────

  Widget _buildContent(
    AppLocalizations l10n,
    List<EvolutionCheckIn> checkins,
    PersonInfo? personInfo,
    String? gender,
    String? businessType,
  ) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          VitruvianBodyCard(personInfo: personInfo, gender: gender, checkins: checkins),
          const SizedBox(height: 20),
          if (checkins.isNotEmpty) ...[
            _buildLatestMetrics(l10n, checkins.first, businessType),
            const SizedBox(height: 20),
          ],
          Text(
            l10n.evolutionCheckinsTitle,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Color(0xFF333333),
            ),
          ),
          const SizedBox(height: 8),
          ...checkins.map((checkin) => _CheckinCard(checkin: checkin, businessType: businessType)),
          const SizedBox(height: 24),
        ],
      ),
    );
  }

  // ── Latest metrics ──────────────────────────────────────────────────────

  Widget _buildLatestMetrics(
    AppLocalizations l10n,
    EvolutionCheckIn latestCheckin,
    String? businessType,
  ) {
    return Container(
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
            l10n.evolutionLatestMetrics,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Color(0xFF333333),
            ),
          ),
          const SizedBox(height: 12),
          if (latestCheckin.composition != null) ...[
            _buildMetricRow(
              l10n.evolutionCompositionWeight,
              latestCheckin.composition!.weight,
              'kg',
              businessType,
              decimalPlaces: 3,
            ),
            _buildMetricRow(
              l10n.evolutionCompositionBodyFat,
              latestCheckin.composition!.bodyFatPct,
              '%',
              businessType,
            ),
            _buildMetricRow(
              l10n.evolutionCompositionMuscleMass,
              latestCheckin.composition!.muscleMassPct,
              '%',
              businessType,
            ),
            _buildMetricRow(
              l10n.evolutionCompositionVisceralFat,
              latestCheckin.composition!.visceralFat,
              '%',
              businessType,
            ),
            const SizedBox(height: 12),
          ],
          if (latestCheckin.circumferences != null) ...[
            _buildMetricRow(
              l10n.evolutionCircumferenceChest,
              latestCheckin.circumferences!.chest,
              'cm',
              businessType,
            ),
            _buildMetricRow(
              l10n.evolutionCircumferenceWaist,
              latestCheckin.circumferences!.waist,
              'cm',
              businessType,
            ),
            _buildMetricRow(
              l10n.evolutionCircumferenceHip,
              latestCheckin.circumferences!.hip,
              'cm',
              businessType,
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildMetricRow(
    String label,
    double? value,
    String unit,
    String? businessType, {
    int decimalPlaces = 1,
  }) {
    if (value == null) return const SizedBox.shrink();
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(fontSize: 13, color: Color(0xFF666666))),
          Text(
            '${value.toStringAsFixed(decimalPlaces)} $unit',
            style: TextStyle(
              fontSize: 13,
              fontWeight: FontWeight.bold,
              color: AppColors.primaryFor(businessType),
            ),
          ),
        ],
      ),
    );
  }

  // ── Empty / Error ───────────────────────────────────────────────────────

  Widget _buildEmpty(AppLocalizations l10n) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.trending_up, size: 64, color: Colors.grey.withAlpha(100)),
            const SizedBox(height: 16),
            Text(
              l10n.evolutionNoData,
              textAlign: TextAlign.center,
              style: const TextStyle(color: Color(0xFF888888), fontSize: 15),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildError(String msg, String? businessType) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 48, color: AppColors.danger),
            const SizedBox(height: 12),
            Text(
              msg,
              textAlign: TextAlign.center,
              style: const TextStyle(color: AppColors.danger),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _load,
              style: ElevatedButton.styleFrom(
                backgroundColor: AppColors.primaryFor(businessType),
                foregroundColor: Colors.white,
              ),
              child: Text(context.read<AppLocalizations>().buttonRetry),
            ),
          ],
        ),
      ),
    );
  }

  // ── Utilities ───────────────────────────────────────────────────────────

  String _fmtDate(DateTime d) =>
      '${d.day.toString().padLeft(2, '0')}/${d.month.toString().padLeft(2, '0')}';
}

// ---------------------------------------------------------------------------
// Filter chip
// ---------------------------------------------------------------------------

class _PeriodChip extends StatelessWidget {
  final String label;
  final bool selected;
  final IconData? icon;
  final VoidCallback onTap;
  final String? businessType;

  const _PeriodChip({
    required this.label,
    required this.selected,
    required this.onTap,
    this.icon,
    this.businessType,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
        decoration: BoxDecoration(
          color: selected ? AppColors.primaryFor(businessType) : Colors.transparent,
          borderRadius: BorderRadius.circular(20),
          border: Border.all(
            color: selected ? AppColors.primaryFor(businessType) : const Color(0xFFCCCCCC),
          ),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (icon != null) ...[
              Icon(icon, size: 14, color: selected ? Colors.white : const Color(0xFF666666)),
              const SizedBox(width: 4),
            ],
            Text(
              label,
              style: TextStyle(
                fontSize: 13,
                fontWeight: FontWeight.w500,
                color: selected ? Colors.white : const Color(0xFF444444),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Checkin card
// ---------------------------------------------------------------------------

class _CheckinCard extends StatelessWidget {
  final EvolutionCheckIn checkin;
  final String? businessType;

  const _CheckinCard({required this.checkin, this.businessType});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final dateStr = checkin.createdAt.toString().split('T')[0];

    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 2)),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Text(
                  checkin.note.isNotEmpty ? checkin.note : l10n.evolutionNoNote,
                  style: const TextStyle(
                    fontSize: 15,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF333333),
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              Text(dateStr, style: const TextStyle(fontSize: 12, color: Color(0xFF888888))),
            ],
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              if (checkin.composition?.weight != null)
                _ValueBadge(
                  label: l10n.evolutionCompositionWeight,
                  value: '${checkin.composition!.weight?.toStringAsFixed(3) ?? '--'} kg',
                  businessType: businessType,
                ),
              const SizedBox(width: 8),
              if (checkin.composition?.bodyFatPct != null)
                _ValueBadge(
                  label: l10n.evolutionCompositionBodyFat,
                  value: '${checkin.composition!.bodyFatPct?.toStringAsFixed(1) ?? '--'}%',
                  businessType: businessType,
                ),
            ],
          ),
        ],
      ),
    );
  }
}

class _ValueBadge extends StatelessWidget {
  final String label;
  final String value;
  final String? businessType;

  const _ValueBadge({required this.label, required this.value, this.businessType});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: AppColors.primaryFor(businessType).withAlpha(20),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: AppColors.primaryFor(businessType).withAlpha(60)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label, style: const TextStyle(fontSize: 10, color: Color(0xFF666666))),
          Text(
            value,
            style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.bold,
              color: AppColors.primaryFor(businessType),
            ),
          ),
        ],
      ),
    );
  }
}
