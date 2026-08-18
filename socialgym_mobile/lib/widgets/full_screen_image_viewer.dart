import 'package:cached_network_image/cached_network_image.dart';
import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:image_gallery_saver_plus/image_gallery_saver_plus.dart';
import 'package:photo_view/photo_view.dart';
import 'package:share_plus/share_plus.dart';
import 'package:video_player/video_player.dart';

import '../models/feed_post.dart';

// ─────────────────────────────────────────────────────────────────────────────
// Full-screen media viewer  (Facebook-style)
// • Images: Horizontal swipe, pinch-to-zoom (photo_view)
// • Videos: Horizontal swipe, play/pause controls
// • Top-left:  ✕ close
// • Top-center: "N / Total" counter
// • Top-right: ⬇ save to gallery  |  ⇧ share
// ─────────────────────────────────────────────────────────────────────────────

class FullScreenImageViewer extends StatefulWidget {
  /// Media list (images and/or videos).
  final List<FeedMedia> media;

  /// Index inside [media] to open first.
  final int initialIndex;

  const FullScreenImageViewer({super.key, required this.media, this.initialIndex = 0});

  @override
  State<FullScreenImageViewer> createState() => _FullScreenImageViewerState();
}

class _FullScreenImageViewerState extends State<FullScreenImageViewer> {
  late int _currentIndex;
  late PageController _pageController;
  bool _isSaving = false;
  
  // Video player management: one controller per media index
  final Map<int, VideoPlayerController> _videoControllers = {};
  final Map<int, bool> _videoInitialized = {};
  final Map<int, bool> _videoPlaying = {};

  @override
  void initState() {
    super.initState();
    _currentIndex = widget.initialIndex;
    _pageController = PageController(initialPage: widget.initialIndex);
    // Immersive full-screen experience
    SystemChrome.setEnabledSystemUIMode(SystemUiMode.immersiveSticky);
    
    // Pre-initialize video at initial index if needed
    _ensureVideoInitialized(_currentIndex);
  }

  @override
  void dispose() {
    _pageController.dispose();
    // Dispose all video controllers
    for (final controller in _videoControllers.values) {
      controller.dispose();
    }
    _videoControllers.clear();
    _videoInitialized.clear();
    _videoPlaying.clear();
    // Restore system UI when viewer is closed
    SystemChrome.setEnabledSystemUIMode(SystemUiMode.edgeToEdge);
    super.dispose();
  }

  FeedMedia get _current => widget.media[_currentIndex];
  
  // ── Video initialization ────────────────────────────────────────────────────
  
  void _ensureVideoInitialized(int index) {
    if (!widget.media[index].isVideo) return;
    if (_videoInitialized[index] == true) return;
    
    _initializeVideo(index);
  }

  void _initializeVideo(int index) {
    if (_videoControllers.containsKey(index)) return;
    
    final controller = VideoPlayerController.networkUrl(Uri.parse(widget.media[index].url))
      ..initialize().then((_) {
        if (mounted) {
          setState(() {
            _videoInitialized[index] = true;
          });
        }
      });
    
    _videoControllers[index] = controller;
    _videoInitialized[index] = false;
    _videoPlaying[index] = false;
  }
  
  void _toggleVideoPlayPause(int index) {
    if (!_videoInitialized[index]!) return;
    
    final controller = _videoControllers[index]!;
    setState(() {
      if (_videoPlaying[index]!) {
        controller.pause();
        _videoPlaying[index] = false;
      } else {
        controller.play();
        _videoPlaying[index] = true;
      }
    });
  }

  // ── Save to gallery ────────────────────────────────────────────────────────

  Future<void> _saveMedia() async {
    if (_isSaving) return;
    setState(() => _isSaving = true);

    try {
      final response = await Dio().get<List<int>>(
        _current.url,
        options: Options(responseType: ResponseType.bytes),
      );

      if (response.data != null) {
        final dynamic result = await ImageGallerySaverPlus.saveImage(
          Uint8List.fromList(response.data!),
          name: 'socialgym_${DateTime.now().millisecondsSinceEpoch}',
          quality: 100,
        );

        if (mounted) {
          final success = result is Map ? result['isSuccess'] == true : result != null;
          final mediaType = _current.isVideo ? 'video' : 'image';
          _showSnackBar(
            success ? '$mediaType saved to gallery ✓' : 'Failed to save $mediaType',
            success ? Colors.green[700]! : Colors.red[700]!,
          );
        }
      }
    } catch (_) {
      if (mounted) {
        final mediaType = _current.isVideo ? 'video' : 'image';
        _showSnackBar('Failed to save $mediaType', Colors.red[700]!);
      }
    } finally {
      if (mounted) setState(() => _isSaving = false);
    }
  }

  // ── Share ──────────────────────────────────────────────────────────────────

  Future<void> _shareMedia() async {
    try {
      final response = await Dio().get<List<int>>(
        _current.url,
        options: Options(responseType: ResponseType.bytes),
      );
      if (response.data != null) {
        final mimeType = _current.isVideo ? 'video/mp4' : 'image/jpeg';
        final fileExt = _current.isVideo ? '.mp4' : '.jpg';
        final fileName = _current.isVideo ? 'socialgym_video$fileExt' : 'socialgym_image$fileExt';
        
        final xFile = XFile.fromData(
          Uint8List.fromList(response.data!),
          mimeType: mimeType,
          name: fileName,
        );
        await SharePlus.instance.share(
          ShareParams(
            files: [xFile],
            text: 'Check this out!',
            subject: 'Shared from SocialGym',
          ),
        );
      }
    } catch (_) {
      // Fallback: share URL as text
      await SharePlus.instance.share(
        ShareParams(
          text: _current.url,
          subject: 'Shared from SocialGym',
        ),
      );
    }
  }

  void _showSnackBar(String message, Color color) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message, style: const TextStyle(color: Colors.white)),
        backgroundColor: color,
        duration: const Duration(seconds: 2),
        behavior: SnackBarBehavior.floating,
        margin: const EdgeInsets.fromLTRB(16, 0, 16, 32),
      ),
    );
  }

  // ── Build ──────────────────────────────────────────────────────────────────

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      body: Stack(
        children: [
          // ── Media gallery (swipe + pinch-to-zoom for images, video player for videos) ────────────────────────
          PageView.builder(
            controller: _pageController,
            physics: const BouncingScrollPhysics(),
            itemCount: widget.media.length,
            onPageChanged: (index) {
              setState(() => _currentIndex = index);
              _ensureVideoInitialized(index);
            },
            itemBuilder: (context, index) {
              final mediaItem = widget.media[index];
              
              if (mediaItem.isVideo) {
                return _buildVideoPage(index);
              } else {
                return _buildImagePage(index);
              }
            },
          ),

          // ── Top bar ───────────────────────────────────────────────────────
          Positioned(
            top: 0,
            left: 0,
            right: 0,
            child: SafeArea(
              child: Container(
                // Gradient so buttons are always readable over any image
                decoration: const BoxDecoration(
                  gradient: LinearGradient(
                    begin: Alignment.topCenter,
                    end: Alignment.bottomCenter,
                    colors: [Colors.black54, Colors.transparent],
                  ),
                ),
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                child: Row(
                  children: [
                    // ── Close (top-left) ───────────────────────────────────
                    _CircleIconButton(
                      icon: Icons.close_rounded,
                      onTap: () => Navigator.pop(context),
                    ),

                    // ── Counter (center) ───────────────────────────────────
                    if (widget.media.length > 1) ...[
                      const Spacer(),
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 5),
                        decoration: BoxDecoration(
                          color: Colors.black54,
                          borderRadius: BorderRadius.circular(16),
                        ),
                        child: Text(
                          '${_currentIndex + 1} / ${widget.media.length}',
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 13,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                      ),
                    ],

                    const Spacer(),

                    // ── Save (top-right) ───────────────────────────────────
                    if (_isSaving)
                      const Padding(
                        padding: EdgeInsets.all(10),
                        child: SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                        ),
                      )
                    else
                      _CircleIconButton(
                        icon: Icons.download_rounded,
                        onTap: _saveMedia,
                        tooltip: 'Save to gallery',
                      ),

                    const SizedBox(width: 8),

                    // ── Share (top-right) ──────────────────────────────────
                    _CircleIconButton(
                      icon: Icons.share_rounded,
                      onTap: _shareMedia,
                      tooltip: 'Share',
                    ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
  
  // ── Image page builder ──────────────────────────────────────────────────────
  
  Widget _buildImagePage(int index) {
    return PhotoView(
      imageProvider: CachedNetworkImageProvider(
        widget.media[index].url,
        cacheKey: widget.media[index].objectKey,
      ),
      heroAttributes: PhotoViewHeroAttributes(tag: 'feed_img_${widget.media[index].url}'),
      minScale: PhotoViewComputedScale.contained,
      maxScale: PhotoViewComputedScale.covered * 4.0,
      errorBuilder: (_, _, _) =>
          const Center(child: Icon(Icons.broken_image, color: Colors.white54, size: 64)),
      loadingBuilder: (_, event) => Center(
        child: CircularProgressIndicator(
          color: Colors.white,
          value: event?.expectedTotalBytes != null
              ? event!.cumulativeBytesLoaded / event.expectedTotalBytes!
              : null,
        ),
      ),
      backgroundDecoration: const BoxDecoration(color: Colors.black),
    );
  }
  
  // ── Video page builder ──────────────────────────────────────────────────────
  
  Widget _buildVideoPage(int index) {
    _ensureVideoInitialized(index);
    
    final isInitialized = _videoInitialized[index] ?? false;
    final isPlaying = _videoPlaying[index] ?? false;
    
    if (!isInitialized) {
      return Container(
        color: Colors.black,
        child: const Center(
          child: CircularProgressIndicator(color: Colors.white),
        ),
      );
    }
    
    final controller = _videoControllers[index]!;
    
    return GestureDetector(
      onTap: () => _toggleVideoPlayPause(index),
      child: Stack(
        alignment: Alignment.center,
        children: [
          AspectRatio(
            aspectRatio: controller.value.aspectRatio,
            child: VideoPlayer(controller),
          ),
          if (!isPlaying)
            Container(
              decoration: const BoxDecoration(color: Colors.black38, shape: BoxShape.circle),
              padding: const EdgeInsets.all(12),
              child: const Icon(Icons.play_arrow, color: Colors.white, size: 36),
            ),
        ],
      ),
    );
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reusable semi-transparent circular icon button
// ─────────────────────────────────────────────────────────────────────────────

class _CircleIconButton extends StatelessWidget {
  final IconData icon;
  final VoidCallback onTap;
  final String? tooltip;

  const _CircleIconButton({required this.icon, required this.onTap, this.tooltip});

  @override
  Widget build(BuildContext context) {
    final button = GestureDetector(
      onTap: onTap,
      child: Container(
        width: 40,
        height: 40,
        decoration: BoxDecoration(
          color: Colors.black54,
          shape: BoxShape.circle,
          border: Border.all(color: Colors.white24),
        ),
        child: Icon(icon, color: Colors.white, size: 20),
      ),
    );

    if (tooltip != null) {
      return Tooltip(message: tooltip!, child: button);
    }
    return button;
  }
}
