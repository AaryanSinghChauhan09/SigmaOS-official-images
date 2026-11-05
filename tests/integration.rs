use sigmaos_official_images::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_official_images::OfficialImages::new("sigmaos-official-images").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-official-images"));
}
