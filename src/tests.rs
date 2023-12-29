use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, TypePath)]
pub struct TestAsset {
    health: u32,
    spritesheet: String,
}

/*
#[test]
fn test_create_asset_loader() {

    <EventLoopBuilder<App> as EventLoopBuilderExtUnix>::with_any_thread;
    let app = App::new().add_plugins(DefaultPlugins).run();
    create_asset_loader!(TestAssetPlugin, TestAssetLoader, TestAsset, &["assets/AssetName.ron"]);


}
*/
/*
#[test]
fn test_asset_loader_error() {
    let mut app = App::new();
    app.add_plugin(TestAssetPlugin);

    let asset_path = "assets/TestAsset.txt";
    let asset = app.assets().load(asset_path).unwrap_err();

    assert_eq!(asset, CustomAssetLoaderError::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "invalid data",
    )));
}
*/
