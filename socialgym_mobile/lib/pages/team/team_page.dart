import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/business_profile.dart';
import '../../models/person.dart';
import '../../providers/person_provider.dart';
import '../../providers/team_member_provider.dart';
import '../../widgets/main_layout.dart';
import '../../widgets/team/invite_person_sheet.dart';
import '../../widgets/team/team_business_card.dart';
import '../../widgets/team/team_member_card.dart';

class TeamPage extends StatelessWidget {
  const TeamPage({super.key});

  @override
  Widget build(BuildContext context) {
    // A business-profile switch always navigates away from this page, but key
    // on the flag anyway so the tab set can never get out of sync with it.
    final isProfessional = context.watch<PersonProvider>().isProfessional;
    return _TeamPageContent(key: ValueKey(isProfessional), isProfessional: isProfessional);
  }
}

class _TeamPageContent extends StatefulWidget {
  final bool isProfessional;

  const _TeamPageContent({super.key, required this.isProfessional});

  @override
  State<_TeamPageContent> createState() => _TeamPageContentState();
}

class _TeamPageContentState extends State<_TeamPageContent> with SingleTickerProviderStateMixin {
  late final TabController _tabController;

  int get _tabCount => widget.isProfessional ? 4 : 2;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: _tabCount, vsync: this);
    WidgetsBinding.instance.addPostFrameCallback((_) => _fetchTeamData());
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  void _fetchTeamData() {
    final personProvider = context.read<PersonProvider>();
    final person = personProvider.person;
    if (person == null) return;

    context.read<TeamMemberProvider>().fetchPage(
      businessProfileId: widget.isProfessional ? personProvider.activeBusinessProfile?.id : null,
      personId: person.id,
    );
  }

  Future<void> _cancelSentInvite(Person invitee) async {
    final l10n = AppLocalizations.of(context)!;
    final businessProfileId = context.read<PersonProvider>().activeBusinessProfile?.id;
    if (businessProfileId == null) return;

    final success = await context.read<TeamMemberProvider>().cancelSentInvite(
      businessProfileId: businessProfileId,
      personId: invitee.id,
    );
    _showActionSnackBar(success, success ? l10n.teamRequestCancelled : l10n.teamActionError);
  }

  Future<void> _acceptInvite(BusinessProfile business) async {
    final l10n = AppLocalizations.of(context)!;
    final personId = context.read<PersonProvider>().person?.id;
    final businessProfileId = business.id;
    if (personId == null || businessProfileId == null) return;

    final success = await context.read<TeamMemberProvider>().acceptInvite(
      businessProfileId: businessProfileId,
      personId: personId,
    );
    _showActionSnackBar(success, success ? l10n.teamRequestAccepted : l10n.teamActionError);
  }

  Future<void> _denyInvite(BusinessProfile business) async {
    final l10n = AppLocalizations.of(context)!;
    final personId = context.read<PersonProvider>().person?.id;
    final businessProfileId = business.id;
    if (personId == null || businessProfileId == null) return;

    final success = await context.read<TeamMemberProvider>().denyInvite(
      businessProfileId: businessProfileId,
      personId: personId,
    );
    _showActionSnackBar(success, success ? l10n.teamRequestDenied : l10n.teamActionError);
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

  void _openInviteSheet() {
    final businessProfileId = context.read<PersonProvider>().activeBusinessProfile?.id;
    if (businessProfileId == null) return;
    showTeamInviteSheet(context, businessProfileId: businessProfileId);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/team',
      body: Column(
        children: [
          _buildHeader(l10n),
          _buildTabBar(l10n),
          Expanded(
            child: Consumer<TeamMemberProvider>(
              builder: (context, provider, _) {
                if (provider.loading) {
                  return const Center(child: CircularProgressIndicator());
                }

                return Stack(
                  children: [
                    TabBarView(
                      controller: _tabController,
                      children: [
                        if (widget.isProfessional) _buildMembersTab(provider, l10n),
                        if (widget.isProfessional) _buildSentInvitesTab(provider, l10n),
                        _buildMyTeamsTab(provider, l10n),
                        _buildReceivedInvitesTab(provider, l10n),
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
          const Icon(Icons.group, color: AppColors.primary, size: 28),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(l10n.teamTitle, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
                Text(l10n.teamDescription, style: TextStyle(fontSize: 14, color: Colors.grey[600])),
              ],
            ),
          ),
          if (widget.isProfessional)
            IconButton(
              icon: const Icon(Icons.person_add_alt),
              onPressed: _openInviteSheet,
              tooltip: l10n.teamInvite,
            ),
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _fetchTeamData,
            tooltip: l10n.tooltipRefresh,
          ),
        ],
      ),
    );
  }

  Widget _buildTabBar(AppLocalizations l10n) {
    return Container(
      decoration: BoxDecoration(border: Border(bottom: BorderSide(color: Colors.grey[300]!))),
      child: TabBar(
        controller: _tabController,
        isScrollable: widget.isProfessional,
        labelColor: AppColors.primary,
        unselectedLabelColor: Colors.grey[600],
        indicatorColor: AppColors.primary,
        tabs: [
          if (widget.isProfessional) Tab(text: l10n.teamTabMembers),
          if (widget.isProfessional)
            _buildBadgedTab(l10n.teamTabSentInvites, (p) => p.sentRequests.length),
          Tab(text: l10n.teamTabMyTeams),
          _buildBadgedTab(l10n.teamTabReceivedInvites, (p) => p.receivedRequests.length),
        ],
      ),
    );
  }

  Widget _buildBadgedTab(String label, int Function(TeamMemberProvider) countOf) {
    return Consumer<TeamMemberProvider>(
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
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(color: AppColors.danger, borderRadius: BorderRadius.circular(10)),
                  child: Text(
                    '$count',
                    style: const TextStyle(color: Colors.white, fontSize: 11, fontWeight: FontWeight.bold),
                  ),
                ),
              ],
            ],
          ),
        );
      },
    );
  }

  Widget _buildMembersTab(TeamMemberProvider provider, AppLocalizations l10n) {
    if (provider.members.isEmpty) {
      return _buildEmptyState(
        icon: Icons.group_outlined,
        title: l10n.teamNoMembers,
        subtitle: l10n.teamNoMembersHint,
      );
    }
    return RefreshIndicator(
      onRefresh: () async => _fetchTeamData(),
      child: ListView.builder(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        itemCount: provider.members.length,
        itemBuilder: (context, index) {
          return TeamMemberCard(person: provider.members[index], trailing: const SizedBox.shrink());
        },
      ),
    );
  }

  Widget _buildSentInvitesTab(TeamMemberProvider provider, AppLocalizations l10n) {
    if (provider.sentRequests.isEmpty) {
      return _buildEmptyState(
        icon: Icons.outgoing_mail,
        title: l10n.teamNoSentInvites,
        subtitle: l10n.teamNoSentInvitesHint,
      );
    }
    return RefreshIndicator(
      onRefresh: () async => _fetchTeamData(),
      child: ListView.builder(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        itemCount: provider.sentRequests.length,
        itemBuilder: (context, index) {
          final invitee = provider.sentRequests[index];
          return TeamMemberCard(
            person: invitee,
            trailing: OutlinedButton(
              onPressed: () => _cancelSentInvite(invitee),
              style: OutlinedButton.styleFrom(
                foregroundColor: AppColors.danger,
                side: const BorderSide(color: AppColors.danger),
                padding: const EdgeInsets.symmetric(horizontal: 12),
                minimumSize: const Size(0, 36),
              ),
              child: Text(l10n.teamCancel),
            ),
          );
        },
      ),
    );
  }

  Widget _buildMyTeamsTab(TeamMemberProvider provider, AppLocalizations l10n) {
    if (provider.teams.isEmpty) {
      return _buildEmptyState(
        icon: Icons.business_outlined,
        title: l10n.teamNoTeams,
        subtitle: l10n.teamNoTeamsHint,
      );
    }
    return RefreshIndicator(
      onRefresh: () async => _fetchTeamData(),
      child: ListView.builder(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        itemCount: provider.teams.length,
        itemBuilder: (context, index) {
          return TeamBusinessCard(business: provider.teams[index], trailing: const SizedBox.shrink());
        },
      ),
    );
  }

  Widget _buildReceivedInvitesTab(TeamMemberProvider provider, AppLocalizations l10n) {
    if (provider.receivedRequests.isEmpty) {
      return _buildEmptyState(
        icon: Icons.mail_outline,
        title: l10n.teamNoReceivedInvites,
        subtitle: l10n.teamNoReceivedInvitesHint,
      );
    }
    return RefreshIndicator(
      onRefresh: () async => _fetchTeamData(),
      child: ListView.builder(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        itemCount: provider.receivedRequests.length,
        itemBuilder: (context, index) {
          final business = provider.receivedRequests[index];
          return TeamBusinessCard(
            business: business,
            trailing: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                IconButton(
                  onPressed: () => _acceptInvite(business),
                  icon: const Icon(Icons.check_circle),
                  color: AppColors.success,
                  tooltip: l10n.teamAccept,
                ),
                IconButton(
                  onPressed: () => _denyInvite(business),
                  icon: const Icon(Icons.cancel),
                  color: AppColors.danger,
                  tooltip: l10n.teamDeny,
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildEmptyState({required IconData icon, required String title, required String subtitle}) {
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
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.w600, color: Colors.grey[600]),
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
