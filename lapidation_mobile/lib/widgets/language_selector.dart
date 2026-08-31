import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../config/app_colors.dart';
import '../l10n/app_localizations.dart';
import '../providers/locale_provider.dart';
import '../providers/person_provider.dart';

class LanguageSelectorButton extends StatelessWidget {
  const LanguageSelectorButton({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;
    return Consumer<LocaleProvider>(
      builder: (context, localeProvider, _) {
        return PopupMenuButton<String>(
          icon: Icon(Icons.language, color: IconTheme.of(context).color),
          tooltip: l10n.tooltipChangeLanguage,
          onSelected: (String localeKey) {
            Locale locale;
            if (localeKey.contains('_')) {
              final parts = localeKey.split('_');
              locale = Locale(parts[0], parts[1]);
            } else {
              locale = Locale(localeKey);
            }
            localeProvider.setLocale(locale);
          },
          itemBuilder: (BuildContext context) {
            return LocaleProvider.localeNames.entries.map((entry) {
              final isSelected = entry.key == localeProvider.currentLocaleKey;
              return PopupMenuItem<String>(
                value: entry.key,
                child: Row(
                  children: [
                    if (isSelected)
                      Icon(
                        Icons.check,
                        color: AppColors.primaryFor(businessType),
                        size: 18,
                      )
                    else
                      const SizedBox(width: 18),
                    const SizedBox(width: 8),
                    Text(
                      entry.value,
                      style: TextStyle(
                        fontWeight: isSelected
                            ? FontWeight.bold
                            : FontWeight.normal,
                        color: isSelected
                            ? AppColors.primaryFor(businessType)
                            : null,
                      ),
                    ),
                  ],
                ),
              );
            }).toList();
          },
        );
      },
    );
  }
}
