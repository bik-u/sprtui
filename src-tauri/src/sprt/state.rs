use std::sync::RwLock;

use tauri_plugin_store::StoreExt;

#[derive(Default)]
pub struct ApiStateInner {
    pub auth_key: Option<String>,
}

pub struct AuthState(pub RwLock<AuthStateInner>);

impl AuthState {
    pub fn populate_key_from_store(&self, app: tauri::AppHandle) {
        let mut inner = self.0.write().unwrap();
        let store = app.store(crate::STORE_PATH).unwrap();
        inner.auth_key = store
            .get("auth_key")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
    }

    pub fn get_has_key(&self) -> bool {
        let inner = self.0.read().unwrap();
        inner.auth_key.is_some()
    }

    pub fn get_key(&self) -> Option<String> {
        let inner = self.0.read().unwrap();
        inner.auth_key.clone()
    }

    pub fn set_key_and_populate_store(&self, app: tauri::AppHandle, auth_key: &str) {
        let mut inner = self.0.write().unwrap();
        inner.auth_key = Some(auth_key.to_string());
        let store = app.store(crate::STORE_PATH).unwrap();
        store.set("auth_key", auth_key);
        let _ = store.save();
    }

    pub fn init() -> Self {
        Self(RwLock::new(AuthStateInner::default()))
    }
}
