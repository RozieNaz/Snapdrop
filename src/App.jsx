import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function App() {
  const [status, setStatus] = useState('Connecting to backend...');
  const [history, setHistory] = useState([]);

  useEffect(() => {
    invoke('app_status')
      .then(setStatus)
      .catch(() => setStatus('Backend unavailable'));

    invoke('list_history')
      .then(setHistory)
      .catch(() => setHistory([]));
  }, []);

  return (
    <div className="container">
      <h1>Snapdrop</h1>
      <p>Windows screenshot and screen recording utility.</p>

      <div className="card">
        <h2>Status</h2>
        <p>{status}</p>
      </div>

      <div className="card">
        <h2>Capture</h2>
        <button type="button">Capture full screen</button>
      </div>

      <div className="card">
        <h2>History</h2>
        <p>{history.length === 0 ? 'No captures yet.' : `${history.length} capture(s)`}</p>
      </div>
    </div>
  );
}
