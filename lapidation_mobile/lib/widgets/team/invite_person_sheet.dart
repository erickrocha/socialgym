import 'dart:async';

import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../providers/person_provider.dart';
import '../../providers/team_member_provider.dart';
import '../../services/grpc/grpc_person_service.dart';
import 'team_member_card.dart';

/// Search-and-invite bottom sheet for a business profile's Members tab.
Future<void> showTeamInviteSheet(
  BuildContext context, {
  required int businessProfileId,
}) {
  return showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
    ),
    builder: (_) => _InvitePersonSheet(businessProfileId: businessProfileId),
  );
}

class _InvitePersonSheet extends StatefulWidget {
  final int businessProfileId;

  const _InvitePersonSheet({required this.businessProfileId});

  @override
  State<_InvitePersonSheet> createState() => _InvitePersonSheetState();
}

class _InvitePersonSheetState extends State<_InvitePersonSheet> {
  final _controller = TextEditingController();
  Timer? _debounce;
  List<Person> _results = [];
  bool _searching = false;
  final Set<int> _busyIds = {};

  @override
  void dispose() {
    _debounce?.cancel();
    _controller.dispose();
    super.dispose();
  }

  void _onChanged(String query) {
    _debounce?.cancel();
    if (query.trim().length < 2) {
      setState(() => _results = []);
      return;
    }
    setState(() => _searching = true);
    _debounce = Timer(const Duration(milliseconds: 400), () async {
      if (!mounted) return;
      final ownerUuid = context.read<PersonProvider>().ownerUuid;
      try {
        final results = await GrpcPersonService.searchPersonsByUuid(
          uuid: ownerUuid,
          query: query.trim(),
          limit: 20,
        );
        if (mounted) {
          setState(() {
            _results = results;
            _searching = false;
          });
        }
      } catch (_) {
        if (mounted) {
          setState(() {
            _results = [];
            _searching = false;
          });
        }
      }
    });
  }

  Future<void> _invite(Person person) async {
    final l10n = AppLocalizations.of(context)!;
    setState(() => _busyIds.add(person.id));
    final provider = context.read<TeamMemberProvider>();
    final success = await provider.inviteMember(
      businessProfileId: widget.businessProfileId,
      personId: person.id,
    );
    if (!mounted) return;
    setState(() {
      _busyIds.remove(person.id);
      if (success) _results.removeWhere((p) => p.id == person.id);
    });
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(success ? l10n.teamRequestSent : l10n.teamActionError),
        backgroundColor: success ? AppColors.success : AppColors.danger,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;
    final showNoResults =
        _results.isEmpty && !_searching && _controller.text.trim().length >= 2;

    return SafeArea(
      child: Padding(
        padding: EdgeInsets.only(
          left: 16,
          right: 16,
          top: 16,
          bottom: MediaQuery.of(context).viewInsets.bottom + 16,
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              l10n.teamInvite,
              style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _controller,
              autofocus: true,
              onChanged: _onChanged,
              decoration: InputDecoration(
                hintText: l10n.teamInviteSearchHint,
                prefixIcon: Icon(
                  Icons.search,
                  color: AppColors.primaryFor(businessType),
                ),
                suffixIcon: _searching
                    ? const Padding(
                        padding: EdgeInsets.all(12),
                        child: SizedBox(
                          width: 16,
                          height: 16,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        ),
                      )
                    : null,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
            ),
            const SizedBox(height: 12),
            Flexible(
              child: showNoResults
                  ? Padding(
                      padding: const EdgeInsets.symmetric(vertical: 24),
                      child: Text(
                        l10n.teamInviteNoResults,
                        textAlign: TextAlign.center,
                        style: TextStyle(color: Colors.grey[600]),
                      ),
                    )
                  : ListView.builder(
                      shrinkWrap: true,
                      itemCount: _results.length,
                      itemBuilder: (context, index) {
                        final person = _results[index];
                        return TeamMemberCard(
                          person: person,
                          trailing: ElevatedButton(
                            onPressed: _busyIds.contains(person.id)
                                ? null
                                : () => _invite(person),
                            child: Text(l10n.teamInviteSend),
                          ),
                        );
                      },
                    ),
            ),
          ],
        ),
      ),
    );
  }
}
