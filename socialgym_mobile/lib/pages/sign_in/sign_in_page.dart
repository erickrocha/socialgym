import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/models/enums.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/auth_provider.dart';
import '../../providers/locale_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/resource_provider.dart';
import '../../providers/settings_provider.dart';
import '../../widgets/language_selector.dart';
import 'package:socialgym_mobile/pages/sign_up/sign_up_page.dart';

class SignInPage extends StatefulWidget {
  const SignInPage({super.key});

  @override
  State<SignInPage> createState() => _SignInPageState();
}

class _SignInPageState extends State<SignInPage> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _emailFocus = FocusNode();
  final _passwordFocus = FocusNode();
  bool _checkingAuth = true;

  @override
  void initState() {
    super.initState();
    _checkExistingAuth();
  }

  Future<void> _checkExistingAuth() async {
    final authProvider = context.read<AuthProvider>();

    // Wait for auth provider to initialize from storage
    if (!authProvider.initialized) {
      authProvider.addListener(_onAuthInitialized);
    } else {
      await _handleAuthCheck();
    }
  }

  void _onAuthInitialized() {
    final authProvider = context.read<AuthProvider>();
    if (authProvider.initialized) {
      authProvider.removeListener(_onAuthInitialized);
      _handleAuthCheck();
    }
  }

  Future<void> _handleAuthCheck() async {
    final authProvider = context.read<AuthProvider>();

    if (authProvider.isAuthenticated && authProvider.auth?.accessToken != null) {
      final personProvider = context.read<PersonProvider>();
      final resourceProvider = context.read<ResourceProvider>();
      final settingsProvider = context.read<SettingsProvider>();
      final localeProvider = context.read<LocaleProvider>();
      final token = authProvider.auth!.accessToken;

      await personProvider.fetchMe();
      await personProvider.restoreActiveBusinessProfileFromToken(token);

      // Only fetch resources if settings are not already cached
      // This avoids redundant API calls when settings exist locally
      if (!settingsProvider.hasSettingsCached()) {
        await resourceProvider.fetchResources(personProvider.ownerId,
          onSettingsReceived: (settings) async {
            await settingsProvider.applySettings(settings);
            if (settings.language != null) {
              await localeProvider.applyUserSettingsLanguage(settings.language);
            }
          },
        );
      } else {
        // Settings already cached locally; fetch only resources (countries, etc)
        await resourceProvider.fetchResources(personProvider.ownerId);
      }

      if (mounted) {
        final homePage = settingsProvider.homePage;
        final targetRoute = homePage == Pages.gallery ? '/gallery' : '/feed';
        Navigator.of(context).pushReplacementNamed(targetRoute);
        return;
      }
    }

    if (mounted) {
      setState(() {
        _checkingAuth = false;
      });
    }
  }

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    _emailFocus.dispose();
    _passwordFocus.dispose();
    super.dispose();
  }

  Future<void> _handleSignIn() async {
    if (!_formKey.currentState!.validate()) return;

    final authProvider = context.read<AuthProvider>();
    final success = await authProvider.signIn(
      email: _emailController.text.trim(),
      password: _passwordController.text,
    );

    if (success && mounted) {
      final pending = authProvider.auth?.pendingAccountDeletion;
      if (pending != null && mounted) {
        await _showPendingDeletionDialog(authProvider, pending.scheduledAt);
      }
      if (!mounted) return;

      // Fetch person data and resources after successful sign in
      final personProvider = context.read<PersonProvider>();
      final resourceProvider = context.read<ResourceProvider>();
      final settingsProvider = context.read<SettingsProvider>();
      final localeProvider = context.read<LocaleProvider>();
      final token = authProvider.auth!.accessToken;

      await personProvider.fetchMe();
      await personProvider.restoreActiveBusinessProfileFromToken(token);

      // Fetch resources and apply settings
      await resourceProvider.fetchResources(personProvider.ownerId,
        onSettingsReceived: (settings) async {
          await settingsProvider.applySettings(settings);
          if (settings.language != null) {
            await localeProvider.applyUserSettingsLanguage(settings.language);
          }
        },
      );

      if (mounted) {
        final homePage = settingsProvider.homePage;
        final targetRoute = homePage == Pages.gallery ? '/gallery' : '/feed';
        Navigator.of(context).pushReplacementNamed(targetRoute);
        return;
      }
    }
  }

  /// Shown right after a fresh login when the account has a pending, not-yet
  /// -purged deletion request — lets the user reactivate it, or dismiss and
  /// keep using the app with the deletion still scheduled.
  Future<void> _showPendingDeletionDialog(
    AuthProvider authProvider,
    DateTime scheduledAt,
  ) async {
    final l10n = AppLocalizations.of(context)!;
    final formattedDate = DateFormat.yMMMd().format(scheduledAt.toLocal());

    final keepAccount = await showDialog<bool>(
      context: context,
      barrierDismissible: false,
      builder: (dialogContext) => AlertDialog(
        title: Text(l10n.signInPendingDeletionTitle),
        content: Text(l10n.signInPendingDeletionBody(formattedDate)),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(dialogContext).pop(false),
            child: Text(l10n.signInPendingDeletionDismissButton),
          ),
          FilledButton(
            onPressed: () => Navigator.of(dialogContext).pop(true),
            child: Text(l10n.signInPendingDeletionKeepButton),
          ),
        ],
      ),
    );

    if (keepAccount != true || !mounted) return;

    final cancelled = await authProvider.cancelAccountDeletion();
    if (!mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          cancelled
              ? l10n.signInPendingDeletionCancelSuccess
              : (authProvider.error ?? l10n.signInPendingDeletionCancelError),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    // Show loading indicator while checking existing auth
    if (_checkingAuth) {
      return Scaffold(
        backgroundColor: AppColors.background,
        body: const Center(child: CircularProgressIndicator()),
      );
    }

    final l10n = AppLocalizations.of(context)!;
    final screenWidth = MediaQuery.of(context).size.width;
    final isDesktop = screenWidth >= 640;
    final businessType =
        context.watch<PersonProvider>().activeBusinessProfile?.businessType;

    return Scaffold(
      backgroundColor: AppColors.background,
      body: SafeArea(
        child: Column(
          children: [
            // Gradient Header
            Container(
              width: double.infinity,
              padding: EdgeInsets.symmetric(vertical: isDesktop ? 60 : 40),
              decoration: const BoxDecoration(gradient: AppColors.gradient3),
            ),

            // Body: scrollable content, vertically centered
            Expanded(
              child: LayoutBuilder(
                builder: (context, constraints) {
                  return SingleChildScrollView(
                    child: ConstrainedBox(
                      constraints: BoxConstraints(minHeight: constraints.maxHeight),
                      child: Padding(
                        padding: EdgeInsets.symmetric(
                          horizontal: isDesktop ? 48 : 24,
                          vertical: 32,
                        ),
                        child: Center(
                          child: isDesktop
                              ? _buildDesktopLayout(l10n, screenWidth, businessType)
                              : _buildMobileLayout(l10n, businessType),
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),

            // Footer with language selector
            const Padding(padding: EdgeInsets.only(bottom: 8), child: LanguageSelectorButton()),
          ],
        ),
      ),
    );
  }

  /// Desktop/Tablet: Logo on the left, form on the right
  Widget _buildDesktopLayout(AppLocalizations l10n, double screenWidth, String? businessType) {
    final logoWidth = screenWidth >= 1024 ? 300.0 : 200.0;

    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 900),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            // Logo on the left
            Image.asset('assets/images/logo.png', width: logoWidth, height: logoWidth),

            const SizedBox(width: 32),

            // Form on the right
            SizedBox(
              width: screenWidth >= 1024 ? 450 : 400,
              child: _buildFormCard(l10n, businessType),
            ),
          ],
        ),
      ),
    );
  }

  /// Mobile: Logo on top, form below
  Widget _buildMobileLayout(AppLocalizations l10n, String? businessType) {
    return Column(
      children: [
        // Logo
        Image.asset('assets/images/logo.png', width: 150, height: 150),

        const SizedBox(height: 32),

        // Form Card
        _buildFormCard(l10n, businessType),
      ],
    );
  }

  Widget _buildFormCard(AppLocalizations l10n, String? businessType) {
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(8),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(25), blurRadius: 6, offset: const Offset(0, 4)),
        ],
      ),
      child: Consumer<AuthProvider>(
        builder: (context, authProvider, _) {
          return Form(
            key: _formKey,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
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
                      style: const TextStyle(color: AppColors.danger, fontSize: 14),
                      textAlign: TextAlign.center,
                    ),
                  ),
                  const SizedBox(height: 16),
                ],

                // Email field
                TextFormField(
                  controller: _emailController,
                  focusNode: _emailFocus,
                  keyboardType: TextInputType.emailAddress,
                  textInputAction: TextInputAction.next,
                  decoration: _inputDecoration(l10n.signInEmail, businessType),
                  onFieldSubmitted: (_) => FocusScope.of(context).requestFocus(_passwordFocus),
                  validator: (value) {
                    if (value == null || value.trim().isEmpty) {
                      return l10n.validationEmailRequired;
                    }
                    return null;
                  },
                ),

                const SizedBox(height: 16),

                // Password field
                TextFormField(
                  controller: _passwordController,
                  focusNode: _passwordFocus,
                  obscureText: true,
                  textInputAction: TextInputAction.done,
                  decoration: _inputDecoration(l10n.signInPassword, businessType),
                  onFieldSubmitted: (_) => {
                    if (!context.read<AuthProvider>().loading) {_handleSignIn()},
                  },
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return l10n.validationPasswordRequired;
                    }
                    return null;
                  },
                ),

                const SizedBox(height: 16),

                // Sign In Button
                SizedBox(
                  height: 48,
                  child: ElevatedButton(
                    onPressed: authProvider.loading ? null : _handleSignIn,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primaryFor(businessType),
                      disabledBackgroundColor: AppColors.primaryDisabledFor(businessType),
                      foregroundColor: Colors.white,
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
                    ),
                    child: authProvider.loading
                        ? const SizedBox(
                            width: 24,
                            height: 24,
                            child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                          )
                        : Text(
                            l10n.signInSubmit,
                            style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                          ),
                  ),
                ),

                const SizedBox(height: 16),

                // Forgot Password
                TextButton(
                  onPressed: () {
                    // TODO: Navigate to password recovery
                  },
                  child: Text(
                    l10n.signInForgotPassword,
                    style: TextStyle(color: AppColors.primaryFor(businessType), fontSize: 14),
                  ),
                ),

                const SizedBox(height: 8),

                // Divider
                Row(
                  children: [
                    const Expanded(child: Divider()),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      child: Text(
                        l10n.signInOr,
                        style: const TextStyle(color: Colors.grey, fontSize: 14),
                      ),
                    ),
                    const Expanded(child: Divider()),
                  ],
                ),

                const SizedBox(height: 16),

                // Create Account Button
                SizedBox(
                  height: 48,
                  child: ElevatedButton(
                    onPressed: authProvider.loading
                        ? null
                        : () {
                            authProvider.clearError();
                            Navigator.of(
                              context,
                            ).push(MaterialPageRoute(builder: (_) => const SignUpPage()));
                          },
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.secondary,
                      disabledBackgroundColor: AppColors.secondaryDisabled,
                      foregroundColor: Colors.white,
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
                    ),
                    child: Text(
                      l10n.signInCreateAccount,
                      style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
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
        borderSide: BorderSide(color: AppColors.primaryHoverFor(businessType), width: 2),
      ),
    );
  }
}
