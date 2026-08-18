import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/services/grpc/grpc_person_service.dart';
import 'package:lapidation_mobile/src/generated/grpc/person.pb.dart' as $person;

void main() {
  group('SearchMentionableFriendsRequest', () {
    test('round-trips person_id, query and limit through protobuf encoding', () {
      final request = SearchMentionableFriendsRequest(
        personId: 132,
        query: '@pe',
        limit: 20,
      );

      final encoded = request.writeToBuffer();
      final decoded = SearchMentionableFriendsRequest.fromBuffer(encoded);

      expect(decoded.personId, 132);
      expect(decoded.query, '@pe');
      expect(decoded.limit, 20);
    });
  });

  group('Person.uuid field', () {
    test('uuid field is populated when present in the proto message', () {
      final person = $person.Person(
        id: 7,
        firstname: 'Alice',
        surname: 'Smith',
        uuid: 'abc-123',
      );

      final encoded = person.writeToBuffer();
      final decoded = $person.Person.fromBuffer(encoded);

      expect(decoded.hasUuid(), isTrue);
      expect(decoded.uuid, 'abc-123');
    });

    test('uuid field is absent (empty string) when not set', () {
      final person = $person.Person(id: 42, firstname: 'Bob', surname: 'Jones');

      final encoded = person.writeToBuffer();
      final decoded = $person.Person.fromBuffer(encoded);

      // hasUuid() returns false; uuid getter returns empty string (proto3 default).
      expect(decoded.hasUuid(), isFalse);
      expect(decoded.uuid, isEmpty);
    });
  });
}
