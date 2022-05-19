pub fn start(args: &mut [String]) {
    tauri::Builder::default()
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error to start tauri instance");
}
