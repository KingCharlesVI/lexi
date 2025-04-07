import React, { useState, useEffect } from 'react';
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/plugin-shell';

function SettingsModal({ theme, setTheme, onClose }) {
  const isDark = theme === 'dark';
  const [activeTab, setActiveTab] = useState('appearance');
  const [currentVersion, setCurrentVersion] = useState('');
  const [latestVersion, setLatestVersion] = useState(null);
  const [hasChecked, setHasChecked] = useState(false);
  const [isOutdated, setIsOutdated] = useState(false);
  const [showUpToDateMsg, setShowUpToDateMsg] = useState(false); // 👈 NEW

  useEffect(() => {
    const loadVersion = async () => {
      try {
        const version = await getVersion();
        setCurrentVersion(version);
      } catch (err) {
        console.error('Error getting current version:', err);
      }
    };
    loadVersion();
  }, []);

  const handleToggle = () => {
    const newTheme = isDark ? 'light' : 'dark';
    setTheme(newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  };

  const compareVersions = (v1, v2) => {
    const p1 = v1.split('.').map(Number);
    const p2 = v2.split('.').map(Number);
    for (let i = 0; i < 3; i++) {
      if ((p1[i] || 0) > (p2[i] || 0)) return 1;
      if ((p1[i] || 0) < (p2[i] || 0)) return -1;
    }
    return 0;
  };

  const checkForUpdates = async () => {
    setHasChecked(true);
    try {
      const res = await fetch('https://api.github.com/repos/KingCharlesVI/lexi/releases/latest');
      const data = await res.json();
      const latest = data.tag_name.replace(/^v/, '');
      setLatestVersion(latest);

      const outdated = compareVersions(currentVersion, latest) < 0;
      setIsOutdated(outdated);

      // If up to date, flash a message for 3 seconds
      if (!outdated) {
        setShowUpToDateMsg(true);
        setTimeout(() => setShowUpToDateMsg(false), 3000);
      }
    } catch (err) {
      console.error('Failed to check for updates:', err);
    }
  };

  return (
    <div className="modal-backdrop" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <button className="modal-close-button" onClick={onClose}>×</button>

        <div className="modal-sidebar">
            <h3>Settings</h3>
            <button
                className={activeTab === 'appearance' ? 'active' : ''}
                onClick={() => setActiveTab('appearance')}
            >
                Appearance
            </button>
            <button
                className={activeTab === 'about' ? 'active' : ''}
                onClick={() => setActiveTab('about')}
            >
                About
            </button>
        </div>

        <div className="modal-content">
          {activeTab === 'appearance' && (
            <div className="setting-item">
              <span>Dark Mode</span>
              <label className="toggle-switch">
                <input
                  type="checkbox"
                  checked={isDark}
                  onChange={handleToggle}
                />
                <span className="slider" />
              </label>
            </div>
          )}

          {activeTab === 'about' && (
            <div style={{ marginTop: '20px' }}>
              <p>Current version: {currentVersion}</p>
              <button style={{ marginTop: '10px' }} onClick={checkForUpdates}>
                Check for Updates
              </button>

              {hasChecked && latestVersion && (
                <div style={{ marginTop: '15px' }}>
                  {isOutdated ? (
                    <>
                      <p>New version available: {latestVersion}</p>
                      <button
                        onClick={() =>
                          open('https://github.com/KingCharlesVI/lexi/releases/latest')
                        }
                      >
                        Download Update
                      </button>
                    </>
                  ) : (
                    showUpToDateMsg && (
                      <p style={{ color: '#4CAF50', marginTop: '10px' }}>
                        You’re up to date!
                      </p>
                    )
                  )}
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default SettingsModal;