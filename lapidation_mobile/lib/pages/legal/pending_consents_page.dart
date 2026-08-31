import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/pending_consent.dart';
import '../../providers/auth_provider.dart';
import '../../providers/consent_provider.dart';
import '../../services/legal_document_service.dart';

/// Full-screen, non-dismissible gate shown whenever the backend reports a
/// missing / out-of-date legal consent. The person must accept the current
/// version of every outstanding document (or sign out) to get back into the
/// app. Rendered by `ConsentGate` in `main.dart`, above the current route.
class PendingConsentsPage extends StatelessWidget {
  /// Key of the app's `MaterialApp` navigator. The gate renders above that
  /// navigator, so logout navigation has to go through this key rather than
  /// the local `Navigator.of(context)`.
  final GlobalKey<NavigatorState>? navigatorKey;

  const PendingConsentsPage({super.key, this.navigatorKey});

  Future<void> _logOut(BuildContext context) async {
    await context.read<AuthProvider>().signOut();
    if (context.mounted) context.read<ConsentProvider>().reset();
    navigatorKey?.currentState?.pushNamedAndRemoveUntil(
      '/login',
      (route) => false,
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final provider = context.watch<ConsentProvider>();
    final outstanding = provider.outstanding;

    return PopScope(
      canPop: false,
      child: Scaffold(
        backgroundColor: AppColors.background,
        appBar: AppBar(
          automaticallyImplyLeading: false,
          title: Text(l10n.consentPendingTitle),
          actions: [
            TextButton(
              onPressed: () => _logOut(context),
              child: Text(l10n.consentLogOut),
            ),
          ],
        ),
        body: SafeArea(
          child: Stack(
            children: [
              ListView(
                padding: const EdgeInsets.all(20),
                children: [
                  Text(
                    l10n.consentPendingSubtitle,
                    style: const TextStyle(
                      fontSize: 15,
                      color: Color(0xFF555555),
                    ),
                  ),
                  const SizedBox(height: 20),
                  if (provider.error != null) ...[
                    _ErrorBanner(
                      message: l10n.consentLoadFailed,
                      onRetry: provider.refresh,
                    ),
                    const SizedBox(height: 16),
                  ],
                  if (outstanding.isEmpty && provider.error == null)
                    _AllClearCard(message: l10n.consentAllAccepted)
                  else
                    ...outstanding.map(
                      (c) => Padding(
                        padding: const EdgeInsets.only(bottom: 12),
                        child: _ConsentCard(consent: c),
                      ),
                    ),
                ],
              ),
              if (provider.loading)
                const Positioned.fill(
                  child: ColoredBox(
                    color: Color(0x33000000),
                    child: Center(child: CircularProgressIndicator()),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

class _ConsentCard extends StatefulWidget {
  final PendingConsent consent;
  const _ConsentCard({required this.consent});

  @override
  State<_ConsentCard> createState() => _ConsentCardState();
}

class _ConsentCardState extends State<_ConsentCard> {
  LegalDocument? _doc;
  bool _failed = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final doc = await LegalDocumentService.get(widget.consent.document);
      if (mounted) setState(() => _doc = doc);
    } catch (_) {
      if (mounted) setState(() => _failed = true);
    }
  }

  Future<void> _showFullText(AppLocalizations l10n) async {
    final doc = _doc;
    if (doc == null) return;
    await showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(doc.title),
        content: SizedBox(
          width: 620,
          child: SingleChildScrollView(child: SelectableText(doc.content)),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text(l10n.consentClose),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final title =
        _doc?.title ??
        (_failed ? widget.consent.document : l10n.consentLoading);

    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(
        side: const BorderSide(color: Color(0xFFE0E0E0)),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 4),
            Text(
              widget.consent.isFirstTime
                  ? l10n.consentNeverAccepted
                  : l10n.consentVersionOutdated(widget.consent.currentVersion),
              style: const TextStyle(fontSize: 13, color: Color(0xFF777777)),
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                OutlinedButton(
                  onPressed: _doc == null ? null : () => _showFullText(l10n),
                  child: Text(l10n.consentReadDocument),
                ),
                const SizedBox(width: 12),
                FilledButton(
                  onPressed: _doc == null
                      ? null
                      : () => context.read<ConsentProvider>().accept(
                          widget.consent.document,
                        ),
                  child: Text(l10n.consentAgree),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _AllClearCard extends StatelessWidget {
  final String message;
  const _AllClearCard({required this.message});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const Icon(
          Icons.check_circle_outline,
          size: 48,
          color: AppColors.success,
        ),
        const SizedBox(height: 12),
        Text(message, textAlign: TextAlign.center),
      ],
    );
  }
}

class _ErrorBanner extends StatelessWidget {
  final String message;
  final Future<void> Function() onRetry;
  const _ErrorBanner({required this.message, required this.onRetry});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0x14D32F2F),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: [
          const Icon(Icons.error_outline, color: AppColors.danger),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              message,
              style: const TextStyle(color: AppColors.danger),
            ),
          ),
          TextButton(onPressed: onRetry, child: Text(l10n.buttonRetry)),
        ],
      ),
    );
  }
}
