import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../providers/auth_provider.dart';
import '../../services/person_service.dart';
import '../../services/base_service.dart';

/// Read-only profile view for viewing a friend's profile.
class PersonProfilePage extends StatefulWidget {
  final Person person;

  const PersonProfilePage({super.key, required this.person});

  @override
  State<PersonProfilePage> createState() => _PersonProfilePageState();
}

class _PersonProfilePageState extends State<PersonProfilePage> {
  late Person _person;
  bool _loading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _person = widget.person;
    _loadFullProfile();
  }

  Future<void> _loadFullProfile() async {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    if (token.isEmpty) return;

    setState(() {
      _loading = true;
      _error = null;
    });

    try {
      final full = await PersonService.fetchPersonByUuid(_person.uuid);
      if (mounted) {
        setState(() {
          _person = full;
          _loading = false;
        });
      }
    } on AppException catch (e) {
      if (mounted) {
        setState(() {
          _error = e.message;
          _loading = false;
        });
      }
    } catch (_) {
      if (mounted) {
        setState(() {
          _loading = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final screenWidth = MediaQuery.of(context).size.width;
    final coverHeight = screenWidth > 600 ? 250.0 : 180.0;
    final avatarSize = screenWidth > 600 ? 140.0 : 100.0;

    return Scaffold(
      backgroundColor: AppColors.background,
      body: Stack(
        children: [
          CustomScrollView(
            slivers: [
              // Collapsing app bar with cover photo
              SliverAppBar(
                expandedHeight: coverHeight,
                pinned: true,
                backgroundColor: AppColors.primary,
                leading: IconButton(
                  icon: const Icon(Icons.arrow_back, color: Colors.white),
                  onPressed: () => Navigator.of(context).pop(),
                ),
                flexibleSpace: FlexibleSpaceBar(
                  background: Stack(
                    fit: StackFit.expand,
                    children: [
                      // Default cover
                      Image.asset('assets/images/cover_foto.png', fit: BoxFit.cover),
                      if (_person.cover != null)
                        CachedNetworkImage(
                          imageUrl: _person.cover!,
                          fit: BoxFit.cover,
                          placeholder: (context, url) =>
                              Image.asset('assets/images/cover_foto.png', fit: BoxFit.cover),
                          errorWidget: (context, url, error) =>
                              Image.asset('assets/images/cover_foto.png', fit: BoxFit.cover),
                        ),
                      // Gradient overlay for readability
                      Container(
                        decoration: BoxDecoration(
                          gradient: LinearGradient(
                            begin: Alignment.topCenter,
                            end: Alignment.bottomCenter,
                            colors: [Colors.transparent, Colors.black.withAlpha(100)],
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              SliverToBoxAdapter(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Avatar + name header row
                    Padding(
                      padding: const EdgeInsets.only(left: 16, right: 16, top: 40),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.end,
                        children: [
                          // Avatar
                          Transform.translate(
                            offset: const Offset(0, -30),
                            child: Container(
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
                                child: _person.avatar != null
                                    ? CachedNetworkImage(
                                        imageUrl: _person.avatar!,
                                        fit: BoxFit.cover,
                                        placeholder: (context, url) => Container(
                                          color: AppColors.primary,
                                          child: const Center(
                                            child: CircularProgressIndicator(color: Colors.white),
                                          ),
                                        ),
                                        errorWidget: (context, url, error) => Image.asset(
                                          _person.gender?.toLowerCase() == 'female'
                                              ? 'assets/images/avatar_female.png'
                                              : 'assets/images/avatar_male.png',
                                          fit: BoxFit.cover,
                                        ),
                                      )
                                    : Image.asset(
                                        _person.gender?.toLowerCase() == 'female'
                                            ? 'assets/images/avatar_female.png'
                                            : 'assets/images/avatar_male.png',
                                        fit: BoxFit.cover,
                                      ),
                              ),
                            ),
                          ),
                          const SizedBox(width: 12),
                          // Name + member since
                          Expanded(
                            child: Padding(
                              padding: const EdgeInsets.only(bottom: 8),
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text(
                                    _person.fullName,
                                    style: const TextStyle(
                                      fontSize: 22,
                                      fontWeight: FontWeight.bold,
                                    ),
                                    overflow: TextOverflow.ellipsis,
                                  ),
                                  const SizedBox(height: 2),
                                  Text(
                                    '${l10n.profileMemberSince} ${_person.createdAt?.toIso8601String() ?? ''}',
                                    style: TextStyle(color: Colors.grey[600], fontSize: 13),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),

                    if (_error != null)
                      Container(
                        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                        padding: const EdgeInsets.all(12),
                        decoration: BoxDecoration(
                          color: AppColors.danger.withAlpha(20),
                          borderRadius: BorderRadius.circular(8),
                          border: Border.all(color: AppColors.danger.withAlpha(80)),
                        ),
                        child: Row(
                          children: [
                            const Icon(
                              Icons.warning_amber_rounded,
                              color: AppColors.danger,
                              size: 18,
                            ),
                            const SizedBox(width: 8),
                            Expanded(
                              child: Text(
                                l10n.profileFriendLoadError,
                                style: const TextStyle(color: AppColors.danger, fontSize: 13),
                              ),
                            ),
                          ],
                        ),
                      ),

                    // Content sections
                    Padding(
                      padding: const EdgeInsets.only(left: 16, right: 16, bottom: 40),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          // Personal Information
                          _buildSectionHeader(l10n.profilePersonalInfo, Icons.person_outline),
                          const SizedBox(height: 12),

                          if (_person.personInfo?.biography != null &&
                              _person.personInfo!.biography!.isNotEmpty) ...[
                            _buildInfoRow(
                              Icons.description_outlined,
                              l10n.profileBiography,
                              _person.personInfo!.biography!,
                            ),
                            const SizedBox(height: 10),
                          ],

                          if (_person.personInfo?.job != null &&
                              _person.personInfo!.job!.isNotEmpty) ...[
                            _buildInfoRow(
                              Icons.work_outline,
                              l10n.profileJob,
                              _person.personInfo!.job!,
                            ),
                            const SizedBox(height: 10),
                          ],

                          if (_person.personInfo?.relationship != null) ...[
                            _buildInfoRow(
                              Icons.favorite_outline,
                              l10n.profileRelationship,
                              _getRelationshipLabel(_person.personInfo!.relationship!, l10n),
                            ),
                            const SizedBox(height: 10),
                          ],

                          if (_person.personInfo?.homeTown != null &&
                              _person.personInfo!.homeTown!.isNotEmpty) ...[
                            _buildInfoRow(
                              Icons.home_outlined,
                              l10n.profileHomeTown,
                              _person.personInfo!.homeTown!,
                            ),
                            const SizedBox(height: 10),
                          ],

                          if (_person.personInfo?.currentCity != null &&
                              _person.personInfo!.currentCity!.isNotEmpty) ...[
                            _buildInfoRow(
                              Icons.location_city_outlined,
                              l10n.profileCurrentCity,
                              _person.personInfo!.currentCity!,
                            ),
                            const SizedBox(height: 10),
                          ],

                          if (_person.gender != null) ...[
                            _buildInfoRow(
                              Icons.people_outline,
                              l10n.profileGender,
                              _getGenderLabel(_person.gender!, l10n),
                            ),
                            const SizedBox(height: 10),
                          ],

                          if (_person.dateOfBirth != null) ...[
                            _buildInfoRow(
                              Icons.calendar_today_outlined,
                              l10n.profileDateOfBirth,
                              DateFormat.yMMMd().format(_person.dateOfBirth!),
                            ),
                          ],

                          const SizedBox(height: 28),

                          // Physical Stats
                          if (_person.personInfo?.weight != null ||
                              _person.personInfo?.height != null) ...[
                            _buildSectionHeader(
                              l10n.profilePhysicalStats,
                              Icons.fitness_center_outlined,
                            ),
                            const SizedBox(height: 12),
                            Row(
                              children: [
                                if (_person.personInfo?.weight != null)
                                  Expanded(
                                    child: _buildStatCard(
                                      icon: Icons.monitor_weight_outlined,
                                      label: l10n.profileWeight,
                                      value: '${_person.personInfo!.weight!.toStringAsFixed(1)} kg',
                                    ),
                                  ),
                                if (_person.personInfo?.weight != null &&
                                    _person.personInfo?.height != null)
                                  const SizedBox(width: 12),
                                if (_person.personInfo?.height != null)
                                  Expanded(
                                    child: _buildStatCard(
                                      icon: Icons.height,
                                      label: l10n.profileHeight,
                                      value: '${_person.personInfo!.height!.toStringAsFixed(0)} cm',
                                    ),
                                  ),
                              ],
                            ),
                            const SizedBox(height: 28),
                          ],

                          // Addresses
                          if (_person.addresses.isNotEmpty) ...[
                            _buildSectionHeader(l10n.addressSection, Icons.location_on_outlined),
                            const SizedBox(height: 12),
                            ..._person.addresses.map((address) => _buildAddressCard(address, l10n)),
                          ],
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),

          // Loading overlay (only while refreshing, basic info already shown)
          if (_loading)
            Positioned(
              top: 0,
              right: 0,
              child: SafeArea(
                child: Padding(
                  padding: const EdgeInsets.all(12),
                  child: Container(
                    padding: const EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      color: Colors.black.withAlpha(100),
                      borderRadius: BorderRadius.circular(20),
                    ),
                    child: const SizedBox(
                      width: 18,
                      height: 18,
                      child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                    ),
                  ),
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildSectionHeader(String title, IconData icon) {
    return Row(
      children: [
        Icon(icon, color: AppColors.primary, size: 22),
        const SizedBox(width: 8),
        Text(title, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
      ],
    );
  }

  Widget _buildInfoRow(IconData icon, String label, String value) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Icon(icon, color: AppColors.primary, size: 18),
        const SizedBox(width: 8),
        Expanded(
          child: RichText(
            text: TextSpan(
              style: const TextStyle(fontSize: 14, color: Colors.black87),
              children: [
                TextSpan(
                  text: '$label: ',
                  style: TextStyle(color: Colors.grey[600], fontWeight: FontWeight.w500),
                ),
                TextSpan(
                  text: value,
                  style: const TextStyle(fontWeight: FontWeight.w600),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildStatCard({required IconData icon, required String label, required String value}) {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 12),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.grey[200]!),
        boxShadow: [
          BoxShadow(color: Colors.grey.withAlpha(30), blurRadius: 6, offset: const Offset(0, 2)),
        ],
      ),
      child: Column(
        children: [
          Icon(icon, color: AppColors.primary, size: 28),
          const SizedBox(height: 6),
          Text(
            value,
            style: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: AppColors.primary,
            ),
          ),
          const SizedBox(height: 2),
          Text(
            label,
            style: TextStyle(fontSize: 12, color: Colors.grey[600]),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Widget _buildAddressCard(PersonAddress address, AppLocalizations l10n) {
    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: address.current ? AppColors.primary.withAlpha(15) : Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: address.current ? AppColors.primary.withAlpha(100) : Colors.grey[200]!,
        ),
        boxShadow: [
          BoxShadow(color: Colors.grey.withAlpha(25), blurRadius: 4, offset: const Offset(0, 1)),
        ],
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(
            address.current ? Icons.location_on : Icons.location_on_outlined,
            color: address.current ? AppColors.primary : Colors.grey[500],
            size: 22,
          ),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (address.current)
                  Container(
                    margin: const EdgeInsets.only(bottom: 4),
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                    decoration: BoxDecoration(
                      color: AppColors.primary,
                      borderRadius: BorderRadius.circular(10),
                    ),
                    child: Text(
                      l10n.addressCurrent,
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                Text(address.formattedAddress, style: const TextStyle(fontSize: 14)),
              ],
            ),
          ),
        ],
      ),
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

  String _getGenderLabel(String value, AppLocalizations l10n) {
    switch (value.toLowerCase()) {
      case 'male':
        return l10n.profileGenderMale;
      case 'female':
        return l10n.profileGenderFemale;
      default:
        return l10n.profileGenderOther;
    }
  }
}
