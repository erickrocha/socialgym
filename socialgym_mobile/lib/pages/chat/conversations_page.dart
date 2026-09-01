import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/conversation.dart';
import '../../providers/auth_provider.dart';
import '../../providers/chat_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/chat_socket.dart';
import '../../widgets/main_layout.dart';
import 'chat_thread_page.dart';

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

  void _bootstrap() {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    final myUuid = context.read<AuthProvider>().auth?.personUuid ??
        context.read<PersonProvider>().ownerUuid;
    if (token.isEmpty) return;
    final chat = context.read<ChatProvider>();
    chat.connect(token, myUuid);
    chat.fetchConversations();
  }

  String _titleFor(Conversation c, AppLocalizations l10n) {
    if (c.conversationType == 'BusinessTeamGroup') {
      return c.businessProfileName ?? l10n.chatTeamGroup;
    }
    if (c.conversationType == 'BusinessDirect') {
      return c.businessProfileName ?? l10n.chatBusinessDirect;
    }
    return c.lastMessage?.senderDisplayName ?? l10n.chatConversationsTitle;
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final chat = context.watch<ChatProvider>();

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/chat',
      body: RefreshIndicator(
        onRefresh: () => chat.fetchConversations(),
        child: Column(
          children: [
            if (chat.socketStatus != ChatSocketStatus.connected)
              Container(
                width: double.infinity,
                color: Colors.amber.shade100,
                padding: const EdgeInsets.symmetric(vertical: 4, horizontal: 12),
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
                            final c = chat.conversations[index];
                            return ListTile(
                              leading: CircleAvatar(
                                backgroundColor: AppColors.primary.withValues(
                                  alpha: 0.15,
                                ),
                                backgroundImage: c.businessProfileLogoUrl != null
                                    ? NetworkImage(c.businessProfileLogoUrl!)
                                    : null,
                                child: c.businessProfileLogoUrl == null
                                    ? const Icon(Icons.chat_bubble_outline)
                                    : null,
                              ),
                              title: Text(
                                _titleFor(c, l10n),
                                style: TextStyle(
                                  fontWeight: c.unread
                                      ? FontWeight.bold
                                      : FontWeight.normal,
                                ),
                              ),
                              subtitle: Text(
                                c.lastMessage?.snippet ?? '',
                                maxLines: 1,
                                overflow: TextOverflow.ellipsis,
                              ),
                              trailing: c.unread
                                  ? const Icon(
                                      Icons.brightness_1,
                                      size: 10,
                                      color: AppColors.primary,
                                    )
                                  : null,
                              onTap: () => Navigator.of(context).push(
                                MaterialPageRoute(
                                  builder: (_) => ChatThreadPage(
                                    conversationUuid: c.uuid,
                                    title: _titleFor(c, l10n),
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
