import 'dart:async';

import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/mentionable_friend.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../providers/feed_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/grpc/grpc_person_service.dart';
import '../../services/upload_service.dart';
import '../../utils/display_name_helper.dart';
import '../../utils/mention_text_utils.dart';
import '../../utils/permissions_utils.dart';

// ─────────────────────────────────────────────────────────────────────────────
// Internal model for a selected media file
// ─────────────────────────────────────────────────────────────────────────────

class _MediaItem {
  final XFile file;
  final bool isVideo;
  final Uint8List? previewBytes;

  _MediaItem({required this.file, required this.isVideo, this.previewBytes});
}

// ─────────────────────────────────────────────────────────────────────────────
// PostComposerPage
// ─────────────────────────────────────────────────────────────────────────────

class PostComposerPage extends StatefulWidget {
  /// Optional pre-filled text content (e.g. workout share summary).
  final String? initialContent;

  /// Optional images already picked by the caller (e.g. the gallery FAB),
  /// attached to the post as soon as the page opens.
  final List<XFile>? initialImages;

  const PostComposerPage({super.key, this.initialContent, this.initialImages});

  /// Pushes the composer and awaits its result.
  /// Returns `true` if a post was successfully created.
  static Future<bool?> push(
    BuildContext context, {
    String? initialContent,
    List<XFile>? initialImages,
  }) {
    return Navigator.of(context).push<bool>(
      MaterialPageRoute(
        builder: (_) => PostComposerPage(
          initialContent: initialContent,
          initialImages: initialImages,
        ),
      ),
    );
  }

  @override
  State<PostComposerPage> createState() => _PostComposerPageState();
}

class _PostComposerPageState extends State<PostComposerPage> {
  late final TextEditingController _controller;
  final ImagePicker _picker = ImagePicker();
  final List<_MediaItem> _mediaItems = [];
  final List<MentionableFriend> mentions = [];

  bool _uploading = false;
  bool _posting = false;
  bool _thirdPartyConsentConfirmed = false;
  String? _error;
  Timer? _mentionDebounce;
  MentionQuery? _activeMention;
  List<MentionableFriend> _mentionSuggestions = const [];
  bool _isSearchingMentions = false;
  int _mentionRequestId = 0;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.initialContent ?? '');
    _controller.addListener(_onComposerTextChanged);

    final initialImages = widget.initialImages;
    if (initialImages != null && initialImages.isNotEmpty) {
      _attachImages(initialImages);
    }
  }

  /// Reads each picked image into memory as a preview and appends it to
  /// [_mediaItems]. Shared by the initial images and the gallery picker.
  Future<void> _attachImages(List<XFile> files) async {
    final items = <_MediaItem>[];
    for (final xf in files) {
      items.add(
        _MediaItem(
          file: xf,
          isVideo: false,
          previewBytes: await xf.readAsBytes(),
        ),
      );
    }
    if (!mounted) return;
    setState(() => _mediaItems.addAll(items));
  }

  @override
  void dispose() {
    _mentionDebounce?.cancel();
    _controller.removeListener(_onComposerTextChanged);
    _controller.dispose();
    super.dispose();
  }

  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';

  int get _personId => context.read<AuthProvider>().auth?.personId ?? 0;

  Person? get _person => context.read<PersonProvider>().person;

  bool get _isBusinessMode => context.read<PersonProvider>().isProfessional;

  int? get _businessProfileId =>
      context.read<PersonProvider>().activeBusinessProfile?.id;

  String? get _businessProfileUuid =>
      context.read<PersonProvider>().activeBusinessProfile?.uuid;

  String? get _businessProfileName {
    final profile = context.read<PersonProvider>().activeBusinessProfile;
    return profile != null
        ? DisplayNameHelper.getBusinessProfileDisplayName(profile)
        : null;
  }

  String? get _businessProfileLogo =>
      context.read<PersonProvider>().activeBusinessProfile?.logo;

  bool get _isLoading => _uploading || _posting;

  void _onComposerTextChanged() {
    final selection = _controller.selection;
    if (!selection.isValid || !selection.isCollapsed) {
      _clearMentionSuggestions();
      return;
    }

    final query = findActiveMentionQuery(
      _controller.text,
      selection.baseOffset,
    );
    if (query == null || query.token.length < 2) {
      _clearMentionSuggestions();
      return;
    }

    _activeMention = query;
    _mentionDebounce?.cancel();
    _mentionDebounce = Timer(const Duration(milliseconds: 300), () {
      _searchMentions(query.token);
    });
  }

  Future<void> _searchMentions(String rawQuery) async {
    final personId = _personId;
    if (personId <= 0) {
      _clearMentionSuggestions();
      return;
    }

    final requestId = ++_mentionRequestId;
    setState(() => _isSearchingMentions = true);

    try {
      final suggestions = await GrpcPersonService.searchMentionableFriends(
        personId: personId,
        query: rawQuery,
        limit: 20,
      );

      if (!mounted || requestId != _mentionRequestId) return;
      setState(() {
        _mentionSuggestions = suggestions;
        _isSearchingMentions = false;
      });
    } catch (error) {
      if (!mounted || requestId != _mentionRequestId) return;
      setState(() {
        _mentionSuggestions = const [];
        _isSearchingMentions = false;
      });
    }
  }

  void _clearMentionSuggestions() {
    _mentionDebounce?.cancel();
    _activeMention = null;
    _mentionRequestId++;
    if (_mentionSuggestions.isEmpty && !_isSearchingMentions) return;
    setState(() {
      _mentionSuggestions = const [];
      _isSearchingMentions = false;
    });
  }

  void _selectMention(MentionableFriend friend) {
    final mention = _activeMention;
    if (mention == null) return;

    final updated = replaceMentionQuery(
      text: _controller.text,
      query: mention,
      mentionDisplay: friend.fullName,
    );
    final cursor = mention.start + friend.fullName.length + 2;
    _controller.value = TextEditingValue(
      text: updated,
      selection: TextSelection.collapsed(offset: cursor),
    );
    mentions.add(friend);
    _clearMentionSuggestions();
  }

  // ── Media picking ───────────────────────────────────────────────────────────

  Future<void> _pickImages() async {
    try {
      final permitted = await PermissionsUtils.requestPhotosPermission();
      if (!permitted) {
        if (!mounted) return;
        final isDenied =
            await PermissionsUtils.isPhotosPermissionPermanentlyDenied();
        _showPermissionDeniedSnackbar('Photo', isDenied);
        return;
      }

      final results = await _picker.pickMultiImage(imageQuality: 85);
      if (results.isEmpty) return;

      await _attachImages(results);
    } catch (_) {
      if (!mounted) return;
      _showGenericMediaError();
    }
  }

  Future<void> _pickImageFromCamera() async {
    try {
      final permitted = await PermissionsUtils.requestCameraPermission();
      if (!permitted) {
        if (!mounted) return;
        final isDenied =
            await PermissionsUtils.isCameraPermissionPermanentlyDenied();
        _showPermissionDeniedSnackbar('Camera', isDenied);
        return;
      }

      final result = await _picker.pickImage(
        source: ImageSource.camera,
        imageQuality: 85,
      );
      if (result == null || !mounted) return;
      setState(() {
        _mediaItems.add(_MediaItem(file: result, isVideo: false));
      });
    } catch (_) {
      if (!mounted) return;
      _showGenericMediaError();
    }
  }

  Future<void> _pickVideo() async {
    try {
      final permitted = await PermissionsUtils.requestPhotosPermission();
      if (!permitted) {
        if (!mounted) return;
        final isDenied =
            await PermissionsUtils.isPhotosPermissionPermanentlyDenied();
        _showPermissionDeniedSnackbar('Photo', isDenied);
        return;
      }

      final result = await _picker.pickVideo(source: ImageSource.gallery);
      if (result == null || !mounted) return;
      setState(() {
        _mediaItems.add(_MediaItem(file: result, isVideo: true));
      });
    } catch (_) {
      if (!mounted) return;
      _showGenericMediaError();
    }
  }

  Future<void> _recordVideo() async {
    try {
      final permitted = await PermissionsUtils.requestCameraPermission();
      if (!permitted) {
        if (!mounted) return;
        final isDenied =
            await PermissionsUtils.isCameraPermissionPermanentlyDenied();
        _showPermissionDeniedSnackbar('Camera', isDenied);
        return;
      }

      final result = await _picker.pickVideo(source: ImageSource.camera);
      if (result == null || !mounted) return;
      setState(() {
        _mediaItems.add(_MediaItem(file: result, isVideo: true));
      });
    } catch (_) {
      if (!mounted) return;
      _showGenericMediaError();
    }
  }

  void _showGenericMediaError() {
    final l10n = AppLocalizations.of(context)!;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(l10n.profileImageUploadError)));
  }

  void _showPermissionDeniedSnackbar(
    String permissionType,
    bool isPermanentlyDenied,
  ) {
    final l10n = AppLocalizations.of(context)!;
    final message = isPermanentlyDenied
        ? '$permissionType permission is permanently denied. Tap ${l10n.labelSettings} to enable it.'
        : '$permissionType permission is required to proceed';

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        action: isPermanentlyDenied
            ? SnackBarAction(
                label: l10n.labelSettings,
                onPressed: () => PermissionsUtils.openAppSettingsDialog(),
              )
            : null,
        duration: Duration(seconds: isPermanentlyDenied ? 5 : 3),
      ),
    );
  }

  void _removeMedia(int index) => setState(() => _mediaItems.removeAt(index));

  // ── Post submission ─────────────────────────────────────────────────────────

  Future<void> _submit() async {
    final content = _controller.text.trim();
    if (content.isEmpty && _mediaItems.isEmpty) return;
    if (_mediaItems.isNotEmpty && !_thirdPartyConsentConfirmed) {
      setState(
        () =>
            _error = 'Confirme que possui autorização das pessoas retratadas.',
      );
      return;
    }

    setState(() {
      _uploading = _mediaItems.isNotEmpty;
      _posting = _mediaItems.isEmpty;
      _error = null;
    });

    try {
      // 1. Upload each media file to S3
      final mediaPayload = <Map<String, String>>[];
      for (final item in _mediaItems) {
        final presigned = await UploadService.uploadPostMedia(
          _token,
          item.file,
          "post",
        );
        // Derive the public URL by stripping S3 query params from the presigned URL
        final uri = Uri.parse(presigned.url);
        final publicUrl = Uri(
          scheme: uri.scheme,
          host: uri.host,
          path: uri.path,
        ).toString();

        mediaPayload.add({
          'url': publicUrl,
          'mediaType': item.isVideo ? 'Video' : 'Image',
          'objectKey': presigned.objectKey,
        });
      }

      setState(() {
        _uploading = false;
        _posting = true;
      });

      List<Map<String, dynamic>>? mentionsPayload = [];
      if (mentions.isNotEmpty) {
        mentionsPayload = mentions
            .map((m) => {'name': m.fullName, 'mentionedUuid': m.uuid})
            .toList();
      }

      // 2. Create the post with content + media
      if (!mounted) return;
      final feedProvider = context.read<FeedProvider>();
      final ok = await feedProvider.createPost({
        'content': content,
        'authorId': _isBusinessMode ? _businessProfileId : _personId,
        'authorUuid': _isBusinessMode
            ? (_businessProfileUuid ?? '')
            : (_person?.uuid ?? ''),
        'authorName': _isBusinessMode
            ? (_businessProfileName ?? _person?.fullName ?? '')
            : (_person?.fullName ?? ''),
        'authorObjectKey': _isBusinessMode
            ? (_businessProfileLogo ?? '')
            : (_person?.objectKey ?? ''),
        if (mediaPayload.isNotEmpty) 'media': mediaPayload,
        if (mediaPayload.isNotEmpty)
          'thirdPartyConsentConfirmed': _thirdPartyConsentConfirmed,
        'mentions': mentionsPayload,
      }, _token);

      if (!mounted) return;
      setState(() => _posting = false);

      if (ok) {
        Navigator.of(context).pop(true);
      } else {
        setState(() => _error = feedProvider.error ?? 'Failed to create post.');
      }
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _uploading = false;
        _posting = false;
        _error = e.toString();
      });
    }
  }

  // ── Build ───────────────────────────────────────────────────────────────────

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final personProvider = context.watch<PersonProvider>();
    final person = personProvider.person;
    final isBusinessMode = personProvider.isProfessional;
    final activeBusinessProfile = personProvider.activeBusinessProfile;
    final businessType = activeBusinessProfile?.businessType;
    final businessProfileLogo = activeBusinessProfile?.logo;
    final businessProfileName = activeBusinessProfile != null
        ? DisplayNameHelper.getBusinessProfileDisplayName(activeBusinessProfile)
        : null;
    final keyboardVisible = MediaQuery.of(context).viewInsets.bottom > 0;

    return PopScope(
      // Don't let a back gesture tear the page down mid upload/post.
      canPop: !_isLoading,
      child: Scaffold(
        backgroundColor: Colors.white,
        appBar: AppBar(
          backgroundColor: Colors.white,
          surfaceTintColor: Colors.white,
          elevation: 0,
          automaticallyImplyLeading: false,
          leadingWidth: 96,
          leading: Align(
            alignment: Alignment.centerLeft,
            child: TextButton(
              onPressed: _isLoading ? null : () => Navigator.pop(context),
              child: Text(
                l10n.buttonCancel,
                style: TextStyle(
                  color: _isLoading ? Colors.grey : Colors.black87,
                ),
              ),
            ),
          ),
          centerTitle: true,
          title: Text(
            l10n.feedCreatePost,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Colors.black87,
            ),
          ),
          actions: [
            Padding(
              padding: const EdgeInsets.only(right: 8),
              child: ElevatedButton(
                onPressed:
                    _isLoading ||
                        (_mediaItems.isNotEmpty && !_thirdPartyConsentConfirmed)
                    ? null
                    : _submit,
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primaryFor(businessType),
                  foregroundColor: Colors.white,
                  disabledBackgroundColor: AppColors.primaryDisabledFor(
                    businessType,
                  ),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(20),
                  ),
                  padding: const EdgeInsets.symmetric(
                    horizontal: 20,
                    vertical: 8,
                  ),
                ),
                child: _isLoading
                    ? const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(
                          strokeWidth: 2,
                          color: Colors.white,
                        ),
                      )
                    : Text(l10n.feedPostButton),
              ),
            ),
          ],
          bottom: const PreferredSize(
            preferredSize: Size.fromHeight(1),
            child: Divider(height: 1),
          ),
        ),
        body: Column(
          children: [
            // ── Author row ──────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 12, 16, 4),
              child: Row(
                children: [
                  _AvatarWidget(
                    person: person,
                    isBusinessMode: isBusinessMode,
                    businessLogo: businessProfileLogo,
                  ),
                  const SizedBox(width: 12),
                  Text(
                    _displayName(person, isBusinessMode, businessProfileName),
                    style: const TextStyle(
                      fontWeight: FontWeight.bold,
                      fontSize: 15,
                    ),
                  ),
                ],
              ),
            ),

            // ── Scrollable body ─────────────────────────────────────────────
            Expanded(
              child: SingleChildScrollView(
                keyboardDismissBehavior:
                    ScrollViewKeyboardDismissBehavior.onDrag,
                padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Text input
                    TextField(
                      controller: _controller,
                      autofocus:
                          widget.initialContent == null &&
                          (widget.initialImages?.isEmpty ?? true),
                      maxLines: null,
                      minLines: 5,
                      decoration: InputDecoration(
                        hintText: l10n.feedWriteSomething,
                        border: InputBorder.none,
                        hintStyle: TextStyle(
                          color: Colors.grey[400],
                          fontSize: 16,
                        ),
                      ),
                      style: const TextStyle(fontSize: 16),
                    ),

                    if (_isSearchingMentions ||
                        _mentionSuggestions.isNotEmpty) ...[
                      const SizedBox(height: 6),
                      _MentionSuggestionsList(
                        searching: _isSearchingMentions,
                        suggestions: _mentionSuggestions,
                        onSelected: _selectMention,
                        businessType: businessType,
                      ),
                    ],

                    // Upload progress
                    if (_uploading) ...[
                      const SizedBox(height: 12),
                      Row(
                        children: [
                          SizedBox(
                            width: 16,
                            height: 16,
                            child: CircularProgressIndicator(
                              strokeWidth: 2,
                              color: AppColors.primaryFor(businessType),
                            ),
                          ),
                          const SizedBox(width: 8),
                          Text(
                            l10n.feedUploading,
                            style: TextStyle(
                              color: AppColors.primaryFor(businessType),
                              fontSize: 13,
                            ),
                          ),
                        ],
                      ),
                    ],

                    // Error banner
                    if (_error != null) ...[
                      const SizedBox(height: 8),
                      Container(
                        padding: const EdgeInsets.all(10),
                        decoration: BoxDecoration(
                          color: AppColors.danger.withAlpha(20),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: Row(
                          children: [
                            const Icon(
                              Icons.error_outline,
                              color: AppColors.danger,
                              size: 16,
                            ),
                            const SizedBox(width: 8),
                            Expanded(
                              child: Text(
                                _error!,
                                style: const TextStyle(
                                  color: AppColors.danger,
                                  fontSize: 13,
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],

                    // Media thumbnails grid
                    if (_mediaItems.isNotEmpty) ...[
                      const SizedBox(height: 12),
                      _MediaGrid(items: _mediaItems, onRemove: _removeMedia),
                      CheckboxListTile(
                        contentPadding: EdgeInsets.zero,
                        controlAffinity: ListTileControlAffinity.leading,
                        value: _thirdPartyConsentConfirmed,
                        onChanged: _isLoading
                            ? null
                            : (value) => setState(
                                () => _thirdPartyConsentConfirmed =
                                    value ?? false,
                              ),
                        title: const Text(
                          'Declaro que tenho autorização das pessoas retratadas para publicar estas mídias.',
                        ),
                      ),
                    ],
                  ],
                ),
              ),
            ),

            // ── Bottom toolbar ──────────────────────────────────────────────
            // Sits inside the Scaffold body, so `resizeToAvoidBottomInset`
            // lifts it above the software keyboard (iOS has no back gesture
            // to dismiss the keyboard with).
            const Divider(height: 1),
            SafeArea(
              top: false,
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 10,
                ),
                child: Row(
                  children: [
                    _MediaPickerMenu(
                      enabled: !_isLoading,
                      onPickGalleryImages: _pickImages,
                      onPickCameraPhoto: _pickImageFromCamera,
                      onPickGalleryVideo: _pickVideo,
                      onPickCameraVideo: _recordVideo,
                      businessType: businessType,
                    ),
                    const Spacer(),
                    if (keyboardVisible)
                      IconButton(
                        tooltip: l10n.labelHideKeyboard,
                        icon: Icon(
                          Icons.keyboard_hide_outlined,
                          color: Colors.grey[700],
                        ),
                        onPressed: () => FocusScope.of(context).unfocus(),
                      ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _displayName(
    Person? p,
    bool isBusinessMode,
    String? businessProfileName,
  ) {
    if (isBusinessMode && (businessProfileName?.isNotEmpty ?? false)) {
      return businessProfileName!;
    }
    if (p == null) return '';
    return p.fullName.trim().isEmpty ? p.firstname : p.fullName;
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Media grid (2 columns)
// ─────────────────────────────────────────────────────────────────────────────

class _MediaGrid extends StatelessWidget {
  final List<_MediaItem> items;
  final void Function(int) onRemove;

  const _MediaGrid({required this.items, required this.onRemove});

  @override
  Widget build(BuildContext context) {
    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      itemCount: items.length,
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 3,
        mainAxisSpacing: 6,
        crossAxisSpacing: 6,
        childAspectRatio: 1,
      ),
      itemBuilder: (_, i) =>
          _MediaThumbnail(item: items[i], onRemove: () => onRemove(i)),
    );
  }
}

class _MediaThumbnail extends StatelessWidget {
  final _MediaItem item;
  final VoidCallback onRemove;

  const _MediaThumbnail({required this.item, required this.onRemove});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Stack(
      fit: StackFit.expand,
      children: [
        ClipRRect(
          borderRadius: BorderRadius.circular(8),
          child: item.isVideo
              ? Container(
                  color: Colors.black87,
                  child: const Center(
                    child: Icon(
                      Icons.videocam,
                      color: Colors.white54,
                      size: 36,
                    ),
                  ),
                )
              : item.previewBytes != null
              ? Image.memory(item.previewBytes!, fit: BoxFit.cover)
              : Image.network(item.file.path, fit: BoxFit.cover),
        ),
        // Video badge
        if (item.isVideo)
          Positioned(
            bottom: 6,
            left: 6,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
              decoration: BoxDecoration(
                color: Colors.black54,
                borderRadius: BorderRadius.circular(4),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Icon(Icons.play_arrow, color: Colors.white, size: 12),
                  const SizedBox(width: 2),
                  Text(
                    l10n.labelVideo,
                    style: const TextStyle(color: Colors.white, fontSize: 9),
                  ),
                ],
              ),
            ),
          ),
        // Remove button
        Positioned(
          top: 4,
          right: 4,
          child: GestureDetector(
            onTap: onRemove,
            child: Container(
              width: 24,
              height: 24,
              decoration: BoxDecoration(
                color: Colors.black.withAlpha(160),
                shape: BoxShape.circle,
              ),
              child: const Icon(Icons.close, color: Colors.white, size: 14),
            ),
          ),
        ),
      ],
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Author avatar
// ─────────────────────────────────────────────────────────────────────────────

class _AvatarWidget extends StatelessWidget {
  final Person? person;
  final String? businessLogo;
  final bool isBusinessMode;

  const _AvatarWidget({
    this.person,
    this.businessLogo,
    this.isBusinessMode = false,
  });

  static const _businessAvatarPlaceholder =
      'assets/images/avatar_personal_trainer.png';

  @override
  Widget build(BuildContext context) {
    if (isBusinessMode) {
      if (businessLogo != null && businessLogo!.isNotEmpty) {
        return CircleAvatar(
          radius: 22,
          child: ClipOval(
            child: CachedNetworkImage(
              imageUrl: businessLogo!,
              width: 44,
              height: 44,
              fit: BoxFit.cover,
              placeholder: (ctx, url) =>
                  Image.asset(_businessAvatarPlaceholder, fit: BoxFit.cover),
              errorWidget: (ctx, url, err) => Image.asset(
                _businessAvatarPlaceholder,
                width: 44,
                height: 44,
                fit: BoxFit.cover,
              ),
            ),
          ),
        );
      }

      return CircleAvatar(
        radius: 22,
        child: ClipOval(
          child: Image.asset(
            _businessAvatarPlaceholder,
            width: 44,
            height: 44,
            fit: BoxFit.cover,
          ),
        ),
      );
    }

    final isFemale = person?.gender?.toLowerCase() == 'female';
    final asset = isFemale
        ? 'assets/images/avatar_female.png'
        : 'assets/images/avatar_male.png';

    if (person?.avatar != null && person!.avatar!.isNotEmpty) {
      return CircleAvatar(
        radius: 22,
        child: ClipOval(
          child: CachedNetworkImage(
            imageUrl: person!.avatar!,
            width: 44,
            height: 44,
            fit: BoxFit.cover,
            placeholder: (ctx, url) => Image.asset(asset, fit: BoxFit.cover),
            errorWidget: (ctx, url, err) =>
                Image.asset(asset, width: 44, height: 44, fit: BoxFit.cover),
          ),
        ),
      );
    }

    return CircleAvatar(
      radius: 22,
      child: ClipOval(
        child: Image.asset(asset, width: 44, height: 44, fit: BoxFit.cover),
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Media Picker Menu with dropdown
// ─────────────────────────────────────────────────────────────────────────────

class _MediaPickerMenu extends StatefulWidget {
  final bool enabled;
  final VoidCallback onPickGalleryImages;
  final VoidCallback onPickCameraPhoto;
  final VoidCallback onPickGalleryVideo;
  final VoidCallback onPickCameraVideo;
  final String? businessType;

  const _MediaPickerMenu({
    required this.enabled,
    required this.onPickGalleryImages,
    required this.onPickCameraPhoto,
    required this.onPickGalleryVideo,
    required this.onPickCameraVideo,
    this.businessType,
  });

  @override
  State<_MediaPickerMenu> createState() => _MediaPickerMenuState();
}

class _MediaPickerMenuState extends State<_MediaPickerMenu> {
  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return PopupMenuButton<String>(
      enabled: widget.enabled,
      onSelected: (value) {
        switch (value) {
          case 'gallery_photo':
            widget.onPickGalleryImages();
            break;
          case 'camera_photo':
            widget.onPickCameraPhoto();
            break;
          case 'gallery_video':
            widget.onPickGalleryVideo();
            break;
          case 'camera_video':
            widget.onPickCameraVideo();
            break;
        }
      },
      itemBuilder: (BuildContext context) => [
        PopupMenuItem(
          value: 'gallery_photo',
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(
                Icons.photo_library_outlined,
                color: AppColors.success,
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(l10n.feedAddPhoto),
            ],
          ),
        ),
        PopupMenuItem(
          value: 'camera_photo',
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                Icons.photo_camera_outlined,
                color: AppColors.primaryFor(widget.businessType),
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(l10n.labelTakePhoto),
            ],
          ),
        ),
        PopupMenuItem(
          value: 'gallery_video',
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(
                Icons.video_library_outlined,
                color: AppColors.danger,
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(l10n.feedAddVideo),
            ],
          ),
        ),
        PopupMenuItem(
          value: 'camera_video',
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(
                Icons.videocam_outlined,
                color: AppColors.danger,
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(l10n.labelRecordVideo),
            ],
          ),
        ),
      ],
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              Icons.attach_file,
              color: widget.enabled
                  ? AppColors.primaryFor(widget.businessType)
                  : Colors.grey[400],
              size: 22,
            ),
            const SizedBox(width: 6),
            Text(
              'Media',
              style: TextStyle(
                color: widget.enabled ? Colors.grey[700] : Colors.grey[400],
                fontSize: 14,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _MentionSuggestionsList extends StatelessWidget {
  final bool searching;
  final List<MentionableFriend> suggestions;
  final ValueChanged<MentionableFriend> onSelected;
  final String? businessType;

  const _MentionSuggestionsList({
    required this.searching,
    required this.suggestions,
    required this.onSelected,
    this.businessType,
  });

  @override
  Widget build(BuildContext context) {
    if (searching) {
      return const Padding(
        padding: EdgeInsets.symmetric(vertical: 8),
        child: Center(
          child: SizedBox(
            width: 18,
            height: 18,
            child: CircularProgressIndicator(strokeWidth: 2),
          ),
        ),
      );
    }

    if (suggestions.isEmpty) {
      return const SizedBox.shrink();
    }

    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: Colors.grey.shade300),
        color: Colors.white,
      ),
      constraints: const BoxConstraints(maxHeight: 220),
      child: ListView.separated(
        shrinkWrap: true,
        itemCount: suggestions.length,
        separatorBuilder: (_, _) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final item = suggestions[index];
          final subtitle = '@${item.firstname.toLowerCase()}';
          return ListTile(
            dense: true,
            leading: CircleAvatar(
              radius: 16,
              backgroundColor: AppColors.primaryFor(businessType).withAlpha(25),
              backgroundImage: item.avatar != null && item.avatar!.isNotEmpty
                  ? NetworkImage(item.avatar!)
                  : null,
              child: item.avatar == null || item.avatar!.isEmpty
                  ? Text(
                      item.fullName.isNotEmpty
                          ? item.fullName[0].toUpperCase()
                          : '?',
                      style: TextStyle(
                        color: AppColors.primaryFor(businessType),
                        fontWeight: FontWeight.w700,
                      ),
                    )
                  : null,
            ),
            title: Text(
              item.fullName,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
            subtitle: Text(
              subtitle,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
            onTap: () => onSelected(item),
          );
        },
      ),
    );
  }
}
