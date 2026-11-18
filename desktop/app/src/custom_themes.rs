// SPDX-License-Identifier: GPL-3.0-or-later
//! Custom themes: JSON files the user adds, kept in `<data dir>/themes`.
//!
//! A theme file rides on one of the built-in themes (`base`, which decides how
//! components are drawn) and overrides its colors, corners, fonts and labels.
//! Any role it leaves out comes from the base. See `docs/design/custom-themes.md`.
//!
//! ```json
//! {
//!   "name": "EGA",
//!   "base": "graphite",
//!   "dark":  { "primary": "#55FFFF", "surface": "#000000", "onSurface": "#AAAAAA" },
//!   "light": { "primary": "#0000AA", "surface": "#AAAAAA", "onSurface": "#000000" },
//!   "shape": { "xs": 0, "sm": 0, "md": 0, "lg": 0, "xl": 0, "full": 0 },
//!   "fonts": { "ui": "Px437 IBM VGA 8x16", "display": "Px437 IBM VGA 8x16", "mono": "Px437 IBM VGA 8x16" },
//!   "label": { "case": "upper", "tracking": 0.05, "size": 12, "weight": 400 }
//! }
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};

/// The built-in themes a custom theme can ride on.
pub const BASES: [&str; 5] = ["bloom", "hive", "glass", "graphite", "fluent"];

/// The color roles a theme may set, as in `docs/design/tokens.json`.
pub const COLOR_ROLES: [&str; 19] = [
    "primary",
    "onPrimary",
    "primaryContainer",
    "onPrimaryContainer",
    "secondaryContainer",
    "onSecondaryContainer",
    "surface",
    "surfaceContainerLow",
    "surfaceContainer",
    "surfaceContainerHigh",
    "surfaceContainerHighest",
    "onSurface",
    "onSurfaceVariant",
    "outline",
    "outlineVariant",
    "success",
    "warning",
    "error",
    "onError",
];

const SHAPE_KEYS: [&str; 6] = ["xs", "sm", "md", "lg", "xl", "full"];
const MAX_FILE_BYTES: u64 = 256 * 1024;

pub fn themes_dir(data_dir: &Path) -> PathBuf {
    data_dir.join("themes")
}

/// Every valid theme in the folder, sorted by name, as a JSON array of
/// cleaned-up theme objects, each with its `id` (the file name without `.json`).
pub fn list_json(data_dir: &Path) -> String {
    let mut themes: Vec<Value> = Vec::new();
    if let Ok(entries) = fs::read_dir(themes_dir(data_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Some(id) = path.file_stem().and_then(|s| s.to_str()).map(str::to_owned) else {
                continue;
            };
            match read_theme(&path) {
                Ok(mut theme) => {
                    theme.insert("id".into(), Value::String(id));
                    themes.push(Value::Object(theme));
                }
                Err(e) => tracing::warn!(file = %path.display(), error = %e, "skipping a custom theme"),
            }
        }
    }
    themes.sort_by_key(|t| t["name"].as_str().unwrap_or_default().to_lowercase());
    serde_json::to_string(&themes).unwrap_or_else(|_| "[]".into())
}

/// Checks a theme file and copies it into the themes folder. Returns the new
/// theme's id; a theme with the same name is replaced.
pub fn import(data_dir: &Path, source: &Path) -> Result<String, String> {
    let theme = read_theme(source)?;
    let name = theme["name"].as_str().unwrap_or_default();
    let id = slug(name);
    let dir = themes_dir(data_dir);
    fs::create_dir_all(&dir).map_err(|e| format!("can't create {}: {e}", dir.display()))?;
    let text = serde_json::to_string_pretty(&Value::Object(theme)).map_err(|e| e.to_string())?;
    let dest = dir.join(format!("{id}.json"));
    fs::write(&dest, text).map_err(|e| format!("can't write {}: {e}", dest.display()))?;
    Ok(id)
}

/// Deletes a theme from the folder.
pub fn remove(data_dir: &Path, id: &str) -> Result<(), String> {
    if id.is_empty() || id != slug(id) {
        return Err("not a theme id".into());
    }
    let path = themes_dir(data_dir).join(format!("{id}.json"));
    fs::remove_file(&path).map_err(|e| format!("can't remove {}: {e}", path.display()))
}

fn read_theme(path: &Path) -> Result<Map<String, Value>, String> {
    let size = fs::metadata(path).map_err(|e| e.to_string())?.len();
    if size > MAX_FILE_BYTES {
        return Err("the file is too big to be a theme".into());
    }
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    parse(&text)
}

/// Validates a theme and keeps only what the app understands.
pub fn parse(text: &str) -> Result<Map<String, Value>, String> {
    let raw: Value = serde_json::from_str(text).map_err(|e| format!("not valid JSON: {e}"))?;
    let Value::Object(raw) = raw else {
        return Err("a theme is a JSON object".into());
    };
    let name = raw
        .get("name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .ok_or("the theme needs a \"name\"")?;
    if slug(name).is_empty() {
        return Err("the theme's name needs a letter or a digit".into());
    }
    let base = raw.get("base").and_then(Value::as_str).unwrap_or("bloom");
    if !BASES.contains(&base) {
        return Err(format!("\"base\" must be one of {}", BASES.join(", ")));
    }

    let mut out = Map::new();
    out.insert("name".into(), Value::String(name.chars().take(40).collect()));
    out.insert("base".into(), Value::String(base.into()));

    let mut has_colors = false;
    for mode in ["light", "dark"] {
        if let Some(colors) = raw.get(mode) {
            let palette = palette(colors).map_err(|e| format!("\"{mode}\": {e}"))?;
            has_colors |= !palette.is_empty();
            out.insert(mode.into(), Value::Object(palette));
        }
    }
    // One palette serves both modes.
    match (out.contains_key("light"), out.contains_key("dark")) {
        (true, false) => {
            out.insert("dark".into(), out["light"].clone());
        }
        (false, true) => {
            out.insert("light".into(), out["dark"].clone());
        }
        _ => {}
    }

    if let Some(shape) = raw.get("shape") {
        let Value::Object(shape) = shape else {
            return Err("\"shape\" is an object of corner radii".into());
        };
        let mut clean = Map::new();
        for key in SHAPE_KEYS {
            if let Some(v) = shape.get(key) {
                let r = v
                    .as_f64()
                    .filter(|r| (0.0..=999.0).contains(r))
                    .ok_or(format!("\"shape.{key}\" is a radius from 0 to 999"))?;
                clean.insert(key.into(), Value::from(r.round() as i64));
            }
        }
        out.insert("shape".into(), Value::Object(clean));
    }

    if let Some(fonts) = raw.get("fonts") {
        let Value::Object(fonts) = fonts else {
            return Err("\"fonts\" is an object: ui, display, mono".into());
        };
        let mut clean = Map::new();
        for key in ["ui", "display", "mono"] {
            if let Some(f) = fonts.get(key).and_then(Value::as_str).map(str::trim).filter(|f| !f.is_empty()) {
                clean.insert(key.into(), Value::String(f.chars().take(80).collect()));
            }
        }
        out.insert("fonts".into(), Value::Object(clean));
    }

    if let Some(label) = raw.get("label") {
        let Value::Object(label) = label else {
            return Err("\"label\" is an object: case, tracking, size, weight".into());
        };
        let mut clean = Map::new();
        if let Some(case) = label.get("case").and_then(Value::as_str) {
            if case != "none" && case != "upper" {
                return Err("\"label.case\" is \"none\" or \"upper\"".into());
            }
            clean.insert("case".into(), Value::String(case.into()));
        }
        for (key, lo, hi) in [("tracking", 0.0, 1.0), ("size", 8.0, 24.0), ("weight", 100.0, 900.0)] {
            if let Some(v) = label.get(key) {
                let n = v
                    .as_f64()
                    .filter(|n| (lo..=hi).contains(n))
                    .ok_or(format!("\"label.{key}\" is a number from {lo} to {hi}"))?;
                clean.insert(key.into(), Value::from(n));
            }
        }
        out.insert("label".into(), Value::Object(clean));
    }

    if !has_colors && !out.contains_key("shape") && !out.contains_key("fonts") && !out.contains_key("label") {
        return Err("the theme changes nothing: give it \"light\" or \"dark\" colors".into());
    }
    Ok(out)
}

fn palette(colors: &Value) -> Result<Map<String, Value>, String> {
    let Value::Object(colors) = colors else {
        return Err("colors are an object of roles".into());
    };
    let mut clean = Map::new();
    for (role, value) in colors {
        if !COLOR_ROLES.contains(&role.as_str()) {
            return Err(format!("unknown color role \"{role}\" (known: {})", COLOR_ROLES.join(", ")));
        }
        let color = value
            .as_str()
            .map(str::trim)
            .filter(|c| is_hex_color(c))
            .ok_or(format!("\"{role}\" is a color like #RRGGBB or #AARRGGBB"))?;
        clean.insert(role.clone(), Value::String(color.to_ascii_uppercase()));
    }
    Ok(clean)
}

fn is_hex_color(c: &str) -> bool {
    let Some(hex) = c.strip_prefix('#') else {
        return false;
    };
    matches!(hex.len(), 3 | 6 | 8) && hex.chars().all(|ch| ch.is_ascii_hexdigit())
}

/// A file-safe id from a theme's name: lowercase letters, digits and dashes.
fn slug(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').chars().take(40).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_minimal_theme_fills_both_modes() {
        let t = parse(r##"{ "name": "EGA", "dark": { "primary": "#55ffff" } }"##).unwrap();
        assert_eq!(t["base"], "bloom");
        assert_eq!(t["light"]["primary"], "#55FFFF");
        assert_eq!(t["dark"]["primary"], "#55FFFF");
    }

    #[test]
    fn bad_themes_are_refused() {
        assert!(parse("[]").is_err());
        assert!(parse(r##"{ "dark": { "primary": "#000" } }"##).is_err());
        assert!(parse(r##"{ "name": "x", "base": "sepia", "dark": { "primary": "#000" } }"##).is_err());
        assert!(parse(r##"{ "name": "x", "dark": { "primary": "red" } }"##).is_err());
        assert!(parse(r##"{ "name": "x", "dark": { "sparkle": "#000" } }"##).is_err());
        assert!(parse(r##"{ "name": "x" }"##).is_err());
        assert!(parse(r##"{ "name": "x", "shape": { "md": -1 } }"##).is_err());
    }

    #[test]
    fn import_list_and_remove() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("mine.json");
        fs::write(&src, r##"{ "name": "Mon Thème EGA!", "base": "graphite", "shape": { "md": 0 } }"##)
            .unwrap();
        let id = import(dir.path(), &src).unwrap();
        assert_eq!(id, "mon-th-me-ega");
        let list: Value = serde_json::from_str(&list_json(dir.path())).unwrap();
        assert_eq!(list[0]["id"], "mon-th-me-ega");
        assert_eq!(list[0]["shape"]["md"], 0);
        remove(dir.path(), &id).unwrap();
        assert_eq!(list_json(dir.path()), "[]");
        assert!(remove(dir.path(), "../x").is_err());
    }
}
