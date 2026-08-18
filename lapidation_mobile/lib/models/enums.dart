enum ContextMenuPosition {
  left,
  top,
  right,
  bottom;

  static ContextMenuPosition fromString(String? value) {
    return ContextMenuPosition.values.firstWhere(
          (e) => e.name.toLowerCase() == value?.toLowerCase(),
      orElse: () => ContextMenuPosition.left,
    );
  }

  String toStringValue() => name;
}

enum Pages {
  feed,
  gallery;

  static Pages fromString(String? value) {
    return Pages.values.firstWhere(
          (e) => e.name.toLowerCase() == value?.toLowerCase(),
      orElse: () => Pages.feed,
    );
  }

  String toStringValue() => name;
}