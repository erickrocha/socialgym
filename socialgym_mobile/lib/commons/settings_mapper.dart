import 'package:socialgym_mobile/commons/mapper.dart';
import 'package:socialgym_mobile/models/enums.dart';
import 'package:socialgym_mobile/models/settings.dart';
import 'package:socialgym_mobile/src/generated/grpc/settings.pb.dart'
    as $settings;

class SettingsMapper implements Mapper<Settings, $settings.Setting> {
  @override
  $settings.Setting toProto(Settings domain) {
    return $settings.Setting(
      id: domain.id,
      uuid: domain.uuid,
      ownerId: domain.ownerId,
      ownerUuid: domain.ownerUuid,
      language: domain.language,
      theme: domain.theme,
      notificationsEnabled: domain.notificationsEnabled,
      contextMenuPosition: domain.contextMenuPosition?.toStringValue(),
      homePage: domain.homePage?.toStringValue(),
      createdAt: domain.createdAt.toIso8601String(),
      updatedAt: domain.updatedAt.toIso8601String(),
    );
  }

  @override
  Settings fromProto($settings.Setting proto) {
    return Settings(
      id: proto.id,
      uuid: proto.uuid,
      ownerId: proto.ownerId,
      ownerUuid: proto.ownerUuid,
      language: proto.language,
      theme: proto.theme,
      notificationsEnabled: proto.notificationsEnabled,
      contextMenuPosition: ContextMenuPosition.fromString(
        proto.contextMenuPosition,
      ),
      homePage: Pages.fromString(proto.homePage),
      createdAt: DateTime.parse(proto.createdAt),
      updatedAt: DateTime.parse(proto.updatedAt),
    );
  }

  @override
  List<$settings.Setting> toProtoList(List<Settings> domainList) {
    return domainList.map((settings) => toProto(settings)).toList();
  }

  @override
  List<Settings> fromProtoList(List<$settings.Setting> protoList) {
    return protoList.map((proto) => fromProto(proto)).toList();
  }
}
