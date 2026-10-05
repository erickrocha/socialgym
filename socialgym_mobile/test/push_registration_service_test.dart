import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/services/push_registration_service.dart';

void main() {
  group('PushRegistrationService.routeForTap (TC-010 routing)', () {
    test('post target opens /feed with postUuid', () {
      final target = PushRegistrationService.routeForTap({
        'targetType': 'post',
        'targetUuid': 'p-1',
      });
      expect(target.route, '/feed');
      expect(target.arguments, {'postUuid': 'p-1'});
    });

    test('friendship request target opens /friends with friendshipUuid', () {
      final target = PushRegistrationService.routeForTap({
        'targetType': 'friendship_request',
        'targetUuid': 'f-1',
      });
      expect(target.route, '/friends');
      expect(target.arguments, {'friendshipUuid': 'f-1'});
    });

    test('unknown, missing or empty target falls back to /notifications', () {
      for (final data in <Map<String, dynamic>>[
        {'targetType': 'chat', 'targetUuid': 'x'},
        {'targetType': 'post'},
        {'targetType': 'post', 'targetUuid': ''},
        {},
      ]) {
        final target = PushRegistrationService.routeForTap(data);
        expect(target.route, '/notifications', reason: '$data');
        expect(target.arguments, isNull);
      }
    });
  });
}
