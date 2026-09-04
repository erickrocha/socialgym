import 'dart:async';

import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';

import '../../utils/open_direct_chat.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../providers/friends_provider.dart';
import '../../config/nav_section.dart';
import '../../providers/person_provider.dart';
import '../../services/friends_service.dart';
import '../../utils/location_utils.dart';
import '../../widgets/main_layout.dart';
import '../profile/person_profile_page.dart';

/// Where a location-aware suggestion/search should be centered.
enum LocationMode { anywhere, savedAddress, currentGps }

class FriendsPage extends StatefulWidget {
  const FriendsPage({super.key});

  @override
  State<FriendsPage> createState() => _FriendsPageState();
}

class _FriendsPageState extends State<FriendsPage>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  LocationMode _suggestionsLocationMode = LocationMode.savedAddress;
  Position? _suggestionsPosition;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
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
      friendsProvider.fetchFriends(
        token,
        latitude: _suggestionsLocationMode == LocationMode.currentGps
            ? _suggestionsPosition?.latitude
            : null,
        longitude: _suggestionsLocationMode == LocationMode.currentGps
            ? _suggestionsPosition?.longitude
            : null,
      );
    }
  }

  Future<void> _setSuggestionsLocationMode(LocationMode mode) async {
    if (mode == LocationMode.currentGps) {
      final position = await LocationUtils.getCurrentPosition(context);
      if (!mounted) return;
      if (position == null) {
        return; // Location unavailable: keep the previous mode.
      }
      setState(() {
        _suggestionsLocationMode = mode;
        _suggestionsPosition = position;
      });
    } else {
      setState(() => _suggestionsLocationMode = mode);
    }
    _fetchFriends();
  }

  String _getToken() {
    return context.read<AuthProvider>().auth?.accessToken ?? '';
  }

  Future<void> _sendFriendRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.sendFriendRequest(
      person.id,
      _getToken(),
    );

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            success ? l10n.friendsRequestSent : l10n.friendsActionError,
          ),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _acceptRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.acceptFriendRequest(
      person.id,
      _getToken(),
    );

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            success ? l10n.friendsRequestAccepted : l10n.friendsActionError,
          ),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _rejectRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.rejectFriendRequest(
      person.id,
      _getToken(),
    );

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            success ? l10n.friendsRequestRejected : l10n.friendsActionError,
          ),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Future<void> _cancelRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final success = await friendsProvider.cancelFriendRequest(
      person.id,
      _getToken(),
    );

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            success ? l10n.friendsRequestCancelled : l10n.friendsActionError,
          ),
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
          content: Text(
            success ? l10n.friendsRemoved : l10n.friendsActionError,
          ),
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
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text(l10n.buttonCancel),
          ),
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
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

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
                    Icon(
                      Icons.people,
                      color: AppColors.primaryFor(businessType),
                      size: 28,
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            l10n.friendsTitle,
                            style: const TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          Text(
                            l10n.friendsDescription,
                            style: TextStyle(
                              fontSize: 14,
                              color: Colors.grey[600],
                            ),
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
              isScrollable: true,
              tabAlignment: TabAlignment.start,
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
                ),
                Tab(text: l10n.friendsTabSuggestions),
                Tab(text: l10n.friendsTabFind),
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
                        _FindFriendsTab(businessType: businessType),
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

  Widget _buildFriendsTab(
    FriendsProvider provider,
    AppLocalizations l10n,
    String? businessType,
  ) {
    if (provider.friends.isEmpty) {
      return _buildEmptyState(
        icon: Icons.people_outline,
        title: l10n.friendsNoFriends,
        subtitle: l10n.friendsNoFriendsHint,
      );
    }

    return RefreshIndicator(
      onRefresh: () async => _fetchFriends(),
      child: GridView.builder(
        physics: const AlwaysScrollableScrollPhysics(),
        padding: const EdgeInsets.fromLTRB(16, 16, 16, 80),
        gridDelegate: _peopleGridDelegate,
        itemCount: provider.friends.length,
        itemBuilder: (context, index) {
          final friend = provider.friends[index];
          return _buildPersonGridCard(
            person: friend,
            businessType: businessType,
            onTap: () => _openPersonProfile(friend),
            action: Row(
              children: [
                Expanded(
                  child: _buildCompactAction(
                    label: l10n.chatMessageAction,
                    color: AppColors.primary,
                    onPressed: () => openDirectChat(
                      context,
                      personUuid: friend.uuid,
                      displayName: friend.fullName,
                    ),
                  ),
                ),
                const SizedBox(width: 6),
                Expanded(
                  child: _buildCompactAction(
                    label: l10n.friendsRemove,
                    color: AppColors.danger,
                    onPressed: () => _showRemoveFriendDialog(friend),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildRequestsTab(
    FriendsProvider provider,
    AppLocalizations l10n,
    String? businessType,
  ) {
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
      child: CustomScrollView(
        physics: const AlwaysScrollableScrollPhysics(),
        slivers: [
          // Received Requests
          if (hasReceived) ...[
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
                child: _buildSectionHeader(
                  l10n.friendsReceivedRequests,
                  provider.receiveRequests.length,
                  businessType,
                ),
              ),
            ),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverGrid(
                gridDelegate: _peopleGridDelegate,
                delegate: SliverChildBuilderDelegate((context, index) {
                  final person = provider.receiveRequests[index];
                  return _buildPersonGridCard(
                    person: person,
                    businessType: businessType,
                    onTap: () => _openPersonProfile(person),
                    action: Row(
                      children: [
                        Expanded(
                          child: _buildCompactAction(
                            label: l10n.friendsAccept,
                            color: AppColors.success,
                            onPressed: () => _acceptRequest(person),
                          ),
                        ),
                        const SizedBox(width: 6),
                        Expanded(
                          child: _buildCompactAction(
                            label: l10n.friendsReject,
                            color: AppColors.danger,
                            onPressed: () => _rejectRequest(person),
                          ),
                        ),
                      ],
                    ),
                  );
                }, childCount: provider.receiveRequests.length),
              ),
            ),
            const SliverToBoxAdapter(child: SizedBox(height: 16)),
          ],

          // Sent Requests
          if (hasSent) ...[
            SliverToBoxAdapter(
              child: Padding(
                padding: EdgeInsets.fromLTRB(16, hasReceived ? 0 : 16, 16, 8),
                child: _buildSectionHeader(
                  l10n.friendsSentRequests,
                  provider.sentRequests.length,
                  businessType,
                ),
              ),
            ),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverGrid(
                gridDelegate: _peopleGridDelegate,
                delegate: SliverChildBuilderDelegate((context, index) {
                  final person = provider.sentRequests[index];
                  return _buildPersonGridCard(
                    person: person,
                    businessType: businessType,
                    onTap: () => _openPersonProfile(person),
                    action: _buildFullWidthAction(
                      label: l10n.friendsCancel,
                      icon: Icons.cancel_outlined,
                      color: AppColors.danger,
                      onPressed: () => _cancelRequest(person),
                    ),
                  );
                }, childCount: provider.sentRequests.length),
              ),
            ),
          ],
          const SliverToBoxAdapter(child: SizedBox(height: 80)),
        ],
      ),
    );
  }

  Widget _buildSuggestionsTab(
    FriendsProvider provider,
    AppLocalizations l10n,
    String? businessType,
  ) {
    return RefreshIndicator(
      onRefresh: () async => _fetchFriends(),
      child: CustomScrollView(
        physics: const AlwaysScrollableScrollPhysics(),
        slivers: [
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(16, 16, 16, 4),
              child: _buildLocationModeChips(
                l10n: l10n,
                businessType: businessType,
                options: const [
                  LocationMode.savedAddress,
                  LocationMode.currentGps,
                ],
                selected: _suggestionsLocationMode,
                onSelected: _setSuggestionsLocationMode,
              ),
            ),
          ),
          if (provider.suggestions.isEmpty)
            SliverFillRemaining(
              hasScrollBody: false,
              child: _buildEmptyState(
                icon: Icons.person_search,
                title: l10n.friendsNoSuggestions,
                subtitle: l10n.friendsNoSuggestionsHint,
              ),
            )
          else
            SliverPadding(
              padding: const EdgeInsets.fromLTRB(16, 8, 16, 80),
              sliver: SliverGrid(
                gridDelegate: _peopleGridDelegate,
                delegate: SliverChildBuilderDelegate((context, index) {
                  final person = provider.suggestions[index];
                  return _buildPersonGridCard(
                    person: person,
                    businessType: businessType,
                    onTap: () => _openPersonProfile(person),
                    action: _buildFullWidthAction(
                      label: l10n.friendsAddFriend,
                      icon: Icons.person_add,
                      color: AppColors.primaryFor(businessType),
                      onPressed: () => _sendFriendRequest(person),
                    ),
                  );
                }, childCount: provider.suggestions.length),
              ),
            ),
        ],
      ),
    );
  }

  void _openPersonProfile(Person person) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => PersonProfilePage(person: person)),
    );
  }

  Widget _buildSectionHeader(String title, int count, String? businessType) {
    return Row(
      children: [
        Text(
          title,
          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
        ),
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

Widget _buildAvatar(Person person, String? businessType, {double size = 50}) {
  return Container(
    width: size,
    height: size,
    decoration: BoxDecoration(
      shape: BoxShape.circle,
      border: Border.all(
        color: AppColors.primaryFor(businessType).withAlpha(51),
        width: 2,
      ),
    ),
    child: ClipOval(
      child: person.avatar != null
          ? CachedNetworkImage(
              imageUrl: person.avatar!,
              fit: BoxFit.cover,
              placeholder: (context, url) => Container(
                color: AppColors.primaryFor(businessType).withAlpha(51),
                child: const Center(
                  child: CircularProgressIndicator(strokeWidth: 2),
                ),
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

const _peopleGridDelegate = SliverGridDelegateWithFixedCrossAxisCount(
  crossAxisCount: 2,
  mainAxisSpacing: 12,
  crossAxisSpacing: 12,
  childAspectRatio: 0.72,
);

/// Shared portrait card for friends, requests, suggestions, and search results.
Widget _buildPersonGridCard({
  required Person person,
  required String? businessType,
  required VoidCallback onTap,
  required Widget action,
}) {
  return Card(
    elevation: 1,
    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
    clipBehavior: Clip.antiAlias,
    child: InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _buildAvatar(person, businessType, size: 84),
            const SizedBox(height: 12),
            Text(
              person.fullName,
              textAlign: TextAlign.center,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(fontSize: 15, fontWeight: FontWeight.w600),
            ),
            if (person.personInfo?.currentCity != null) ...[
              const SizedBox(height: 2),
              Text(
                person.personInfo!.currentCity!,
                textAlign: TextAlign.center,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(fontSize: 12, color: Colors.grey[600]),
              ),
            ],
            const Spacer(),
            const SizedBox(height: 8),
            SizedBox(width: double.infinity, child: action),
          ],
        ),
      ),
    ),
  );
}

Widget _buildFullWidthAction({
  required String label,
  required IconData icon,
  required Color color,
  required VoidCallback onPressed,
}) {
  return OutlinedButton.icon(
    onPressed: onPressed,
    icon: Icon(icon, size: 18),
    label: Text(label, maxLines: 1, overflow: TextOverflow.ellipsis),
    style: OutlinedButton.styleFrom(
      foregroundColor: color,
      side: BorderSide(color: color),
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
    ),
  );
}

Widget _buildCompactAction({
  required String label,
  required Color color,
  required VoidCallback onPressed,
}) {
  return OutlinedButton(
    onPressed: onPressed,
    style: OutlinedButton.styleFrom(
      foregroundColor: color,
      side: BorderSide(color: color),
      padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 8),
    ),
    child: Text(
      label,
      maxLines: 1,
      overflow: TextOverflow.ellipsis,
      style: const TextStyle(fontSize: 12),
    ),
  );
}

/// Shared location-mode picker used by both the Suggestions tab (always one
/// of the two location-backed modes) and the Find Friends tab (which also
/// offers `anywhere`, i.e. no location filter at all).
Widget _buildLocationModeChips({
  required AppLocalizations l10n,
  required String? businessType,
  required List<LocationMode> options,
  required LocationMode selected,
  required ValueChanged<LocationMode> onSelected,
}) {
  String labelFor(LocationMode mode) {
    switch (mode) {
      case LocationMode.anywhere:
        return l10n.friendsLocationAnywhere;
      case LocationMode.savedAddress:
        return l10n.friendsLocationSavedAddress;
      case LocationMode.currentGps:
        return l10n.friendsLocationCurrentGps;
    }
  }

  return Wrap(
    spacing: 8,
    runSpacing: 8,
    children: options.map((mode) {
      final isSelected = mode == selected;
      return ChoiceChip(
        label: Text(labelFor(mode)),
        selected: isSelected,
        onSelected: (_) => onSelected(mode),
        selectedColor: AppColors.primaryFor(businessType).withAlpha(51),
        labelStyle: TextStyle(
          color: isSelected
              ? AppColors.primaryFor(businessType)
              : Colors.grey[700],
          fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
        ),
      );
    }).toList(),
  );
}

/// A page-local search: name/username text plus an optional location filter,
/// combined into a single "find friends" query. Results are transient, so
/// this keeps its own state rather than living on the shared [FriendsProvider].
class _FindFriendsTab extends StatefulWidget {
  final String? businessType;

  const _FindFriendsTab({required this.businessType});

  @override
  State<_FindFriendsTab> createState() => _FindFriendsTabState();
}

class _FindFriendsTabState extends State<_FindFriendsTab> {
  final _queryController = TextEditingController();
  Timer? _debounce;

  LocationMode _locationMode = LocationMode.anywhere;
  Position? _position;

  bool _searching = false;
  bool _hasSearched = false;
  List<Person> _results = [];

  @override
  void dispose() {
    _debounce?.cancel();
    _queryController.dispose();
    super.dispose();
  }

  void _onQueryChanged(String _) {
    _debounce?.cancel();
    _debounce = Timer(const Duration(milliseconds: 400), _runSearch);
  }

  Future<void> _setLocationMode(LocationMode mode) async {
    if (mode == LocationMode.currentGps) {
      final position = await LocationUtils.getCurrentPosition(context);
      if (!mounted) return;
      if (position == null) {
        return; // Location unavailable: keep the previous mode.
      }
      setState(() {
        _locationMode = mode;
        _position = position;
      });
    } else {
      setState(() => _locationMode = mode);
    }
    _runSearch();
  }

  Future<void> _runSearch() async {
    final query = _queryController.text.trim();
    final hasLocation =
        _locationMode == LocationMode.currentGps && _position != null;

    if (query.isEmpty && !hasLocation) {
      setState(() {
        _results = [];
        _hasSearched = false;
        _searching = false;
      });
      return;
    }

    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    if (token.isEmpty) return;

    setState(() => _searching = true);
    try {
      final results = await FriendsService.searchFriends(
        token: token,
        query: query.isEmpty ? null : query,
        latitude: hasLocation ? _position!.latitude : null,
        longitude: hasLocation ? _position!.longitude : null,
      );
      if (!mounted) return;
      setState(() {
        _results = results;
        _searching = false;
        _hasSearched = true;
      });
    } catch (_) {
      if (!mounted) return;
      setState(() {
        _results = [];
        _searching = false;
        _hasSearched = true;
      });
    }
  }

  Future<void> _sendFriendRequest(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    final friendsProvider = context.read<FriendsProvider>();
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    final success = await friendsProvider.sendFriendRequest(person.id, token);

    if (!mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          success ? l10n.friendsRequestSent : l10n.friendsActionError,
        ),
        backgroundColor: success ? AppColors.success : AppColors.danger,
      ),
    );
    if (success) {
      setState(() => _results.removeWhere((p) => p.id == person.id));
    }
  }

  void _openPersonProfile(Person person) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => PersonProfilePage(person: person)),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 16, 16, 4),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              TextField(
                controller: _queryController,
                onChanged: _onQueryChanged,
                decoration: InputDecoration(
                  hintText: l10n.friendsFindSearchHint,
                  prefixIcon: const Icon(Icons.search),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  isDense: true,
                ),
              ),
              const SizedBox(height: 12),
              _buildLocationModeChips(
                l10n: l10n,
                businessType: widget.businessType,
                options: const [
                  LocationMode.anywhere,
                  LocationMode.savedAddress,
                  LocationMode.currentGps,
                ],
                selected: _locationMode,
                onSelected: _setLocationMode,
              ),
            ],
          ),
        ),
        Expanded(child: _buildBody(l10n)),
      ],
    );
  }

  Widget _buildBody(AppLocalizations l10n) {
    if (_searching) {
      return const Center(child: CircularProgressIndicator());
    }

    if (!_hasSearched) {
      return _buildEmptyState(
        icon: Icons.search,
        title: l10n.friendsFindPromptTitle,
        subtitle: l10n.friendsFindPromptHint,
      );
    }

    if (_results.isEmpty) {
      return _buildEmptyState(
        icon: Icons.person_search,
        title: l10n.friendsFindNoResults,
        subtitle: l10n.friendsFindNoResultsHint,
      );
    }

    return GridView.builder(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 80),
      gridDelegate: _peopleGridDelegate,
      itemCount: _results.length,
      itemBuilder: (context, index) {
        final person = _results[index];
        return _buildPersonGridCard(
          person: person,
          businessType: widget.businessType,
          onTap: () => _openPersonProfile(person),
          action: _buildFullWidthAction(
            label: l10n.friendsAddFriend,
            icon: Icons.person_add,
            color: AppColors.primaryFor(widget.businessType),
            onPressed: () => _sendFriendRequest(person),
          ),
        );
      },
    );
  }
}
