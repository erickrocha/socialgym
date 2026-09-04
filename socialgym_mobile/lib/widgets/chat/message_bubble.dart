import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/chat_message.dart';
import '../../utils/chat_time.dart';

/// One message. Own messages sit right and carry a delivery state; incoming
/// ones sit left and carry the sender's name (which matters in group chats and
/// for business senders).
class MessageBubble extends StatelessWidget {
  final ChatMessage message;
  final bool mine;
  final bool readByCounterpart;
  final VoidCallback? onRetry;

  const MessageBubble({
    super.key,
    required this.message,
    required this.mine,
    required this.readByCounterpart,
    this.onRetry,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final locale = Localizations.localeOf(context).toString();
    final onBubble = mine ? Colors.white : Colors.black87;

    return Align(
      alignment: mine ? Alignment.centerRight : Alignment.centerLeft,
      child: GestureDetector(
        onTap: message.failed ? onRetry : null,
        child: Container(
          margin: const EdgeInsets.symmetric(vertical: 3),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          constraints: BoxConstraints(
            maxWidth: MediaQuery.of(context).size.width * 0.72,
          ),
          decoration: BoxDecoration(
            color: mine
                ? (message.failed
                      ? AppColors.danger
                      : AppColors.primary.withValues(
                          alpha: message.pending ? 0.6 : 1,
                        ))
                : Colors.grey.shade200,
            borderRadius: BorderRadius.circular(14),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (!mine)
                Text(
                  message.isFromBusiness
                      ? '${message.senderDisplayName} 🏢'
                      : message.senderDisplayName,
                  style: const TextStyle(fontSize: 11, color: Colors.black54),
                ),
              if (message.body.isNotEmpty)
                Text(message.body, style: TextStyle(color: onBubble)),
              for (final media in message.media)
                if (media.url.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 4),
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: CachedNetworkImage(
                        imageUrl: media.url,
                        width: 200,
                        fit: BoxFit.cover,
                        placeholder: (_, _) => const SizedBox(
                          width: 200,
                          height: 120,
                          child: Center(child: CircularProgressIndicator()),
                        ),
                        errorWidget: (_, _, _) =>
                            const Icon(Icons.broken_image_outlined),
                      ),
                    ),
                  ),
              const SizedBox(height: 2),
              _Footer(
                message: message,
                mine: mine,
                readByCounterpart: readByCounterpart,
                l10n: l10n,
                locale: locale,
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _Footer extends StatelessWidget {
  final ChatMessage message;
  final bool mine;
  final bool readByCounterpart;
  final AppLocalizations l10n;
  final String locale;

  const _Footer({
    required this.message,
    required this.mine,
    required this.readByCounterpart,
    required this.l10n,
    required this.locale,
  });

  @override
  Widget build(BuildContext context) {
    final faint = mine ? Colors.white70 : Colors.black45;

    if (message.failed) {
      return Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          const Icon(Icons.error_outline, size: 13, color: Colors.white),
          const SizedBox(width: 4),
          Flexible(
            child: Text(
              l10n.chatFailedTap,
              style: const TextStyle(fontSize: 10, color: Colors.white),
            ),
          ),
        ],
      );
    }

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          ChatTime.messageStamp(message.sentAt, locale),
          style: TextStyle(fontSize: 10, color: faint),
        ),
        if (mine) ...[
          const SizedBox(width: 4),
          if (message.pending)
            Icon(Icons.schedule, size: 12, color: faint)
          else
            Tooltip(
              message: readByCounterpart ? l10n.chatRead : l10n.chatSent,
              child: Icon(
                readByCounterpart ? Icons.done_all : Icons.done,
                size: 13,
                color: readByCounterpart ? Colors.lightBlueAccent : faint,
              ),
            ),
        ],
      ],
    );
  }
}

/// The `Today` / `Yesterday` / date chip between two days of messages.
class DaySeparator extends StatelessWidget {
  final DateTime day;

  const DaySeparator({super.key, required this.day});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final locale = Localizations.localeOf(context).toString();
    return Center(
      child: Container(
        margin: const EdgeInsets.symmetric(vertical: 10),
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 3),
        decoration: BoxDecoration(
          color: Colors.black.withValues(alpha: 0.06),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Text(
          ChatTime.dayLabel(day, l10n, locale),
          style: const TextStyle(fontSize: 11, color: Colors.black54),
        ),
      ),
    );
  }
}
