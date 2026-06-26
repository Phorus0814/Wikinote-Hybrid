
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 로컬 폴더를 운영체제 전용 셸로 즉각 구동시키는 커맨드 설계
#[tauri::command]
fn open_workspace_dir(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_workspace_dir // 여기에 선언하여 프론트 브릿지와 통신을 연결합니다.
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}