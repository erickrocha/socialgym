import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/models/enums.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/settings.dart';
import '../../providers/auth_provider.dart';
import '../../providers/locale_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/settings_provider.dart';
import '../../widgets/main_layout.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  late String _language;
  late bool _notificationsEnabled;
  late ContextMenuPosition _contextMenuPosition;
  late Pages _homePage;
  bool _dirty = false;

  @override
  void initState() {
    super.initState();
    final current = context.read<SettingsProvider>().settings ?? Settings();
    _language = current.language ?? 'en';
    _notificationsEnabled = current.notificationsEnabled ?? true;
    _contextMenuPosition = current.contextMenuPosition ?? ContextMenuPosition.left;
    _homePage = current.homePage ?? Pages.feed;

    WidgetsBinding.instance.addPostFrameCallback((_) => _refreshFromServer());
  }

  Future<void> _refreshFromServer() async {
    final person = context.read<PersonProvider>().person;
    if (person == null) return;

    final settingsProvider = context.read<SettingsProvider>();
    await settingsProvider.fetchFromServer(personId: person.id, personUuid: person.uuid);
    if (!mounted || _dirty) return;

    final refreshed = settingsProvider.settings;
    if (refreshed == null) return;
    setState(() {
      _language = refreshed.language ?? _language;
      _notificationsEnabled = refreshed.notificationsEnabled ?? _notificationsEnabled;
      _contextMenuPosition = refreshed.contextMenuPosition ?? _contextMenuPosition;
      _homePage = refreshed.homePage ?? _homePage;
    });
  }

  Future<void> _save() async {
    final l10n = AppLocalizations.of(context)!;
    final person = context.read<PersonProvider>().person;
    if (person == null) return;

    final settingsProvider = context.read<SettingsProvider>();
    final base = settingsProvider.settings ?? Settings();
    final languageChanged = base.language != _language;

    final updated = base.copyWith(
      personId: person.id,
      personUuid: person.uuid,
      language: _language,
      notificationsEnabled: _notificationsEnabled,
      contextMenuPosition: _contextMenuPosition,
      homePage: _homePage,
    );

    final success = await settingsProvider.persistToServer(updated);
    if (!mounted) return;

    if (success) {
      setState(() => _dirty = false);
      if (languageChanged) {
        await context.read<LocaleProvider>().applyUserSettingsLanguage(_language);
      }
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(l10n.settingsSaveSuccess)));
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(l10n.settingsSaveError),
          action: SnackBarAction(label: l10n.buttonRetry, onPressed: _save),
        ),
      );
    }
  }

  Future<void> _showDeleteAccountDialog() async {
    final l10n = AppLocalizations.of(context)!;
    bool immediate = false;

    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) {
        return StatefulBuilder(
          builder: (dialogContext, setDialogState) {
            return AlertDialog(
              title: Text(l10n.settingsDeleteAccountConfirmTitle),
              content: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(l10n.settingsDeleteAccountConfirmBody),
                  const SizedBox(height: 8),
                  RadioListTile<bool>(
                    contentPadding: EdgeInsets.zero,
                    title: Text(l10n.settingsDeleteAccountOptionGracePeriod),
                    value: false,
                    groupValue: immediate,
                    onChanged: (value) => setDialogState(() => immediate = value!),
                  ),
                  RadioListTile<bool>(
                    contentPadding: EdgeInsets.zero,
                    title: Text(l10n.settingsDeleteAccountOptionImmediate),
                    value: true,
                    groupValue: immediate,
                    onChanged: (value) => setDialogState(() => immediate = value!),
                  ),
                ],
              ),
              actions: [
                TextButton(
                  onPressed: () => Navigator.of(dialogContext).pop(false),
                  child: Text(l10n.buttonCancel),
                ),
                TextButton(
                  style: TextButton.styleFrom(foregroundColor: AppColors.danger),
                  onPressed: () => Navigator.of(dialogContext).pop(true),
                  child: Text(l10n.settingsDeleteAccountConfirmButton),
                ),
              ],
            );
          },
        );
      },
    );

    if (confirmed == true && mounted) {
      await _performAccountDeletion(immediate);
    }
  }

  Future<void> _performAccountDeletion(bool immediate) async {
    final l10n = AppLocalizations.of(context)!;
    final authProvider = context.read<AuthProvider>();

    final status = await authProvider.requestAccountDeletion(immediate: immediate);
    if (!mounted) return;

    if (status == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(authProvider.error ?? l10n.settingsDeleteAccountError)),
      );
      return;
    }

    final formattedDate = DateFormat.yMMMd().format(status.scheduledAt.toLocal());
    await showDialog<void>(
      context: context,
      barrierDismissible: false,
      builder: (dialogContext) => AlertDialog(
        title: Text(l10n.settingsDeleteAccountScheduledTitle),
        content: Text(l10n.settingsDeleteAccountScheduledBody(formattedDate)),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(dialogContext).pop(),
            child: Text(l10n.buttonConfirm),
          ),
        ],
      ),
    );
    if (!mounted) return;

    await authProvider.signOut();
    if (!mounted) return;
    context.read<PersonProvider>().clear();
    Navigator.of(context).pushNamedAndRemoveUntil('/login', (route) => false);
  }

  String _homePageLabel(AppLocalizations l10n, Pages page) {
    switch (page) {
      case Pages.feed:
        return l10n.settingsHomePageFeed;
      case Pages.gallery:
        return l10n.settingsHomePageGallery;
    }
  }

  String _positionLabel(AppLocalizations l10n, ContextMenuPosition position) {
    switch (position) {
      case ContextMenuPosition.left:
        return l10n.settingsContextMenuPositionLeft;
      case ContextMenuPosition.top:
        return l10n.settingsContextMenuPositionTop;
      case ContextMenuPosition.right:
        return l10n.settingsContextMenuPositionRight;
      case ContextMenuPosition.bottom:
        return l10n.settingsContextMenuPositionBottom;
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final settingsProvider = context.watch<SettingsProvider>();

    return MainLayout(
      currentRoute: '/settings',
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(l10n.settingsTitle, style: Theme.of(context).textTheme.headlineSmall),
            const SizedBox(height: 16),
            if (settingsProvider.error != null && !settingsProvider.saving)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: Text(
                  l10n.settingsLoadError,
                  style: const TextStyle(color: Colors.redAccent),
                ),
              ),
            Text(l10n.settingsLanguage, style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            DropdownButtonFormField<String>(
              initialValue: _language,
              items: LocaleProvider.localeNames.entries
                  .map((entry) => DropdownMenuItem(value: entry.key, child: Text(entry.value)))
                  .toList(),
              onChanged: (value) {
                if (value == null) return;
                setState(() {
                  _language = value;
                  _dirty = true;
                });
              },
            ),
            const SizedBox(height: 24),
            SwitchListTile(
              contentPadding: EdgeInsets.zero,
              title: Text(l10n.settingsNotificationsEnabled),
              value: _notificationsEnabled,
              onChanged: (value) => setState(() {
                _notificationsEnabled = value;
                _dirty = true;
              }),
            ),
            const SizedBox(height: 16),
            Text(l10n.settingsContextMenuPosition, style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            SegmentedButton<ContextMenuPosition>(
              segments: ContextMenuPosition.values
                  .map(
                    (position) => ButtonSegment(
                      value: position,
                      label: Text(_positionLabel(l10n, position)),
                    ),
                  )
                  .toList(),
              selected: {_contextMenuPosition},
              onSelectionChanged: (selection) => setState(() {
                _contextMenuPosition = selection.first;
                _dirty = true;
              }),
            ),
            const SizedBox(height: 24),
            Text(l10n.settingsHomePage, style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            DropdownButtonFormField<Pages>(
              initialValue: _homePage,
              items: Pages.values
                  .map((page) => DropdownMenuItem(value: page, child: Text(_homePageLabel(l10n, page))))
                  .toList(),
              onChanged: (value) {
                if (value == null) return;
                setState(() {
                  _homePage = value;
                  _dirty = true;
                });
              },
            ),
            const SizedBox(height: 32),
            SizedBox(
              width: double.infinity,
              child: FilledButton(
                onPressed: settingsProvider.saving ? null : _save,
                child: settingsProvider.saving
                    ? const SizedBox(
                        width: 20,
                        height: 20,
                        child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                      )
                    : Text(l10n.buttonSave),
              ),
            ),
            const SizedBox(height: 40),
            const Divider(),
            const SizedBox(height: 16),
            Text(
              l10n.settingsDangerZoneTitle,
              style: Theme.of(context).textTheme.titleMedium?.copyWith(color: AppColors.danger),
            ),
            const SizedBox(height: 8),
            Text(
              l10n.settingsDeleteAccountDescription,
              style: TextStyle(color: Colors.grey[600]),
            ),
            const SizedBox(height: 12),
            SizedBox(
              width: double.infinity,
              child: OutlinedButton.icon(
                onPressed: _showDeleteAccountDialog,
                style: OutlinedButton.styleFrom(
                  foregroundColor: AppColors.danger,
                  side: BorderSide(color: AppColors.danger),
                ),
                icon: const Icon(Icons.delete_forever),
                label: Text(l10n.settingsDeleteAccountButton),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
