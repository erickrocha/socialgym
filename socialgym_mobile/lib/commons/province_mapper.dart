import 'package:socialgym_mobile/commons/mapper.dart';
import 'package:socialgym_mobile/models/province.dart';
import 'package:socialgym_mobile/src/generated/grpc/province.pb.dart' as $province;

class ProvinceMapper implements Mapper<Province,$province.Province> {

  @override
  $province.Province toProto(Province domain) {
    return $province.Province(
      id: domain.id,
      name: domain.name,
      acronym: domain.acronym,
      countryId: domain.countryId,
    );
  }

  @override
  List<$province.Province> toProtoList(List<Province> domainList) {
    return domainList.map((domain) => toProto(domain)).toList();
  }

  @override
  Province fromProto($province.Province proto) {
    return Province(
        id: proto.id,
        name: proto.name,
        acronym: proto.acronym,
        countryId: proto.countryId,
    );
  }

  @override
  List<Province> fromProtoList(List<$province.Province> protoList) {
    return protoList.map((proto) => fromProto(proto)).toList();
  }
}
