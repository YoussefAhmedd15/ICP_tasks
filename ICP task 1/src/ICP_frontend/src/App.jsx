import { useState, useEffect } from 'react';
import { ICP_backend } from 'declarations/ICP_backend';
import './App.css';

function App() {
  const [notes, setNotes] = useState([]);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [editingId, setEditingId] = useState(null);

  
  const loadNotes = async () => {
    try {
      const allNotes = await ICP_backend.get_all_notes();
      setNotes(allNotes);
    } catch (error) {
      console.error('Error loading notes:', error);
    }
  };

  
  const createNote = async (e) => {
    e.preventDefault();
    if (!title.trim() || !content.trim()) return;
    
    try {
      await ICP_backend.create_note(title, content);
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
      await ICP_backend.update_note(id, title, content);
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
      await ICP_backend.delete_note(id);
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
    loadNotes();
  }, []);

  return (
    <div className="app">
      <h1>📝 Note Taking dApp</h1>
      
      
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
