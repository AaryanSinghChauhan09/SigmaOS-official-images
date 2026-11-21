//! Arch official image definitions + release tag plan.

/// An official Arch container image.
#[derive(Debug, Clone)]
pub struct ArchImage {
    /// Image name (`archlinux`, `base-devel`, ...).
    pub name: String,
    /// Description.
    pub description: String,
    /// Variants (tags like `latest`, `base-devel`).
    pub variants: Vec<ImageVariant>,
}

/// An image variant/tag.
#[derive(Debug, Clone)]
pub struct ImageVariant {
    /// Tag suffix.
    pub tag: String,
    /// Dockerfile template body.
    pub dockerfile_template: String,
}

/// The official Arch images upstream publishes.
pub fn official_images() -> Vec<ArchImage> {
    vec![
        ArchImage {
            name: "archlinux".into(),
            description: "Minimal Arch Linux base image".into(),
            variants: vec![ImageVariant {
                tag: "latest".into(),
                dockerfile_template: "FROM scratch\nCOPY rootfs.tar.zst /\n".into(),
            }],
        },
        ArchImage {
            name: "base-devel".into(),
            description: "Arch Linux with base-devel toolchain".into(),
            variants: vec![ImageVariant {
                tag: "latest".into(),
                dockerfile_template: "FROM {{base}}\nRUN pacman -Syu --noconfirm base-devel\n".into(),
            }],
        },
    ]
}

/// Compute release tags for an image at a version: `latest`, `version`,
/// `version-arch`.
pub fn release_tags(image: &ArchImage, version: &str, arch: &str) -> Vec<String> {
    let mut tags = vec![
        "latest".to_string(),
        format!("{version}"),
        format!("{version}-{arch}"),
    ];
    tags.sort();
    tags.dedup();
    tags
}
