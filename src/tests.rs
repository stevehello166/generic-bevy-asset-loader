use bevy::prelude::*;
use CustomAssetLoaderError;

#[derive(AssetCollection)]
struct AssetName {
    health: Health,
    spritesheet: String,
}


#[test]
fn test_asset_loader() {
    let mut app = App::new();
    app.add_plugin(AssetNamePlugin);

    let asset_path = "assets/AssetName.ron";
    let asset = app.assets().load(asset_path).unwrap();

    assert_eq!(asset.health, Health::Normal);
    assert_eq!(asset.spritesheet, "spritesheet.png");
}

#[test]
fn test_asset_loader_error() {
    let mut app = App::new();
    app.add_plugin(AssetNamePlugin);

    let asset_path = "assets/AssetName.txt";
    let asset = app.assets().load(asset_path).unwrap_err();

    assert_eq!(asset, CustomAssetLoaderError::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "invalid data",
    )));
}