import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/utils/lazy_list_scroll.dart';

/// Mirrors FeedPage: a SliverList whose keys are created only when an item is built.
class _LazyFeed extends StatelessWidget {
  const _LazyFeed({required this.controller, required this.keys});

  final ScrollController controller;
  final Map<String, GlobalKey> keys;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: CustomScrollView(
          controller: controller,
          slivers: [
            SliverList(
              delegate: SliverChildBuilderDelegate((ctx, i) {
                final key = keys.putIfAbsent('post-$i', GlobalKey.new);
                return KeyedSubtree(
                  key: key,
                  child: SizedBox(height: 200, child: Text('post-$i')),
                );
              }, childCount: 80),
            ),
          ],
        ),
      ),
    );
  }
}


/// Runs the scroll while pumping frames, since the helper waits for each frame to build.
Future<bool> _scroll(
  WidgetTester tester,
  ScrollController controller,
  GlobalKey? Function() keyOf,
) async {
  bool? result;
  scrollToLazyListItem(controller, keyOf, duration: Duration.zero).then((v) => result = v);
  for (var i = 0; i < 400 && result == null; i++) {
    await tester.pump(const Duration(milliseconds: 16));
  }
  return result!;
}

void main() {
  testWidgets('scrolls to a post that the lazy list has not built yet', (
    tester,
  ) async {
    final controller = ScrollController();
    final keys = <String, GlobalKey>{};
    await tester.pumpWidget(_LazyFeed(controller: controller, keys: keys));

    // Before the fix the key was never created for an unbuilt item, so nothing scrolled.
    expect(keys['post-60'], isNull);
    expect(find.text('post-60'), findsNothing);

    final found = await _scroll(tester, controller, () => keys['post-60']);
    await tester.pumpAndSettle();

    expect(found, isTrue);
    expect(find.text('post-60'), findsOneWidget);
    final top = tester.getTopLeft(find.text('post-60')).dy;
    final height = tester.view.physicalSize.height / tester.view.devicePixelRatio;
    expect(top, inInclusiveRange(0, height), reason: 'the target is inside the viewport');
  });

  testWidgets('keeps an already visible post where it is', (tester) async {
    final controller = ScrollController();
    final keys = <String, GlobalKey>{};
    await tester.pumpWidget(_LazyFeed(controller: controller, keys: keys));

    final found = await _scroll(tester, controller, () => keys['post-1']);
    await tester.pumpAndSettle();

    expect(found, isTrue);
    expect(find.text('post-1'), findsOneWidget);
  });

  testWidgets('gives up at the end of the list when the post does not exist', (
    tester,
  ) async {
    final controller = ScrollController();
    final keys = <String, GlobalKey>{};
    await tester.pumpWidget(_LazyFeed(controller: controller, keys: keys));

    final found = await _scroll(tester, controller, () => keys['post-999']);

    expect(found, isFalse);
    expect(controller.position.pixels, controller.position.maxScrollExtent);
  });
}
