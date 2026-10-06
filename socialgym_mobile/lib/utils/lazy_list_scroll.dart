import 'dart:math' as math;

import 'package:flutter/widgets.dart';

/// Scrolls a lazily built sliver list until the item behind [keyOf] is on screen.
///
/// A `SliverList` only builds the items near the viewport, so an item further
/// down has no `GlobalKey.currentContext` yet and `Scrollable.ensureVisible`
/// cannot reach it. This moves one viewport at a time so the list builds the
/// item, then aligns it. Returns false when the end of the list is reached
/// without the item ever being built.
Future<bool> scrollToLazyListItem(
  ScrollController controller,
  GlobalKey? Function() keyOf, {
  double alignment = 0.08,
  Duration duration = const Duration(milliseconds: 350),
  int maxScreens = 200,
}) async {
  for (var screen = 0; screen <= maxScreens; screen++) {
    final targetContext = keyOf()?.currentContext;
    if (targetContext != null && targetContext.mounted) {
      await Scrollable.ensureVisible(
        targetContext,
        alignment: alignment,
        duration: duration,
      );
      return true;
    }
    if (!controller.hasClients) return false;

    final position = controller.position;
    if (position.pixels >= position.maxScrollExtent) return false;
    controller.jumpTo(
      math.min(
        position.pixels + position.viewportDimension,
        position.maxScrollExtent,
      ),
    );
    // Let the list build the newly exposed items before looking again.
    await WidgetsBinding.instance.endOfFrame;
  }
  return false;
}
