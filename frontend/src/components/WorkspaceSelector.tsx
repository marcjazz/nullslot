import React, { useState } from 'react';
import { useQuery } from 'urql';
import { MY_WORKSPACES } from '../graphql/queries';
import { useAuth } from '../contexts/AuthContext';
import CreateWorkspaceDialog from './CreateWorkspaceDialog';

const WorkspaceSelector: React.FC = () => {
  const { currentWorkspaceId, switchWorkspace } = useAuth();
  const [{ data, fetching, error }, reexecuteMyWorkspaces] = useQuery({
    query: MY_WORKSPACES,
  });
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  if (fetching) return <div>Loading workspaces...</div>;
  if (error) return <div>Error loading workspaces: {error.message}</div>;

  const workspaces = data?.myWorkspaces || [];

  return (
    <div className="workspace-selector" style={{ display: 'flex', alignItems: 'center', gap: '1rem', padding: '1rem', borderBottom: '1px solid #ccc' }}>
      <label htmlFor="workspace-select">Workspace:</label>
      <select
        id="workspace-select"
        value={currentWorkspaceId || ''}
        onChange={(e) => switchWorkspace(e.target.value || null)}
        style={{ padding: '0.5rem' }}
      >
        <option value="">Select a workspace</option>
        {workspaces.map((ws: { id: string; name: string }) => (
          <option key={ws.id} value={ws.id}>
            {ws.name}
          </option>
        ))}
      </select>
      <button onClick={() => setIsDialogOpen(true)} style={{ padding: '0.5rem 1rem' }}>
        + New Workspace
      </button>

      {isDialogOpen && (
        <CreateWorkspaceDialog
          onClose={() => setIsDialogOpen(false)}
          onSuccess={() => reexecuteMyWorkspaces({ requestPolicy: 'network-only' })}
        />
      )}
    </div>
  );
};

export default WorkspaceSelector;
