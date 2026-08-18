import 'dart:io';

import 'package:image/image.dart';

void main() {
  const size = 1024;
  final image = Image(width: size, height: size);
  fill(image, color: ColorRgb8(247, 241, 232));

  final taupe = ColorRgb8(139, 115, 85);
  final warmBlack = ColorRgb8(43, 39, 35);
  final paleGold = ColorRgb8(216, 207, 192);

  // Quiet, faceted field behind the central gem.
  drawPolygon(
    image,
    vertices: [
      Point(512, 128),
      Point(854, 382),
      Point(720, 826),
      Point(304, 826),
      Point(170, 382),
    ],
    color: paleGold,
  );

  const width = 24;
  void line(int x1, int y1, int x2, int y2, [Color? color]) {
    drawLine(
      image,
      x1: x1,
      y1: y1,
      x2: x2,
      y2: y2,
      color: color ?? warmBlack,
      thickness: width,
    );
  }

  final points = <Point>[
    Point(190, 360),
    Point(350, 170),
    Point(674, 170),
    Point(834, 360),
    Point(512, 850),
  ];
  for (var i = 0; i < points.length; i++) {
    final a = points[i];
    final b = points[(i + 1) % points.length];
    line(a.x.toInt(), a.y.toInt(), b.x.toInt(), b.y.toInt(), taupe);
  }
  line(190, 360, 834, 360, taupe);
  line(350, 170, 430, 360);
  line(674, 170, 594, 360);
  line(430, 360, 512, 850);
  line(594, 360, 512, 850);

  File('assets/images/icon.png').writeAsBytesSync(encodePng(image));
  File('assets/images/logo.png').writeAsBytesSync(encodePng(image));
}
