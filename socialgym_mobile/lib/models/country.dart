class Country {
  final int id;
  final String ddi;
  final String name;
  final String acronym;
  final String currency;

  Country({
    required this.id,
    required this.ddi,
    required this.name,
    required this.acronym,
    required this.currency,
  });

  factory Country.fromJson(Map<String, dynamic> json) {
    return Country(
      id: json['id'] as int,
      ddi: json['ddi'] as String,
      name: json['name'] as String,
      acronym: json['acronym'] as String,
      currency: json['currency'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'ddi': ddi,
      'name': name,
      'acronym': acronym,
      'currency': currency,
    };
  }

  /// Unicode regional-indicator flag emoji derived from [acronym] (ISO 3166-1 alpha-2).
  String get flagEmoji {
    if (acronym.length != 2) return '';
    const base = 0x1F1E6;
    final aCode = 'A'.codeUnitAt(0);
    return acronym.toUpperCase().codeUnits
        .map((c) => String.fromCharCode(base + (c - aCode)))
        .join();
  }

  @override
  String toString() => name;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Country && runtimeType == other.runtimeType && id == other.id;

  @override
  int get hashCode => id.hashCode;
}
