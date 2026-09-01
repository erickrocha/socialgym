import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/chat_message.dart';
import '../../providers/chat_provider.dart';

class ChatThreadPage extends StatefulWidget {
  final String conversationUuid;
  final String title;

  const ChatThreadPage({
    super.key,
    required this.conversationUuid,
    required this.title,
  });

  @override
  State<ChatThreadPage> createState() => _ChatThreadPageState();
}

class _ChatThreadPageState extends State<ChatThreadPage> {
  final TextEditingController _controller = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  final ImagePicker _picker = ImagePicker();
  List<XFile> _pendingImages = [];

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      final chat = context.read<ChatProvider>();
      await chat.fetchMessages(widget.conversationUuid);
      await chat.markRead(widget.conversationUuid);
      _jumpToBottom();
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _jumpToBottom() {
    if (!_scrollController.hasClients) return;
    _scrollController.jumpTo(_scrollController.position.maxScrollExtent);
  }

  Future<void> _pickImages() async {
    final picked = await _picker.pickMultiImage();
    if (picked.isNotEmpty) {
      setState(() => _pendingImages = picked);
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
    await Future.delayed(const Duration(milliseconds: 100));
    _jumpToBottom();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final chat = context.watch<ChatProvider>();
    final messages = chat.messagesFor(widget.conversationUuid);
    final myUuid = chat.myPersonUuid;

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (messages.isNotEmpty) chat.markRead(widget.conversationUuid);
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
                    padding: const EdgeInsets.all(12),
                    itemCount: messages.length,
                    itemBuilder: (context, index) =>
                        _bubble(messages[index], myUuid),
                  ),
          ),
          if (chat.error != null)
            Container(
              width: double.infinity,
              color: Colors.red.shade50,
              padding: const EdgeInsets.all(8),
              child: Text(
                l10n.chatNotFriends,
                style: TextStyle(color: Colors.red.shade700, fontSize: 12),
              ),
            ),
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
                  if (_pendingImages.isNotEmpty)
                    Text('${_pendingImages.length} 🖼️'),
                  Expanded(
                    child: TextField(
                      controller: _controller,
                      minLines: 1,
                      maxLines: 4,
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

  Widget _bubble(ChatMessage message, String myUuid) {
    final mine = message.senderPersonUuid == myUuid;
    return Align(
      alignment: mine ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: const EdgeInsets.symmetric(vertical: 3),
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        constraints: BoxConstraints(
          maxWidth: MediaQuery.of(context).size.width * 0.72,
        ),
        decoration: BoxDecoration(
          color: mine ? AppColors.primary : Colors.grey.shade200,
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
              Text(
                message.body,
                style: TextStyle(color: mine ? Colors.white : Colors.black87),
              ),
            for (final media in message.media)
              if (media.url.isNotEmpty)
                Padding(
                  padding: const EdgeInsets.only(top: 4),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Image.network(
                      media.url,
                      width: 200,
                      fit: BoxFit.cover,
                    ),
                  ),
                ),
          ],
        ),
      ),
    );
  }
}
