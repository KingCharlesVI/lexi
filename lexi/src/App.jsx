import React, { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";
import SettingsModal from './components/SettingsModal';
import './styles/App.css';

function App() {
  const [letters, setLetters] = useState('');
  const [matchedWords, setMatchedWords] = useState([]);
  const [isLoading, setIsLoading] = useState(true);
  const [status, setStatus] = useState('Loading dictionary...');
  const [theme, setTheme] = useState('light');
  const [showSettings, setShowSettings] = useState(false);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  // Poll for dictionary load state instead of using events
  useEffect(() => {
    const checkDictionaryLoaded = async () => {
      try {
        const loaded = await invoke('is_dictionary_loaded');
        if (loaded) {
          setIsLoading(false);
          setStatus('Ready');
          // Clear the interval once loaded
          clearInterval(intervalId);
        }
      } catch (error) {
        console.error('Error checking dictionary loaded state:', error);
      }
    };

    // Check every 500ms until dictionary is loaded
    const intervalId = setInterval(checkDictionaryLoaded, 500);
    
    // Initial check
    checkDictionaryLoaded();
    
    // Cleanup interval on component unmount
    return () => {
      clearInterval(intervalId);
    };
  }, []);

  // Find matching words when letters change
  useEffect(() => {
    if (letters && !isLoading) {
      findMatchingWords();
    } else {
      setMatchedWords([]);
    }
  }, [letters, isLoading]);

  const findMatchingWords = async () => {
    try {
      setStatus('Finding words...');
      const words = await invoke('find_matching_words', { pattern: letters });
      setMatchedWords(words);
      setStatus(`Found ${words.length} words`);
    } catch (error) {
      console.error(error);
      setStatus('Error finding words');
    }
  };

  const handleLettersChange = (e) => {
    // Only allow letters (a-z, A-Z) and limit to 4 characters
    const input = e.target.value.replace(/[^a-zA-Z]/g, '').toLowerCase().slice(0, 4);
    setLetters(input);
  };

  const typeWord = async (word) => {
    try {
      setStatus(`Typing word: ${word}`);
      await invoke("type_word", { word });
      await invoke("on_typing_complete");
      setStatus("Word typed and complete");
    } catch (error) {
      console.error("Error typing word:", error);
      setStatus("Error typing word");
    }
  };

  const onWordSelected = async () => {
    try {
      await invoke("focus_roblox_window");
    } catch (error) {
      console.error("Error focusing Roblox window:", error);
      setStatus("Error focusing window");
    }
  };

  const onWordClicked = async (word) => {
    await onWordSelected(); // Focus the window
    await typeWord(word);   // Then type the word
    setLetters('');
  };

  return (
    <div className="container">
      <h1>Lexi</h1>
      <h2>Your friendly neighbourhood lexicographer for Word Bomb!</h2>
      
      {isLoading ? (
        <div className="loading">Loading dictionary...</div>
      ) : (
        <>
          <div className="controls">
            <div className="input-container">
              <label htmlFor="letters-input">Enter letters:</label>
              <input
                id="letters-input"
                type="text"
                value={letters}
                onChange={handleLettersChange}
                placeholder="Type letters here"
                maxLength={4}
                autoFocus
              />
            </div>
            
            <div className="status">Status: {status}</div>
          </div>
          
          <div className="words-container">
            <h2>Matching Words ({matchedWords.length})</h2>
            <div className="word-list">
              {matchedWords.map((word) => (
                <button
                  key={word}
                  className="word-item"
                  onClick={() => onWordClicked(word)}
                >
                  {word}
                </button>
              ))}
            </div>
            <button onClick={() => setShowSettings(true)} className="settings-button">⚙️</button>
          </div>

          {showSettings && (
            <SettingsModal
              theme={theme}
              setTheme={setTheme}
              onClose={() => setShowSettings(false)}
            />
          )}
        </>
      )}
    </div>
  );
}

export default App;