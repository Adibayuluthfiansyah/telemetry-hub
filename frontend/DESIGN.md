---
name: Telemetry Hub
colors:
  surface: '#0e1415'
  surface-dim: '#0e1415'
  surface-bright: '#343a3b'
  surface-container-lowest: '#090f10'
  surface-container-low: '#171d1e'
  surface-container: '#1b2122'
  surface-container-high: '#252b2c'
  surface-container-highest: '#303637'
  on-surface: '#dee3e4'
  on-surface-variant: '#c0c9c1'
  inverse-surface: '#dee3e4'
  inverse-on-surface: '#2b3132'
  outline: '#8a938c'
  outline-variant: '#414943'
  surface-tint: '#9ed2b2'
  primary: '#9ed2b2'
  on-primary: '#013822'
  primary-container: '#6a9b7e'
  on-primary-container: '#00311d'
  inverse-primary: '#38684e'
  secondary: '#e3c376'
  on-secondary: '#3e2e00'
  secondary-container: '#5c4602'
  on-secondary-container: '#d4b569'
  tertiary: '#ffb3b2'
  on-tertiary: '#561e20'
  tertiary-container: '#ca7b7b'
  on-tertiary-container: '#4d171a'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#baefce'
  primary-fixed-dim: '#9ed2b2'
  on-primary-fixed: '#002112'
  on-primary-fixed-variant: '#1f5038'
  secondary-fixed: '#ffdf94'
  secondary-fixed-dim: '#e3c376'
  on-secondary-fixed: '#251a00'
  on-secondary-fixed-variant: '#594400'
  tertiary-fixed: '#ffdad8'
  tertiary-fixed-dim: '#ffb3b2'
  on-tertiary-fixed: '#3a090d'
  on-tertiary-fixed-variant: '#723434'
  background: '#0e1415'
  on-background: '#dee3e4'
  surface-variant: '#303637'
typography:
  headline-lg:
    fontFamily: Geist
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Geist
    fontSize: 18px
    fontWeight: '600'
    lineHeight: 24px
    letterSpacing: -0.01em
  body-sm:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  label-caps:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: '700'
    lineHeight: 16px
    letterSpacing: 0.08em
  data-mono:
    fontFamily: JetBrains Mono
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  data-mono-lg:
    fontFamily: JetBrains Mono
    fontSize: 16px
    fontWeight: '500'
    lineHeight: 20px
spacing:
  unit: 4px
  container-padding: 16px
  gutter: 8px
  panel-gap: 1px
---

## Brand & Style
The design system is engineered for high-stakes operational environments where information density and clarity are paramount. The brand personality is authoritative, utilitarian, and technical. It avoids aesthetic flourishes in favor of mechanical precision, evoking the feel of an industrial command center or a mission control terminal.

The style is **Professional/Utilitarian**, characterized by:
- **High Density:** Maximizing screen real estate to present complex datasets without clutter.
- **Flat Depth:** Minimal use of shadows; hierarchy is established through tonal shifts and hairline borders.
- **Strict Logic:** Every color and typographic choice serves a functional purpose, primarily for status indication and data categorization.
- **Geometric Rigidity:** A commitment to a structured grid and sharp intersections.

## Colors
This design system utilizes a low-light, high-contrast palette optimized for long-duration monitoring.

- **Backgrounds:** The foundation is `#0B0F10`. Surface layers use `#111718` for base panels and `#151B1C` for active or elevated states.
- **Accents (Operational Status):**
    - **Primary (Operational Green):** `#5B8C70` represents active, healthy, or "Go" states.
    - **Warning (Amber):** `#B89B52` signifies thresholds exceeded or non-critical issues.
    - **Error (Offline Red):** `#9E5757` is reserved for system failures and immediate alerts.
- **Borders:** A consistent `#283131` is used for all structural containment.
- **Typography:** Primary data uses `#E5E7E7`. Supporting labels and metadata use `#8D9996`.

## Typography
The typography system distinguishes between structural UI and raw telemetry data.

- **Headings & UI:** Use **Geist** for a sharp, modern sans-serif feel.
- **Labels:** Small labels use **Inter** set in All-Caps with increased tracking to denote system sections.
- **Telemetry & Logs:** All numerical values, timestamps, and IDs must use **JetBrains Mono**. This ensures tabular alignment for scanning vertical columns of data.
- **Rhythm:** Line heights are kept tight (approx 1.2x to 1.4x) to maintain the dense vertical rhythm required for a command-center interface.

## Layout & Spacing
This design system employs a **Fixed Grid** philosophy with an 8px base unit, though internal padding often drops to 4px for extreme density.

- **Panels:** Layouts are composed of modular panels separated by 1px borders or 8px gutters. 
- **Density:** Components should minimize white space. Standard vertical padding for list items and table rows is 4px to 6px.
- **Breakpoints:**
    - **Desktop (1440px+):** Full multi-column dashboard with persistent sidebar.
    - **Tablet (768px - 1439px):** Collapsible sidebars, stacked data widgets.
    - **Mobile:** Single column feed. Monospace data may scale down to 11px to ensure horizontal fit.

## Elevation & Depth
Depth is communicated through **Tonal Layers** rather than shadows.

- **Level 0 (Canvas):** `#0B0F10` — The deepest background.
- **Level 1 (Panel):** `#111718` — Standard container for charts and tables.
- **Level 2 (Active/Elevated):** `#151B1C` — Used for hovered states, active tabs, or modal overlays.
- **Structural Outlines:** All panels must have a 1px solid border of `#283131`. Do not use drop shadows or blurs.

## Shapes
The shape language is **Sharp**. 

- **Standard Elements:** Buttons, inputs, and panels use a 0px corner radius.
- **Minor Softening:** If technical constraints require, a maximum of 2px (`rounded-sm`) may be applied to interactive components, but 0px is the preferred standard to reinforce the "industrial console" aesthetic.

## Components
- **Buttons:** Sharp corners. Primary buttons use a ghost style with `#5B8C70` borders and text, filling only on hover. Labels are uppercase `label-caps`.
- **Inputs:** Background of `#0B0F10`, 1px border of `#283131`. Use JetBrains Mono for user-entered text.
- **Data Tables:** High-density. Rows are separated by 1px `#283131` lines. No alternating row stripes; use hover highlights of `#151B1C` instead.
- **Status Indicators:** Simple 8px squares or circles with flat fills (Operational Green, Warning Amber, Offline Red). No glow or pulse effects unless indicating a critical real-time alert.
- **Telemetry Cards:** Header contains `label-caps` text with a subtle background tint of the status color. Body contains `data-mono-lg` values.
- **Scrollbars:** Custom slim styling. Track: transparent; Thumb: `#283131`.
