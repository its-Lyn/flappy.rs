use std::{env, error::Error, path::{Path, PathBuf}};

use macroquad::texture::{load_texture, Texture2D};

pub enum AssetType {
    Audio,
    Sprite
}

pub fn get_binary_dir() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path: PathBuf = env::current_exe()?;
    Ok(exe_path.parent().ok_or("Failed to get binary directory.")?.to_path_buf())
}

pub fn get_asset_path(asset_name: &str, asset_type: AssetType) -> Result<PathBuf, Box<dyn Error>> {
    let type_str: &str = match asset_type {
        AssetType::Audio  => "audio",
        AssetType::Sprite => "sprites"
    };

    Ok(Path::new(&get_binary_dir()?).join("assets").join(type_str).join(asset_name))
}

pub async fn get_asset(asset_name: &str, asset_type: AssetType) -> Result<Texture2D, Box<dyn Error>> {
    let binding: PathBuf = get_asset_path(asset_name, asset_type)?;
    let path_str: &str = binding.to_str().ok_or("Could not convert path to &str.")?;

    Ok(load_texture(path_str).await?) 
}