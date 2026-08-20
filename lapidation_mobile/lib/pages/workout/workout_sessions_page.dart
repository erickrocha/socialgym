import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/workout_session.dart';
import '../../providers/auth_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/workout_session_provider.dart';
import '../../config/nav_section.dart';
import '../../widgets/main_layout.dart';
import 'workout_session_detail_page.dart';

// ---------------------------------------------------------------------------
// Filter period enum
// ---------------------------------------------------------------------------

enum _FilterPeriod { lastWeek, lastMonth, last3Months, custom }

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

class WorkoutSessionsPage extends StatefulWidget {
  const WorkoutSessionsPage({super.key});

  @override
  State<WorkoutSessionsPage> createState() => _WorkoutSessionsPageState();
}

class _WorkoutSessionsPageState extends State<WorkoutSessionsPage> {
  _FilterPeriod _selectedPeriod = _FilterPeriod.lastWeek;
  DateTime? _customStart;
  DateTime? _customEnd;

  @override
  void initState() {
    super.initState();
    // Defer until the first frame so providers are available
    WidgetsBinding.instance.addPostFrameCallback((_) => _load());
  }

  // ── Data loading ────────────────────────────────────────────────────────────

  void _load() {
    final (start, end) = _dateRange();
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    final personUuid = context.read<PersonProvider>().person?.uuid ?? '';

    context.read<WorkoutSessionProvider>().fetchSessions(
      personUuid: personUuid,
      startDate: start,
      endDate: end,
      token: token,
    );
  }

  (DateTime, DateTime) _dateRange() {
    final now = DateTime.now();
    // endDate is always the very end of today
    final endOfToday = DateTime(now.year, now.month, now.day, 23, 59, 59);
    switch (_selectedPeriod) {
      case _FilterPeriod.lastWeek:
        final start = now.subtract(const Duration(days: 7));
        return (DateTime(start.year, start.month, start.day), endOfToday);
      case _FilterPeriod.lastMonth:
        final start = DateTime(now.year, now.month - 1, now.day);
        return (DateTime(start.year, start.month, start.day), endOfToday);
      case _FilterPeriod.last3Months:
        final start = DateTime(now.year, now.month - 3, now.day);
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
        ).copyWith(colorScheme: const ColorScheme.light(primary: AppColors.primary)),
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

  // ── Build ────────────────────────────────────────────────────────────────────

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return MainLayout(
      navSection: NavSection.workout,
      currentRoute: '/workout-sessions',
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildHeader(l10n),
          _buildFilterRow(l10n),
          Expanded(
            child: Consumer<WorkoutSessionProvider>(
              builder: (context, provider, _) {
                if (provider.fetching) {
                  return const Center(child: CircularProgressIndicator());
                }
                if (provider.fetchError != null) {
                  return _buildError(provider.fetchError!);
                }
                if (provider.sessions.isEmpty) {
                  return _buildEmpty(l10n);
                }
                return _buildContent(l10n, provider.sessions);
              },
            ),
          ),
        ],
      ),
    );
  }

  // ── Header ───────────────────────────────────────────────────────────────────

  Widget _buildHeader(AppLocalizations l10n) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(16, 20, 16, 8),
      decoration: const BoxDecoration(gradient: AppColors.gradient3),
      child: Text(
        l10n.sessionsTitle,
        style: const TextStyle(color: Colors.white, fontSize: 22, fontWeight: FontWeight.bold),
      ),
    );
  }

  // ── Filter row ───────────────────────────────────────────────────────────────

  Widget _buildFilterRow(AppLocalizations l10n) {
    final customLabel =
        _selectedPeriod == _FilterPeriod.custom && _customStart != null && _customEnd != null
        ? '${_fmtDate(_customStart!)} – ${_fmtDate(_customEnd!)}'
        : l10n.sessionsFilterCustom;

    return Container(
      color: Colors.white,
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          children: [
            _PeriodChip(
              label: l10n.sessionsFilterLastWeek,
              selected: _selectedPeriod == _FilterPeriod.lastWeek,
              onTap: () => _selectPeriod(_FilterPeriod.lastWeek),
            ),
            const SizedBox(width: 8),
            _PeriodChip(
              label: l10n.sessionsFilterLastMonth,
              selected: _selectedPeriod == _FilterPeriod.lastMonth,
              onTap: () => _selectPeriod(_FilterPeriod.lastMonth),
            ),
            const SizedBox(width: 8),
            _PeriodChip(
              label: l10n.sessionsFilterLast3Months,
              selected: _selectedPeriod == _FilterPeriod.last3Months,
              onTap: () => _selectPeriod(_FilterPeriod.last3Months),
            ),
            const SizedBox(width: 8),
            _PeriodChip(
              label: customLabel,
              selected: _selectedPeriod == _FilterPeriod.custom,
              icon: Icons.calendar_month_outlined,
              onTap: _pickCustomRange,
            ),
          ],
        ),
      ),
    );
  }

  // ── Main content ─────────────────────────────────────────────────────────────

  Widget _buildContent(AppLocalizations l10n, List<WorkoutSession> sessions) {
    final totalVolume = sessions.fold<double>(0, (acc, s) => acc + s.totalVolume);
    final totalSets = sessions.fold<int>(0, (acc, s) => acc + s.totalSets);
    final avgDuration = sessions.isEmpty
        ? 0
        : sessions.fold<int>(0, (acc, s) => acc + s.duration) ~/ sessions.length;

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Summary cards
          _buildSummaryRow(l10n, sessions.length, totalVolume, totalSets, avgDuration, sessions),
          const SizedBox(height: 20),

          // Bar chart
          _buildBarChart(l10n, sessions),
          const SizedBox(height: 20),

          // Session list
          Text(
            l10n.sessionsTitle,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Color(0xFF333333),
            ),
          ),
          const SizedBox(height: 8),
          ...sessions.map(
            (s) => _SessionCard(
              session: s,
              onTap: () => Navigator.of(
                context,
              ).push(MaterialPageRoute(builder: (_) => WorkoutSessionDetailPage(session: s))),
            ),
          ),
          const SizedBox(height: 24),
        ],
      ),
    );
  }

  // ── Summary row ──────────────────────────────────────────────────────────────

  Widget _buildSummaryRow(
    AppLocalizations l10n,
    int count,
    double totalVolume,
    int totalSets,
    int avgDuration,
    List<WorkoutSession> sessions,
  ) {
    final mins = avgDuration ~/ 60;
    final secs = avgDuration % 60;
    final avgDurationStr = '$mins:${secs.toString().padLeft(2, '0')}';

    return Row(
      children: [
        _SummaryCard(
          icon: Icons.fitness_center,
          value: '$count',
          label: l10n.sessionsCount,
          color: AppColors.primary,
        ),
        const SizedBox(width: 8),
        _SummaryCard(
          icon: Icons.bar_chart,
          value: '${totalVolume.toStringAsFixed(3)} kg',
          label: l10n.executionTotalVolume,
          color: AppColors.secondary,
        ),
        const SizedBox(width: 8),
        _SummaryCard(
          icon: Icons.timer_outlined,
          value: avgDurationStr,
          label: l10n.executionDuration,
          color: AppColors.third,
        ),
      ],
    );
  }

  // ── Bar chart ─────────────────────────────────────────────────────────────────

  Widget _buildBarChart(AppLocalizations l10n, List<WorkoutSession> sessions) {
    // Limit chart to last 10 sessions for readability
    final displayed = sessions.length > 10 ? sessions.sublist(sessions.length - 10) : sessions;

    final maxVolume = displayed.fold<double>(0, (m, s) => s.totalVolume > m ? s.totalVolume : m);
    final maxY = maxVolume <= 0 ? 100.0 : (maxVolume * 1.2).ceilToDouble();

    final barGroups = displayed.asMap().entries.map((entry) {
      final idx = entry.key;
      final session = entry.value;
      return BarChartGroupData(
        x: idx,
        barRods: [
          BarChartRodData(
            toY: session.totalVolume,
            width: 16,
            color: AppColors.primary,
            borderRadius: const BorderRadius.vertical(top: Radius.circular(4)),
          ),
        ],
      );
    }).toList();

    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(15), blurRadius: 6, offset: const Offset(0, 2)),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            l10n.sessionsChartVolumeTitle,
            style: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: Color(0xFF333333),
            ),
          ),
          const SizedBox(height: 16),
          SizedBox(
            height: 200,
            child: BarChart(
              BarChartData(
                maxY: maxY,
                barGroups: barGroups,
                gridData: const FlGridData(show: true),
                borderData: FlBorderData(show: false),
                titlesData: FlTitlesData(
                  topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                  rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                  leftTitles: AxisTitles(
                    sideTitles: SideTitles(
                      showTitles: true,
                      reservedSize: 40,
                      getTitlesWidget: (value, meta) => Text(
                        value.toInt().toString(),
                        style: const TextStyle(fontSize: 10, color: Color(0xFF888888)),
                      ),
                    ),
                  ),
                  bottomTitles: AxisTitles(
                    sideTitles: SideTitles(
                      showTitles: true,
                      reservedSize: 28,
                      getTitlesWidget: (value, meta) {
                        final idx = value.toInt();
                        if (idx < 0 || idx >= displayed.length) {
                          return const SizedBox.shrink();
                        }
                        final date = displayed[idx].completedAtDate;
                        if (date == null) return const SizedBox.shrink();
                        return Padding(
                          padding: const EdgeInsets.only(top: 4),
                          child: Text(
                            '${date.day}/${date.month}',
                            style: const TextStyle(fontSize: 10, color: Color(0xFF888888)),
                          ),
                        );
                      },
                    ),
                  ),
                ),
                barTouchData: BarTouchData(
                  touchTooltipData: BarTouchTooltipData(
                    getTooltipItem: (group, groupIndex, rod, rodIndex) {
                      final session = displayed[group.x.toInt()];
                      return BarTooltipItem(
                        '${session.workoutName}\n${rod.toY.toStringAsFixed(0)} kg',
                        const TextStyle(color: Colors.white, fontSize: 12),
                      );
                    },
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  // ── Empty / Error ─────────────────────────────────────────────────────────────

  Widget _buildEmpty(AppLocalizations l10n) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.bar_chart, size: 64, color: Colors.grey.withAlpha(100)),
            const SizedBox(height: 16),
            Text(
              l10n.sessionsNoData,
              textAlign: TextAlign.center,
              style: const TextStyle(color: Color(0xFF888888), fontSize: 15),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildError(String msg) {
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
                backgroundColor: AppColors.primary,
                foregroundColor: Colors.white,
              ),
              child: const Text('Retry'),
            ),
          ],
        ),
      ),
    );
  }

  // ── Utilities ─────────────────────────────────────────────────────────────────

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

  const _PeriodChip({required this.label, required this.selected, required this.onTap, this.icon});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
        decoration: BoxDecoration(
          color: selected ? AppColors.primary : Colors.transparent,
          borderRadius: BorderRadius.circular(20),
          border: Border.all(color: selected ? AppColors.primary : const Color(0xFFCCCCCC)),
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
// Summary card
// ---------------------------------------------------------------------------

class _SummaryCard extends StatelessWidget {
  final IconData icon;
  final String value;
  final String label;
  final Color color;

  const _SummaryCard({
    required this.icon,
    required this.value,
    required this.label,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.all(12),
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
            Icon(icon, size: 20, color: color),
            const SizedBox(height: 6),
            Text(
              value,
              style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: color),
              overflow: TextOverflow.ellipsis,
            ),
            Text(
              label,
              style: const TextStyle(fontSize: 11, color: Color(0xFF888888)),
              overflow: TextOverflow.ellipsis,
            ),
          ],
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Session card
// ---------------------------------------------------------------------------

class _SessionCard extends StatelessWidget {
  final WorkoutSession session;
  final VoidCallback? onTap;

  const _SessionCard({required this.session, this.onTap});

  @override
  Widget build(BuildContext context) {
    final date = session.completedAtDate;
    final dateStr = date != null
        ? '${date.day.toString().padLeft(2, '0')}/'
              '${date.month.toString().padLeft(2, '0')}/'
              '${date.year}'
        : session.dayOfWeek;

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
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
              children: [
                Expanded(
                  child: Text(
                    session.workoutName,
                    style: const TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF333333),
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                const Icon(Icons.chevron_right, size: 18, color: Color(0xFFAAAAAA)),
                Text(dateStr, style: const TextStyle(fontSize: 12, color: Color(0xFF888888))),
              ],
            ),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              runSpacing: 4,
              children: [
                _StatChip(
                  icon: Icons.timer_outlined,
                  label: session.formattedDuration,
                  color: AppColors.primary,
                ),
                _StatChip(
                  icon: Icons.repeat,
                  label: '${session.totalSets} sets',
                  color: AppColors.secondary,
                ),
                _StatChip(
                  icon: Icons.fitness_center,
                  label: '${session.totalVolume.toStringAsFixed(3)} kg',
                  color: AppColors.third,
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _StatChip extends StatelessWidget {
  final IconData icon;
  final String label;
  final Color color;

  const _StatChip({required this.icon, required this.label, required this.color});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withAlpha(20),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withAlpha(60)),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 12, color: color),
          const SizedBox(width: 4),
          Text(label, style: TextStyle(fontSize: 12, color: color)),
        ],
      ),
    );
  }
}
