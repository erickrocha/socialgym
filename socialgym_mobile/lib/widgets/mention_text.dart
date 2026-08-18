import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

import '../config/app_colors.dart';
import '../models/feed_post.dart';
import '../utils/mention_parser.dart';

/// Widget to display text with clickable mention links
class MentionText extends StatelessWidget {
  final String content;
  final List<Mention>? mentions;
  final TextStyle? style;
  final TextAlign textAlign;
  final int? maxLines;
  final TextOverflow overflow;
  final VoidCallback? onMentionTap;

  /// Callback when a mention is tapped, receives the mention data
  final Function(Mention mention)? onMentionTapped;

  const MentionText(
    this.content, {
    super.key,
    this.mentions,
    this.style,
    this.textAlign = TextAlign.start,
    this.maxLines,
    this.overflow = TextOverflow.clip,
    this.onMentionTap,
    this.onMentionTapped,
  });

  @override
  Widget build(BuildContext context) {
    // Parse mentions from content
    final spans = MentionParser.parseMentions(content, mentions);

    // Ensure we have a base style with color
    final baseStyle = style ?? const TextStyle(fontSize: 15, color: Colors.black);
    final baseStyleWithColor = baseStyle.color == null 
        ? baseStyle.copyWith(color: Colors.black)
        : baseStyle;

    // Build text spans for RichText
    final textSpans = <TextSpan>[];
    final mentionMap = <String, Mention>{};

    // Create mention lookup map
    if (mentions != null) {
      for (final mention in mentions!) {
        mentionMap['@${mention.name}'] = mention;
      }
    }

    for (final span in spans) {
      if (span.isMention) {
        // This is a mention - make it blue and tappable
        final mention = mentionMap[span.text];
        textSpans.add(
          TextSpan(
            text: span.text,
            style: baseStyleWithColor.copyWith(
              color: AppColors.primary,
              fontWeight: FontWeight.w600,
            ),
            recognizer: TapGestureRecognizer()
              ..onTap = () {
                onMentionTap?.call();
                if (mention != null) {
                  onMentionTapped?.call(mention);
                }
              },
          ),
        );
      } else {
        // Regular text - use base style
        textSpans.add(
          TextSpan(
            text: span.text,
            style: baseStyleWithColor,
          ),
        );
      }
    }

    return RichText(
      text: TextSpan(
        style: baseStyleWithColor,
        children: textSpans,
      ),
      textAlign: textAlign,
      maxLines: maxLines,
      overflow: overflow,
    );
  }
}
