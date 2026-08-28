import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/sign_up_request.dart';
import '../../providers/auth_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/legal_document_service.dart';

class SignUpPage extends StatefulWidget {
  const SignUpPage({super.key});

  @override
  State<SignUpPage> createState() => _SignUpPageState();
}

class _SignUpPageState extends State<SignUpPage> {
  final _formKey = GlobalKey<FormState>();
  final _firstnameController = TextEditingController();
  final _surnameController = TextEditingController();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _firstnameFocus = FocusNode();
  final _surnameFocus = FocusNode();
  final _customGenderFocus = FocusNode();
  final _emailFocus = FocusNode();
  final _passwordFocus = FocusNode();

  String? _selectedMonth;
  String? _selectedDay;
  String? _selectedYear;
  String? _selectedGender;
  final _customGenderController = TextEditingController();

  bool _obscurePassword = true;
  bool _termsAccepted = false;
  bool _privacyAccepted = false;
  String _termsVersion = '1.0.0';
  String _privacyVersion = '1.0.0';

  Future<void> _showLegalDocument(String document) async {
    try {
      final legal = await LegalDocumentService.get(document);
      if (!mounted) return;
      setState(() {
        if (document == 'terms') _termsVersion = legal.version;
        if (document == 'privacy') _privacyVersion = legal.version;
      });
      await showDialog<void>(
        context: context,
        builder: (context) => AlertDialog(
          title: Text(legal.title),
          content: SizedBox(
            width: 620,
            child: SingleChildScrollView(child: SelectableText(legal.content)),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Fechar'),
            ),
          ],
        ),
      );
    } catch (_) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Documento legal indisponível. Tente novamente.'),
        ),
      );
    }
  }

  List<Map<String, String>> _getMonths(AppLocalizations l10n) => [
    {'value': '01', 'label': l10n.monthJanuary},
    {'value': '02', 'label': l10n.monthFebruary},
    {'value': '03', 'label': l10n.monthMarch},
    {'value': '04', 'label': l10n.monthApril},
    {'value': '05', 'label': l10n.monthMay},
    {'value': '06', 'label': l10n.monthJune},
    {'value': '07', 'label': l10n.monthJuly},
    {'value': '08', 'label': l10n.monthAugust},
    {'value': '09', 'label': l10n.monthSeptember},
    {'value': '10', 'label': l10n.monthOctober},
    {'value': '11', 'label': l10n.monthNovember},
    {'value': '12', 'label': l10n.monthDecember},
  ];

  List<String> get _days =>
      List.generate(31, (i) => (i + 1).toString().padLeft(2, '0'));

  List<String> get _years {
    final currentYear = DateTime.now().year;
    return List.generate(100, (i) => (currentYear - i).toString());
  }

  @override
  void dispose() {
    _firstnameController.dispose();
    _surnameController.dispose();
    _emailController.dispose();
    _passwordController.dispose();
    _customGenderController.dispose();
    _firstnameFocus.dispose();
    _surnameFocus.dispose();
    _customGenderFocus.dispose();
    _emailFocus.dispose();
    _passwordFocus.dispose();
    super.dispose();
  }

  Future<void> _handleSignUp() async {
    final l10n = AppLocalizations.of(context)!;
    if (!_formKey.currentState!.validate()) return;

    if (_selectedMonth == null ||
        _selectedDay == null ||
        _selectedYear == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(l10n.validationDateOfBirthRequired),
          backgroundColor: AppColors.danger,
        ),
      );
      return;
    }

    if (_selectedGender == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(l10n.validationGenderRequired),
          backgroundColor: AppColors.danger,
        ),
      );
      return;
    }

    final gender = _selectedGender == 'Custom'
        ? _customGenderController.text.trim()
        : _selectedGender!;

    final dateOfBirth = '$_selectedYear-$_selectedMonth-$_selectedDay';
    final birthDate = DateTime.tryParse(dateOfBirth);
    final now = DateTime.now();
    final cutoff = DateTime(now.year - 18, now.month, now.day);
    if (birthDate == null || birthDate.isAfter(cutoff)) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Você deve ter pelo menos 18 anos.')),
      );
      return;
    }
    if (!_termsAccepted || !_privacyAccepted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text(
            'Aceite os Termos e a Política de Privacidade para continuar.',
          ),
        ),
      );
      return;
    }

    final request = SignUpRequest(
      firstname: _firstnameController.text.trim(),
      surname: _surnameController.text.trim(),
      email: _emailController.text.trim(),
      password: _passwordController.text,
      dateOfBirth: dateOfBirth,
      gender: gender,
      termsVersion: _termsVersion,
      privacyVersion: _privacyVersion,
      termsAccepted: _termsAccepted,
      privacyAccepted: _privacyAccepted,
    );

    final authProvider = context.read<AuthProvider>();
    final success = await authProvider.signUp(request);

    if (success && mounted) {
      Navigator.of(context).pushReplacementNamed('/feed');
    }
  }

  InputDecoration _inputDecoration(String hint, String? businessType) {
    return InputDecoration(
      hintText: hint,
      contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 14),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: BorderSide(color: AppColors.primaryFor(businessType)),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: BorderSide(color: AppColors.primaryFor(businessType)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: BorderSide(
          color: AppColors.primaryHoverFor(businessType),
          width: 2,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final months = _getMonths(l10n);
    final screenWidth = MediaQuery.of(context).size.width;
    final isDesktop = screenWidth >= 640;
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

    return Scaffold(
      body: Container(
        width: double.infinity,
        height: double.infinity,
        decoration: const BoxDecoration(gradient: AppColors.gradient3),
        child: SafeArea(
          child: LayoutBuilder(
            builder: (context, constraints) {
              return SingleChildScrollView(
                child: ConstrainedBox(
                  constraints: BoxConstraints(minHeight: constraints.maxHeight),
                  child: Padding(
                    padding: EdgeInsets.symmetric(
                      vertical: 24,
                      horizontal: isDesktop ? 48 : 0,
                    ),
                    child: Center(
                      child: isDesktop
                          ? _buildDesktopLayout(
                              l10n,
                              months,
                              screenWidth,
                              businessType,
                            )
                          : _buildMobileLayout(l10n, months, businessType),
                    ),
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }

  /// Desktop/Tablet: Logo on the left, form on the right
  Widget _buildDesktopLayout(
    AppLocalizations l10n,
    List<Map<String, String>> months,
    double screenWidth,
    String? businessType,
  ) {
    final logoWidth = screenWidth >= 1024 ? 250.0 : 180.0;

    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 950),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            // Logo on the left
            Image.asset(
              'assets/images/logo.png',
              width: logoWidth,
              height: logoWidth,
            ),

            const SizedBox(width: 32),

            // Form on the right
            Expanded(
              child: ConstrainedBox(
                constraints: BoxConstraints(
                  maxWidth: screenWidth >= 1024 ? 500 : 450,
                ),
                child: _buildFormCard(l10n, months, businessType),
              ),
            ),
          ],
        ),
      ),
    );
  }

  /// Mobile: Logo on top, form below
  Widget _buildMobileLayout(
    AppLocalizations l10n,
    List<Map<String, String>> months,
    String? businessType,
  ) {
    return Column(
      children: [
        // Logo
        Image.asset('assets/images/logo.png', width: 120, height: 120),

        const SizedBox(height: 24),

        // Form Card
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 24),
          child: _buildFormCard(l10n, months, businessType),
        ),

        const SizedBox(height: 32),
      ],
    );
  }

  Widget _buildFormCard(
    AppLocalizations l10n,
    List<Map<String, String>> months,
    String? businessType,
  ) {
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: AppColors.background,
        borderRadius: BorderRadius.circular(8),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(25),
            blurRadius: 6,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Consumer<AuthProvider>(
        builder: (context, authProvider, _) {
          return Form(
            key: _formKey,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Title
                Text(
                  l10n.signUpTitle,
                  style: const TextStyle(
                    fontSize: 22,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF333333),
                  ),
                  textAlign: TextAlign.center,
                ),

                const SizedBox(height: 24),

                // Error message
                if (authProvider.error != null) ...[
                  Container(
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: AppColors.danger.withAlpha(25),
                      borderRadius: BorderRadius.circular(4),
                      border: Border.all(color: AppColors.danger.withAlpha(76)),
                    ),
                    child: Text(
                      authProvider.error!,
                      style: const TextStyle(
                        color: AppColors.danger,
                        fontSize: 14,
                      ),
                      textAlign: TextAlign.center,
                    ),
                  ),
                  const SizedBox(height: 16),
                ],

                // First Name
                TextFormField(
                  controller: _firstnameController,
                  focusNode: _firstnameFocus,
                  textInputAction: TextInputAction.next,
                  decoration: _inputDecoration(
                    l10n.signUpFirstName,
                    businessType,
                  ),
                  onFieldSubmitted: (_) {
                    FocusScope.of(context).requestFocus(_surnameFocus);
                  },
                  validator: (value) {
                    if (value == null || value.trim().isEmpty) {
                      return l10n.validationFirstNameRequired;
                    }
                    return null;
                  },
                ),

                const SizedBox(height: 16),

                // Surname
                TextFormField(
                  controller: _surnameController,
                  focusNode: _surnameFocus,
                  textInputAction: TextInputAction.next,
                  decoration: _inputDecoration(
                    l10n.signUpSurname,
                    businessType,
                  ),
                  onFieldSubmitted: (_) {
                    final nextFocus = _selectedGender == 'Custom'
                        ? _customGenderFocus
                        : _emailFocus;
                    FocusScope.of(context).requestFocus(nextFocus);
                  },
                  validator: (value) {
                    if (value == null || value.trim().isEmpty) {
                      return l10n.validationSurnameRequired;
                    }
                    return null;
                  },
                ),

                const SizedBox(height: 16),

                // Birthday - Row of dropdowns
                Row(
                  children: [
                    // Month
                    Expanded(
                      flex: 4,
                      child: DropdownButtonFormField<String>(
                        initialValue: _selectedMonth,
                        hint: Text(
                          l10n.signUpBirthdayMonth,
                          style: const TextStyle(fontSize: 14),
                        ),
                        isExpanded: true,
                        decoration: _inputDecoration('', businessType),
                        items: months.map((m) {
                          return DropdownMenuItem(
                            value: m['value'],
                            child: Text(
                              m['label']!,
                              style: const TextStyle(fontSize: 14),
                            ),
                          );
                        }).toList(),
                        onChanged: (value) {
                          setState(() {
                            _selectedMonth = value;
                          });
                        },
                      ),
                    ),
                    const SizedBox(width: 8),
                    // Day
                    Expanded(
                      flex: 3,
                      child: DropdownButtonFormField<String>(
                        initialValue: _selectedDay,
                        hint: Text(
                          l10n.signUpBirthdayDay,
                          style: const TextStyle(fontSize: 14),
                        ),
                        isExpanded: true,
                        decoration: _inputDecoration('', businessType),
                        items: _days.map((d) {
                          return DropdownMenuItem(
                            value: d,
                            child: Text(
                              d,
                              style: const TextStyle(fontSize: 14),
                            ),
                          );
                        }).toList(),
                        onChanged: (value) {
                          setState(() {
                            _selectedDay = value;
                          });
                        },
                      ),
                    ),
                    const SizedBox(width: 8),
                    // Year
                    Expanded(
                      flex: 3,
                      child: DropdownButtonFormField<String>(
                        initialValue: _selectedYear,
                        hint: Text(
                          l10n.signUpBirthdayYear,
                          style: const TextStyle(fontSize: 14),
                        ),
                        isExpanded: true,
                        decoration: _inputDecoration('', businessType),
                        items: _years.map((y) {
                          return DropdownMenuItem(
                            value: y,
                            child: Text(
                              y,
                              style: const TextStyle(fontSize: 14),
                            ),
                          );
                        }).toList(),
                        onChanged: (value) {
                          setState(() {
                            _selectedYear = value;
                          });
                        },
                      ),
                    ),
                  ],
                ),

                const SizedBox(height: 16),

                // Gender Radio Group
                Text(
                  l10n.signUpGender,
                  style: const TextStyle(
                    fontSize: 14,
                    fontWeight: FontWeight.w500,
                    color: Color(0xFF333333),
                  ),
                ),
                const SizedBox(height: 8),
                Row(
                  children: [
                    _genderRadio(
                      'Female',
                      l10n.signUpGenderFemale,
                      businessType,
                    ),
                    _genderRadio('Male', l10n.signUpGenderMale, businessType),
                    _genderRadio(
                      'Custom',
                      l10n.signUpGenderCustom,
                      businessType,
                    ),
                  ],
                ),

                // Custom Gender Field
                if (_selectedGender == 'Custom') ...[
                  const SizedBox(height: 12),
                  TextFormField(
                    controller: _customGenderController,
                    focusNode: _customGenderFocus,
                    textInputAction: TextInputAction.next,
                    decoration: _inputDecoration(
                      l10n.signUpGenderCustomPlaceholder,
                      businessType,
                    ),
                    onFieldSubmitted: (_) {
                      FocusScope.of(context).requestFocus(_emailFocus);
                    },
                  ),
                ],

                const SizedBox(height: 16),

                // Email
                TextFormField(
                  controller: _emailController,
                  focusNode: _emailFocus,
                  keyboardType: TextInputType.emailAddress,
                  textInputAction: TextInputAction.next,
                  decoration: _inputDecoration(
                    l10n.signUpMobileOrEmail,
                    businessType,
                  ),
                  onFieldSubmitted: (_) {
                    FocusScope.of(context).requestFocus(_passwordFocus);
                  },
                  validator: (value) {
                    if (value == null || value.trim().isEmpty) {
                      return l10n.validationEmailRequired;
                    }
                    return null;
                  },
                ),

                const SizedBox(height: 16),

                // Password
                TextFormField(
                  controller: _passwordController,
                  focusNode: _passwordFocus,
                  obscureText: _obscurePassword,
                  textInputAction: TextInputAction.done,
                  decoration:
                      _inputDecoration(
                        l10n.signUpPassword,
                        businessType,
                      ).copyWith(
                        suffixIcon: IconButton(
                          icon: Icon(
                            _obscurePassword
                                ? Icons.visibility_off
                                : Icons.visibility,
                            color: AppColors.primaryFor(businessType),
                          ),
                          onPressed: () {
                            setState(() {
                              _obscurePassword = !_obscurePassword;
                            });
                          },
                        ),
                      ),
                  onFieldSubmitted: (_) {
                    if (!context.read<AuthProvider>().loading) {
                      _handleSignUp();
                    }
                  },
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return l10n.validationPasswordRequired;
                    }
                    if (value.length < 6) {
                      return l10n.validationPasswordMinLength;
                    }
                    return null;
                  },
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 4),
                  child: Text(
                    l10n.signUpPasswordHint,
                    style: const TextStyle(
                      fontSize: 12,
                      color: Color(0xFF666666),
                      fontStyle: FontStyle.italic,
                    ),
                  ),
                ),

                const SizedBox(height: 16),
                CheckboxListTile(
                  contentPadding: EdgeInsets.zero,
                  controlAffinity: ListTileControlAffinity.leading,
                  value: _termsAccepted,
                  onChanged: (value) =>
                      setState(() => _termsAccepted = value ?? false),
                  title: Wrap(
                    children: [
                      const Text('Li e aceito os '),
                      InkWell(
                        onTap: () => _showLegalDocument('terms'),
                        child: const Text(
                          'Termos de Uso',
                          style: TextStyle(
                            decoration: TextDecoration.underline,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                CheckboxListTile(
                  contentPadding: EdgeInsets.zero,
                  controlAffinity: ListTileControlAffinity.leading,
                  value: _privacyAccepted,
                  onChanged: (value) =>
                      setState(() => _privacyAccepted = value ?? false),
                  title: Wrap(
                    children: [
                      const Text('Li e aceito a '),
                      InkWell(
                        onTap: () => _showLegalDocument('privacy'),
                        child: const Text(
                          'Política de Privacidade',
                          style: TextStyle(
                            decoration: TextDecoration.underline,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),

                const SizedBox(height: 24),

                // Sign Up Button
                SizedBox(
                  height: 48,
                  child: ElevatedButton(
                    onPressed:
                        authProvider.loading ||
                            !_termsAccepted ||
                            !_privacyAccepted
                        ? null
                        : _handleSignUp,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primaryFor(businessType),
                      disabledBackgroundColor: AppColors.primaryDisabledFor(
                        businessType,
                      ),
                      foregroundColor: Colors.white,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(4),
                      ),
                    ),
                    child: authProvider.loading
                        ? const SizedBox(
                            width: 24,
                            height: 24,
                            child: CircularProgressIndicator(
                              color: Colors.white,
                              strokeWidth: 2,
                            ),
                          )
                        : Text(
                            l10n.signUpSubmit,
                            style: const TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                  ),
                ),

                const SizedBox(height: 16),

                // Divider
                Row(
                  children: [
                    const Expanded(child: Divider()),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      child: Text(
                        l10n.signInOr,
                        style: const TextStyle(
                          color: Colors.grey,
                          fontSize: 14,
                        ),
                      ),
                    ),
                    const Expanded(child: Divider()),
                  ],
                ),

                const SizedBox(height: 16),

                // Already have an account
                Center(
                  child: TextButton(
                    onPressed: () {
                      authProvider.clearError();
                      Navigator.of(context).pop();
                    },
                    child: Text(
                      l10n.signUpAlreadyHaveAccount,
                      style: TextStyle(
                        color: AppColors.primaryFor(businessType),
                        fontSize: 14,
                      ),
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _genderRadio(String value, String label, String? businessType) {
    return Expanded(
      child: InkWell(
        onTap: () {
          setState(() {
            _selectedGender = value;
          });
        },
        borderRadius: BorderRadius.circular(4),
        child: Container(
          padding: const EdgeInsets.symmetric(vertical: 8),
          decoration: BoxDecoration(
            border: Border.all(
              color: _selectedGender == value
                  ? AppColors.primaryFor(businessType)
                  : Colors.grey.shade300,
            ),
            borderRadius: BorderRadius.circular(4),
            color: _selectedGender == value
                ? AppColors.primaryFor(businessType).withAlpha(25)
                : null,
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                _selectedGender == value
                    ? Icons.radio_button_checked
                    : Icons.radio_button_unchecked,
                color: _selectedGender == value
                    ? AppColors.primaryFor(businessType)
                    : Colors.grey,
                size: 20,
              ),
              const SizedBox(width: 4),
              Text(
                label,
                style: TextStyle(
                  fontSize: 13,
                  color: _selectedGender == value
                      ? AppColors.primaryFor(businessType)
                      : const Color(0xFF333333),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
