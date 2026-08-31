abstract interface class Mapper<DOMAIN, PROTO> {
  PROTO toProto(DOMAIN domain);

  DOMAIN fromProto(PROTO proto);

  List<PROTO> toProtoList(List<DOMAIN> domainList);

  List<DOMAIN> fromProtoList(List<PROTO> protoList);
}
