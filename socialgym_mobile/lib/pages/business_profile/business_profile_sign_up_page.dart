import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/services/grpc/grpc_person_service.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/business_profile.dart';
import '../../providers/auth_provider.dart';
import '../../providers/business_profile_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/grpc/grpc_business_profile_service.dart';

class BusinessProfileSignUpPage extends StatefulWidget {
  const BusinessProfileSignUpPage({super.key});

  @override
  State<BusinessProfileSignUpPage> createState() => _BusinessProfileSignUpPageState();
}

class _BusinessProfileSignUpPageState extends State<BusinessProfileSignUpPage> {
  final _formKey = GlobalKey<FormState>();
  final _businessNameController = TextEditingController();
  final _socialNameController = TextEditingController();
  final _taxIdController = TextEditingController();
  bool _submitting = false;
  String? _error;

  @override
  void dispose() {
    _businessNameController.dispose();
    _socialNameController.dispose();
    _taxIdController.dispose();
    super.dispose();
  }

  InputDecoration _inputDecoration(String hint) {
    return InputDecoration(
      hintText: hint,
      filled: true,
      fillColor: Colors.white,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: const BorderSide(color: AppColors.primary),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: const BorderSide(color: AppColors.primary),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: const BorderSide(color: AppColors.primaryHover, width: 2),
      ),
    );
  }

  Future<void> _handleSubmit(String businessType) async {
    if (!_formKey.currentState!.validate()) return;

    final authProvider = context.read<AuthProvider>();
    final personProvider = context.read<PersonProvider>();
    final businessProfileProvider = context.read<BusinessProfileProvider>();
    final token = authProvider.auth?.accessToken ?? '';
    final person = personProvider.person;

    if (token.isEmpty || person == null) {
      setState(() => _error = AppLocalizations.of(context)!.businessProfileFormError);
      return;
    }

    setState(() {
      _submitting = true;
      _error = null;
    });

    try {
      final created = await GrpcBusinessProfileService.addBusinessProfile(
        BusinessProfile(
          ownerId: person.id,
          ownerUuid: person.uuid,
          taxId: _taxIdController.text.trim(),
          businessName: _businessNameController.text.trim(),
          businessType: businessType,
          socialName: _socialNameController.text.trim(),
        ),
      );

      final refreshedPerson = await GrpcPersonService.getPerson(id: created.ownerId);
      final index = refreshedPerson.businessProfiles.indexWhere((p) => p.uuid == created.uuid);

      if (index == -1) {
        throw Exception('Created business profile not found in refreshed profile list');
      }

      await personProvider.switchProfile(
        index,
        token,
        onTokenIssued: authProvider.applySwitchedToken,
      );
      await businessProfileProvider.load(uuid: created.uuid);

      if (!mounted) return;
      Navigator.of(context).pushNamedAndRemoveUntil('/feed', (route) => false);
    } catch (_) {
      if (!mounted) return;
      setState(() => _error = AppLocalizations.of(context)!.businessProfileFormError);
    } finally {
      if (mounted) {
        setState(() => _submitting = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = ModalRoute.of(context)!.settings.arguments as String;

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(gradient: AppColors.gradient3),
        child: SafeArea(
          child: Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24),
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 480),
                child: Card(
                  elevation: 4,
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Form(
                      key: _formKey,
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            businessType == 'Company'
                                ? l10n.businessProfileFormTitleGym
                                : l10n.businessProfileFormTitlePersonalTrainer,
                            style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
                          ),
                          const SizedBox(height: 16),
                          if (_error != null) ...[
                            Container(
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: AppColors.danger.withAlpha(25),
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(color: AppColors.danger.withAlpha(76)),
                              ),
                              child: Text(_error!, style: const TextStyle(color: AppColors.danger, fontSize: 14)),
                            ),
                            const SizedBox(height: 16),
                          ],
                          TextFormField(
                            key: const ValueKey('business_name_field'),
                            controller: _businessNameController,
                            decoration: _inputDecoration(l10n.businessProfileFormBusinessName),
                            validator: (value) => (value == null || value.trim().isEmpty)
                                ? l10n.businessProfileFormBusinessNameRequired
                                : null,
                          ),
                          const SizedBox(height: 12),
                          TextFormField(
                            key: const ValueKey('social_name_field'),
                            controller: _socialNameController,
                            decoration: _inputDecoration(l10n.businessProfileFormSocialName),
                            validator: (value) => (value == null || value.trim().isEmpty)
                                ? l10n.businessProfileFormSocialNameRequired
                                : null,
                          ),
                          const SizedBox(height: 12),
                          TextFormField(
                            key: const ValueKey('tax_id_field'),
                            controller: _taxIdController,
                            decoration: _inputDecoration(l10n.businessProfileFormTaxId),
                            validator: (value) => (value == null || value.trim().isEmpty)
                                ? l10n.businessProfileFormTaxIdRequired
                                : null,
                          ),
                          const SizedBox(height: 24),
                          SizedBox(
                            width: double.infinity,
                            child: ElevatedButton(
                              onPressed: _submitting ? null : () => _handleSubmit(businessType),
                              style: ElevatedButton.styleFrom(
                                backgroundColor: AppColors.primary,
                                disabledBackgroundColor: AppColors.primaryDisabled,
                                foregroundColor: Colors.white,
                                padding: const EdgeInsets.symmetric(vertical: 14),
                              ),
                              child: _submitting
                                  ? const SizedBox(
                                      width: 20,
                                      height: 20,
                                      child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                                    )
                                  : Text(l10n.businessProfileFormSubmit),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
