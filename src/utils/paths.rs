use std::{env, error::Error, path::{Path, PathBuf}};

use macroquad::{audio::{self, Sound}, texture::{load_texture, Texture2D}};

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

pub async fn get_asset(asset_name: &str) -> Result<Texture2D, Box<dyn Error>> {
    let binding: PathBuf = get_asset_path(asset_name, AssetType::Sprite)?;
    let path_str: &str = binding.to_str().ok_or("Could not convert sprite path to &str.")?;
 
    Ok(load_texture(path_str).await?) 
}

pub async fn get_audio(audio_name: &str) -> Result<Sound, Box<dyn Error>> {
    let binding: PathBuf = get_asset_path(audio_name, AssetType::Audio)?;
    let path_str: &str = binding.to_str().ok_or("Failed to convert audio path to &str.")?;

    Ok(audio::load_sound(path_str).await?)
}