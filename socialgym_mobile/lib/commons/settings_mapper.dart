import 'package:socialgym_mobile/commons/mapper.dart';
import 'package:socialgym_mobile/models/enums.dart';
import 'package:socialgym_mobile/models/settings.dart';
import 'package:socialgym_mobile/services/grpc/grpc_settings_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/settings.pb.dart'
    as $settings;
import 'package:socialgym_mobile/utils/weight_unit.dart';

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
      contextMenuPosition: domain.contextMenuPosition != null
          ? GrpcSettingsService.positionToWire(domain.contextMenuPosition!)
          : null,
      homePage: domain.homePage?.toStringValue(),
      weightUnit: domain.weightUnit?.wireValue ?? '',
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
      weightUnit:
          proto.weightUnit.isNotEmpty ? WeightUnit.fromString(proto.weightUnit) : null,
      createdAt: proto.createdAt.isNotEmpty
          ? DateTime.parse(proto.createdAt)
          : null,
      updatedAt: proto.updatedAt.isNotEmpty
          ? DateTime.parse(proto.updatedAt)
          : null,
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
