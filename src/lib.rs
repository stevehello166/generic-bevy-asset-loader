//! This Wizardry is courtesy of Tantan and updated to 0.12.1 by me
//! 
//! This is based on <https://www.youtube.com/watch?v=ModFC1bhobA&t=403s?>
//! 

use bevy::utils::thiserror;
use thiserror::Error;
/// a macro to create a generic asset loader for ron files in bevy 0.12.1
///
/// How to use the macro
        ///```
        ///#[derive(Debug, Deserialize, TypeUuid, TypePath, Asset)]
        ///#[uuid = "95af7f47-8033-4169-bcaf-23f2f41f9b50"]
        ///pub struct AssetType {
        ///    health: Health,
        ///    spritesheet: String,
        ///}
        ///
        /// create_asset_loader!(AssetNamePlugin, AssetNameLoader, AssetType, &["assets/AssetName.ron"]);
        /// ```
#[macro_export]
macro_rules! create_asset_loader {
    (  
        $plugin_name: ident,
        $loader_name: ident,
        $asset_type: ident,
        $extensions: expr    
    ) => {
        use bevy::{
            asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext, io::Reader},
            app::{App, Plugin},
            
        };
        use crate::utils::CustomAssetLoaderError;


        pub struct $plugin_name;

        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app.register_asset_loader($loader_name)
                    .add_asset::<$asset_type>();
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
                $extensions
            }
        }
    }
}

/// An error type for the asset loader
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}




    
