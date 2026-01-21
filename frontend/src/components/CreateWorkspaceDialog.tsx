import React, { useState } from 'react';
import { useMutation } from 'urql';
import { CREATE_WORKSPACE } from '../graphql/mutations';

interface CreateWorkspaceDialogProps {
  onClose: () => void;
  onSuccess?: () => void;
}

const CreateWorkspaceDialog: React.FC<CreateWorkspaceDialogProps> = ({ onClose, onSuccess }) => {
  const [name, setName] = useState('');
  const [createWorkspaceResult, createWorkspace] = useMutation(CREATE_WORKSPACE);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const result = await createWorkspace({ input: { name } });
    if (result.data?.createWorkspace) {
      if (onSuccess) onSuccess();
      onClose();
    }
  };

  return (
    <div className="modal-overlay" style={overlayStyle}>
      <div className="modal-content" style={modalStyle}>
        <h2>Create Workspace</h2>
        <form onSubmit={handleSubmit}>
          <div style={{ marginBottom: '1rem' }}>
            <label htmlFor="name" style={{ display: 'block', marginBottom: '0.5rem' }}>Workspace Name</label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
              style={{ width: '100%', padding: '0.5rem' }}
            />
          </div>
          {createWorkspaceResult.error && (
            <p style={{ color: 'red' }}>{createWorkspaceResult.error.message}</p>
          )}
          <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '0.5rem' }}>
            <button type="button" onClick={onClose} disabled={createWorkspaceResult.fetching}>
              Cancel
            </button>
            <button type="submit" disabled={createWorkspaceResult.fetching}>
              {createWorkspaceResult.fetching ? 'Creating...' : 'Create'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

const overlayStyle: React.CSSProperties = {
  position: 'fixed',
  top: 0,
  left: 0,
  right: 0,
  bottom: 0,
  backgroundColor: 'rgba(0, 0, 0, 0.5)',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  zIndex: 1000,
};

const modalStyle: React.CSSProperties = {
  backgroundColor: 'white',
  padding: '2rem',
  borderRadius: '8px',
  width: '100%',
  maxWidth: '400px',
  boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
};

export default CreateWorkspaceDialog;
