import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/business_profile.dart';
import '../../models/business_profile_address.dart';
import '../../providers/auth_provider.dart';
import '../../providers/business_profile_provider.dart';
import '../../providers/person_provider.dart';
import '../../widgets/main_layout.dart';

class BusinessProfilePage extends StatefulWidget {
  const BusinessProfilePage({super.key});

  @override
  State<BusinessProfilePage> createState() => _BusinessProfilePageState();
}

class _BusinessProfilePageState extends State<BusinessProfilePage> {
  final _businessNameController = TextEditingController();
  final _socialNameController = TextEditingController();
  final _taxIdController = TextEditingController();
  bool _isEditing = false;

  bool _isAddressFormExpanded = false;
  BusinessProfileAddress? _editingAddress;
  final _addressLine1Controller = TextEditingController();
  final _addressLine2Controller = TextEditingController();
  final _localityController = TextEditingController();
  final _administrativeAreaController = TextEditingController();
  final _postalCodeController = TextEditingController();
  final _countryCodeController = TextEditingController();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _ensureLoaded());
  }

  Future<void> _ensureLoaded() async {
    final provider = context.read<BusinessProfileProvider>();
    if (provider.current != null) {
      _captureOriginalValues(provider.current!);
      return;
    }
    final activeProfile = context.read<PersonProvider>().activeBusinessProfile;
    if (activeProfile == null) return;
    await provider.load(uuid: activeProfile.uuid);
    final loaded = provider.current;
    if (loaded != null && mounted) _captureOriginalValues(loaded);
  }

  void _captureOriginalValues(BusinessProfile profile) {
    _businessNameController.text = profile.businessName;
    _socialNameController.text = profile.socialName ?? '';
    _taxIdController.text = profile.taxId;
  }

  @override
  void dispose() {
    _businessNameController.dispose();
    _socialNameController.dispose();
    _taxIdController.dispose();
    _addressLine1Controller.dispose();
    _addressLine2Controller.dispose();
    _localityController.dispose();
    _administrativeAreaController.dispose();
    _postalCodeController.dispose();
    _countryCodeController.dispose();
    super.dispose();
  }

  Future<void> _saveChanges() async {
    final provider = context.read<BusinessProfileProvider>();
    final current = provider.current;
    if (current == null) return;
    final updated = BusinessProfile(
      id: current.id,
      uuid: current.uuid,
      ownerId: current.ownerId,
      ownerUuid: current.ownerUuid,
      taxId: _taxIdController.text.trim(),
      businessName: _businessNameController.text.trim(),
      businessType: current.businessType,
      socialName: _socialNameController.text.trim(),
      logo: current.logo,
      coverImage: current.coverImage,
    );
    final success = await provider.update(updated);
    if (!mounted) return;
    if (success) {
      setState(() => _isEditing = false);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(provider.error ?? 'Failed to save changes.')),
      );
    }
  }

  Future<void> _pickImage(ImageSource source, {required bool isLogo}) async {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    final provider = context.read<BusinessProfileProvider>();
    final picked = await ImagePicker().pickImage(
      source: source,
      maxWidth: isLogo ? 512 : 1920,
      maxHeight: isLogo ? 512 : 1080,
      imageQuality: 85,
    );
    if (picked == null) return;

    final success = isLogo
        ? await provider.uploadLogo(token, picked)
        : await provider.uploadCover(token, picked);
    if (!mounted) return;
    if (!success) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(provider.error ?? 'Failed to upload image.')),
      );
    }
  }

  void _showImageSourceDialog({required bool isLogo}) {
    showModalBottomSheet(
      context: context,
      builder: (sheetContext) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: const Icon(Icons.camera_alt_outlined),
              title: const Text('Camera'),
              onTap: () {
                Navigator.pop(sheetContext);
                _pickImage(ImageSource.camera, isLogo: isLogo);
              },
            ),
            ListTile(
              leading: const Icon(Icons.photo_library_outlined),
              title: const Text('Gallery'),
              onTap: () {
                Navigator.pop(sheetContext);
                _pickImage(ImageSource.gallery, isLogo: isLogo);
              },
            ),
          ],
        ),
      ),
    );
  }

  void _openAddressForm(BusinessProfileAddress? address) {
    setState(() {
      _editingAddress = address;
      _isAddressFormExpanded = true;
      _addressLine1Controller.text = address?.addressLine1 ?? '';
      _addressLine2Controller.text = address?.addressLine2 ?? '';
      _localityController.text = address?.locality ?? '';
      _administrativeAreaController.text = address?.administrativeArea ?? '';
      _postalCodeController.text = address?.postalCode ?? '';
      _countryCodeController.text = address?.countryCode ?? '';
    });
  }

  void _closeAddressForm() {
    setState(() {
      _editingAddress = null;
      _isAddressFormExpanded = false;
    });
  }

  Future<void> _saveAddress() async {
    final provider = context.read<BusinessProfileProvider>();
    final current = provider.current;
    if (current == null || current.id == null) return;
    final address = BusinessProfileAddress(
      id: _editingAddress?.id,
      uuid: _editingAddress?.uuid,
      businessProfileId: current.id!,
      addressLine1: _addressLine1Controller.text.trim(),
      addressLine2: _addressLine2Controller.text.trim(),
      locality: _localityController.text.trim(),
      administrativeArea: _administrativeAreaController.text.trim(),
      postalCode: _postalCodeController.text.trim(),
      countryCode: _countryCodeController.text.trim(),
    );
    final success = _editingAddress == null
        ? await provider.addAddress(address)
        : await provider.updateAddress(address);
    if (!mounted) return;
    if (success) {
      _closeAddressForm();
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(provider.error ?? 'Failed to save address.')),
      );
    }
  }

  Future<void> _confirmDeleteAddress(BusinessProfileAddress address) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        title: const Text('Delete address?'),
        content: Text(address.addressLine1),
        actions: [
          TextButton(onPressed: () => Navigator.pop(dialogContext, false), child: const Text('Cancel')),
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, true),
            child: const Text('Delete', style: TextStyle(color: AppColors.danger)),
          ),
        ],
      ),
    );
    if (confirmed != true) return;
    if (!mounted) return;
    await context.read<BusinessProfileProvider>().removeAddress(id: address.id, uuid: address.uuid);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/business-profile',
      body: Consumer<BusinessProfileProvider>(
        builder: (context, provider, _) {
          final profile = provider.current;
          if (provider.loading || profile == null) {
            return const Center(child: CircularProgressIndicator());
          }
          return SingleChildScrollView(
            child: Column(
              children: [
                _buildHeader(profile),
                Padding(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      _buildFieldsSection(l10n, profile),
                      const SizedBox(height: 24),
                      _buildAddressesSection(l10n, profile),
                    ],
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildHeader(BusinessProfile profile) {
    return Stack(
      clipBehavior: Clip.none,
      children: [
        SizedBox(
          height: 180,
          width: double.infinity,
          child: profile.coverImage != null
              ? CachedNetworkImage(imageUrl: profile.coverImage!, fit: BoxFit.cover)
              : Container(color: AppColors.professionalSecondaryDisabled),
        ),
        Positioned(
          top: 8,
          right: 8,
          child: IconButton(
            style: IconButton.styleFrom(backgroundColor: Colors.black.withAlpha(102)),
            icon: const Icon(Icons.camera_alt, color: Colors.white),
            onPressed: () => _showImageSourceDialog(isLogo: false),
          ),
        ),
        Positioned(
          left: 16,
          bottom: -32,
          child: Stack(
            children: [
              CircleAvatar(
                radius: 48,
                backgroundColor: Colors.white,
                child: ClipOval(
                  child: SizedBox(
                    width: 88,
                    height: 88,
                    child: profile.logo != null
                        ? CachedNetworkImage(imageUrl: profile.logo!, fit: BoxFit.cover)
                        : const Icon(Icons.storefront, size: 40, color: AppColors.professionalSecondary),
                  ),
                ),
              ),
              Positioned(
                right: 0,
                bottom: 0,
                child: IconButton(
                  style: IconButton.styleFrom(backgroundColor: AppColors.professionalSecondary),
                  icon: const Icon(Icons.camera_alt, color: Colors.white, size: 16),
                  onPressed: () => _showImageSourceDialog(isLogo: true),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildFieldsSection(AppLocalizations l10n, BusinessProfile profile) {
    return Padding(
      padding: const EdgeInsets.only(top: 40),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                profile.businessType == 'Company'
                    ? l10n.addProfileGymTitle
                    : l10n.addProfilePersonalTrainerTitle,
                style: const TextStyle(fontSize: 12, color: Colors.grey, fontWeight: FontWeight.w600),
              ),
              TextButton(
                onPressed: () {
                  if (_isEditing) {
                    _saveChanges();
                  } else {
                    setState(() => _isEditing = true);
                  }
                },
                child: Text(_isEditing ? l10n.businessProfilePageSave : l10n.businessProfilePageEdit),
              ),
            ],
          ),
          const SizedBox(height: 8),
          _buildTextField(l10n.businessProfileFormBusinessName, _businessNameController, enabled: _isEditing),
          const SizedBox(height: 12),
          _buildTextField(l10n.businessProfileFormSocialName, _socialNameController, enabled: _isEditing),
          const SizedBox(height: 12),
          _buildTextField(l10n.businessProfileFormTaxId, _taxIdController, enabled: _isEditing),
        ],
      ),
    );
  }

  Widget _buildTextField(String label, TextEditingController controller, {required bool enabled}) {
    return TextFormField(
      controller: controller,
      enabled: enabled,
      decoration: InputDecoration(
        labelText: label,
        filled: !enabled,
        fillColor: enabled ? null : Colors.grey[50],
        border: const OutlineInputBorder(),
      ),
    );
  }

  Widget _buildAddressesSection(AppLocalizations l10n, BusinessProfile profile) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(l10n.businessProfilePageAddresses, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
            IconButton(
              icon: const Icon(Icons.add_circle_outline, color: AppColors.professionalSecondary),
              onPressed: () => _openAddressForm(null),
            ),
          ],
        ),
        if (_isAddressFormExpanded) _buildAddressForm(),
        ...profile.addresses.map((address) => _buildAddressCard(address)),
      ],
    );
  }

  Widget _buildAddressForm() {
    return Card(
      margin: const EdgeInsets.symmetric(vertical: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          children: [
            TextFormField(
              controller: _addressLine1Controller,
              decoration: const InputDecoration(labelText: 'Address line 1'),
            ),
            const SizedBox(height: 8),
            TextFormField(
              controller: _addressLine2Controller,
              decoration: const InputDecoration(labelText: 'Address line 2'),
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: TextFormField(
                    controller: _localityController,
                    decoration: const InputDecoration(labelText: 'City'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: TextFormField(
                    controller: _administrativeAreaController,
                    decoration: const InputDecoration(labelText: 'State'),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: TextFormField(
                    controller: _postalCodeController,
                    decoration: const InputDecoration(labelText: 'Postal code'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: TextFormField(
                    controller: _countryCodeController,
                    decoration: const InputDecoration(labelText: 'Country code'),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                TextButton(onPressed: _closeAddressForm, child: const Text('Cancel')),
                const SizedBox(width: 8),
                ElevatedButton(
                  onPressed: _saveAddress,
                  style: ElevatedButton.styleFrom(backgroundColor: AppColors.professionalSecondary),
                  child: const Text('Save', style: TextStyle(color: Colors.white)),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildAddressCard(BusinessProfileAddress address) {
    return Card(
      margin: const EdgeInsets.symmetric(vertical: 4),
      child: ListTile(
        title: Text(address.addressLine1),
        subtitle: Text('${address.locality}, ${address.administrativeArea} ${address.postalCode}'),
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            IconButton(
              icon: const Icon(Icons.edit_outlined, size: 20),
              onPressed: () => _openAddressForm(address),
            ),
            IconButton(
              icon: const Icon(Icons.delete_outline, size: 20, color: AppColors.danger),
              onPressed: () => _confirmDeleteAddress(address),
            ),
          ],
        ),
      ),
    );
  }
}
