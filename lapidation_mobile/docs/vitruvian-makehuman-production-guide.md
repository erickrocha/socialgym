# Vitruvian 3D Body — MakeHuman Production Guide

Manual asset-production steps to replace the 18 placeholder GLBs in
`assets/vitruvian3d/` with real MakeHuman/Blender exports. This is the
"artist/tooling work" step called out in
`docs/superpowers/specs/2026-07-05-vitruvian-3d-body-design.md` — not
automated by Claude Code.

## 1. What you're producing

18 static meshes: 2 genders × 3 fat buckets × 3 muscle buckets. No rigging,
no animation, no morph targets at runtime — each bucket combination is a
fully separate, already-shaped mesh. The app just swaps which GLB it loads
(`lib/utils/evolution_body_state_mapper.dart`); it never blends between them.

Exact output filenames the app expects (case-sensitive, must match precisely):

| | fat: low | fat: medium | fat: high |
|---|---|---|---|
| **muscle: low** | `male_low_low.glb` | `male_medium_low.glb` | `male_high_low.glb` |
| **muscle: medium** | `male_low_medium.glb` | `male_medium_medium.glb` | `male_high_medium.glb` |
| **muscle: high** | `male_low_high.glb` | `male_medium_high.glb` | `male_high_high.glb` |

(same 9 again with `female_` prefix)

Filename pattern: `<gender>_<fatBucket>_<muscleBucket>.glb`, all lowercase.
Destination folder: `assets/vitruvian3d/` (already declared in `pubspec.yaml`
as a Flutter asset directory — no pubspec changes needed).

## 2. MakeHuman workflow

Do this twice (once per gender), producing 9 exports each time.

### 2.1 Base mesh per gender

1. Open MakeHuman → **Modifiers → Macrodetails**.
2. Set **Gender** slider: `0.0` for female, `1.0` for male (fully at one
   extreme — avoid androgynous in-between values, since the app only has
   two gender buckets).
3. Leave **Age** at a neutral adult value (e.g. `0.5`, ~25 y/o) and keep it
   identical across all 9 exports for that gender — age shouldn't vary
   between buckets, only weight/muscle should.
4. Leave **African/Asian/Caucasian** race sliders at default/neutral —
   these don't map to anything in the app; pick one look and keep it
   consistent across the whole matrix so all 18 models feel like the same
   character family.
5. Save this as a MakeHuman preset (`.mhm`) per gender, e.g.
   `male_base.mhm` / `female_base.mhm`, so you can reopen it instead of
   redoing gender/age/race each time.

### 2.2 Fat and muscle sliders per combination

Still under **Modifiers → Macrodetails**, the two sliders that map to the
app's buckets are:

- **Weight** (thin → heavy) → drives the **fat** bucket
- **Muscle** (flabby → muscular) → drives the **muscle** bucket

Use these slider values for the three levels (adjust to taste, but keep
them consistent across both genders so the visual "jump" between buckets
looks similar for male and female):

| Bucket | Weight slider | Muscle slider |
|---|---|---|
| low | `0.15` | `0.15` |
| medium | `0.50` | `0.50` |
| high | `0.85` | `0.85` |

For each gender, produce all 9 Weight × Muscle combinations (reload the
gender's `.mhm` base preset each time so Gender/Age/race stay locked, then
only touch Weight and Muscle).

**Pose**: use MakeHuman's default **T-pose** (or the standard relaxed
A-pose if you prefer — just use the *same* pose for all 18 exports, since
each is a static, non-rigged mesh in this app). Do not bake a walk/action
pose.

**Orientation**: character should face `-Z` (MakeHuman's default forward),
standing centered on the origin, feet at `y = 0`. Keep this identical
across all 18 so the camera framing in the Flutter viewer doesn't jump
between buckets.

### 2.3 Export from MakeHuman

For each of the 9 combinations per gender:

1. Skip **Pose/Animate** and **Materials** tabs (default skin material is
   fine — no photorealistic skin per the spec's non-goals).
2. Go to **Files → Export**.
3. Export format: **Collada (.dae)** or **FBX** — pick whichever your
   Blender version's importer handles most cleanly with MakeHuman meshes
   (Collada is the most battle-tested MakeHuman→Blender path). Do **not**
   rely on MakeHuman's built-in glTF export (if your build has the
   community glTF plugin) — Blender's glTF exporter (step 3.4 below) is
   more reliable for Draco compression and Flutter/`<model-viewer>`
   compatibility.
4. Name the export something you can track back to its bucket, e.g.
   `male_low_low.dae`, matching the final GLB name so nothing gets mixed
   up during the Blender pass.

## 3. Blender pass (per exported mesh)

Repeat for all 18 files.

1. **File → Import** the `.dae`/`.fbx` from MakeHuman.
2. Delete the imported skeleton/armature and any animation data — this app
   needs a static mesh only; a bundled rig just bloats file size for no
   benefit (per spec: no real-time morphing, no per-region deformation).
3. **Object → Apply → All Transforms** (so scale/rotation are baked in,
   not left as non-1.0 transform values that some GLB viewers mishandle).
4. Double check the mesh is centered on the origin, feet at `y = 0`,
   facing `-Z`, matching every other export — consistency here matters
   more than any single "correct" orientation, since the app renders all
   18 through the same fixed-height viewer.
5. Merge into a single mesh object if MakeHuman exported multiple parts
   (body + eyes + teeth, etc.) — **Select All → Ctrl+J** — to keep things
   simple and reduce draw calls in the mobile viewer.
6. **File → Export → glTF 2.0 (.glb)**:
   - Format: **glTF Binary (.glb)**
   - Include: check "Selected Objects" if you merged/isolated the mesh
   - Transform: `+Y Up` (glTF standard; Blender's exporter handles the
     axis conversion from Blender's `+Z` up automatically)
   - Geometry: enable **Apply Modifiers**
   - Compression: enable **Draco mesh compression** (spec calls for this
     explicitly to keep files in the "low single-digit MB" range)
   - Textures: embed (glTF Binary embeds textures by default — don't use
     "Separate Textures")
   - Animation: uncheck/exclude — there is none, and excluding it avoids
     the exporter emitting empty animation chunks
7. Export directly to `assets/vitruvian3d/<gender>_<fatBucket>_<muscleBucket>.glb`,
   overwriting the matching placeholder file generated by
   `tool/generate_placeholder_vitruvian_glb.dart`.

## 4. QA checklist before committing

- [ ] All 18 filenames match the table in §1 exactly (lowercase, correct
      bucket order `<gender>_<fat>_<muscle>`).
- [ ] Each file is a few MB or less (`ls -lh assets/vitruvian3d/`) — large
      outliers usually mean Draco compression wasn't enabled or textures
      weren't embedded/compressed.
- [ ] Open a couple of files in a standalone viewer to sanity-check scale
      and orientation before wiring them into the app — drag-and-drop onto
      https://gltf-viewer.donmccurdy.com is the fastest way, or use
      Blender's own viewport.
- [ ] Run the app and visually verify in `VitruvianBodyCard`
      (`lib/widgets/evolution/vitruvian_body_card.dart`) that the model:
  - Loads without the `flutter_3d_controller` viewer erroring out.
  - Faces the camera the same way across different check-in states
    (trigger different buckets by editing a test check-in's
    `bodyFatPct`/`muscleMassPct` in `evolution_body_state_mapper.dart`'s
    consumers, or via the app's evolution check-in flow).
  - Doesn't clip out of the fixed `height: 340` viewer frame once the
    BMI-based `Transform.scale` (`0.9×`–`1.15×`, see
    `evolution_body_state_mapper.dart`) is applied at the extremes.
- [ ] Male and female sets look like a consistent progression across the
      3×3 matrix (low→high fat/muscle should read as a visibly increasing
      change, not a subtle one).

## 5. Once all 18 are in place

No code changes should be required — `VitruvianBodyCard` already resolves
`assets/vitruvian3d/<gender>_<fatBucket>_<muscleBucket>.glb` dynamically.
Just replace the placeholder files in `assets/vitruvian3d/` and rebuild the
app. You can delete `tool/generate_placeholder_vitruvian_glb.dart` once real
assets are confirmed working end-to-end, or keep it around for future
regeneration of stand-ins if the bucket matrix ever changes.
