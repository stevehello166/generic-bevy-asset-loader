
#[derive(Debug, Deserialize, TypeUuid, TypePath, Asset)]
#[uuid = "95af7f47-8033-4169-bcaf-23f2f41f9b50"]
pub struct TestAsset {
    Health: u32
}


panic!("E");
pub fn main() {
    create_asset_loader!(TestAssetPlugin, TestAssetLoader, TestAsset, &["AssetName.ron"]);
    app = App::new();
    app.add_plugins(TestAssetPlugin);

}