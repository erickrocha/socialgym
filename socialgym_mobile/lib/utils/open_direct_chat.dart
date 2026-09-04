import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../pages/chat/chat_thread_page.dart';
import '../providers/chat_provider.dart';

/// Single entry point for "start (or resume) a direct chat with this person",
/// shared by the messages FAB, the friend picker, the friends list and the
/// person profile so the four never drift apart.
///
/// [replace] swaps the current route instead of stacking one — used by the
/// picker, so Back from the thread lands on the conversation list rather than
/// on the picker the user already finished with.
Future<void> openDirectChat(
  BuildContext context, {
  required String personUuid,
  required String displayName,
  bool replace = false,
}) async {
  final navigator = Navigator.of(context);
  final messenger = ScaffoldMessenger.of(context);
  final chat = context.read<ChatProvider>();

  final conversation = await chat.openDirect(personUuid);
  if (conversation == null) {
    final error = chat.error;
    if (error != null) {
      messenger.showSnackBar(SnackBar(content: Text(error)));
      chat.clearError();
    }
    return;
  }

  final route = MaterialPageRoute<void>(
    builder: (_) => ChatThreadPage(
      conversationUuid: conversation.uuid,
      title: displayName,
      counterpartPersonUuid: personUuid,
    ),
  );
  if (replace) {
    navigator.pushReplacement(route);
  } else {
    navigator.push(route);
  }
}
