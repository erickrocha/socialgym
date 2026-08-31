import '../models/feed_post.dart';

/// Represents a parsed mention segment in text
class MentionSpan {
  final int start;
  final int end;
  final String text;
  final String? mentionedUuid;
  final bool isMention;

  const MentionSpan({
    required this.start,
    required this.end,
    required this.text,
    this.mentionedUuid,
    required this.isMention,
  });
}

/// Parses content text and detects mentions
/// Returns a list of MentionSpan objects indicating where mentions are
class MentionParser {
  /// Parse content and return list of mention spans
  /// Mentions are detected by searching for exact mention names from the mentions list
  static List<MentionSpan> parseMentions(
    String content,
    List<Mention>? mentions,
  ) {
    if (content.isEmpty) return [];

    final spans = <MentionSpan>[];
    if (mentions == null || mentions.isEmpty) {
      // No mentions, return entire content as plain text
      spans.add(
        MentionSpan(
          start: 0,
          end: content.length,
          text: content,
          isMention: false,
        ),
      );
      return spans;
    }

    // Create a map of mention names for quick lookup
    final mentionMap = <String, Mention>{};
    for (final mention in mentions) {
      mentionMap['@${mention.name}'] = mention;
    }

    // Sort mention names by length (longest first) to match longer names first
    final sortedMentions = mentionMap.keys.toList()
      ..sort((a, b) => b.length.compareTo(a.length));

    // Parse text by searching for exact mention names
    int currentPos = 0;
    final allMatches = <({String text, int start, int end, Mention mention})>[];

    // Find all mention occurrences
    for (final mentionText in sortedMentions) {
      final mention = mentionMap[mentionText]!;
      int searchPos = 0;
      while (true) {
        final index = content.indexOf(mentionText, searchPos);
        if (index < 0) break;

        // Check if this is a word boundary (not part of a larger word)
        final beforeOk = index == 0 || !isWordChar(content[index - 1]);
        final afterIdx = index + mentionText.length;
        final afterOk =
            afterIdx >= content.length || !isWordChar(content[afterIdx]);

        if (beforeOk && afterOk) {
          allMatches.add((
            text: mentionText,
            start: index,
            end: afterIdx,
            mention: mention,
          ));
        }

        searchPos = index + 1;
      }
    }

    // Sort matches by position
    allMatches.sort((a, b) => a.start.compareTo(b.start));

    // Remove overlapping matches (keep the first one)
    final filteredMatches =
        <({String text, int start, int end, Mention mention})>[];
    for (final match in allMatches) {
      bool overlaps = false;
      for (final existing in filteredMatches) {
        if ((match.start >= existing.start && match.start < existing.end) ||
            (match.end > existing.start && match.end <= existing.end)) {
          overlaps = true;
          break;
        }
      }
      if (!overlaps) {
        filteredMatches.add(match);
      }
    }

    // Build spans from matches
    currentPos = 0;
    for (final match in filteredMatches) {
      // Add text before this mention
      if (currentPos < match.start) {
        spans.add(
          MentionSpan(
            start: currentPos,
            end: match.start,
            text: content.substring(currentPos, match.start),
            isMention: false,
          ),
        );
      }

      // Add the mention
      spans.add(
        MentionSpan(
          start: match.start,
          end: match.end,
          text: match.text,
          mentionedUuid: match.mention.mentionedUuid,
          isMention: true,
        ),
      );

      currentPos = match.end;
    }

    // Add remaining text
    if (currentPos < content.length) {
      spans.add(
        MentionSpan(
          start: currentPos,
          end: content.length,
          text: content.substring(currentPos),
          isMention: false,
        ),
      );
    }

    // If no spans were created, return entire content as plain text
    if (spans.isEmpty) {
      spans.add(
        MentionSpan(
          start: 0,
          end: content.length,
          text: content,
          isMention: false,
        ),
      );
    }

    return spans;
  }

  /// Check if a character is a word character (letter, digit, or underscore)
  static bool isWordChar(String char) {
    return RegExp(r'\w').hasMatch(char);
  }
}
