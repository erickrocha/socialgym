import 'dart:async';

import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';
import 'package:provider/provider.dart';

import '../../l10n/app_localizations.dart';
import '../../models/address_candidate.dart';
import '../../models/country.dart';
import '../../providers/auth_provider.dart';
import '../../providers/resource_provider.dart';
import '../../services/address_search_service.dart';
import '../../utils/location_utils.dart';

/// Plain result of the address form, read by the embedding page on save.
/// Transport-agnostic: the page maps this into whatever domain object it
/// persists (a `PersonAddress`/Map for the REST-ish person flow, a
/// `BusinessProfileAddress` object for the gRPC business-profile flow).
class AddressFormValues {
  final String addressLine1;
  final String addressLine2;
  final String locality;
  final String administrativeArea;
  final String postalCode;
  final String countryCode;
  final double? latitude;
  final double? longitude;

  const AddressFormValues({
    required this.addressLine1,
    required this.addressLine2,
    required this.locality,
    required this.administrativeArea,
    required this.postalCode,
    required this.countryCode,
    this.latitude,
    this.longitude,
  });
}

/// Shared address entry form: GPS capture, Google Places autocomplete when
/// location services are available, a country dropdown (the only dropdown),
/// and free-text fields everywhere else — including `administrativeArea`,
/// which is never resolved against a lookup table.
class AddressFormFields extends StatefulWidget {
  final String? initialAddressLine1;
  final String? initialAddressLine2;
  final String? initialLocality;
  final String? initialAdministrativeArea;
  final String? initialPostalCode;
  final String? initialCountryCode;
  final bool isEditing;
  final Color accentColor;
  final VoidCallback onCancel;
  final ValueChanged<AddressFormValues> onSave;

  const AddressFormFields({
    super.key,
    this.initialAddressLine1,
    this.initialAddressLine2,
    this.initialLocality,
    this.initialAdministrativeArea,
    this.initialPostalCode,
    this.initialCountryCode,
    required this.isEditing,
    required this.accentColor,
    required this.onCancel,
    required this.onSave,
  });

  @override
  State<AddressFormFields> createState() => _AddressFormFieldsState();
}

class _AddressFormFieldsState extends State<AddressFormFields> {
  late final TextEditingController _addressLine1Controller;
  late final TextEditingController _addressLine2Controller;
  late final TextEditingController _localityController;
  late final TextEditingController _administrativeAreaController;
  late final TextEditingController _postalCodeController;

  Country? _selectedCountry;

  Timer? _searchDebounce;
  List<AddressCandidate> _suggestions = [];
  bool _searchingAddress = false;

  Position? _capturedPosition;
  bool _locationAvailable = false;
  double? _selectedCandidateLatitude;
  double? _selectedCandidateLongitude;

  @override
  void initState() {
    super.initState();
    _addressLine1Controller = TextEditingController(
      text: widget.initialAddressLine1 ?? '',
    );
    _addressLine2Controller = TextEditingController(
      text: widget.initialAddressLine2 ?? '',
    );
    _localityController = TextEditingController(
      text: widget.initialLocality ?? '',
    );
    _administrativeAreaController = TextEditingController(
      text: widget.initialAdministrativeArea ?? '',
    );
    _postalCodeController = TextEditingController(
      text: widget.initialPostalCode ?? '',
    );
    _selectedCountry = context.read<ResourceProvider>().getCountryByCode(
      widget.initialCountryCode,
    );
    unawaited(_checkLocationAvailability());
  }

  @override
  void dispose() {
    _searchDebounce?.cancel();
    _addressLine1Controller.dispose();
    _addressLine2Controller.dispose();
    _localityController.dispose();
    _administrativeAreaController.dispose();
    _postalCodeController.dispose();
    super.dispose();
  }

  Future<void> _checkLocationAvailability() async {
    final position = await LocationUtils.getCurrentPosition(context);
    if (!mounted || position == null) return;
    setState(() {
      _capturedPosition = position;
      _locationAvailable = true;
    });
  }

  void _onAddressLine1Changed(String text) {
    _searchDebounce?.cancel();

    if (!_locationAvailable || text.trim().length < 3) {
      setState(() => _suggestions = []);
      return;
    }

    _searchDebounce = Timer(const Duration(milliseconds: 400), () async {
      setState(() => _searchingAddress = true);
      try {
        final results = await AddressSearchService.search(
          text: text.trim(),
          token: context.read<AuthProvider>().auth!.accessToken,
          latitude: _capturedPosition?.latitude,
          longitude: _capturedPosition?.longitude,
        );
        if (!mounted) return;
        setState(() {
          _suggestions = results;
          _searchingAddress = false;
        });
      } catch (_) {
        if (!mounted) return;
        setState(() {
          _suggestions = [];
          _searchingAddress = false;
        });
      }
    });
  }

  void _selectCandidate(AddressCandidate candidate) {
    final resourceProvider = context.read<ResourceProvider>();
    setState(() {
      _addressLine1Controller.text = candidate.addressLine1;
      _addressLine2Controller.text = candidate.addressLine2 ?? '';
      _localityController.text = candidate.locality;
      _administrativeAreaController.text = candidate.administrativeArea;
      _postalCodeController.text = candidate.postalCode ?? '';
      _selectedCountry = resourceProvider.getCountryByCode(
        candidate.countryCode,
      );
      _selectedCandidateLatitude = candidate.latitude;
      _selectedCandidateLongitude = candidate.longitude;
      _suggestions = [];
    });
    FocusScope.of(context).unfocus();
  }

  void _handleSave() {
    widget.onSave(
      AddressFormValues(
        addressLine1: _addressLine1Controller.text.trim(),
        addressLine2: _addressLine2Controller.text.trim(),
        locality: _localityController.text.trim(),
        administrativeArea: _administrativeAreaController.text.trim(),
        postalCode: _postalCodeController.text.trim(),
        countryCode: _selectedCountry?.acronym ?? '',
        latitude: _selectedCandidateLatitude ?? _capturedPosition?.latitude,
        longitude: _selectedCandidateLongitude ?? _capturedPosition?.longitude,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: widget.accentColor.withAlpha(100)),
        boxShadow: [
          BoxShadow(
            color: widget.accentColor.withAlpha(20),
            blurRadius: 8,
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
                widget.isEditing
                    ? Icons.edit_location_alt
                    : Icons.add_location_alt,
                color: widget.accentColor,
              ),
              const SizedBox(width: 8),
              Text(
                widget.isEditing ? l10n.addressEdit : l10n.addressAdd,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const Spacer(),
              IconButton(
                onPressed: widget.onCancel,
                icon: const Icon(Icons.close),
                color: Colors.grey[600],
              ),
            ],
          ),
          const Divider(),
          const SizedBox(height: 12),

          TextField(
            controller: _addressLine1Controller,
            onChanged: _locationAvailable ? _onAddressLine1Changed : null,
            decoration: InputDecoration(
              labelText: l10n.addressLine1,
              hintText: 'e.g., 123 Main Street',
              prefixIcon: const Icon(Icons.route_outlined),
              suffixIcon: _searchingAddress
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
                borderRadius: BorderRadius.circular(12),
              ),
            ),
          ),
          if (_suggestions.isNotEmpty)
            Container(
              margin: const EdgeInsets.only(top: 4),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: Colors.grey[300]!),
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: _suggestions.map((candidate) {
                  return ListTile(
                    dense: true,
                    leading: const Icon(Icons.place_outlined),
                    title: Text(
                      candidate.formattedAddress,
                      overflow: TextOverflow.ellipsis,
                    ),
                    onTap: () => _selectCandidate(candidate),
                  );
                }).toList(),
              ),
            ),
          const SizedBox(height: 12),

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

          Row(
            children: [
              Expanded(
                flex: 2,
                child: TextField(
                  controller: _localityController,
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
                  controller: _administrativeAreaController,
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
                            '${country.flagEmoji}  ${country.name}',
                            overflow: TextOverflow.ellipsis,
                          ),
                        );
                      }).toList(),
                      onChanged: (value) =>
                          setState(() => _selectedCountry = value),
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
                  controller: _postalCodeController,
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

          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              TextButton(
                onPressed: widget.onCancel,
                child: Text(l10n.buttonCancel),
              ),
              const SizedBox(width: 12),
              ElevatedButton.icon(
                onPressed: _handleSave,
                icon: const Icon(Icons.save),
                label: Text(l10n.buttonSave),
                style: ElevatedButton.styleFrom(
                  backgroundColor: widget.accentColor,
                  foregroundColor: Colors.white,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
