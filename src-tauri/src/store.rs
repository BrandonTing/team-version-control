use tauri::EventLoopMessage;
use tauri_plugin_store::{Store, StoreBuilder};
use tauri_runtime_wry::Wry;

pub fn get_store(app_handle: tauri::AppHandle) -> Store<Wry<EventLoopMessage>> {
    let mut store = StoreBuilder::new(app_handle, ".team.dat".parse().unwrap()).build();

    let _ = store.load();
    return store;
}
