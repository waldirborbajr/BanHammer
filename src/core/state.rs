
use crate::{
    core::config::Config,
    storage::memory::MemoryStorage,
};


#[derive(Clone)]
pub struct AppState {

    pub config: Config,

    pub memory: MemoryStorage,
}


impl AppState {


    pub async fn new(
        config: Config,
    ) -> Result<Self, Box<dyn std::error::Error>> {


        Ok(Self {
            config,
            memory: MemoryStorage::new(),
        })
    }
}