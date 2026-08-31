enum ReactionType {
  like,
  love,
  haha,
  wow,
  sad,
  angry;

  static ReactionType fromString(String value) {
    return ReactionType.values.firstWhere(
      (e) => e.name.toLowerCase() == value.toLowerCase(),
      orElse: () => ReactionType.like,
    );
  }

  String get emoji {
    switch (this) {
      case ReactionType.like:
        return '👍';
      case ReactionType.love:
        return '❤️';
      case ReactionType.haha:
        return '😂';
      case ReactionType.wow:
        return '😮';
      case ReactionType.sad:
        return '😢';
      case ReactionType.angry:
        return '😡';
    }
  }

  String get label {
    return name; // 'Like', 'Love', etc.
  }
}

class FeedMedia {
  final String url;
  final String mediaType; // "Image" or "Video"
  final String?
  objectKey; // S3 object key – used as CachedNetworkImage cache key

  const FeedMedia({required this.url, required this.mediaType, this.objectKey});

  factory FeedMedia.fromJson(Map<String, dynamic> json) => FeedMedia(
    url: json['url'] as String? ?? '',
    mediaType: json['mediaType'] as String? ?? 'Image',
    objectKey: json['objectKey'] as String?,
  );

  bool get isImage => mediaType.toLowerCase() == 'image';

  bool get isVideo => mediaType.toLowerCase() == 'video';
}

class FeedReaction {
  final String type;
  final String? authorId;
  final String? authorName;
  final String? authorObjectKey;
  final String? authorAvatarUrl;

  const FeedReaction({
    required this.type,
    this.authorId,
    this.authorName,
    this.authorObjectKey,
    this.authorAvatarUrl,
  });

  factory FeedReaction.fromJson(Map<String, dynamic> json) => FeedReaction(
    type: json['reactionType'] as String? ?? 'Like',
    authorId: json['authorId'] as String?,
    authorName: json['authorName'] as String?,
  );

  ReactionType get reactionType => ReactionType.fromString(type);
}

class FeedComment {
  final String uuid;
  final String postUuid;
  final String authorUuid;
  final String? authorName;
  final String? authorObjectKey; // S3 object key for cache
  final String? authorAvatar; // CDN signed URL
  final String content;
  final String? parentUuid;
  final DateTime createdAt;
  final DateTime updatedAt;
  final List<Mention>? mentions;

  const FeedComment({
    required this.uuid,
    required this.postUuid,
    required this.authorUuid,
    this.authorName,
    this.authorObjectKey,
    this.authorAvatar,
    required this.content,
    this.parentUuid,
    required this.createdAt,
    required this.updatedAt,
    this.mentions,
  });

  factory FeedComment.fromJson(Map<String, dynamic> json) => FeedComment(
    uuid: json['uuid'] as String? ?? '',
    postUuid: json['postUuid'] as String? ?? '',
    authorUuid: json['authorUuid'] as String? ?? '',
    authorName: json['authorName'] as String?,
    authorObjectKey: json['authorObjectKey'] as String?,
    authorAvatar: json['authorAvatar'] as String?,
    content: json['content'] as String? ?? '',
    parentUuid: json['parentUuid'] as String?,
    createdAt: _parseDate(json['createdAt']),
    updatedAt: _parseDate(json['updatedAt']),
  );

  static DateTime _parseDate(dynamic value) {
    if (value is String) {
      try {
        return DateTime.parse(value);
      } catch (_) {}
    }
    return DateTime.now();
  }
}

class FeedPost {
  final String uuid;
  final int authorId;
  final String authorUuid;
  final String? authorName;
  final String? authorObjectKey; // S3 object key for cache
  final String? authorAvatar; // CDN signed URL
  final String content;
  final List<FeedMedia> media;
  final List<FeedReaction> reactions;
  final List<FeedComment> comments;
  final DateTime createdAt;
  final DateTime updatedAt;
  final List<Mention>? mentions;

  const FeedPost({
    required this.uuid,
    required this.authorId,
    required this.authorUuid,
    this.authorName,
    this.authorObjectKey,
    this.authorAvatar,
    required this.content,
    required this.media,
    required this.reactions,
    required this.comments,
    required this.createdAt,
    required this.updatedAt,
    this.mentions,
  });

  factory FeedPost.fromJson(Map<String, dynamic> json) => FeedPost(
    uuid: json['uuid'] as String? ?? '',
    authorId: json['authorId'] as int? ?? 0,
    authorUuid: json['authorUuid'] as String? ?? '',
    authorName: json['authorName'] as String?,
    authorObjectKey: json['authorObjectKey'] as String?,
    authorAvatar: json['authorAvatar'] as String?,
    content: json['content'] as String? ?? '',
    media:
        (json['media'] as List<dynamic>?)
            ?.map((e) => FeedMedia.fromJson(e as Map<String, dynamic>))
            .toList() ??
        [],
    reactions:
        (json['reactions'] as List<dynamic>?)
            ?.map((e) => FeedReaction.fromJson(e as Map<String, dynamic>))
            .toList() ??
        [],
    comments:
        (json['comments'] as List<dynamic>?)
            ?.map((e) => FeedComment.fromJson(e as Map<String, dynamic>))
            .toList() ??
        [],
    createdAt: FeedComment._parseDate(json['createdAt']),
    updatedAt: FeedComment._parseDate(json['updatedAt']),
    mentions:
        (json['mentions'] as List<dynamic>?)
            ?.map((e) => Mention.fromJson(e as Map<String, dynamic>))
            .toList() ??
        [],
  );

  int reactionCountByType(ReactionType type) =>
      reactions.where((r) => r.reactionType == type).length;

  int get totalReactions => reactions.length;

  /// Returns the top 3 reaction types sorted by count (for summary display).
  List<ReactionType> get topReactionTypes {
    final counts = <ReactionType, int>{};
    for (final r in reactions) {
      counts[r.reactionType] = (counts[r.reactionType] ?? 0) + 1;
    }
    final sorted = counts.entries.toList()
      ..sort((a, b) => b.value.compareTo(a.value));
    return sorted.take(3).map((e) => e.key).toList();
  }

  FeedPost copyWith({
    String? uuid,
    int? authorId,
    String? authorUuid,
    String? authorName,
    String? authorObjectKey,
    String? authorAvatar,
    String? content,
    List<FeedMedia>? media,
    List<FeedReaction>? reactions,
    List<FeedComment>? comments,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) => FeedPost(
    uuid: uuid ?? this.uuid,
    authorId: authorId ?? this.authorId,
    authorUuid: authorUuid ?? this.authorUuid,
    authorName: authorName ?? this.authorName,
    authorObjectKey: authorObjectKey ?? this.authorObjectKey,
    authorAvatar: authorAvatar ?? this.authorAvatar,
    content: content ?? this.content,
    media: media ?? this.media,
    reactions: reactions ?? this.reactions,
    comments: comments ?? this.comments,
    createdAt: createdAt ?? this.createdAt,
    updatedAt: updatedAt ?? this.updatedAt,
  );
}

class Mention {
  final String name;
  final String mentionedUuid;

  const Mention({required this.name, required this.mentionedUuid});

  factory Mention.fromJson(Map<String, dynamic> json) => Mention(
    name: json['name'] as String? ?? '',
    mentionedUuid: json['mentionedUuid'] as String? ?? '',
  );
}
