#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod word_finder;
mod key_simulator;

use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use word_finder::WordDictionary;

use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow},
};

pub struct AppState {
    dictionary: Mutex<WordDictionary>,
    is_loaded: Mutex<bool>,
}

#[tauri::command]
async fn find_matching_words(
    pattern: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let dictionary = state.dictionary.lock().unwrap();
    dictionary.find_matching_words(&pattern)
}

#[tauri::command]
async fn is_dictionary_loaded(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(*state.is_loaded.lock().unwrap())
}

#[tauri::command]
async fn type_word(word: String) -> Result<(), String> {
    key_simulator::type_word(&word)
}

#[tauri::command]
fn focus_roblox_window() -> Result<(), String> {
    let window_title = "Roblox";
    let wide: Vec<u16> = window_title.encode_utf16().chain(Some(0)).collect();

    unsafe {
        let hwnd =
            FindWindowW(None, PCWSTR(wide.as_ptr())).map_err(|e| format!("Failed to find window: {}", e))?;

        if hwnd.0.is_null() {
            return Err("Roblox window not found.".to_string());
        }

        SetForegroundWindow(hwnd);
        Ok(())
    }
}

#[tauri::command]
fn on_typing_complete(app_handle: AppHandle) -> Result<(), String> {
    use tauri::Emitter;

    let window_title = "Lexi";
    let wide: Vec<u16> = window_title.encode_utf16().chain(Some(0)).collect();

    unsafe {
        let hwnd =
            FindWindowW(None, PCWSTR(wide.as_ptr())).map_err(|e| format!("Failed to find window: {}", e))?;

        if hwnd.0.is_null() {
            return Err("Lexi window not found.".to_string());
        }

        SetForegroundWindow(hwnd);
    }

    if let Some(window) = app_handle.get_webview_window("main") {
        window.emit("clear-input", {}).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn main() {
    let app_state = AppState {
        dictionary: Mutex::new(WordDictionary::new()),
        is_loaded: Mutex::new(true),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .manage(app_state)
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![
            find_matching_words,
            is_dictionary_loaded,
            type_word,
            focus_roblox_window,
            on_typing_complete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
