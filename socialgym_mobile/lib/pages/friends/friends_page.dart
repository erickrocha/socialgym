import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../providers/friends_provider.dart';
import '../../config/nav_section.dart';
import '../../providers/person_provider.dart';
import '../../widgets/main_layout.dart';
import '../profile/person_profile_page.dart';

class FriendsPage extends StatefulWidget {
  const FriendsPage({super.key});

  @override
  State<FriendsPage> createState() => _FriendsPageState();
}

class _FriendsPageState extends State<FriendsPage> with SingleTickerProviderStateMixin {
  late TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _fetchFriends();
    });
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  void _fetchFriends() {
    final authProvider = context.read<AuthProvider>();
    final friendsProvider = context.read<FriendsProvider>();
    final token = authProvider.auth?.accessToken ?? '';

    if (token.isNotEmpty) {
      friendsProvider.fetchFriends(token);
    }
  }

  String _getToken() {
    return context.read<AuthProvider>().auth?.accessToken ?? '';
  }

  Future<void> _sendFriendRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.sendFriendRequest(person.id, _getToken());

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? l10n.friendsRequestSent : l10n.friendsActionError),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _acceptRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.acceptFriendRequest(person.id, _getToken());

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? l10n.friendsRequestAccepted : l10n.friendsActionError),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _rejectRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.rejectFriendRequest(person.id, _getToken());

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? l10n.friendsRequestRejected : l10n.friendsActionError),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _cancelRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.cancelFriendRequest(person.id, _getToken());

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? l10n.friendsRequestCancelled : l10n.friendsActionError),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _removeFriend(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.removeFriend(person.id, _getToken());

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(success ? l10n.friendsRemoved : l10n.friendsActionError),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  void _showRemoveFriendDialog(Person person) {
    final l10n = AppLocalizations.of(context)!;
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.friendsRemove),
        content: Text('Remove ${person.fullName} from your friends?'),
        actions: [
          TextButton(onPressed: () => Navigator.pop(context), child: Text(l10n.buttonCancel)),
          TextButton(
            onPressed: () {
              Navigator.pop(context);
              _removeFriend(person);
            },
            style: TextButton.styleFrom(foregroundColor: AppColors.danger),
            child: Text(l10n.friendsRemove),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = context.watch<PersonProvider>().activeBusinessProfile?.businessType;

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/friends',
      body: Column(
        children: [
          // Header
          Container(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.people, color: AppColors.primaryFor(businessType), size: 28),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            l10n.friendsTitle,
                            style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                          ),
                          Text(
                            l10n.friendsDescription,
                            style: TextStyle(fontSize: 14, color: Colors.grey[600]),
                          ),
                        ],
                      ),
                    ),
                    IconButton(
                      icon: const Icon(Icons.refresh),
                      onPressed: _fetchFriends,
                      tooltip: l10n.tooltipRefresh,
                    ),
                  ],
                ),
              ],
            ),
          ),

          // Tab bar
          Container(
            decoration: BoxDecoration(
              border: Border(bottom: BorderSide(color: Colors.grey[300]!)),
            ),
            child: TabBar(
              controller: _tabController,
              labelColor: AppColors.primaryFor(businessType),
              unselectedLabelColor: Colors.grey[600],
              indicatorColor: AppColors.primaryFor(businessType),
              tabs: [
                Tab(text: l10n.friendsTabAll),
                Consumer<FriendsProvider>(
                  builder: (context, provider, _) {
                    final count = provider.receiveRequests.length;
                    return Tab(
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Text(l10n.friendsTabRequests),
                          if (count > 0) ...[
                            const SizedBox(width: 6),
                            Container(
                              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
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
                ),
                Tab(text: l10n.friendsTabSuggestions),
              ],
            ),
          ),

          // Tab content
          Expanded(
            child: Consumer<FriendsProvider>(
              builder: (context, provider, _) {
                if (provider.loading) {
                  return const Center(child: CircularProgressIndicator());
                }

                return Stack(
                  children: [
                    TabBarView(
                      controller: _tabController,
                      children: [
                        _buildFriendsTab(provider, l10n, businessType),
                        _buildRequestsTab(provider, l10n, businessType),
                        _buildSuggestionsTab(provider, l10n, businessType),
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

  Widget _buildFriendsTab(FriendsProvider provider, AppLocalizations l10n, String? businessType) {
    if (provider.friends.isEmpty) {
      return _buildEmptyState(
        icon: Icons.people_outline,
        title: l10n.friendsNoFriends,
        subtitle: l10n.friendsNoFriendsHint,
      );
    }

    return RefreshIndicator(
      onRefresh: () async => _fetchFriends(),
      child: ListView.builder(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        itemCount: provider.friends.length,
        itemBuilder: (context, index) {
          final friend = provider.friends[index];
          return _buildPersonCard(
            person: friend,
            businessType: businessType,
            trailing: PopupMenuButton<String>(
              icon: const Icon(Icons.more_vert),
              onSelected: (value) {
                if (value == 'remove') {
                  _showRemoveFriendDialog(friend);
                }
              },
              itemBuilder: (context) => [
                PopupMenuItem(
                  value: 'remove',
                  child: Row(
                    children: [
                      const Icon(Icons.person_remove, color: AppColors.danger, size: 20),
                      const SizedBox(width: 12),
                      Text(l10n.friendsRemove, style: const TextStyle(color: AppColors.danger)),
                    ],
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildRequestsTab(FriendsProvider provider, AppLocalizations l10n, String? businessType) {
    final hasReceived = provider.receiveRequests.isNotEmpty;
    final hasSent = provider.sentRequests.isNotEmpty;

    if (!hasReceived && !hasSent) {
      return _buildEmptyState(
        icon: Icons.mail_outline,
        title: l10n.friendsNoRequests,
        subtitle: l10n.friendsNoRequestsHint,
      );
    }

    return RefreshIndicator(
      onRefresh: () async => _fetchFriends(),
      child: ListView(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        children: [
          // Received Requests
          if (hasReceived) ...[
            _buildSectionHeader(
              l10n.friendsReceivedRequests,
              provider.receiveRequests.length,
              businessType,
            ),
            const SizedBox(height: 8),
            ...provider.receiveRequests.map(
              (person) => _buildPersonCard(
                person: person,
                businessType: businessType,
                trailing: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    IconButton(
                      onPressed: () => _acceptRequest(person),
                      icon: const Icon(Icons.check_circle),
                      color: AppColors.success,
                      tooltip: l10n.friendsAccept,
                    ),
                    IconButton(
                      onPressed: () => _rejectRequest(person),
                      icon: const Icon(Icons.cancel),
                      color: AppColors.danger,
                      tooltip: l10n.friendsReject,
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 24),
          ],

          // Sent Requests
          if (hasSent) ...[
            _buildSectionHeader(
              l10n.friendsSentRequests,
              provider.sentRequests.length,
              businessType,
            ),
            const SizedBox(height: 8),
            ...provider.sentRequests.map(
              (person) => _buildPersonCard(
                person: person,
                businessType: businessType,
                trailing: OutlinedButton(
                  onPressed: () => _cancelRequest(person),
                  style: OutlinedButton.styleFrom(
                    foregroundColor: AppColors.danger,
                    side: const BorderSide(color: AppColors.danger),
                    padding: const EdgeInsets.symmetric(horizontal: 12),
                    minimumSize: const Size(0, 36),
                  ),
                  child: Text(l10n.friendsCancel),
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildSuggestionsTab(
    FriendsProvider provider,
    AppLocalizations l10n,
    String? businessType,
  ) {
    if (provider.suggestions.isEmpty) {
      return _buildEmptyState(
        icon: Icons.person_search,
        title: l10n.friendsNoSuggestions,
        subtitle: l10n.friendsNoSuggestionsHint,
      );
    }

    return RefreshIndicator(
      onRefresh: () async => _fetchFriends(),
      child: ListView.builder(
        padding: const EdgeInsets.only(left: 16, right: 16, top: 16, bottom: 80),
        itemCount: provider.suggestions.length,
        itemBuilder: (context, index) {
          final person = provider.suggestions[index];
          return _buildPersonCard(
            person: person,
            businessType: businessType,
            trailing: IconButton(
              onPressed: () => _sendFriendRequest(person),
              icon: const Icon(Icons.person_add),
              color: AppColors.primaryFor(businessType),
              tooltip: l10n.friendsAddFriend,
            ),
          );
        },
      ),
    );
  }

  Widget _buildPersonCard({
    required Person person,
    required Widget trailing,
    required String? businessType,
  }) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 1,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        onTap: () => Navigator.of(
          context,
        ).push(MaterialPageRoute(builder: (_) => PersonProfilePage(person: person))),
        child: Padding(
          padding: const EdgeInsets.all(12),
          child: Row(
            children: [
              // Avatar
              _buildAvatar(person, businessType),
              const SizedBox(width: 12),
              // Name and info
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      person.fullName,
                      style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                    ),
                    if (person.personInfo?.currentCity != null)
                      Text(
                        person.personInfo!.currentCity!,
                        style: TextStyle(fontSize: 13, color: Colors.grey[600]),
                      ),
                  ],
                ),
              ),
              // Action buttons (stop propagation with a separate widget area)
              trailing,
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildAvatar(Person person, String? businessType) {
    return Container(
      width: 50,
      height: 50,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        border: Border.all(color: AppColors.primaryFor(businessType).withAlpha(51), width: 2),
      ),
      child: ClipOval(
        child: person.avatar != null
            ? CachedNetworkImage(
                imageUrl: person.avatar!,
                fit: BoxFit.cover,
                placeholder: (context, url) => Container(
                  color: AppColors.primaryFor(businessType).withAlpha(51),
                  child: const Center(child: CircularProgressIndicator(strokeWidth: 2)),
                ),
                errorWidget: (context, url, error) => _buildDefaultAvatar(person),
              )
            : _buildDefaultAvatar(person),
      ),
    );
  }

  Widget _buildDefaultAvatar(Person person) {
    return Image.asset(
      person.gender?.toLowerCase() == 'female'
          ? 'assets/images/avatar_female.png'
          : 'assets/images/avatar_male.png',
      fit: BoxFit.cover,
    );
  }

  Widget _buildSectionHeader(String title, int count, String? businessType) {
    return Row(
      children: [
        Text(title, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
        const SizedBox(width: 8),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
          decoration: BoxDecoration(
            color: AppColors.primaryFor(businessType).withAlpha(51),
            borderRadius: BorderRadius.circular(10),
          ),
          child: Text(
            '$count',
            style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.bold,
              color: AppColors.primaryFor(businessType),
            ),
          ),
        ),
      ],
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
