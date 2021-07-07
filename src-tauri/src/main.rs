#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  // todo: starry exit with starries
  tauri::api::process::Command::new_sidecar("starry")
    .unwrap()
    .spawn()
    .expect("Failed to start starry server");

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

