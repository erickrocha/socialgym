import 'package:intl/intl.dart';

import '../l10n/app_localizations.dart';

/// Formatting shared by the conversation list and the message bubbles so a
/// timestamp reads the same in both places.
class ChatTime {
  const ChatTime._();

  static bool _isSameDay(DateTime a, DateTime b) =>
      a.year == b.year && a.month == b.month && a.day == b.day;

  /// `14:32` today, `Yesterday`, `12/03` this year, `12/03/24` before that —
  /// what the conversation list shows on the right of each row.
  static String conversationStamp(
    DateTime when,
    AppLocalizations l10n,
    String locale,
  ) {
    final now = DateTime.now();
    final local = when.toLocal();
    if (_isSameDay(local, now)) {
      return DateFormat.Hm(locale).format(local);
    }
    if (_isSameDay(local, now.subtract(const Duration(days: 1)))) {
      return l10n.chatYesterday;
    }
    if (local.year == now.year) {
      return DateFormat.Md(locale).format(local);
    }
    return DateFormat.yMd(locale).format(local);
  }

  /// `14:32` — the time under a message bubble.
  static String messageStamp(DateTime when, String locale) =>
      DateFormat.Hm(locale).format(when.toLocal());

  /// `Today` / `Yesterday` / `Monday, 12 March` — the separator between days.
  static String dayLabel(
    DateTime when,
    AppLocalizations l10n,
    String locale,
  ) {
    final now = DateTime.now();
    final local = when.toLocal();
    if (_isSameDay(local, now)) return l10n.chatToday;
    if (_isSameDay(local, now.subtract(const Duration(days: 1)))) {
      return l10n.chatYesterday;
    }
    return DateFormat.yMMMMd(locale).format(local);
  }

  /// True when [current] starts a new calendar day relative to [previous].
  static bool startsNewDay(DateTime? previous, DateTime current) =>
      previous == null || !_isSameDay(previous.toLocal(), current.toLocal());
}
