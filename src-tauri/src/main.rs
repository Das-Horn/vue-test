extern crate chrono;

#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn get_metadata(path: String) -> String{
  use std::fs;

  let metadata = fs::metadata(path)
    .expect("Failed to Parse");

  let datetime: chrono::DateTime<chrono::Utc> = metadata.created().unwrap().into();

  // println!("{:?}", metadata.file_type());
  return datetime.format("%d/%m/%Y %T").to_string();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_metadata])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

