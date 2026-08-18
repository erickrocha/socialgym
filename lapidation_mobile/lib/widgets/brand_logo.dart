import 'package:flutter/material.dart';

import '../config/app_colors.dart';

class BrandLogo extends StatelessWidget {
  const BrandLogo({
    super.key,
    this.size = 120,
    this.showWordmark = true,
    this.light = false,
  });

  final double size;
  final bool showWordmark;
  final bool light;

  @override
  Widget build(BuildContext context) {
    final color = light ? AppColors.surface : AppColors.foreground;
    return Semantics(
      label: 'Lapidation Clinic',
      image: true,
      child: SizedBox(
        width: showWordmark ? size * 1.55 : size,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            CustomPaint(
              size: Size.square(size * (showWordmark ? .58 : 1)),
              painter: _GemPainter(color),
            ),
            if (showWordmark) ...[
              SizedBox(height: size * .09),
              FittedBox(
                child: Text(
                  'LAPIDATION',
                  style: TextStyle(
                    color: color,
                    fontSize: size * .16,
                    fontWeight: FontWeight.w500,
                    letterSpacing: size * .035,
                  ),
                ),
              ),
              SizedBox(height: size * .02),
              Text(
                'C L I N I C',
                style: TextStyle(
                  color: light ? AppColors.third : AppColors.muted,
                  fontSize: size * .075,
                  fontWeight: FontWeight.w500,
                  letterSpacing: size * .025,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

class _GemPainter extends CustomPainter {
  const _GemPainter(this.color);

  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final stroke = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = size.shortestSide * .035
      ..strokeCap = StrokeCap.round
      ..strokeJoin = StrokeJoin.round;
    final w = size.width;
    final h = size.height;
    final outline = Path()
      ..moveTo(w * .18, h * .34)
      ..lineTo(w * .34, h * .14)
      ..lineTo(w * .66, h * .14)
      ..lineTo(w * .82, h * .34)
      ..lineTo(w * .5, h * .86)
      ..close();
    canvas.drawPath(outline, stroke);
    canvas.drawLine(Offset(w * .18, h * .34), Offset(w * .82, h * .34), stroke);
    canvas.drawLine(Offset(w * .34, h * .14), Offset(w * .42, h * .34), stroke);
    canvas.drawLine(Offset(w * .66, h * .14), Offset(w * .58, h * .34), stroke);
    canvas.drawLine(Offset(w * .42, h * .34), Offset(w * .5, h * .86), stroke);
    canvas.drawLine(Offset(w * .58, h * .34), Offset(w * .5, h * .86), stroke);
  }

  @override
  bool shouldRepaint(covariant _GemPainter oldDelegate) =>
      oldDelegate.color != color;
}
