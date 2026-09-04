import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../providers/chat_provider.dart';
import '../../providers/friends_provider.dart';
import '../../utils/open_direct_chat.dart';
import '../../widgets/person_avatar_widget.dart';

/// Friend picker that opens (or reuses) a direct conversation.
///
/// Only friends are listed, and the search filters that same list rather than
/// hitting `FriendsService.searchFriends`: that endpoint also returns people
/// you are not friends with, and the backend refuses a direct chat with them
/// (`chatNotFriends`) — offering them here would only produce dead taps.
class NewConversationPage extends StatefulWidget {
  const NewConversationPage({super.key});

  @override
  State<NewConversationPage> createState() => _NewConversationPageState();
}

class _NewConversationPageState extends State<NewConversationPage> {
  final TextEditingController _search = TextEditingController();
  bool _opening = false;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _bootstrap());
  }

  @override
  void dispose() {
    _search.dispose();
    super.dispose();
  }

  Future<void> _bootstrap() async {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    if (token.isEmpty) return;
    final friends = context.read<FriendsProvider>();
    if (friends.friends.isEmpty) {
      await friends.fetchFriends(token);
    }
    if (!mounted) return;
    await context.read<ChatProvider>().refreshPresence(
      friends.friends.map((f) => f.uuid).toList(),
    );
  }

  List<Person> _visible(List<Person> friends) {
    final query = _search.text.trim().toLowerCase();
    if (query.isEmpty) return friends;
    return friends
        .where((f) => f.fullName.toLowerCase().contains(query))
        .toList();
  }

  Future<void> _openWith(Person person) async {
    if (_opening) return;
    setState(() => _opening = true);
    await openDirectChat(
      context,
      personUuid: person.uuid,
      displayName: person.fullName,
      replace: true,
    );
    if (mounted) setState(() => _opening = false);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final chat = context.watch<ChatProvider>();
    final all = context.watch<FriendsProvider>().friends;
    final visible = _visible(all);

    return Scaffold(
      appBar: AppBar(title: Text(l10n.chatStartConversationTitle)),
      body: Column(
        children: [
          if (all.length > 8)
            Padding(
              padding: const EdgeInsets.fromLTRB(12, 12, 12, 4),
              child: TextField(
                controller: _search,
                onChanged: (_) => setState(() {}),
                decoration: InputDecoration(
                  hintText: l10n.chatSearchHint,
                  prefixIcon: const Icon(Icons.search),
                  border: const OutlineInputBorder(),
                  isDense: true,
                  suffixIcon: _search.text.isEmpty
                      ? null
                      : IconButton(
                          icon: const Icon(Icons.close),
                          onPressed: () {
                            _search.clear();
                            setState(() {});
                          },
                        ),
                ),
              ),
            ),
          Expanded(
            child: all.isEmpty
                ? Center(
                    child: Padding(
                      padding: const EdgeInsets.all(32),
                      child: Text(
                        l10n.chatNoFriendsYet,
                        textAlign: TextAlign.center,
                      ),
                    ),
                  )
                : ListView.separated(
                    itemCount: visible.length,
                    separatorBuilder: (_, _) => const Divider(height: 1),
                    itemBuilder: (context, index) {
                      final person = visible[index];
                      final online = chat.isOnline(person.uuid);
                      return ListTile(
                        enabled: !_opening,
                        leading: PersonAvatar(person: person),
                        title: Text(person.fullName),
                        subtitle: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Icon(
                              Icons.brightness_1,
                              size: 10,
                              color: online
                                  ? AppColors.success
                                  : Colors.grey,
                            ),
                            const SizedBox(width: 6),
                            Text(online ? l10n.chatOnline : l10n.chatOffline),
                          ],
                        ),
                        onTap: () => _openWith(person),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }
}
