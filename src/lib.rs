//! This Wizardry is courtesy of Tantan
//! The project this is based on https://www.youtube.com/watch?v=ModFC1bhobA&t=403s
//! 
/// You are much better off watching the video as documentation(because i made them mostly compatible) than reading my scitzophrenic intrepretation of stuff i wrote late at night
#[macro_export]
macro_rules! create_asset_loader {
    (
        /// This code is an input that does not need to exist before the macro call 
        /// for plugin name make sure you have a plugin group to add it too that is initiated with the rest of the project otherwise it's dead code
        $plugin_name: ident,
        $loader_name: ident,
        ///*This code needs to exist in a format like this in the file you want to call it from
        ///```
        ///#[derive(Debug, Deserialize, TypeUuid, TypePath, Asset)]
        ///#[uuid = "95af7f47-8033-4169-bcaf-23f2f41f9b50"]
        ///pub struct AssetType {
        ///    health: Health,
        ///    spritesheet: String,
        ///}
        ///```
        $asset_type: ident    
    ) => {
        use bevy::asset::AssetLoader;
        use bevy::asset::io::Reader;
        use bevy::asset::LoadContext;
        use bevy::asset::AsyncReadExt;

        pub struct $plugin_name;

        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app.register_asset_loader($loader_name);
            }
        }

        pub struct $loader_name;
            
        impl AssetLoader for $loader_name {
            type Asset = $asset_type;
            type Settings = ();
    
            fn load<'a>(
                &'a self,
                reader: &'a mut Reader,
                _settings: &'a Self::Settings,
                _load_context: &'a mut LoadContext,
            )  -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
            Box::pin(async move {
                let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;
                let asset: $asset_type =
                    ron::de::from_bytes::<$asset_type>(&bytes).expect("unable to decode asset");
                Ok(asset)
            })
        }
    
            type Error = CustomAssetLoaderError;
    
            fn extensions(&self) -> &[&str] {
                &["custom"]
            }
        }
    }
}

