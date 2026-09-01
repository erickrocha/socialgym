import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/workout.dart';
import '../../providers/auth_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/workout_invite_provider.dart';
import '../../providers/workout_provider.dart';
import '../../widgets/main_layout.dart';
import '../../widgets/workout/workout_invite_card.dart';

class WorkoutInvitesPage extends StatelessWidget {
  const WorkoutInvitesPage({super.key});

  @override
  Widget build(BuildContext context) {
    final personProvider = context.watch<PersonProvider>();
    return _WorkoutInvitesContent(
      key: ValueKey(personProvider.isProfessional),
      isProfessional: personProvider.isProfessional,
      businessType: personProvider.activeBusinessProfile?.businessType,
    );
  }
}

class _WorkoutInvitesContent extends StatefulWidget {
  final bool isProfessional;
  final String? businessType;

  const _WorkoutInvitesContent({
    super.key,
    required this.isProfessional,
    required this.businessType,
  });

  @override
  State<_WorkoutInvitesContent> createState() => _WorkoutInvitesContentState();
}

class _WorkoutInvitesContentState extends State<_WorkoutInvitesContent>
    with SingleTickerProviderStateMixin {
  late final TabController _tabController;

  int get _tabCount => widget.isProfessional ? 2 : 1;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: _tabCount, vsync: this);
    WidgetsBinding.instance.addPostFrameCallback((_) => _fetch());
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  void _fetch() {
    final personProvider = context.read<PersonProvider>();
    final person = personProvider.person;
    if (person == null) return;

    context.read<WorkoutInviteProvider>().fetch(
      personUuid: person.uuid,
      businessProfileId: widget.isProfessional
          ? personProvider.activeBusinessProfile?.id
          : null,
    );
  }

  Future<void> _accept(Workout workout) async {
    final l10n = AppLocalizations.of(context)!;
    final uuid = workout.uuid;
    if (uuid == null) return;
    final success = await context.read<WorkoutInviteProvider>().accept(uuid);
    _showActionSnackBar(
      success,
      success ? l10n.workoutInviteAccepted : l10n.workoutInviteActionError,
    );
    if (success && mounted) {
      // The accepted workout now belongs in the normal Workouts list.
      final personProvider = context.read<PersonProvider>();
      context.read<WorkoutProvider>().fetchWorkouts(
        personProvider.activeAuthorUuid,
        context.read<AuthProvider>().auth?.accessToken ?? '',
      );
    }
  }

  Future<void> _reject(Workout workout) async {
    final l10n = AppLocalizations.of(context)!;
    final uuid = workout.uuid;
    if (uuid == null) return;
    final success = await context.read<WorkoutInviteProvider>().reject(uuid);
    _showActionSnackBar(
      success,
      success ? l10n.workoutInviteRejected : l10n.workoutInviteActionError,
    );
  }

  Future<void> _cancel(Workout workout) async {
    final l10n = AppLocalizations.of(context)!;
    final uuid = workout.uuid;
    if (uuid == null) return;
    final success = await context.read<WorkoutInviteProvider>().cancel(uuid);
    _showActionSnackBar(
      success,
      success ? l10n.workoutInviteCancelled : l10n.workoutInviteActionError,
    );
  }

  void _showActionSnackBar(bool success, String message) {
    if (!mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: success ? AppColors.success : AppColors.danger,
      ),
    );
  }

  String _statusLabel(AppLocalizations l10n, String status) {
    switch (status) {
      case 'Accepted':
        return l10n.workoutStatusAccepted;
      case 'Rejected':
        return l10n.workoutStatusRejected;
      case 'Cancelled':
        return l10n.workoutStatusCancelled;
      default:
        return l10n.workoutStatusPending;
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return MainLayout(
      navSection: NavSection.workout,
      currentRoute: '/workout-invites',
      body: Column(
        children: [
          _buildHeader(l10n),
          _buildTabBar(l10n),
          Expanded(
            child: Consumer<WorkoutInviteProvider>(
              builder: (context, provider, _) {
                if (provider.loading) {
                  return const Center(child: CircularProgressIndicator());
                }
                return Stack(
                  children: [
                    TabBarView(
                      controller: _tabController,
                      children: [
                        _buildReceivedTab(provider, l10n),
                        if (widget.isProfessional)
                          _buildSentTab(provider, l10n),
                      ],
                    ),
                    if (provider.actionLoading)
                      Container(
                        color: Colors.black26,
                        child: const Center(child: CircularProgressIndicator()),
                      ),
                  ],
                );
              },
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildHeader(AppLocalizations l10n) {
    return Container(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          Icon(
            Icons.assignment_turned_in,
            color: AppColors.primaryFor(widget.businessType),
            size: 28,
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  l10n.workoutInvitesTitle,
                  style: const TextStyle(
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Text(
                  l10n.workoutInvitesDescription,
                  style: TextStyle(fontSize: 14, color: Colors.grey[600]),
                ),
              ],
            ),
          ),
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _fetch,
            tooltip: l10n.tooltipRefresh,
          ),
        ],
      ),
    );
  }

  Widget _buildTabBar(AppLocalizations l10n) {
    return Container(
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: Colors.grey[300]!)),
      ),
      child: TabBar(
        controller: _tabController,
        labelColor: AppColors.primaryFor(widget.businessType),
        unselectedLabelColor: Colors.grey[600],
        indicatorColor: AppColors.primaryFor(widget.businessType),
        tabs: [
          Tab(text: l10n.workoutInvitesTabReceived),
          if (widget.isProfessional)
            _buildBadgedTab(
              l10n.workoutInvitesTabSent,
              (p) => p.pendingSentCount,
            ),
        ],
      ),
    );
  }

  Widget _buildBadgedTab(
    String label,
    int Function(WorkoutInviteProvider) countOf,
  ) {
    return Consumer<WorkoutInviteProvider>(
      builder: (context, provider, _) {
        final count = countOf(provider);
        return Tab(
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(label),
              if (count > 0) ...[
                const SizedBox(width: 6),
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 6,
                    vertical: 2,
                  ),
                  decoration: BoxDecoration(
                    color: AppColors.danger,
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Text(
                    '$count',
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ],
            ],
          ),
        );
      },
    );
  }

  Widget _buildReceivedTab(
    WorkoutInviteProvider provider,
    AppLocalizations l10n,
  ) {
    if (provider.received.isEmpty) {
      return _buildEmptyState(
        icon: Icons.mail_outline,
        title: l10n.workoutInviteNoReceived,
        subtitle: l10n.workoutInviteNoReceivedHint,
      );
    }
    return RefreshIndicator(
      onRefresh: () async => _fetch(),
      child: ListView.builder(
        padding: const EdgeInsets.only(
          left: 16,
          right: 16,
          top: 16,
          bottom: 80,
        ),
        itemCount: provider.received.length,
        itemBuilder: (context, index) {
          final workout = provider.received[index];
          final assigner = provider.assignerFor(workout.assignedByProfileUuid);
          return WorkoutInviteCard(
            workout: workout,
            subtitle: l10n.workoutInviteFrom(
              assigner?.businessName ?? l10n.workoutInviteFromTrainer,
            ),
            trailing: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                IconButton(
                  onPressed: () => _accept(workout),
                  icon: const Icon(Icons.check_circle),
                  color: AppColors.success,
                  tooltip: l10n.workoutInviteAccept,
                ),
                IconButton(
                  onPressed: () => _reject(workout),
                  icon: const Icon(Icons.cancel),
                  color: AppColors.danger,
                  tooltip: l10n.workoutInviteReject,
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildSentTab(WorkoutInviteProvider provider, AppLocalizations l10n) {
    if (provider.sent.isEmpty) {
      return _buildEmptyState(
        icon: Icons.outgoing_mail,
        title: l10n.workoutInviteNoSent,
        subtitle: l10n.workoutInviteNoSentHint,
      );
    }
    return RefreshIndicator(
      onRefresh: () async => _fetch(),
      child: ListView.builder(
        padding: const EdgeInsets.only(
          left: 16,
          right: 16,
          top: 16,
          bottom: 80,
        ),
        itemCount: provider.sent.length,
        itemBuilder: (context, index) {
          final workout = provider.sent[index];
          final recipient = provider.recipientFor(workout.ownerUuid);
          return WorkoutInviteCard(
            workout: workout,
            subtitle: l10n.workoutInviteTo(recipient?.fullName ?? '…'),
            statusLabel: _statusLabel(l10n, workout.status),
            trailing: workout.status == 'Pending'
                ? OutlinedButton(
                    onPressed: () => _cancel(workout),
                    style: OutlinedButton.styleFrom(
                      foregroundColor: AppColors.danger,
                      side: const BorderSide(color: AppColors.danger),
                      padding: const EdgeInsets.symmetric(horizontal: 12),
                      minimumSize: const Size(0, 36),
                    ),
                    child: Text(l10n.workoutInviteCancel),
                  )
                : const SizedBox.shrink(),
          );
        },
      ),
    );
  }

  Widget _buildEmptyState({
    required IconData icon,
    required String title,
    required String subtitle,
  }) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 64, color: Colors.grey[400]),
            const SizedBox(height: 16),
            Text(
              title,
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.w600,
                color: Colors.grey[600],
              ),
            ),
            const SizedBox(height: 8),
            Text(
              subtitle,
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 14, color: Colors.grey[500]),
            ),
          ],
        ),
      ),
    );
  }
}
