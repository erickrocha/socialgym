import 'dart:io';

import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/chat_message.dart';
import '../../providers/chat_provider.dart';
import '../../utils/chat_time.dart';
import '../../widgets/chat/message_bubble.dart';

class ChatThreadPage extends StatefulWidget {
  final String conversationUuid;
  final String title;

  /// The person on the other end of a direct chat, when known. Only used to
  /// label the typing indicator — the thread works without it.
  final String? counterpartPersonUuid;

  const ChatThreadPage({
    super.key,
    required this.conversationUuid,
    required this.title,
    this.counterpartPersonUuid,
  });

  @override
  State<ChatThreadPage> createState() => _ChatThreadPageState();
}

class _ChatThreadPageState extends State<ChatThreadPage> {
  final TextEditingController _controller = TextEditingController();
  // Reversed list: offset 0 is the newest message, so new arrivals never need
  // a scroll nudge and loading older history never shifts what you are reading.
  final ScrollController _scrollController = ScrollController();
  final ImagePicker _picker = ImagePicker();
  List<XFile> _pendingImages = [];
  String? _lastMarkedUuid;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      final chat = context.read<ChatProvider>();
      await chat.fetchMessages(widget.conversationUuid);
      if (!mounted) return;
      _markRead();
    });
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _controller.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (!_scrollController.hasClients) return;
    final position = _scrollController.position;
    // In a reversed list the far end is the oldest message.
    if (position.pixels >= position.maxScrollExtent - 240) {
      context.read<ChatProvider>().loadOlder(widget.conversationUuid);
    }
  }

  /// Marks the thread read at most once per incoming message, instead of on
  /// every rebuild.
  void _markRead() {
    final chat = context.read<ChatProvider>();
    final messages = chat.messagesFor(widget.conversationUuid);
    if (messages.isEmpty) return;
    final last = messages.last;
    if (last.senderPersonUuid == chat.myPersonUuid) return;
    if (_lastMarkedUuid == last.uuid) return;
    _lastMarkedUuid = last.uuid;
    chat.markRead(widget.conversationUuid);
  }

  Future<void> _pickImages() async {
    final picked = await _picker.pickMultiImage();
    if (picked.isNotEmpty && mounted) {
      setState(() => _pendingImages = [..._pendingImages, ...picked]);
    }
  }

  Future<void> _send() async {
    final text = _controller.text.trim();
    if (text.isEmpty && _pendingImages.isEmpty) return;
    final chat = context.read<ChatProvider>();
    final images = _pendingImages;
    _controller.clear();
    setState(() => _pendingImages = []);
    await chat.send(widget.conversationUuid, body: text, images: images);
  }

  /// Messages plus day separators, newest first (the list renders reversed).
  List<Widget> _entries(List<ChatMessage> messages, ChatProvider chat) {
    final entries = <Widget>[];
    DateTime? previousDay;
    for (final message in messages) {
      if (ChatTime.startsNewDay(previousDay, message.sentAt)) {
        entries.add(DaySeparator(key: ValueKey('day-${message.uuid}-${message.clientMessageId}'), day: message.sentAt));
      }
      previousDay = message.sentAt;
      final mine = message.senderPersonUuid == chat.myPersonUuid;
      entries.add(
        MessageBubble(
          key: ValueKey(
            message.uuid.isNotEmpty ? message.uuid : message.clientMessageId,
          ),
          message: message,
          mine: mine,
          readByCounterpart:
              mine &&
              chat.isReadByCounterpart(widget.conversationUuid, message.uuid),
          onRetry: () =>
              chat.resend(widget.conversationUuid, message.clientMessageId),
        ),
      );
    }
    return entries.reversed.toList();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final chat = context.watch<ChatProvider>();
    final messages = chat.messagesFor(widget.conversationUuid);
    final entries = _entries(messages, chat);

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _markRead();
    });

    return Scaffold(
      appBar: AppBar(title: Text(widget.title)),
      body: Column(
        children: [
          Expanded(
            child: chat.loadingMessages && messages.isEmpty
                ? const Center(child: CircularProgressIndicator())
                : ListView.builder(
                    controller: _scrollController,
                    reverse: true,
                    padding: const EdgeInsets.all(12),
                    itemCount: entries.length + (chat.loadingOlder ? 1 : 0),
                    itemBuilder: (context, index) {
                      if (index >= entries.length) {
                        return const Padding(
                          padding: EdgeInsets.symmetric(vertical: 12),
                          child: Center(
                            child: SizedBox(
                              width: 18,
                              height: 18,
                              child: CircularProgressIndicator(strokeWidth: 2),
                            ),
                          ),
                        );
                      }
                      return entries[index];
                    },
                  ),
          ),
          if (chat.isTypingIn(widget.conversationUuid))
            Align(
              alignment: Alignment.centerLeft,
              child: Padding(
                padding: const EdgeInsets.only(left: 16, bottom: 4),
                child: Text(
                  '${widget.title} ${l10n.chatTyping}',
                  style: const TextStyle(
                    fontSize: 12,
                    fontStyle: FontStyle.italic,
                    color: Colors.black54,
                  ),
                ),
              ),
            ),
          if (chat.error != null)
            Container(
              width: double.infinity,
              color: Colors.red.shade50,
              padding: const EdgeInsets.all(8),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      chat.error!,
                      style: TextStyle(
                        color: Colors.red.shade700,
                        fontSize: 12,
                      ),
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.close, size: 16),
                    onPressed: chat.clearError,
                  ),
                ],
              ),
            ),
          if (_pendingImages.isNotEmpty) _pendingImageStrip(),
          SafeArea(
            top: false,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
              child: Row(
                children: [
                  IconButton(
                    icon: const Icon(Icons.attach_file),
                    onPressed: _pickImages,
                  ),
                  Expanded(
                    child: TextField(
                      controller: _controller,
                      minLines: 1,
                      maxLines: 4,
                      textInputAction: TextInputAction.send,
                      onSubmitted: (_) => _send(),
                      decoration: InputDecoration(
                        hintText: l10n.chatMessageHint,
                        border: const OutlineInputBorder(),
                        isDense: true,
                      ),
                      onChanged: (_) =>
                          chat.sendTyping(widget.conversationUuid),
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.send),
                    color: AppColors.primary,
                    onPressed: chat.sending ? null : _send,
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _pendingImageStrip() {
    return SizedBox(
      height: 72,
      child: ListView.separated(
        scrollDirection: Axis.horizontal,
        padding: const EdgeInsets.symmetric(horizontal: 12),
        itemCount: _pendingImages.length,
        separatorBuilder: (_, _) => const SizedBox(width: 8),
        itemBuilder: (context, index) {
          final file = _pendingImages[index];
          return Stack(
            clipBehavior: Clip.none,
            children: [
              ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: Image.file(
                  File(file.path),
                  width: 64,
                  height: 64,
                  fit: BoxFit.cover,
                ),
              ),
              Positioned(
                top: -6,
                right: -6,
                child: IconButton(
                  iconSize: 16,
                  visualDensity: VisualDensity.compact,
                  icon: const CircleAvatar(
                    radius: 10,
                    backgroundColor: Colors.black54,
                    child: Icon(Icons.close, size: 12, color: Colors.white),
                  ),
                  onPressed: () =>
                      setState(() => _pendingImages.removeAt(index)),
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}
