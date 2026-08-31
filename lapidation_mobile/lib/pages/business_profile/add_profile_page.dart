import 'package:flutter/material.dart';

import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../widgets/main_layout.dart';

class BusinessProfileTypeOption {
  final String businessType;
  final String Function(AppLocalizations) title;
  final String Function(AppLocalizations) description;
  final String imageAsset;

  const BusinessProfileTypeOption({
    required this.businessType,
    required this.title,
    required this.description,
    required this.imageAsset,
  });
}

const List<BusinessProfileTypeOption> businessProfileTypeOptions = [
  BusinessProfileTypeOption(
    businessType: 'Professional',
    title: _personalTrainerTitle,
    description: _personalTrainerDescription,
    imageAsset: 'assets/images/avatar_male.png',
  ),
  BusinessProfileTypeOption(
    businessType: 'Company',
    title: _gymTitle,
    description: _gymDescription,
    imageAsset: 'assets/images/cover_foto.png',
  ),
];

String _personalTrainerTitle(AppLocalizations l10n) =>
    l10n.addProfilePersonalTrainerTitle;
String _personalTrainerDescription(AppLocalizations l10n) =>
    l10n.addProfilePersonalTrainerDescription;
String _gymTitle(AppLocalizations l10n) => l10n.addProfileGymTitle;
String _gymDescription(AppLocalizations l10n) => l10n.addProfileGymDescription;

class AddProfilePage extends StatelessWidget {
  const AddProfilePage({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/add-profile',
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              l10n.addProfileTitle,
              style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            Text(
              l10n.addProfileSubtitle,
              style: const TextStyle(fontSize: 14, color: Colors.grey),
            ),
            const SizedBox(height: 24),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: businessProfileTypeOptions
                  .map((option) => _BusinessProfileTypeCard(option: option))
                  .toList(),
            ),
          ],
        ),
      ),
    );
  }
}

class _BusinessProfileTypeCard extends StatelessWidget {
  final BusinessProfileTypeOption option;

  const _BusinessProfileTypeCard({required this.option});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return SizedBox(
      width: 260,
      child: Card(
        elevation: 2,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        clipBehavior: Clip.antiAlias,
        child: InkWell(
          key: ValueKey('business_profile_type_card_${option.businessType}'),
          onTap: () => Navigator.of(
            context,
          ).pushNamed('/add-profile/form', arguments: option.businessType),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Image.asset(
                option.imageAsset,
                height: 140,
                width: double.infinity,
                fit: BoxFit.cover,
              ),
              Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      option.title(l10n),
                      style: const TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    const SizedBox(height: 6),
                    Text(
                      option.description(l10n),
                      style: const TextStyle(fontSize: 13, color: Colors.grey),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
