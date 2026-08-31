import 'package:flutter/material.dart';

import '../models/pending_consent.dart';
import '../services/consent_service.dart';

/// Drives the app-wide "pending legal consents" gate.
///
/// The backend rejects almost every request with `403 CONSENT_REQUIRED` when
/// the signed-in person has not accepted the current version of `terms`,
/// `privacy` (enforced on every request) or `health_data` (evolution
/// check-in creation). A Dio interceptor calls [trigger] on that response;
/// [blocking] then drives an overlay that lets the person re-accept.
class ConsentProvider extends ChangeNotifier {
  bool _blocking = false;
  bool _loading = false;
  String? _error;
  List<PendingConsent> _outstanding = const [];

  /// True while the blocking screen should be shown.
  bool get blocking => _blocking;
  bool get loading => _loading;
  String? get error => _error;
  List<PendingConsent> get outstanding => List.unmodifiable(_outstanding);

  /// Called by the Dio interceptor when a request fails with
  /// `403 CONSENT_REQUIRED`. Debounced: does nothing while already showing
  /// the gate or a check is in flight.
  Future<void> trigger() async {
    if (_blocking || _loading) return;
    _blocking = true;
    notifyListeners();
    await refresh();
  }

  /// Re-query which documents are still outstanding. When none remain the
  /// gate dismisses itself.
  Future<void> refresh() async {
    _loading = true;
    _error = null;
    notifyListeners();
    try {
      _outstanding = await ConsentService.pending();
      _blocking = _outstanding.isNotEmpty;
    } catch (e) {
      // Keep the gate up on failure — it is safer to make the person retry
      // than to silently drop them back into a blocked app.
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  /// Accept the current version of [document], then re-check.
  Future<void> accept(String document) async {
    _loading = true;
    _error = null;
    notifyListeners();
    try {
      await ConsentService.accept(document);
      _outstanding = await ConsentService.pending();
      _blocking = _outstanding.isNotEmpty;
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  /// Force-clear the gate (e.g. after signing out).
  void reset() {
    _blocking = false;
    _loading = false;
    _error = null;
    _outstanding = const [];
    notifyListeners();
  }
}
