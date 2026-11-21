//! Integration tests for sigmaos-official-images.

use official_images::{official_images, release_tags, render_template, TemplateContext};

#[test]
fn test_render_base_devel_template() {
    let ctx = TemplateContext { base: "archlinux".into(), ..Default::default() };
    let images = official_images();
    let bd = images.iter().find(|i| i.name == "base-devel").unwrap();
    let out = render_template(&bd.variants[0].dockerfile_template, &ctx).unwrap();
    assert!(out.contains("FROM archlinux"));
}

#[test]
fn test_release_tags() {
    let img = official_images()[0].clone();
    let tags = release_tags(&img, "2025.06.01", "x86_64");
    assert!(tags.contains(&"latest".to_string()));
    assert!(tags.contains(&"2025.06.01".to_string()));
    assert!(tags.contains(&"2025.06.01-x86_64".to_string()));
}

#[test]
fn test_official_image_set() {
    let imgs = official_images();
    assert!(imgs.iter().any(|i| i.name == "archlinux"));
}
