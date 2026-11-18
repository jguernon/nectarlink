# Custom themes

A custom theme is a JSON file you add in **Settings › Appearance › Custom themes › Add a theme**.
NectarLink checks it, copies it into its themes folder (`<data dir>/themes/`, e.g.
`~/.local/share/Nectarlink/themes/` on Linux) and switches to it. Pick a built-in theme to go back.

You can also drop files in that folder by hand (**Themes folder** opens it) and press **Reload**
after editing one.

## Format

```json
{
  "name": "EGA",
  "base": "graphite",
  "dark":  { "primary": "#55FFFF", "surface": "#000000", "onSurface": "#FFFFFF" },
  "light": { "primary": "#0000AA", "surface": "#AAAAAA", "onSurface": "#000000" },
  "shape": { "xs": 0, "sm": 0, "md": 0, "lg": 0, "xl": 0, "full": 0 },
  "fonts": { "ui": "PxPlus IBM VGA 8x16", "display": "PxPlus IBM VGA 8x16", "mono": "PxPlus IBM VGA 8x16" },
  "label": { "case": "upper", "tracking": 0.05, "size": 12, "weight": 400 }
}
```

| Key | Required | What it does |
|---|---|---|
| `name` | yes | Shown in Settings; also gives the file its id (a theme with the same name is replaced) |
| `base` | no (`bloom`) | The built-in theme that draws the components: `bloom`, `hive`, `glass`, `graphite` or `fluent`. Graphite gives hairline outlines and flat cards, Fluent filled cards, Hive the comb, Glass translucency |
| `dark`, `light` | at least one of the keys | Color roles (`#RGB`, `#RRGGBB` or `#AARRGGBB`). Roles left out come from the base. Give only one and it serves both modes |
| `shape` | no | Corner radii in px: `xs`, `sm`, `md`, `lg`, `xl`, `full` (the "pill") |
| `fonts` | no | Installed font families: `ui`, `display` (headlines), `mono` (labels, code) |
| `label` | no | Small labels: `case` (`none` or `upper`), `tracking` (0–1 em), `size` (8–24), `weight` (100–900) |

Color roles, as in `tokens.json`: `primary`, `onPrimary`, `primaryContainer`, `onPrimaryContainer`,
`secondaryContainer`, `onSecondaryContainer`, `surface`, `surfaceContainerLow`, `surfaceContainer`,
`surfaceContainerHigh`, `surfaceContainerHighest`, `onSurface`, `onSurfaceVariant`, `outline`,
`outlineVariant`, `success`, `warning`, `error`, `onError`, plus `indicator` and `onIndicator`:
the fill and text of what's selected (navigation, tabs, chips) and, in Graphite, of filled controls;
`card` and `cardBorder`: the fill and edge of cards (the Home page's boxes, Settings' groups);
`chrome`: a title bar band across the top of the window (its text keeps `onSurface`).

Examples: [`custom-themes/`](custom-themes/).
