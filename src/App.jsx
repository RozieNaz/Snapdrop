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

  return (
    <div className="container">
      <h1>Snapdrop</h1>
      <div className="card">
        <h2>Status</h2>
        <p>{status}</p>
      </div>

      <div className="card">
        <button type="button" onClick={handleCapture}>Capture Full Screen</button>
      </div>

      <div className="card">
        <h2>History</h2>
        {history.length === 0 ? (
          <p>No captures yet.</p>
        ) : (
          <ul>
            {history.map((entry) => (
              <li key={entry.file_path}>{entry.file_name}</li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}
