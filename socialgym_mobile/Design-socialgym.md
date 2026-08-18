# SocialGym — Visual Style & Identity

This document captures the visual design language **as actually implemented** today in `socialgym_web` and `socialgym_mobile`. It is a descriptive reference (reverse-engineered from source), not an aspirational spec — if a page doesn't follow a rule below, the rule still reflects the dominant pattern used elsewhere in the codebase.

> Scope note: this file is independent of `Design-lapidation.md` and does not reference or reconcile with it.

---

## 1. Brand identity

### Logo mark

`socialgym_web/src/assets/img/logo.png`

The mark is a three-petal lotus flower resting on a barbell — the wellness (lotus) and gym (barbell) halves of the product name fused into one glyph. Each petal is filled with one of the three brand accent colors (coral, peach, teal), outlined in the dark teal ink color. Wordmark below reads **"SOCIAL GYM"** in a bold, evenly-tracked sans/slab caps, with **"WELLNESS"** as a smaller tracked-out subtitle.

- Web favicon: `socialgym_web/public/socialgym.svg`
- The logo is rendered through a generic `<Logo src alt />` component (`commons/gui/Logo`), not hardcoded — any page can swap the src.

### Naming

Product name in UI/config: **"Social Gym"** (`main.dart` MaterialApp title). Repo/package names: `socialgym_web`, `socialgym_mobile`.

---

## 2. Color palette

The palette is defined once in web SCSS and mirrored 1:1 in Flutter, with an explicit "matching web" comment trail — this is the real source of truth to keep in sync.

| Token | Hex | Web source | Mobile source |
|---|---|---|---|
| **Primary** (sage/teal) | `#7CB2A8` | `_colors.scss` `$color-primary` | `AppColors.primary` |
| Primary hover | `#6AA298` | `$color-primary-hover` | `AppColors.primaryHover` |
| Primary disabled | `#B0D9D3` | `$color-primary-disabled` | `AppColors.primaryDisabled` |
| **Secondary** (coral) | `#F19A9A` | `$color-secondary` | `AppColors.secondary` |
| Secondary hover | `#E67E7E` | `$color-secondary-hover` | `AppColors.secondaryHover` |
| Secondary disabled | `#F9D6D6` | `$color-secondary-disabled` | `AppColors.secondaryDisabled` |
| **Third** (peach/apricot) | `#F8C491` | `$color-third` | `AppColors.third` |
| Third hover | `#F5B073` | `$color-third-hover` | `AppColors.thirdHover` |
| Third disabled | `#FDE9D9` | `$color-third-disabled` | `AppColors.thirdDisabled` |
| **Professional secondary** (indigo) | `#1B1795` | referenced in mobile comment as web's `$color-professional-secondary` | `AppColors.professionalSecondary` |
| Success | `#4CAF50` | `$color-success` | `AppColors.success` |
| Danger | `#F44336` | `$color-danger` | `AppColors.danger` |
| Background | `#F5F5F5` | `$background-color` | `AppColors.background` |
| Foreground / ink | `#050505` | `$foreground-color` | `AppColors.foreground` |

**Palette logic:** primary/secondary/third are literally the three lotus-petal colors from the logo — teal, coral, peach — used together everywhere a "brand" gradient or trio is needed. There is no separate arbitrary UI palette; the mark *is* the palette.

**Persona-adaptive accent:** mobile seeds its Material `ColorScheme` from `AppColors.primary` (teal) for a Person, but swaps to `AppColors.professionalSecondary` (indigo, `#1B1795`) when the active profile is a Professional business profile (`main.dart`, `ColorScheme.fromSeed(seedColor: personProvider.isProfessional ? professionalSecondary : primary)`). A parallel `professionalGradient3` (primary → professionalSecondary → third) exists for this mode. Web does not yet implement this persona-swap — it's mobile-only today.

**Neutrals in practice** (not centralized as tokens, but used consistently across `.scss` files):
- Card/surface white: `#ffffff`
- Body text secondary / muted (Messenger-style grey): `#65676b`
- Borders / hairlines: `#e5e5e5`, `#e5e7eb`, `#ddd`
- Placeholder/disabled text: `#999`
- Sidebar surface: `#f8f9fa`
- Hover fill on neutral surfaces: `#e0e0e0`, `#f0f2f5`

### Gradients

Defined as SCSS mixins (`_effects.scss`) and mirrored as a Flutter `LinearGradient` constant, always built from the same three brand colors, left-to-right:

```scss
@mixin gradient-3($from: primary, $mid: secondary, $to: third, $deg: 90deg) { ... }
```
```dart
AppColors.gradient3 = LinearGradient(colors: [primary, secondary, third], stops: [0.0, 0.5, 1.0]);
```

This tri-color gradient is the signature brand treatment — used on the web `AppHeader` bar background and the `Modal` header strip. There's also a simpler 2-color `gradient()` mixin (primary → secondary) available but less used.

---

## 3. Typography

- **Web:** system font stack, `Roboto` first: `"Roboto", -apple-system, BlinkMacSystemFont, "Segoe UI", "Oxygen", "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue", sans-serif` (`index.scss`). No custom webfont is loaded — this is effectively the OS-native sans stack with Roboto as the aspirational default.
- **Mobile:** Flutter/Material default (Roboto on Android, San Francisco on iOS) plus an explicit fallback chain for CJK and emoji glyph coverage: `Noto Sans`, `Noto Sans CJK SC/JP/KR`, `Noto Sans Symbols 2`, `Noto Color Emoji`, `Apple Color Emoji`, `Segoe UI Emoji` (`main.dart`). No bundled custom font family (`pubspec.yaml`'s `fonts:` block is commented out) — i.e. both platforms deliberately ride system fonts rather than ship a brand typeface.
- **Weights/sizes in practice:** headings/names bold at `font-weight: 600`; body copy `0.85–1rem`; timestamps/meta text small and muted at `0.75–0.8rem` in `#65676b`. No formal type scale file exists — sizes are set ad hoc per component, but consistently follow this small/medium/emphasis pattern.

---

## 4. Shape, elevation & spacing

- **Corner radius:** small controls (buttons, inputs, chips) `0.25–0.375rem` (4–6px); cards and modals `8px`; mobile cards commonly `BorderRadius.circular(12)`, pills/avatars fully round (`50%` / `999px`-style circular). No single global radius token — but the ramp is consistently "small controls tighter, cards/avatars rounder."
- **Elevation:** flat, low-elevation "social feed" look rather than heavy skeuomorphic shadows:
  - Post/card shadow: `0 1px 2px rgba(0,0,0,0.1)` with a hairline `1px solid #e5e5e5` border — shadow and border are used together, not shadow alone.
  - Modal: heavier `0 10px 40px rgba(0,0,0,0.2)` for the one true overlay elevation in the system.
  - Header: `0 1px 3px rgba(0,0,0,0.1)`.
  - Mobile cards: `Colors.black.withAlpha(15–30)` soft shadow + `12px` radius, `AppBar elevation: 1`.
- **Spacing:** rem-based, mobile-first steps of `0.25rem / 0.5rem / 0.75rem / 1rem / 1.5rem`; gaps between flex items typically `0.5–1rem`.
- **Motion:** short, consistent transitions — `0.2–0.3s ease` for color/background/border changes; `transform: scale(1.1)` hover micro-interactions on icon buttons and avatars; modal open uses `fadeIn` (overlay) + `slideUp` (content) keyframes at `0.3s ease-out`; mobile sidebar collapse animates width over `200ms`.

---

## 5. Layout patterns

- **Mobile-first responsive breakpoints** used throughout web SCSS: `480px`, `640px`, `768px` — components start as compact/stacked and progressively reveal labels/columns as viewport grows (see `Sidebar.scss`, `Avatar.scss`, `AppHeader.scss`).
- **Sidebar/navigation:** a single `Sidebar` component that is a horizontal scrollable icon bar below `768px` and flips to a vertical, collapsible (`80px ↔ 250px`) left rail above it — collapse persists via a toggle button, active item marked with a soft tinted background (`rgba(primary, 0.22)`) plus a bold, darkened teal label color (`#26423d`), not just a solid fill.
- **Top header (web):** fixed `50px` bar carrying the tri-color brand gradient, logo, center nav tabs (active tab underlined in a white gradient bar), and a right-aligned avatar/user section with a notification badge (`#ef4444` red pill).
- **Feed/post card (web) and workout card (mobile):** the recurring "social card" shape — white surface, rounded corners, thin border, header row (avatar + name + timestamp), body content, footer action row. Web posts explicitly borrow Facebook/Messenger conventions: grey meta text `#65676b`, light-grey hover fill `#f0f2f5`, comment bubbles as rounded grey pills.
- **Mobile drawer:** `Drawer` width `280px` expanded / `80px` collapsed, white background, same collapse-toggle idea as the web sidebar — the two platforms share one navigation *interaction model* even though they're built with different widgets.

---

## 6. Component conventions

- **Buttons:** base `.Button`/`ButtonLink` class + a `variant` modifier (`primary` / `secondary`), never per-instance inline colors — `primary-button`/`secondary-button` classes pull hover/disabled states straight from the color tokens above. Min width `100px`, `0.75rem` padding, `600` weight text, white label color.
- **Inputs:** consistent focus treatment — border becomes `$color-primary` and gains a soft `3px` primary-tinted glow (`box-shadow: 0 0 0 3px rgba(124,178,168,0.1)`) on focus; disabled state fades to the neutral background with muted text.
- **Avatars:** circular, primary-teal placeholder background with white initials when no photo; size scale `sm (32px) / md (36–40px) / lg (44–48px) / xl (120–160px)`, growing at the same `480px`/`768px` breakpoints as everything else. Camera/edit affordance is a small circular teal badge pinned bottom-right with a white ring border; save/cancel action badges reuse the success/danger tokens.
- **Toasts:** left-accent-bar style (4px colored border) with a pale tint fill per status — not solid-fill banners: error `#fee` bg / `#f44` bar, success `#efe`/`#4f4`, warning `#ffe`/`#ff4`, info `#eef`/`#44f`.
- **Icons (web):** hand-authored inline SVGs (`Icons.tsx`), `24×24` viewBox, single-color `fill="currentColor"` glyphs (e.g. `HalterIcon`, `ArmsIcon`, `LegsIcon`, `ChessIcon`) — simple geometric/line-fill style themed around fitness body parts and equipment, not a third-party icon font. Mobile leans on stock Material Icons instead (e.g. `Icons.keyboard_arrow_down`).
- **Modals:** overlay `rgba(0,0,0,0.5)` scrim, white content panel, gradient-3 header strip, dedicated `--fullscreen` variant that drops radius/animation for edge-to-edge mobile-style takeover.

---

## 7. Cross-platform consistency

Web and mobile deliberately share one design system rather than each inventing their own:

- Same six-color palette, same hex values, cross-referenced in code comments ("matching web: $color-primary").
- Same signature tri-color gradient, ported from an SCSS mixin to a Flutter `LinearGradient` constant with identical stops.
- Same interaction shape for primary navigation (collapsible rail/drawer, active-item soft-tint highlight).
- Same "social card" anatomy for feed content.
- Same restraint on typography — both platforms ride system fonts rather than bundling a custom typeface.

The one intentional platform divergence is the **persona-adaptive accent** (mobile swaps its seed color to the professional indigo `#1B1795` when a Professional business profile is active) — a mobile-only affordance not yet ported to web.

---

## 8. Overall character

SocialGym reads as a **soft, approachable wellness-meets-fitness product**: pastel-leaning secondary/tertiary tones (coral, peach) balanced by a muted sage-teal primary, low-contrast neutral surfaces (`#F5F5F5` background, near-black `#050505` ink used sparingly), and a social-feed visual grammar borrowed from mainstream social apps (card shadows, grey meta text, pill-shaped comment inputs). Nothing about the UI is loud or saturated — the boldest color moment in the whole system is the tri-color header/modal gradient, and even that runs through the same three logo-petal hues rather than introducing new ones.
