use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{Runtime, Window};

use super::typst_service::SystemWorld;
pub struct AppState<R: Runtime> {
    docs: RwLock<HashMap<Window<R>, Arc<SystemWorld>>>,
}

impl<R: Runtime> AppState<R> {
    pub fn new() -> Self {
        Self {
            docs: RwLock::new(HashMap::new()),
        }
    }
    pub fn get_world(&self, window: &Window<R>) -> Option<Arc<SystemWorld>> {
        self.docs.read().unwrap().get(window).cloned()
    }

    pub fn set_world(&self, window: &Window<R>, world: Option<Arc<SystemWorld>>) {
        let mut docs = self.docs.write().unwrap();

        match world {
            None => {
                if let Some(old) = docs.remove(window) {
                    // !todo: cleanup
                }
            }
            Some(sw) => {
                if let Some(old) = docs.insert(window.clone(), sw) {
                    // !todo
                }
            }
        };
    }
    pub fn load_world_from_path(&self, path: &PathBuf, window: &Window<R>) -> bool {
        match SystemWorld::load_from_path(path) {
            Ok(world) => {
                self.set_world(window, Some(Arc::new(world)));
                info!("load_world_from_path succes");
                true
            }
            Err(e) => {
                info!("load_world_from_path succes");
                error!("Failed to load world from path: {:?}", e);
                false
            }
        }
    }
}
