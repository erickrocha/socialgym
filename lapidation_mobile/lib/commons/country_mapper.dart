
import 'package:lapidation_mobile/commons/mapper.dart';
import 'package:lapidation_mobile/models/country.dart';
import 'package:lapidation_mobile/src/generated/grpc/country.pb.dart' as $country;

class CountryMapper implements Mapper<Country,$country.Country> {


  @override
  $country.Country toProto(Country domain) {
    return $country.Country(
      id: domain.id,
      ddi: domain.ddi,
      name: domain.ddi,
      acronym: domain.acronym,
      currency: domain.currency
    );
  }

  @override
  List<$country.Country> toProtoList(List<Country> domainList) {
    return domainList.map((domain) => toProto(domain)).toList();
  }

  @override
  Country fromProto($country.Country proto) {
    return Country(
        id: proto.id,
        ddi: proto.ddi,
        name: proto.ddi,
        acronym: proto.acronym,
        currency: proto.currency
    );
  }

  @override
  List<Country> fromProtoList(List<$country.Country> protoList) {
    return protoList.map((proto) => fromProto(proto)).toList();
  }
}