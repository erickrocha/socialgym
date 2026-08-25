import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';
import 'package:image_picker/image_picker.dart';
import 'package:intl/intl.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/country.dart';
import '../../models/person.dart';
import '../../providers/person_provider.dart';
import '../../providers/resource_provider.dart';
import '../../config/nav_section.dart';
import '../../widgets/main_layout.dart';

class ProfilePage extends StatefulWidget {
  const ProfilePage({super.key});

  @override
  State<ProfilePage> createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  static const _genderValues = {'Male', 'Female', 'Other'};
  static const _relationshipValues = {
    'Single',
    'In a Relationship',
    'Engaged',
    'Married',
    'Complicated',
  };

  final _formKey = GlobalKey<FormState>();
  final _imagePicker = ImagePicker();

  // Personal info controllers
  late TextEditingController _firstNameController;
  late TextEditingController _surnameController;
  late TextEditingController _biographyController;
  late TextEditingController _jobController;
  late TextEditingController _homeTownController;
  late TextEditingController _currentCityController;
  late TextEditingController _weightController;
  late TextEditingController _heightController;

  // Address form controllers
  late TextEditingController _addressLine1Controller;
  late TextEditingController _addressLine2Controller;
  late TextEditingController _addressLocalityController;
  late TextEditingController _addressPostalCodeController;

  // Address form dropdown selections and fields
  Country? _selectedCountry;
  late TextEditingController _addressAdministrativeAreaController;

  String? _selectedGender;
  String? _selectedRelationship;
  DateTime? _selectedDateOfBirth;

  // Original values for dirty tracking (only call API if changed)
  String _originalFirstName = '';
  String _originalSurname = '';
  String? _originalGender;
  DateTime? _originalDateOfBirth;
  String _originalBiography = '';
  String _originalJob = '';
  String? _originalRelationship;
  String _originalHomeTown = '';
  String _originalCurrentCity = '';
  String _originalWeight = '';
  String _originalHeight = '';

  bool _isEditing = false;

  // Address form state
  bool _isAddressFormExpanded = false;
  PersonAddress? _editingAddress;
  Position? _capturedPosition; // Store GPS position when opening form

  @override
  void initState() {
    super.initState();
    _initControllers();
  }

  void _initControllers() {
    final person = context.read<PersonProvider>().person;
    _firstNameController = TextEditingController(text: person?.firstname ?? '');
    _surnameController = TextEditingController(text: person?.surname ?? '');
    _biographyController = TextEditingController(
      text: person?.personInfo?.biography ?? '',
    );
    _jobController = TextEditingController(text: person?.personInfo?.job ?? '');
    _homeTownController = TextEditingController(
      text: person?.personInfo?.homeTown ?? '',
    );
    _currentCityController = TextEditingController(
      text: person?.personInfo?.currentCity ?? '',
    );
    _weightController = TextEditingController(
      text: person?.personInfo?.weight?.toString() ?? '',
    );
    _heightController = TextEditingController(
      text: person?.personInfo?.height?.toString() ?? '',
    );
    _selectedGender = _validDropdownValue(person?.gender, _genderValues);
    _selectedRelationship = _validDropdownValue(
      person?.personInfo?.relationship,
      _relationshipValues,
    );
    _selectedDateOfBirth = person?.dateOfBirth;
    _captureOriginalValues();

    // Initialize address form controllers
    _addressLine1Controller = TextEditingController();
    _addressLine2Controller = TextEditingController();
    _addressLocalityController = TextEditingController();
    _addressPostalCodeController = TextEditingController();
    _addressAdministrativeAreaController = TextEditingController();
  }

  void _refreshControllers() {
    final person = context.read<PersonProvider>().person;
    _firstNameController.text = person?.firstname ?? '';
    _surnameController.text = person?.surname ?? '';
    _biographyController.text = person?.personInfo?.biography ?? '';
    _jobController.text = person?.personInfo?.job ?? '';
    _homeTownController.text = person?.personInfo?.homeTown ?? '';
    _currentCityController.text = person?.personInfo?.currentCity ?? '';
    _weightController.text = person?.personInfo?.weight?.toString() ?? '';
    _heightController.text = person?.personInfo?.height?.toString() ?? '';
    _selectedGender = _validDropdownValue(person?.gender, _genderValues);
    _selectedRelationship = _validDropdownValue(
      person?.personInfo?.relationship,
      _relationshipValues,
    );
    _selectedDateOfBirth = person?.dateOfBirth;
    _captureOriginalValues();
  }

  String? _validDropdownValue(String? value, Set<String> validValues) {
    return validValues.contains(value) ? value : null;
  }

  void _captureOriginalValues() {
    _originalFirstName = _firstNameController.text;
    _originalSurname = _surnameController.text;
    _originalGender = _selectedGender;
    _originalDateOfBirth = _selectedDateOfBirth;
    _originalBiography = _biographyController.text;
    _originalJob = _jobController.text;
    _originalRelationship = _selectedRelationship;
    _originalHomeTown = _homeTownController.text;
    _originalCurrentCity = _currentCityController.text;
    _originalWeight = _weightController.text;
    _originalHeight = _heightController.text;
  }

  @override
  void dispose() {
    _firstNameController.dispose();
    _surnameController.dispose();
    _biographyController.dispose();
    _jobController.dispose();
    _homeTownController.dispose();
    _currentCityController.dispose();
    _weightController.dispose();
    _heightController.dispose();
    _addressLine1Controller.dispose();
    _addressLine2Controller.dispose();
    _addressLocalityController.dispose();
    _addressPostalCodeController.dispose();
    _addressAdministrativeAreaController.dispose();
    super.dispose();
  }

  Future<bool> _requestPermission(ImageSource source) async {
    PermissionStatus status;

    if (source == ImageSource.camera) {
      status = await Permission.camera.request();
    } else {
      // For gallery, request photos permission
      status = await Permission.photos.request();

      // On Android, if photos permission is not available, try storage
      if (status.isPermanentlyDenied || status.isDenied) {
        status = await Permission.storage.request();
      }
    }

    if (status.isGranted || status.isLimited) {
      return true;
    }

    if (status.isPermanentlyDenied && mounted) {
      final l10n = AppLocalizations.of(context)!;
      final shouldOpenSettings = await showDialog<bool>(
        context: context,
        builder: (context) => AlertDialog(
          title: Text(l10n.permissionRequired),
          content: Text(
            source == ImageSource.camera
                ? l10n.cameraPermissionDenied
                : l10n.photosPermissionDenied,
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: Text(l10n.buttonCancel),
            ),
            TextButton(
              onPressed: () => Navigator.pop(context, true),
              child: Text(l10n.openSettings),
            ),
          ],
        ),
      );

      if (shouldOpenSettings == true) {
        await openAppSettings();
      }
    }

    return false;
  }

  Future<void> _pickImage(ImageSource source, {required bool isAvatar}) async {
    // Request permission first
    final hasPermission = await _requestPermission(source);
    if (!hasPermission) return;

    try {
      final pickedFile = await _imagePicker.pickImage(
        source: source,
        maxWidth: isAvatar ? 512 : 1920,
        maxHeight: isAvatar ? 512 : 1080,
        imageQuality: 85,
      );

      if (pickedFile != null && mounted) {
        final personProvider = context.read<PersonProvider>();

        bool success;
        if (isAvatar) {
          success = await personProvider.uploadAvatar(pickedFile);
        } else {
          success = await personProvider.uploadCover(pickedFile);
        }

        if (mounted) {
          final l10n = AppLocalizations.of(context)!;
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(
                success
                    ? l10n.profileImageUploadSuccess
                    : l10n.profileImageUploadError,
              ),
              backgroundColor: success ? AppColors.success : AppColors.danger,
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        final l10n = AppLocalizations.of(context)!;
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(l10n.profileImageUploadError),
            backgroundColor: AppColors.danger,
          ),
        );
      }
    }
  }

  void _showImageSourceDialog({required bool isAvatar}) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = context.read<PersonProvider>().activeBusinessProfile?.businessType;

    showModalBottomSheet(
      context: context,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (context) => SafeArea(
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 16),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                l10n.profileSelectImage,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 16),
              ListTile(
                leading: Icon(
                  Icons.camera_alt,
                  color: AppColors.primaryFor(businessType),
                ),
                title: Text(l10n.profileTakePhoto),
                onTap: () {
                  Navigator.pop(context);
                  _pickImage(ImageSource.camera, isAvatar: isAvatar);
                },
              ),
              ListTile(
                leading: Icon(
                  Icons.photo_library,
                  color: AppColors.primaryFor(businessType),
                ),
                title: Text(l10n.profileChooseFromGallery),
                onTap: () {
                  Navigator.pop(context);
                  _pickImage(ImageSource.gallery, isAvatar: isAvatar);
                },
              ),
              Consumer<PersonProvider>(
                builder: (context, personProvider, _) {
                  final hasImage = isAvatar
                      ? personProvider.person?.avatar != null
                      : personProvider.person?.cover != null;

                  if (!hasImage) return const SizedBox.shrink();

                  return ListTile(
                    leading: const Icon(Icons.delete, color: AppColors.danger),
                    title: Text(
                      l10n.profileRemovePhoto,
                      style: const TextStyle(color: AppColors.danger),
                    ),
                    onTap: () async {
                      Navigator.pop(context);
                      final scaffoldMessenger = ScaffoldMessenger.of(context);
                      bool success;
                      if (isAvatar) {
                        success = await personProvider.removeAvatar();
                      } else {
                        success = await personProvider.removeCover();
                      }
                      if (mounted) {
                        scaffoldMessenger.showSnackBar(
                          SnackBar(
                            content: Text(
                              success
                                  ? l10n.profileImageUploadSuccess
                                  : l10n.profileImageUploadError,
                            ),
                            backgroundColor: success
                                ? AppColors.success
                                : AppColors.danger,
                          ),
                        );
                      }
                    },
                  );
                },
              ),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _selectDateOfBirth() async {
    final l10n = AppLocalizations.of(context)!;
    final now = DateTime.now();
    final initialDate = _selectedDateOfBirth ?? DateTime(now.year - 25);

    final pickedDate = await showDatePicker(
      context: context,
      initialDate: initialDate,
      firstDate: DateTime(1900),
      lastDate: now,
      helpText: l10n.profileDateOfBirth,
    );

    if (pickedDate != null) {
      setState(() {
        _selectedDateOfBirth = pickedDate;
      });
    }
  }

  Future<void> _saveChanges() async {
    if (!_formKey.currentState!.validate()) return;

    final l10n = AppLocalizations.of(context)!;
    final personProvider = context.read<PersonProvider>();
    final person = personProvider.person;

    if (person == null) return;

    // Determine what actually changed
    final personChanged =
        _firstNameController.text.trim() != _originalFirstName ||
        _surnameController.text.trim() != _originalSurname ||
        _selectedGender != _originalGender ||
        _selectedDateOfBirth != _originalDateOfBirth;

    final personInfoChanged =
        _biographyController.text.trim() != _originalBiography ||
        _jobController.text.trim() != _originalJob ||
        _selectedRelationship != _originalRelationship ||
        _homeTownController.text.trim() != _originalHomeTown ||
        _currentCityController.text.trim() != _originalCurrentCity ||
        _weightController.text.trim() != _originalWeight ||
        _heightController.text.trim() != _originalHeight;

    // Nothing changed — just exit edit mode
    if (!personChanged && !personInfoChanged) {
      setState(() => _isEditing = false);
      return;
    }

    bool personSuccess = true;
    bool infoSuccess = true;
    String? updateError;

    // Only call updatePerson if person fields changed
    if (personChanged) {
      final personData = {
        'firstname': _firstNameController.text.trim(),
        'surname': _surnameController.text.trim(),
        'gender': _selectedGender,
        'dateOfBirth': _selectedDateOfBirth?.toIso8601String().split('T').first,
      };
      personSuccess = await personProvider.updatePerson(personData);
      if (!personSuccess) updateError = personProvider.error;
    }

    // Only call updatePersonInfo if personInfo fields changed and personInfo exists
    if (personInfoChanged && person.personInfo != null) {
      final personInfoData = {
        'id': person.personInfo!.id,
        'personId': person.id,
        'biography': _biographyController.text.trim().isEmpty
            ? null
            : _biographyController.text.trim(),
        'job': _jobController.text.trim().isEmpty
            ? null
            : _jobController.text.trim(),
        'relationship': _selectedRelationship,
        'homeTown': _homeTownController.text.trim().isEmpty
            ? null
            : _homeTownController.text.trim(),
        'currentCity': _currentCityController.text.trim().isEmpty
            ? null
            : _currentCityController.text.trim(),
        'weight': _weightController.text.trim().isEmpty
            ? null
            : double.tryParse(_weightController.text.trim()),
        'height': _heightController.text.trim().isEmpty
            ? null
            : double.tryParse(_heightController.text.trim()),
      };

      infoSuccess = await personProvider.updatePersonInfo(personInfoData);
      if (!infoSuccess) updateError = personProvider.error;
    }

    if (mounted) {
      final success = personSuccess && infoSuccess;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            success
                ? l10n.profileUpdateSuccess
                : (updateError?.trim().isNotEmpty == true
                      ? updateError!
                      : l10n.profileUpdateError),
          ),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );

      if (success) {
        setState(() {
          _isEditing = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/profile',
      body: Consumer<PersonProvider>(
        builder: (context, personProvider, _) {
          if (personProvider.loading) {
            return const Center(child: CircularProgressIndicator());
          }

          final person = personProvider.person;
          if (person == null) {
            return Center(child: Text(l10n.profileUpdateError));
          }

          final businessType = personProvider.activeBusinessProfile?.businessType;

          return Stack(
            children: [
              SingleChildScrollView(
                child: Column(
                  children: [
                    _buildHeader(person, l10n, businessType),
                    _buildProfileContent(person, l10n, businessType),
                  ],
                ),
              ),
              if (personProvider.updating)
                Container(
                  color: Colors.black26,
                  child: const Center(child: CircularProgressIndicator()),
                ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildHeader(Person person, AppLocalizations l10n, String? businessType) {
    final screenWidth = MediaQuery.of(context).size.width;
    final coverHeight = screenWidth > 600 ? 250.0 : 180.0;
    final avatarSize = screenWidth > 600 ? 140.0 : 100.0;

    return SizedBox(
      height: coverHeight + (avatarSize / 2),
      child: Stack(
        clipBehavior: Clip.none,
        children: [
          // Cover photo
          Positioned(
            top: 0,
            left: 0,
            right: 0,
            height: coverHeight,
            child: Stack(
              fit: StackFit.expand,
              children: [
                // Default cover image shown when no cover URL or on error
                Image.asset('assets/images/cover_foto.png', fit: BoxFit.cover),
                if (person.cover != null)
                  CachedNetworkImage(
                    imageUrl: person.cover!,
                    fit: BoxFit.cover,
                    placeholder: (context, url) => Image.asset(
                      'assets/images/cover_foto.png',
                      fit: BoxFit.cover,
                    ),
                    errorWidget: (context, url, error) => Image.asset(
                      'assets/images/cover_foto.png',
                      fit: BoxFit.cover,
                    ),
                  ),
              ],
            ),
          ),

          // Edit cover button - top right
          Positioned(
            top: 8,
            right: 8,
            child: Container(
              decoration: BoxDecoration(
                color: Colors.black.withAlpha(128),
                shape: BoxShape.circle,
              ),
              child: IconButton(
                onPressed: () => _showImageSourceDialog(isAvatar: false),
                icon: const Icon(
                  Icons.camera_alt,
                  color: Colors.white,
                  size: 20,
                ),
                tooltip: l10n.profileChangeCover,
                constraints: const BoxConstraints(minWidth: 40, minHeight: 40),
                padding: EdgeInsets.zero,
              ),
            ),
          ),

          // Avatar with edit button
          Positioned(
            bottom: 0,
            left: 16,
            child: SizedBox(
              width: avatarSize + 8,
              height: avatarSize + 8,
              child: Stack(
                clipBehavior: Clip.none,
                children: [
                  Container(
                    width: avatarSize,
                    height: avatarSize,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(color: Colors.white, width: 4),
                      boxShadow: [
                        BoxShadow(
                          color: Colors.black.withAlpha(51),
                          blurRadius: 8,
                          offset: const Offset(0, 2),
                        ),
                      ],
                    ),
                    child: ClipOval(
                      child: person.avatar != null
                          ? CachedNetworkImage(
                              imageUrl: person.avatar!,
                              fit: BoxFit.cover,
                              placeholder: (context, url) => Container(
                                color: AppColors.primaryFor(businessType),
                                child: const Center(
                                  child: CircularProgressIndicator(
                                    color: Colors.white,
                                  ),
                                ),
                              ),
                              errorWidget: (context, url, error) => Image.asset(
                                person.gender?.toLowerCase() == 'female'
                                    ? 'assets/images/avatar_female.png'
                                    : 'assets/images/avatar_male.png',
                                fit: BoxFit.cover,
                              ),
                            )
                          : Image.asset(
                              person.gender?.toLowerCase() == 'female'
                                  ? 'assets/images/avatar_female.png'
                                  : 'assets/images/avatar_male.png',
                              fit: BoxFit.cover,
                            ),
                    ),
                  ),
                  // Edit avatar button - bottom right of avatar
                  Positioned(
                    bottom: 4,
                    right: 0,
                    child: GestureDetector(
                      behavior: HitTestBehavior.opaque,
                      onTap: () {
                        _showImageSourceDialog(isAvatar: true);
                      },
                      child: Container(
                        width: 36,
                        height: 36,
                        decoration: BoxDecoration(
                          color: AppColors.primaryFor(businessType),
                          shape: BoxShape.circle,
                          border: Border.all(color: Colors.white, width: 2),
                        ),
                        child: const Icon(
                          Icons.camera_alt,
                          color: Colors.white,
                          size: 18,
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildProfileContent(Person person, AppLocalizations l10n, String? businessType) {
    return Padding(
      padding: const EdgeInsets.only(top: 16, left: 16, right: 16, bottom: 80),
      child: Form(
        key: _formKey,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Name and member since
            if (!_isEditing) ...[
              Text(
                person.fullName,
                style: const TextStyle(
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                '${l10n.profileMemberSince} ${person.createdAt?.toIso8601String() ?? ''}',
                style: TextStyle(color: Colors.grey[600], fontSize: 14),
              ),
              const SizedBox(height: 24),
            ],

            // Personal Information Section
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Flexible(
                  child: _buildSectionHeader(
                    l10n.profilePersonalInfo,
                    Icons.person_outline,
                    businessType,
                  ),
                ),
                if (!_isEditing)
                  IconButton(
                    onPressed: () {
                      _refreshControllers();
                      setState(() {
                        _isEditing = true;
                      });
                    },
                    icon: const Icon(Icons.edit, size: 20),
                    color: AppColors.primaryFor(businessType),
                    tooltip: l10n.profileEditProfile,
                  )
                else
                  Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      IconButton(
                        onPressed: () {
                          _refreshControllers();
                          setState(() {
                            _isEditing = false;
                          });
                        },
                        icon: const Icon(Icons.close, size: 20),
                        color: Colors.grey,
                        tooltip: l10n.buttonCancel,
                      ),
                      IconButton(
                        onPressed: _saveChanges,
                        icon: const Icon(Icons.save, size: 20),
                        color: AppColors.primaryFor(businessType),
                        tooltip: l10n.profileSaveChanges,
                      ),
                    ],
                  ),
              ],
            ),
            const SizedBox(height: 16),

            if (_isEditing) ...[
              _buildTextField(
                controller: _firstNameController,
                label: l10n.profileFirstName,
                icon: Icons.person_outline,
                businessType: businessType,
                validator: (value) {
                  if (value == null || value.trim().isEmpty) {
                    return l10n.validationFirstNameRequired;
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              _buildTextField(
                controller: _surnameController,
                label: l10n.profileSurname,
                icon: Icons.person_outline,
                businessType: businessType,
                validator: (value) {
                  if (value == null || value.trim().isEmpty) {
                    return l10n.validationSurnameRequired;
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              _buildDateOfBirthField(l10n),
              const SizedBox(height: 16),
              _buildGenderDropdown(l10n),
              const SizedBox(height: 16),
            ],

            _buildTextField(
              controller: _biographyController,
              label: l10n.profileBiography,
              hint: l10n.profileBiographyHint,
              icon: Icons.description_outlined,
              maxLines: 3,
              enabled: _isEditing,
              businessType: businessType,
            ),
            const SizedBox(height: 16),

            _buildTextField(
              controller: _jobController,
              label: l10n.profileJob,
              hint: l10n.profileJobHint,
              icon: Icons.work_outline,
              enabled: _isEditing,
              businessType: businessType,
            ),
            const SizedBox(height: 16),

            if (_isEditing)
              _buildRelationshipDropdown(l10n)
            else if (person.personInfo?.relationship != null)
              _buildInfoRow(
                Icons.favorite_outline,
                l10n.profileRelationship,
                _getRelationshipLabel(person.personInfo!.relationship!, l10n),
                businessType,
              ),

            const SizedBox(height: 16),

            _buildTextField(
              controller: _homeTownController,
              label: l10n.profileHomeTown,
              icon: Icons.home_outlined,
              enabled: _isEditing,
              businessType: businessType,
            ),
            const SizedBox(height: 16),

            _buildTextField(
              controller: _currentCityController,
              label: l10n.profileCurrentCity,
              icon: Icons.location_city_outlined,
              enabled: _isEditing,
              businessType: businessType,
            ),

            const SizedBox(height: 32),

            // Physical Stats Section
            _buildSectionHeader(
              l10n.profilePhysicalStats,
              Icons.fitness_center_outlined,
              businessType,
            ),
            const SizedBox(height: 16),

            Row(
              children: [
                Expanded(
                  child: _buildTextField(
                    controller: _weightController,
                    label: l10n.profileWeight,
                    icon: Icons.monitor_weight_outlined,
                    keyboardType: TextInputType.number,
                    enabled: _isEditing,
                    businessType: businessType,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildTextField(
                    controller: _heightController,
                    label: l10n.profileHeight,
                    icon: Icons.height,
                    keyboardType: TextInputType.number,
                    enabled: _isEditing,
                    businessType: businessType,
                  ),
                ),
              ],
            ),

            const SizedBox(height: 32),

            // Addresses Section
            _buildAddressesSection(person, l10n, businessType),
          ],
        ),
      ),
    );
  }

  Widget _buildAddressesSection(Person person, AppLocalizations l10n, String? businessType) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Flexible(
              child: _buildSectionHeader(
                l10n.addressSection,
                Icons.location_on_outlined,
                businessType,
              ),
            ),
            if (!_isAddressFormExpanded)
              IconButton(
                onPressed: () => _openAddressForm(null),
                icon: const Icon(Icons.add_circle_outline),
                color: AppColors.primaryFor(businessType),
                tooltip: l10n.addressAdd,
              ),
          ],
        ),
        const SizedBox(height: 16),

        // Expandable Address Form
        AnimatedCrossFade(
          duration: const Duration(milliseconds: 300),
          crossFadeState: _isAddressFormExpanded
              ? CrossFadeState.showFirst
              : CrossFadeState.showSecond,
          firstChild: _buildAddressForm(l10n, businessType),
          secondChild: const SizedBox.shrink(),
        ),

        if (person.addresses.isEmpty && !_isAddressFormExpanded)
          Container(
            padding: const EdgeInsets.all(24),
            decoration: BoxDecoration(
              color: Colors.grey[100],
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: Colors.grey[300]!),
            ),
            child: Column(
              children: [
                Icon(
                  Icons.location_off_outlined,
                  size: 48,
                  color: Colors.grey[400],
                ),
                const SizedBox(height: 12),
                Text(
                  l10n.addressNoAddresses,
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w500,
                    color: Colors.grey[600],
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  l10n.addressNoAddressesHint,
                  style: TextStyle(fontSize: 14, color: Colors.grey[500]),
                ),
                const SizedBox(height: 16),
                ElevatedButton.icon(
                  onPressed: () => _openAddressForm(null),
                  icon: const Icon(Icons.add),
                  label: Text(l10n.addressAdd),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.primaryFor(businessType),
                    foregroundColor: Colors.white,
                  ),
                ),
              ],
            ),
          )
        else if (!_isAddressFormExpanded)
          ...(person.addresses
                ..sort((a, b) => b.current ? -1 : (a.current ? 1 : 0)))
              .map((address) => _buildAddressCard(address, l10n, businessType)),
      ],
    );
  }

  void _openAddressForm(PersonAddress? address) async {
    final resourceProvider = context.read<ResourceProvider>();

    setState(() {
      _editingAddress = address;
      _isAddressFormExpanded = true;

      // Populate form with address data or clear for new address
      _addressLine1Controller.text = address?.addressLine1 ?? '';
      _addressLine2Controller.text = address?.addressLine2 ?? '';
      _addressLocalityController.text = address?.locality ?? '';
      _addressAdministrativeAreaController.text =
          address?.administrativeArea ?? '';
      _addressPostalCodeController.text = address?.postalCode ?? '';

      // Set country from address
      _selectedCountry = resourceProvider.getCountryByCode(
        address?.countryCode,
      );
    });

    // If adding a new address (not editing), capture GPS location immediately
    if (address == null) {
      final l10n = AppLocalizations.of(context)!;
      _capturedPosition = await _getCurrentLocation(l10n);
    }
  }

  void _closeAddressForm() {
    setState(() {
      _isAddressFormExpanded = false;
      _editingAddress = null;
      _selectedCountry = null;
      _capturedPosition = null; // Clear captured GPS position

      // Clear form
      _addressLine1Controller.clear();
      _addressLine2Controller.clear();
      _addressLocalityController.clear();
      _addressAdministrativeAreaController.clear();
      _addressPostalCodeController.clear();
    });
  }

  Widget _buildAddressForm(AppLocalizations l10n, String? businessType) {
    final isEditing = _editingAddress != null;

    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: AppColors.primaryFor(businessType).withAlpha(100)),
        boxShadow: [
          BoxShadow(
            color: AppColors.primaryFor(businessType).withAlpha(20),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              Icon(
                isEditing ? Icons.edit_location_alt : Icons.add_location_alt,
                color: AppColors.primaryFor(businessType),
              ),
              const SizedBox(width: 8),
              Text(
                isEditing ? l10n.addressEdit : l10n.addressAdd,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const Spacer(),
              IconButton(
                onPressed: _closeAddressForm,
                icon: const Icon(Icons.close),
                color: Colors.grey[600],
              ),
            ],
          ),
          const Divider(),
          const SizedBox(height: 12),

          // Address Line 1 (Street and Number)
          TextField(
            controller: _addressLine1Controller,
            decoration: InputDecoration(
              labelText: l10n.addressLine1,
              hintText: 'e.g., 123 Main Street',
              prefixIcon: const Icon(Icons.route_outlined),
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(12),
              ),
            ),
          ),
          const SizedBox(height: 12),

          // Address Line 2 (Apartment, Suite, etc.)
          TextField(
            controller: _addressLine2Controller,
            decoration: InputDecoration(
              labelText: l10n.addressLine2,
              hintText: 'e.g., Apt 4B, Suite 100',
              prefixIcon: const Icon(Icons.home_outlined),
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(12),
              ),
            ),
          ),
          const SizedBox(height: 12),

          // Locality (City) and Administrative Area (State/Province)
          Row(
            children: [
              Expanded(
                flex: 2,
                child: TextField(
                  controller: _addressLocalityController,
                  decoration: InputDecoration(
                    labelText: l10n.addressLocality,
                    prefixIcon: const Icon(Icons.location_city_outlined),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                flex: 2,
                child: TextField(
                  controller: _addressAdministrativeAreaController,
                  decoration: InputDecoration(
                    labelText: l10n.addressAdministrativeArea,
                    hintText: 'e.g., State, Province',
                    prefixIcon: const Icon(Icons.map_outlined),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),

          // Country and Postal Code
          Row(
            children: [
              Expanded(
                child: Consumer<ResourceProvider>(
                  builder: (context, resourceProvider, _) {
                    return DropdownButtonFormField<Country>(
                      initialValue: _selectedCountry,
                      decoration: InputDecoration(
                        labelText: l10n.addressCountry,
                        prefixIcon: const Icon(Icons.flag_outlined),
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                        contentPadding: const EdgeInsets.symmetric(
                          horizontal: 12,
                          vertical: 16,
                        ),
                      ),
                      isExpanded: true,
                      items: resourceProvider.countries.map((country) {
                        return DropdownMenuItem(
                          value: country,
                          child: Text(
                            country.name,
                            overflow: TextOverflow.ellipsis,
                          ),
                        );
                      }).toList(),
                      onChanged: (value) {
                        setState(() {
                          _selectedCountry = value;
                        });
                      },
                      hint: Text(
                        l10n.addressSelectCountry,
                        style: TextStyle(color: Colors.grey[600]),
                      ),
                    );
                  },
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: TextField(
                  controller: _addressPostalCodeController,
                  decoration: InputDecoration(
                    labelText: l10n.addressPostalCode,
                    prefixIcon: const Icon(Icons.pin_outlined),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 20),

          // Action buttons
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              TextButton(
                onPressed: _closeAddressForm,
                child: Text(l10n.buttonCancel),
              ),
              const SizedBox(width: 12),
              ElevatedButton.icon(
                onPressed: () => _saveAddress(l10n),
                icon: const Icon(Icons.save),
                label: Text(l10n.buttonSave),
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primaryFor(businessType),
                  foregroundColor: Colors.white,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Future<void> _saveAddress(AppLocalizations l10n) async {
    // Use the GPS position captured when opening the form (for new addresses)
    // or keep existing coordinates when editing
    final Map<String, dynamic> data = {
      'addressLine1': _addressLine1Controller.text.trim(),
      'addressLine2': _addressLine2Controller.text.trim(),
      'locality': _addressLocalityController.text.trim(),
      'administrativeArea': _addressAdministrativeAreaController.text.trim(),
      'countryCode': _selectedCountry?.acronym ?? '',
      'postalCode': _addressPostalCodeController.text.trim(),
      'latitude': _capturedPosition?.latitude ?? _editingAddress?.latitude,
      'longitude': _capturedPosition?.longitude ?? _editingAddress?.longitude,
    };

    final personProvider = context.read<PersonProvider>();
    bool success;

    if (_editingAddress != null) {
      success = await personProvider.updateAddress(_editingAddress!.id!, data);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              success ? l10n.addressUpdateSuccess : l10n.addressActionError,
            ),
            backgroundColor: success ? AppColors.success : AppColors.danger,
          ),
        );
      }
    } else {
      success = await personProvider.addAddress(data);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              success ? l10n.addressAddSuccess : l10n.addressActionError,
            ),
            backgroundColor: success ? AppColors.success : AppColors.danger,
          ),
        );
      }
    }

    if (success) {
      _closeAddressForm();
    }
  }

  Widget _buildAddressCard(PersonAddress address, AppLocalizations l10n, String? businessType) {
    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: address.current
            ? AppColors.primaryFor(businessType).withAlpha(20)
            : Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: address.current ? AppColors.primaryFor(businessType) : Colors.grey[300]!,
          width: address.current ? 2 : 1,
        ),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(13),
            blurRadius: 4,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(
                Icons.location_on,
                color: address.current ? AppColors.primaryFor(businessType) : Colors.grey[600],
                size: 20,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  address.formattedAddress,
                  style: const TextStyle(
                    fontSize: 14,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ),
              if (address.current)
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 4,
                  ),
                  decoration: BoxDecoration(
                    color: AppColors.primaryFor(businessType),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    l10n.addressCurrent,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
            ],
          ),
          const SizedBox(height: 12),
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              IconButton(
                onPressed: () => _openAddressForm(address),
                icon: const Icon(Icons.edit_outlined, size: 20),
                color: Colors.grey[600],
                tooltip: l10n.addressEdit,
              ),
              IconButton(
                onPressed: () => _confirmDeleteAddress(address, l10n),
                icon: const Icon(Icons.delete_outline, size: 20),
                color: AppColors.danger,
                tooltip: l10n.addressDelete,
              ),
            ],
          ),
        ],
      ),
    );
  }

  Future<Position?> _getCurrentLocation(AppLocalizations l10n) async {
    try {
      // Check if location services are enabled
      bool serviceEnabled = await Geolocator.isLocationServiceEnabled();
      if (!serviceEnabled) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(l10n.addressLocationServicesDisabled),
              backgroundColor: AppColors.danger,
            ),
          );
        }
        return null;
      }

      // Check location permission
      LocationPermission permission = await Geolocator.checkPermission();
      if (permission == LocationPermission.denied) {
        permission = await Geolocator.requestPermission();
        if (permission == LocationPermission.denied) {
          if (mounted) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(l10n.addressLocationPermissionDenied),
                backgroundColor: AppColors.danger,
              ),
            );
          }
          return null;
        }
      }

      if (permission == LocationPermission.deniedForever) {
        if (mounted) {
          final shouldOpenSettings = await showDialog<bool>(
            context: context,
            builder: (context) => AlertDialog(
              title: Text(l10n.permissionRequired),
              content: Text(l10n.addressLocationPermissionDeniedForever),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context, false),
                  child: Text(l10n.buttonCancel),
                ),
                TextButton(
                  onPressed: () => Navigator.pop(context, true),
                  child: Text(l10n.openSettings),
                ),
              ],
            ),
          );

          if (shouldOpenSettings == true) {
            await Geolocator.openAppSettings();
          }
        }
        return null;
      }

      // Get current position
      final position = await Geolocator.getCurrentPosition(
        locationSettings: const LocationSettings(
          accuracy: LocationAccuracy.high,
          timeLimit: Duration(seconds: 10),
        ),
      );

      return position;
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(l10n.addressLocationError),
            backgroundColor: AppColors.danger,
          ),
        );
      }
      return null;
    }
  }

  void _confirmDeleteAddress(PersonAddress address, AppLocalizations l10n) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.addressDeleteConfirmTitle),
        content: Text(l10n.addressDeleteConfirm),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text(l10n.buttonCancel),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.pop(context);
              _deleteAddress(address, l10n);
            },
            style: ElevatedButton.styleFrom(
              backgroundColor: AppColors.danger,
              foregroundColor: Colors.white,
            ),
            child: Text(l10n.addressDelete),
          ),
        ],
      ),
    );
  }

  Future<void> _deleteAddress(
    PersonAddress address,
    AppLocalizations l10n,
  ) async {
    final personProvider = context.read<PersonProvider>();

    final success = await personProvider.deleteAddress(address.id!);

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            success ? l10n.addressDeleteSuccess : l10n.addressActionError,
          ),
          backgroundColor: success ? AppColors.success : AppColors.danger,
        ),
      );
    }
  }

  Widget _buildSectionHeader(String title, IconData icon, String? businessType) {
    return Row(
      children: [
        Icon(icon, color: AppColors.primaryFor(businessType)),
        const SizedBox(width: 8),
        Text(
          title,
          style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
        ),
      ],
    );
  }

  Widget _buildTextField({
    required TextEditingController controller,
    required String label,
    String? hint,
    IconData? icon,
    int maxLines = 1,
    TextInputType? keyboardType,
    bool enabled = true,
    String? Function(String?)? validator,
    String? businessType,
  }) {
    return TextFormField(
      controller: controller,
      maxLines: maxLines,
      keyboardType: keyboardType,
      enabled: enabled,
      validator: validator,
      decoration: InputDecoration(
        labelText: label,
        hintText: hint,
        prefixIcon: icon != null ? Icon(icon) : null,
        border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: Colors.grey[300]!),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: AppColors.primaryFor(businessType), width: 2),
        ),
        disabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: Colors.grey[200]!),
        ),
        filled: !enabled,
        fillColor: enabled ? null : Colors.grey[50],
      ),
    );
  }

  Widget _buildDateOfBirthField(AppLocalizations l10n) {
    return GestureDetector(
      onTap: _selectDateOfBirth,
      child: AbsorbPointer(
        child: TextFormField(
          decoration: InputDecoration(
            labelText: l10n.profileDateOfBirth,
            prefixIcon: const Icon(Icons.calendar_today_outlined),
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
            enabledBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(12),
              borderSide: BorderSide(color: Colors.grey[300]!),
            ),
          ),
          controller: TextEditingController(
            text: _selectedDateOfBirth != null
                ? DateFormat.yMMMd().format(_selectedDateOfBirth!)
                : '',
          ),
        ),
      ),
    );
  }

  Widget _buildGenderDropdown(AppLocalizations l10n) {
    final genderOptions = [
      ('Male', l10n.profileGenderMale),
      ('Female', l10n.profileGenderFemale),
      ('Other', l10n.profileGenderOther),
    ];

    return DropdownButtonFormField<String>(
      initialValue: _selectedGender,
      decoration: InputDecoration(
        labelText: l10n.profileGender,
        prefixIcon: const Icon(Icons.people_outline),
        border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: Colors.grey[300]!),
        ),
      ),
      items: genderOptions.map((option) {
        return DropdownMenuItem(value: option.$1, child: Text(option.$2));
      }).toList(),
      onChanged: (value) {
        setState(() {
          _selectedGender = value;
        });
      },
    );
  }

  Widget _buildRelationshipDropdown(AppLocalizations l10n) {
    final relationshipOptions = [
      ('Single', l10n.profileRelationshipSingle),
      ('In a Relationship', l10n.profileRelationshipInRelationship),
      ('Engaged', l10n.profileRelationshipEngaged),
      ('Married', l10n.profileRelationshipMarried),
      ('Complicated', l10n.profileRelationshipComplicated),
    ];

    return DropdownButtonFormField<String>(
      initialValue: _selectedRelationship,
      decoration: InputDecoration(
        labelText: l10n.profileRelationship,
        prefixIcon: const Icon(Icons.favorite_outline),
        border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: Colors.grey[300]!),
        ),
      ),
      items: relationshipOptions.map((option) {
        return DropdownMenuItem(value: option.$1, child: Text(option.$2));
      }).toList(),
      onChanged: (value) {
        setState(() {
          _selectedRelationship = value;
        });
      },
    );
  }

  Widget _buildInfoRow(IconData icon, String label, String value, String? businessType) {
    return Row(
      children: [
        Icon(icon, color: AppColors.primaryFor(businessType), size: 20),
        const SizedBox(width: 8),
        Text(
          '$label: ',
          style: TextStyle(color: Colors.grey[600], fontSize: 14),
        ),
        Text(
          value,
          style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500),
        ),
      ],
    );
  }

  String _getRelationshipLabel(String value, AppLocalizations l10n) {
    switch (value) {
      case 'Single':
        return l10n.profileRelationshipSingle;
      case 'In a Relationship':
        return l10n.profileRelationshipInRelationship;
      case 'Engaged':
        return l10n.profileRelationshipEngaged;
      case 'Married':
        return l10n.profileRelationshipMarried;
      case 'Complicated':
        return l10n.profileRelationshipComplicated;
      default:
        return value;
    }
  }
}
