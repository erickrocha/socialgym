import 'package:flutter/material.dart';

import '../l10n/app_localizations.dart';
import '../models/visibility_option.dart';

class VisibilityDropdownField extends StatelessWidget {
  final String value;
  final ValueChanged<String?> onChanged;
  final InputDecoration? decoration;
  final bool isExpanded;
  final VisibilityOption fallback;
  final bool useFriendsOnlyAlias;

  const VisibilityDropdownField({
    super.key,
    required this.value,
    required this.onChanged,
    this.decoration,
    this.isExpanded = true,
    this.fallback = VisibilityOption.privateAccess,
    this.useFriendsOnlyAlias = false,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final selectedVisibility = VisibilityOption.fromApiValue(value, fallback: fallback);

    return DropdownButtonFormField<VisibilityOption>(
      key: ValueKey<String>(
        selectedVisibility.apiValueFor(useFriendsOnlyAlias: useFriendsOnlyAlias),
      ),
      initialValue: selectedVisibility,
      isExpanded: isExpanded,
      decoration: decoration ?? InputDecoration(labelText: l10n.labelVisibility),
      items: VisibilityOption.values.map((option) {
        return DropdownMenuItem<VisibilityOption>(
          value: option,
          child: Text('${option.emoji} ${option.label(l10n)}', overflow: TextOverflow.ellipsis),
        );
      }).toList(),
      onChanged: (option) =>
          onChanged(option?.apiValueFor(useFriendsOnlyAlias: useFriendsOnlyAlias)),
    );
  }
}
