# Vitruvian Man → 3D Body Model

## Problem

The current "Vitruvian Man" evolution feature (`VitruvianProgressCard`) renders a flat, static SVG figure (front/back) with a `CustomPaint` color overlay that tints anatomical regions by how much each metric changed between a user's first and latest evolution check-in. Feedback: it looks flat, cartoonish, and "like a doll" — it doesn't read as a real body, and coloring regions doesn't actually make the figure look bigger, leaner, or more muscular.

## Goal

Replace the flat SVG + heatmap with a rotatable 3D human body model whose shape reflects the user's **current** body composition (from their latest evolution check-in), while keeping a simple before/after stats summary alongside it for the "progress" story.

## Non-goals

- Photorealistic face/skin rendering.
- Per-region localized deformation (e.g. bigger left bicep specifically) — out of scope for this iteration; overall build only.
- Real-time/animated morphing between check-ins — model swaps are instant when the user's bucket changes.
- Baking "then vs. now" comparison into the model itself (handled via separate stats, not a toggleable mesh).

## Asset pipeline

Two base meshes (male, female — from `PersonInfo.gender`) are built in **MakeHuman**, then brought into **Blender** for shape-key application and GLB export. Rather than shipping continuous morph targets, we pre-bake a **discrete bucket matrix**:

- **Fat level**: `low` / `medium` / `high`, derived from `bodyFatPct`, gender-specific cutoffs:
  - Male: low <15%, medium 15–25%, high >25%
  - Female: low <22%, medium 22–32%, high >32%
- **Muscle level**: `low` / `medium` / `high`, derived from `muscleMassPct`, gender-specific cutoffs (defaults, tunable constants):
  - Male: low <33%, medium 33–38%, high >38%
  - Female: low <24%, medium 24–30%, high >30%
- **Overall size**: not a separate mesh — approximated via a **uniform scale factor** derived from weight vs. height (BMI-like ratio), clamped to roughly 0.9×–1.15× so it never looks absurd.

This yields **3 × 3 = 9 mesh combinations per gender, 18 GLB files total**, stored as Flutter assets at:

```
assets/vitruvian3d/<gender>_<fatBucket>_<muscleBucket>.glb
```

Each GLB should be optimized (Draco/mesh compression where possible) to keep individual file size in the low single-digit MB range.

**Manual production step (outside this codebase's automation):** producing the 18 GLB exports in MakeHuman/Blender is artist/tooling work done by the user, not by Claude Code. Implementation should proceed with **one placeholder GLB** stood in for all 18 slots so the Flutter integration can be built and tested end-to-end; swapping in the real 18 files is a drop-in step once available.

## Rendering approach

Rejected: `flutter_3d_controller` / `model_viewer_plus` with live morph-target control — neither exposes morph/blend-shape weight APIs (both wrap `<model-viewer>`, which doesn't surface this). Rejected: a custom WebView + three.js JS bridge — unnecessary complexity given the bucket-swapping approach needs no live morph control. Rejected: native Filament/SceneKit platform views — disproportionate engineering cost for this feature.

**Note on WebView:** `flutter_3d_controller` itself hosts Google's `<model-viewer>` web component in an internal WebView (via `flutter_inappwebview`, confirmed as a transitive dependency) — this is invisible to app code; we only ever call its Dart controller API. Every mature Flutter GLB viewer with built-in touch-orbit works this way today. Confirmed acceptable during implementation (see plan Task 1 review).

**Chosen:** `flutter_3d_controller` (pub.dev), loading the appropriate pre-baked GLB for the user's current bucket combination. Since it wraps `<model-viewer>`, drag-to-orbit and pinch-zoom work out of the box with no extra code. When the user's bucket changes (new check-in crosses a threshold), the widget simply loads a different GLB — an instant cut, not an animated transition, which is an acceptable trade-off given check-ins are infrequent (not real-time).

## Data mapping

New file `lib/utils/evolution_body_state_mapper.dart` (parallel to, not a replacement of, the existing `evolution_body_progress_mapper.dart`):

```dart
enum BodyLevel { low, medium, high }

class VitruvianBodyState {
  final String gender;        // 'male' | 'female', defaults to 'male' if unset
  final BodyLevel fatBucket;
  final BodyLevel muscleBucket;
  final double scale;         // clamped ~0.9-1.15
  final bool hasCompositionData;
  final bool hasGenderData;
}

VitruvianBodyState computeVitruvianBodyState({
  required List<EvolutionCheckIn> checkins,
  PersonInfo? personInfo,
});
```

Behavior:
- Uses the **latest** check-in only (no baseline/delta logic here — that stays in the existing mapper for the badges).
- Missing `bodyFatPct` or `muscleMassPct` on the latest check-in → defaults that metric's bucket to `medium` and sets `hasCompositionData = false` (surfaced as a hint in the UI, not a blocker).
- Missing `gender` → defaults to `male` mesh set, `hasGenderData = false`.
- Scale factor: computed from `weight` and `PersonInfo.height` (BMI-like ratio → linear map into the clamp range); if either is missing, scale defaults to `1.0`.

The existing region-delta badges (waist −3cm, muscle mass +2%, etc.) are **unchanged** — they keep consuming `evolution_body_progress_mapper.dart`'s output, just no longer paint onto a body silhouette.

## Widget

New `VitruvianBodyCard` in `lib/widgets/evolution/`, replacing `VitruvianProgressCard` at its single call site (`lib/pages/workout/evolution_page.dart:260`):

- Computes `VitruvianBodyState`, resolves `assets/vitruvian3d/<gender>_<fatBucket>_<muscleBucket>.glb`, renders via `Flutter3DController` in a fixed-height container.
- Applies `scale` via whatever scale/camera API `flutter_3d_controller` exposes (spike needed during implementation to confirm exact API; fall back to wrapping in `Transform.scale` if not supported natively).
- Retains: card header/title, subtitle (baseline→latest date range), the region-badge row and its "used interpolation" note — all sourced unchanged from the existing mapper.
- Drops: front/back toggle (replaced by free rotation), the low/high change gradient legend (no longer meaningful without the heatmap).
- Fallback states: `!hasGenderData` → male model + inline hint to set gender in profile. `!hasCompositionData` → medium/medium model, no blocking state (matches today's graceful-degradation pattern where the card just shows less).

## Cleanup

Delete once the new widget is wired in and verified:
- `lib/services/vitruvian_svg_service.dart`
- `lib/widgets/evolution/vitruvian_progress_painter.dart`
- `lib/widgets/evolution/vitruvian_progress_card.dart`
- `assets/vitruvian/*.svg` and their `pubspec.yaml` asset entries
- `test/vitruvian_svg_service_test.dart`

Keep: `lib/utils/evolution_body_progress_mapper.dart` (still powers the badges).

## Testing

- Unit tests for `evolution_body_state_mapper.dart`: bucket boundary values (both genders), missing-gender fallback, missing-composition fallback, scale clamping at both extremes — mirroring the style of the existing `evolution_body_progress_mapper_test.dart`.
- 3D rendering itself is not unit-testable; verify visually on-device (per the project's `verify` skill) once real or placeholder GLBs are wired in.

## Dependencies

- Add `flutter_3d_controller` to `pubspec.yaml`.
- No new dependency needed for asset loading (bundled Flutter assets).
