import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/conversation.dart';
import '../../models/person.dart';
import '../../utils/chat_time.dart';
import '../person_avatar_widget.dart';

/// Resolves who a direct conversation is *with*.
///
/// The backend only ships participant uuids, and a direct chat is friends-only
/// (it rejects anyone else with `chatNotFriends`), so the counterpart is always
/// somewhere in the friends list — no extra round-trip needed.
Person? counterpartOf(
  Conversation conversation,
  String myPersonUuid,
  List<Person> friends,
) {
  final uuid = conversation.counterpartUuidFor(myPersonUuid);
  if (uuid == null) return null;
  for (final friend in friends) {
    if (friend.uuid == uuid) return friend;
  }
  return null;
}

/// Title for a conversation row: the friend's name for a direct chat, the
/// business name for a business one. Falls back to the last message's sender —
/// but never when that sender is me, which is what used to put the user's own
/// name on their own conversations.
String conversationTitle(
  Conversation conversation,
  Person? counterpart,
  String myPersonUuid,
  AppLocalizations l10n,
) {
  if (conversation.conversationType == 'BusinessTeamGroup') {
    return conversation.businessProfileName ?? l10n.chatTeamGroup;
  }
  if (conversation.conversationType == 'BusinessDirect') {
    return conversation.businessProfileName ?? l10n.chatBusinessDirect;
  }
  if (counterpart != null) return counterpart.fullName;
  final last = conversation.lastMessage;
  if (last != null &&
      last.senderPersonUuid != myPersonUuid &&
      last.senderDisplayName.isNotEmpty) {
    return last.senderDisplayName;
  }
  return l10n.chatConversationsTitle;
}

class ConversationTile extends StatelessWidget {
  final Conversation conversation;
  final Person? counterpart;
  final String myPersonUuid;
  final bool online;
  final VoidCallback onTap;

  const ConversationTile({
    super.key,
    required this.conversation,
    required this.counterpart,
    required this.myPersonUuid,
    required this.online,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final locale = Localizations.localeOf(context).toString();
    final title = conversationTitle(
      conversation,
      counterpart,
      myPersonUuid,
      l10n,
    );
    final last = conversation.lastMessage;
    final mine = last != null && last.senderPersonUuid == myPersonUuid;

    return ListTile(
      leading: _Avatar(
        conversation: conversation,
        counterpart: counterpart,
        online: online,
      ),
      title: Text(
        title,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
        style: TextStyle(
          fontWeight: conversation.unread ? FontWeight.bold : FontWeight.normal,
        ),
      ),
      subtitle: last == null
          ? null
          : Text(
              mine ? '${l10n.chatYou}: ${last.snippet}' : last.snippet,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
      trailing: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Text(
            ChatTime.conversationStamp(conversation.updatedAt, l10n, locale),
            style: TextStyle(
              fontSize: 11,
              color: conversation.unread
                  ? AppColors.primary
                  : Colors.grey.shade600,
              fontWeight: conversation.unread
                  ? FontWeight.bold
                  : FontWeight.normal,
            ),
          ),
          const SizedBox(height: 4),
          if (conversation.unread)
            const Icon(
              Icons.brightness_1,
              size: 10,
              color: AppColors.primary,
            )
          else
            const SizedBox(height: 10),
        ],
      ),
      onTap: onTap,
    );
  }
}

/// Person avatar for a direct chat, business logo for a business one, with the
/// online dot overlaid.
class _Avatar extends StatelessWidget {
  final Conversation conversation;
  final Person? counterpart;
  final bool online;

  const _Avatar({
    required this.conversation,
    required this.counterpart,
    required this.online,
  });

  @override
  Widget build(BuildContext context) {
    final logo = conversation.businessProfileLogoUrl;
    final Widget avatar = counterpart != null
        ? PersonAvatar(person: counterpart)
        : CircleAvatar(
            backgroundColor: AppColors.primary.withValues(alpha: 0.15),
            backgroundImage: logo != null ? NetworkImage(logo) : null,
            child: logo == null ? const Icon(Icons.chat_bubble_outline) : null,
          );

    if (!online) return avatar;
    return Stack(
      clipBehavior: Clip.none,
      children: [
        avatar,
        Positioned(
          right: -1,
          bottom: -1,
          child: Container(
            width: 13,
            height: 13,
            decoration: BoxDecoration(
              color: AppColors.success,
              shape: BoxShape.circle,
              border: Border.all(color: Colors.white, width: 2),
            ),
          ),
        ),
      ],
    );
  }
}
