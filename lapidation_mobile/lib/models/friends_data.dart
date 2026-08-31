import 'person.dart';

/// Represents the friends data returned by the /api/friends endpoint.
class FriendsData {
  final List<Person> suggestions;
  final List<Person> friends;
  final List<Person> receiveRequests;
  final List<Person> sentRequests;

  FriendsData({
    required this.suggestions,
    required this.friends,
    required this.receiveRequests,
    required this.sentRequests,
  });

  factory FriendsData.fromJson(Map<String, dynamic> json) {
    return FriendsData(
      suggestions:
          (json['suggestions'] as List<dynamic>?)
              ?.map((e) => Person.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      friends:
          (json['friends'] as List<dynamic>?)
              ?.map((e) => Person.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      receiveRequests:
          (json['receiveRequests'] as List<dynamic>?)
              ?.map((e) => Person.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      sentRequests:
          (json['sentRequests'] as List<dynamic>?)
              ?.map((e) => Person.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'suggestions': suggestions.map((e) => e.toJson()).toList(),
      'friends': friends.map((e) => e.toJson()).toList(),
      'receiveRequests': receiveRequests.map((e) => e.toJson()).toList(),
      'sentRequests': sentRequests.map((e) => e.toJson()).toList(),
    };
  }

  /// Returns true if there are no friends, requests, or suggestions.
  bool get isEmpty =>
      suggestions.isEmpty &&
      friends.isEmpty &&
      receiveRequests.isEmpty &&
      sentRequests.isEmpty;

  /// Returns the total count of pending requests (both received and sent).
  int get pendingRequestsCount => receiveRequests.length + sentRequests.length;
}
