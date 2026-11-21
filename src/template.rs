//! Dockerfile template rendering (docker-library `{{ }}` syntax).

use std::collections::BTreeMap;

use crate::ImageError;

/// Template variables available to Dockerfile templates.
#[derive(Debug, Clone, Default)]
pub struct TemplateContext {
    /// `{{base}}` — base image name (e.g. `archlinux`).
    pub base: String,
    /// `{{version}}` — version string for the build.
    pub version: String,
    /// `{{release}}` — release identifier.
    pub release: String,
    /// `{{arch}}` — target arch.
    pub arch: String,
    /// Arbitrary extra variables.
    pub extra: BTreeMap<String, String>,
}

impl TemplateContext {
    /// Look up a variable by name.
    pub fn get(&self, name: &str) -> Option<&str> {
        match name {
            "base" => Some(&self.base),
            "version" => Some(&self.version),
            "release" => Some(&self.release),
            "arch" => Some(&self.arch),
            _ => self.extra.get(name).map(String::as_str),
        }
    }
}

/// Render a docker-library-style template (`{{ var }}` / `{{#if}}` not
/// supported — Arch templates only use variable substitution).
pub fn render_template(template: &str, ctx: &TemplateContext) -> Result<String, ImageError> {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            return Err(ImageError::Template("unclosed {{".into()));
        };
        let var = after[..end].trim();
        // Strip optional whitespace-control markers.
        let var = var.trim_start_matches('-').trim_end_matches('-').trim();
        match ctx.get(var) {
            Some(v) => out.push_str(v),
            None => {
                return Err(ImageError::Template(format!("unknown variable `{var}`")))
            }
        }
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_vars() {
        let ctx = TemplateContext {
            base: "archlinux".into(),
            version: "2025.01".into(),
            arch: "x86_64".into(),
            ..Default::default()
        };
        let out = render_template("FROM {{base}}:{{version}}-{{arch}}\n", &ctx).unwrap();
        assert_eq!(out, "FROM archlinux:2025.01-x86_64\n");
    }

    #[test]
    fn test_unknown_var_errors() {
        let ctx = TemplateContext::default();
        assert!(render_template("{{nope}}", &ctx).is_err());
    }
}
