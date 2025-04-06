#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod word_finder;
mod key_simulator;

use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use word_finder::WordDictionary;

use std::fs;
use serde::{Deserialize, Serialize};

use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow},
};

pub struct AppState {
    dictionary: Mutex<WordDictionary>,
    is_loaded: Mutex<bool>,
}

#[derive(Serialize, Deserialize)]
struct Dictionary {
    words: Vec<String>,
}

#[tauri::command]
fn load_dictionary() -> Result<Dictionary, String> {
    let content = fs::read_to_string("dictionary.json").map_err(|e| e.to_string())?;
    let dict: Dictionary = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(dict)
}

#[tauri::command]
fn save_dictionary(words: Vec<String>) -> Result<(), String> {
    let dict = Dictionary { words };
    let json = serde_json::to_string_pretty(&dict).map_err(|e| e.to_string())?;
    fs::write("dictionary.json", json).map_err(|e| e.to_string())?;
    Ok(())
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
async fn is_dictionary_loaded(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
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
        let hwnd = FindWindowW(None, PCWSTR(wide.as_ptr()))
            .map_err(|e| format!("Failed to find window: {}", e))?;

        if hwnd.0.is_null() {
            return Err("Roblox window not found.".to_string());
        }

        SetForegroundWindow(hwnd);
        Ok(())
    }
}

#[tauri::command]
fn on_typing_complete(app_handle: tauri::AppHandle) -> Result<(), String> {
    use tauri::Emitter;
    
    let window_title = "Lexi";
    let wide: Vec<u16> = window_title.encode_utf16().chain(Some(0)).collect();

    unsafe {
        let hwnd = FindWindowW(None, PCWSTR(wide.as_ptr()))
            .map_err(|e| format!("Failed to find window: {}", e))?;

        if hwnd.0.is_null() {
            return Err("Lexi window not found.".to_string());
        }

        SetForegroundWindow(hwnd);
    }

    // Emit event to frontend to clear input
    if let Some(window) = app_handle.get_webview_window("main") {
        window.emit("clear-input", {}).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn find_dictionary_file() -> Result<String, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| "Couldn't get parent directory of executable".to_string())?;

    let candidates = [
        exe_dir.join("public/dictionary.txt"),
        exe_dir.join("dictionary.txt"),
        PathBuf::from("public/dictionary.txt"),
        PathBuf::from("../public/dictionary.txt"),
    ];

    for path in &candidates {
        println!("Trying dictionary path: {}", path.display());
        if path.exists() {
            return Ok(path.to_string_lossy().to_string());
        }
    }

    #[cfg(debug_assertions)]
    {
        let fallback = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
        )
        .join("../public/dictionary.txt");

        println!("Trying dev fallback: {}", fallback.display());
        if fallback.exists() {
            return Ok(fallback.to_string_lossy().to_string());
        }
    }

    Err(format!(
        "Dictionary file not found. Tried: {}",
        candidates[0].display()
    ))
}

fn main() {
    let app_state = AppState {
        dictionary: Mutex::new(WordDictionary::new()),
        is_loaded: Mutex::new(false),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .manage(app_state)
        .setup(|app| {
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let path = find_dictionary_file().unwrap_or_else(|e| {
                    eprintln!("Error finding dictionary: {}", e);
                    String::new()
                });

                if !path.is_empty() {
                    println!("Loading dictionary from: {}", path);
                    let mut dict = WordDictionary::new();
                    if let Err(e) = dict.load_from_file(&path) {
                        eprintln!("Error loading dictionary: {}", e);
                    }
                    let state = app_handle.state::<AppState>();
                    *state.dictionary.lock().unwrap() = dict;
                }

                let state = app_handle.state::<AppState>();
                *state.is_loaded.lock().unwrap() = true;
                println!("Dictionary loaded successfully");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_dictionary,
            save_dictionary,
            find_matching_words,
            is_dictionary_loaded,
            type_word,
            focus_roblox_window,
            on_typing_complete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}