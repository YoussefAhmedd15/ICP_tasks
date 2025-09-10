import { useState, useEffect } from 'react';
import { createActor, canisterId } from 'declarations/ICP_backend';
import { HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { useAuth } from './AuthContext';
import './App.css';

function App() {
  const { isAuthenticated, principal, login, logout, isLoading, authClient } = useAuth();
  const [notes, setNotes] = useState([]);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [editingId, setEditingId] = useState(null);
  const [balance, setBalance] = useState('0');
  const [toPrincipal, setToPrincipal] = useState('');
  const [amount, setAmount] = useState('');
  const [backend, setBackend] = useState(null);

  useEffect(() => {
    const init = async () => {
      if (!isAuthenticated || !authClient) {
        setBackend(null);
        return;
      }
      const identity = authClient.getIdentity();
      // Use same-origin agent in dev to satisfy CSP (rely on Vite proxy /api->4943)
      const agent = new HttpAgent({ identity });
      try { await agent.fetchRootKey(); } catch (e) { console.warn('fetchRootKey failed', e); }
      const actor = createActor(canisterId, { agent });
      setBackend(actor);
    };
    init();
  }, [isAuthenticated, authClient]);

  
  const loadNotes = async () => {
    try {
      if (!backend) return;
      const allNotes = await backend.get_all_notes();
      setNotes(allNotes);
    } catch (error) {
      console.error('Error loading notes:', error);
    }
  };

  const loadBalance = async () => {
    try {
      if (!backend) return;
      const bal = await backend.my_balance();
      // Ensure robust string conversion for bigint/candid nat types
      const balStr = (bal !== undefined && bal !== null)
        ? (typeof bal === 'bigint' ? bal.toString() : (bal.toString ? bal.toString() : String(bal)))
        : '0';
      setBalance(balStr);
    } catch (e) {
      console.error('Error loading balance:', e);
    }
  };

  
  const createNote = async (e) => {
    e.preventDefault();
    if (!title.trim() || !content.trim()) return;
    
    try {
      if (!backend) return;
      await backend.create_note(title, content);
      setTitle('');
      setContent('');
      loadNotes();
    } catch (error) {
      console.error('Error creating note:', error);
    }
  };

  
  const updateNote = async (id) => {
    if (!title.trim() || !content.trim()) return;
    
    try {
      if (!backend) return;
      await backend.update_note(id, title, content);
      setTitle('');
      setContent('');
      setEditingId(null);
      loadNotes();
    } catch (error) {
      console.error('Error updating note:', error);
    }
  };

  
  const deleteNote = async (id) => {
    try {
      if (!backend) return;
      await backend.delete_note(id);
      loadNotes();
    } catch (error) {
      console.error('Error deleting note:', error);
    }
  };

  
  const startEdit = (note) => {
    setTitle(note.title);
    setContent(note.content);
    setEditingId(note.id);
  };

  
  const cancelEdit = () => {
    setTitle('');
    setContent('');
    setEditingId(null);
  };

  useEffect(() => {
    if (isAuthenticated && backend) {
      loadNotes();
      loadBalance();
    } else {
      setNotes([]);
      setBalance('0');
    }
  }, [isAuthenticated, backend]);

  if (isLoading) {
    return (
      <div className="app">
        <div className="loading">Loading...</div>
      </div>
    );
  }

  if (!isAuthenticated) {
    return (
      <div className="app">
        <h1>📝 Note Taking dApp</h1>
        <div className="auth-container">
          <h2>Welcome to Your Secure Note-Taking App</h2>
          <p>Please authenticate with Internet Identity to access your notes.</p>
          <button onClick={login} className="login-button">
            🔐 Login with Internet Identity
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="app">
      <div className="header">
        <h1>📝 Note Taking dApp</h1>
        <div className="user-info">
          <span className="principal">User: {principal}</span>
          <span className="balance">Balance: {balance}</span>
          <button onClick={logout} className="logout-button">
            Logout
          </button>
        </div>
      </div>

      <div className="token-container">
        <h2>Transfer Tokens</h2>
        <div className="token-form">
          <input
            type="text"
            placeholder="Receiver principal"
            value={toPrincipal}
            onChange={(e) => setToPrincipal(e.target.value)}
          />
          <input
            type="number"
            min="0"
            placeholder="Amount"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
          />
          <button
            onClick={async () => {
              try {
                if (!toPrincipal || !amount || !backend) return;
                const to = Principal.fromText(toPrincipal);
                await backend.transfer(to, BigInt(amount));
                setAmount('');
                setToPrincipal('');
                loadBalance();
              } catch (e) {
                console.error('Transfer failed:', e);
              }
            }}
          >
            Send
          </button>
        </div>
      </div>
      
      <div className="form-container">
        <h2>{editingId ? 'Edit Note' : 'Create New Note'}</h2>
        <form onSubmit={createNote}>
          <input
            type="text"
            placeholder="Note title"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
          />
          <textarea
            placeholder="Note content"
            value={content}
            onChange={(e) => setContent(e.target.value)}
          />
          <div className="button-group">
            {editingId ? (
              <>
                <button type="button" onClick={() => updateNote(editingId)}>
                  Update Note
                </button>
                <button type="button" onClick={cancelEdit}>
                  Cancel
                </button>
              </>
            ) : (
              <button type="submit">Create Note</button>
            )}
          </div>
        </form>
      </div>

      
      <div className="notes-container">
        <h2>Your Notes</h2>
        {notes.length === 0 ? (
          <p>No notes yet. Create your first note!</p>
        ) : (
          <div className="notes-grid">
            {notes.map((note) => (
              <div key={note.id} className="note-card">
                <h3>{note.title}</h3>
                <p>{note.content}</p>
                <div className="note-actions">
                  <button onClick={() => startEdit(note)}>Edit</button>
                  <button onClick={() => deleteNote(note.id)}>Delete</button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
