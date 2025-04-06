use std::thread;
use std::time::Duration;
use rand::Rng;
use inputbot::KeybdKey;

pub fn type_word(word: &str) -> Result<(), String> {
    // Add small delay to allow user to switch focus to game
    thread::sleep(Duration::from_millis(200));
    
    // Type each character with a small random delay between keystrokes
    for c in word.chars() {
        // Get the corresponding keyboard key
        let key = match c.to_lowercase().next().unwrap() {
            'a' => KeybdKey::AKey,
            'b' => KeybdKey::BKey,
            'c' => KeybdKey::CKey,
            'd' => KeybdKey::DKey,
            'e' => KeybdKey::EKey,
            'f' => KeybdKey::FKey,
            'g' => KeybdKey::GKey,
            'h' => KeybdKey::HKey,
            'i' => KeybdKey::IKey,
            'j' => KeybdKey::JKey,
            'k' => KeybdKey::KKey,
            'l' => KeybdKey::LKey,
            'm' => KeybdKey::MKey,
            'n' => KeybdKey::NKey,
            'o' => KeybdKey::OKey,
            'p' => KeybdKey::PKey,
            'q' => KeybdKey::QKey,
            'r' => KeybdKey::RKey,
            's' => KeybdKey::SKey,
            't' => KeybdKey::TKey,
            'u' => KeybdKey::UKey,
            'v' => KeybdKey::VKey,
            'w' => KeybdKey::WKey,
            'x' => KeybdKey::XKey,
            'y' => KeybdKey::YKey,
            'z' => KeybdKey::ZKey,
            _ => return Err(format!("Unsupported character: {}", c)),
        };
        
        // Press and release the key
        key.press();
        thread::sleep(Duration::from_millis(30));
        key.release();
        
        // Add a small random delay between keystrokes (50-150ms)
        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(0..100);
        thread::sleep(Duration::from_millis(delay));
    }
    
    // Press Enter after typing the word
    KeybdKey::EnterKey.press();
    thread::sleep(Duration::from_millis(50));
    KeybdKey::EnterKey.release();
    
    Ok(())
}