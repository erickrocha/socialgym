class MentionQuery {
  final int start;
  final int end;
  final String token;

  const MentionQuery({
    required this.start,
    required this.end,
    required this.token,
  });
}

MentionQuery? findActiveMentionQuery(String text, int cursorOffset) {
  if (cursorOffset < 0 || cursorOffset > text.length) return null;

  final beforeCursor = text.substring(0, cursorOffset);
  final start = beforeCursor.lastIndexOf('@');
  if (start < 0) return null;

  if (start > 0) {
    final prev = text[start - 1];
    final isBoundary = prev.trim().isEmpty;
    if (!isBoundary) return null;
  }

  final token = text.substring(start, cursorOffset);
  if (token.contains(RegExp(r'\s'))) return null;

  return MentionQuery(start: start, end: cursorOffset, token: token);
}

String replaceMentionQuery({
  required String text,
  required MentionQuery query,
  required String mentionDisplay,
}) {
  final replacement = '@$mentionDisplay ';
  final after = text.substring(query.end);
  final normalizedAfter = after.startsWith(' ') && replacement.endsWith(' ')
      ? after.substring(1)
      : after;
  return '${text.substring(0, query.start)}$replacement$normalizedAfter';
}
