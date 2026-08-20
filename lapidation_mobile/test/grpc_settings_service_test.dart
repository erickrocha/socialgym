import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/commons/settings_mapper.dart';
import 'package:lapidation_mobile/models/enums.dart';
import 'package:lapidation_mobile/models/settings.dart';
import 'package:lapidation_mobile/services/grpc/grpc_settings_service.dart';
import 'package:lapidation_mobile/src/generated/grpc/settings.pb.dart' as $settings;

void main() {
  group('GrpcSettingsService.positionToWire', () {
    test('capitalizes the position name so the backend Position::from_string matches it', () {
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.left), 'Left');
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.top), 'Top');
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.right), 'Right');
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.bottom), 'Bottom');
    });
  });

  group('GrpcSettingsService.toProto / toDomain', () {
    test('round-trips a fully-populated Settings through the proto message', () {
      final settings = Settings(
        id: 5,
        uuid: 'setting-uuid',
        ownerId: 9,
        ownerUuid: 'person-uuid',
        language: 'pt_BR',
        theme: 'dark',
        notificationsEnabled: false,
        contextMenuPosition: ContextMenuPosition.bottom,
        homePage: Pages.gallery,
      );

      final proto = SettingsMapper().toProto(settings);
      expect(proto.id, 5);
      expect(proto.uuid, 'setting-uuid');
      expect(proto.ownerId, 9);
      expect(proto.ownerUuid, 'person-uuid');
      expect(proto.language, 'pt_BR');
      expect(proto.theme, 'dark');
      expect(proto.notificationsEnabled, false);
      expect(proto.contextMenuPosition, 'Bottom');
      expect(proto.homePage, 'gallery');

      // Simulate the wire round-trip (encode/decode) before mapping back.
      final decoded = $settings.Setting.fromBuffer(proto.writeToBuffer());
      final domain = SettingsMapper().fromProto(decoded);

      expect(domain.id, 5);
      expect(domain.uuid, 'setting-uuid');
      expect(domain.ownerId, 9);
      expect(domain.ownerUuid, 'person-uuid');
      expect(domain.language, 'pt_BR');
      expect(domain.theme, 'dark');
      expect(domain.notificationsEnabled, false);
      expect(domain.contextMenuPosition, ContextMenuPosition.bottom);
      expect(domain.homePage, Pages.gallery);
    });

    test('toDomain parses a lowercase contextMenuPosition from the backend without erroring', () {
      final proto = $settings.Setting(contextMenuPosition: 'left', homePage: 'feed');
      final domain = SettingsMapper().fromProto(proto);
      expect(domain.contextMenuPosition, ContextMenuPosition.left);
    });
  });
}
