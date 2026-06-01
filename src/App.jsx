import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function App() {
  const [status, setStatus] = useState('Connecting to backend...');
  const [history, setHistory] = useState([]);

  async function loadHistory() {
    try {
      const entries = await invoke('list_history');
      setHistory(entries);
    } catch {
      setHistory([]);
    }
  }

  useEffect(() => {
    invoke('app_status')
      .then(setStatus)
      .catch(() => setStatus('Backend unavailable'));

    loadHistory();
  }, []);

  async function handleCapture() {
    try {
      const result = await invoke('capture_fullscreen');
      setStatus(`Saved: ${result.file_name}`);
      await loadHistory();
    } catch {
      setStatus('Capture failed');
    }
  }

  async function handleAreaCapture() {
    try {
      const result = await invoke('capture_area');
      setStatus(result);
      await loadHistory();
    } catch (error) {
      setStatus(String(error));
    }
  }

  async function handleCopyPath(path) {
    try {
      await invoke('copy_path_to_clipboard', { path });
      setStatus('Screenshot path copied to clipboard');
    } catch {
      setStatus('Copy failed');
    }
  }

  async function handleOpen(path) {
    try {
      await invoke('open_capture', { path });
      setStatus('Opened screenshot');
    } catch {
      setStatus('Open failed');
    }
  }

  return (
    <div className="container">
      <h1>Snapdrop</h1>
      <div className="card">
        <h2>Status</h2>
        <p>{status}</p>
      </div>

      <div className="card">
        <button type="button" onClick={handleCapture}>Capture Full Screen</button>{' '}
        <button type="button" onClick={handleAreaCapture}>Capture Area</button>
      </div>

      <div className="card">
        <h2>History</h2>
        {history.length === 0 ? (
          <p>No captures yet.</p>
        ) : (
          <ul>
            {history.map((entry) => (
              <li key={entry.file_path}>
                <span>{entry.file_name}</span>{' '}
                <button type="button" onClick={() => handleOpen(entry.file_path)}>
                  Open
                </button>{' '}
                <button type="button" onClick={() => handleCopyPath(entry.file_path)}>
                  Copy Path
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}
