import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/auth_provider.dart';
import '../../providers/chat_provider.dart';
import '../../providers/friends_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/chat_socket.dart';
import '../../widgets/chat/conversation_tile.dart';
import '../../widgets/main_layout.dart';
import 'chat_thread_page.dart';
import 'new_conversation_page.dart';

class ConversationsPage extends StatefulWidget {
  const ConversationsPage({super.key});

  @override
  State<ConversationsPage> createState() => _ConversationsPageState();
}

class _ConversationsPageState extends State<ConversationsPage> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _bootstrap());
  }

  Future<void> _bootstrap() async {
    final auth = context.read<AuthProvider>();
    final token = auth.auth?.accessToken ?? '';
    if (token.isEmpty) return;
    final myUuid =
        auth.auth?.personUuid ?? context.read<PersonProvider>().ownerUuid;
    final chat = context.read<ChatProvider>();
    final friends = context.read<FriendsProvider>();

    chat.connect(token, myUuid);
    await chat.fetchConversations();
    // Names and avatars for direct conversations come from the friends list —
    // the conversation payload only carries participant uuids.
    if (!mounted) return;
    if (friends.friends.isEmpty) await friends.fetchFriends(token);
    if (!mounted) return;
    await chat.refreshPresence(
      friends.friends.map((f) => f.uuid).toList(),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final chat = context.watch<ChatProvider>();
    final friends = context.watch<FriendsProvider>().friends;
    final myUuid = chat.myPersonUuid;

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/chat',
      floatingActionButton: FloatingActionButton(
        tooltip: l10n.chatNewConversation,
        onPressed: () => Navigator.of(context).push(
          MaterialPageRoute(builder: (_) => const NewConversationPage()),
        ),
        child: const Icon(Icons.add_comment_outlined),
      ),
      body: RefreshIndicator(
        onRefresh: _bootstrap,
        child: Column(
          children: [
            if (chat.socketStatus != ChatSocketStatus.connected)
              Container(
                width: double.infinity,
                color: Colors.amber.shade100,
                padding: const EdgeInsets.symmetric(
                  vertical: 4,
                  horizontal: 12,
                ),
                child: Text(
                  l10n.chatReconnecting,
                  style: const TextStyle(fontSize: 12),
                ),
              ),
            Expanded(
              child: chat.loadingConversations && chat.conversations.isEmpty
                  ? const Center(child: CircularProgressIndicator())
                  : chat.conversations.isEmpty
                  ? ListView(
                      children: [
                        const SizedBox(height: 120),
                        Center(child: Text(l10n.chatEmpty)),
                      ],
                    )
                  : ListView.separated(
                      itemCount: chat.conversations.length,
                      separatorBuilder: (_, _) => const Divider(height: 1),
                      itemBuilder: (context, index) {
                        final conversation = chat.conversations[index];
                        final counterpart = counterpartOf(
                          conversation,
                          myUuid,
                          friends,
                        );
                        return ConversationTile(
                          conversation: conversation,
                          counterpart: counterpart,
                          myPersonUuid: myUuid,
                          online:
                              counterpart != null &&
                              chat.isOnline(counterpart.uuid),
                          onTap: () => Navigator.of(context).push(
                            MaterialPageRoute(
                              builder: (_) => ChatThreadPage(
                                conversationUuid: conversation.uuid,
                                title: conversationTitle(
                                  conversation,
                                  counterpart,
                                  myUuid,
                                  l10n,
                                ),
                                counterpartPersonUuid: counterpart?.uuid,
                              ),
                            ),
                          ),
                        );
                      },
                    ),
            ),
          ],
        ),
      ),
    );
  }
}
