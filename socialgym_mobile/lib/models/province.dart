class Province {
  final int id;
  final String name;
  final String acronym;
  final int countryId;

  Province({
    required this.id,
    required this.name,
    required this.acronym,
    required this.countryId,
  });

  factory Province.fromJson(Map<String, dynamic> json) {
    return Province(
      id: json['id'] as int,
      name: json['name'] as String,
      acronym: json['acronym'] as String,
      countryId: json['countryId'] as int,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'acronym': acronym,
      'countryId': countryId,
    };
  }

  @override
  String toString() => name;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Province && runtimeType == other.runtimeType && id == other.id;

  @override
  int get hashCode => id.hashCode;
}
