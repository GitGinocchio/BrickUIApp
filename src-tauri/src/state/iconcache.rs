use crate::config::icons::IconsMap;
use std::path::PathBuf;




pub struct BrickUIconCacheState {
    pub dir: PathBuf,
    pub map: IconsMap,
}

impl BrickUIconCacheState {
    pub async fn new(dir: &PathBuf) -> Result<Self, String> {
        let map = IconsMap::load(&dir).await?;
        
        let dir = dir.join("cache").join("icons");

        Ok(Self { dir: dir, map })
    }
}