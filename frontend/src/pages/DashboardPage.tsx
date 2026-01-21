import React from 'react';
import { useAuth } from '../contexts/AuthContext';
import WorkspaceSelector from '../components/WorkspaceSelector';

const DashboardPage: React.FC = () => {
  const { user, logout } = useAuth();

  return (
    <div style={{ padding: '2rem' }}>
      <WorkspaceSelector />
      <h1>Dashboard</h1>
      {user ? (
        <>
          <p>Welcome, <strong>{user.email}</strong>!</p>
          <button onClick={logout}>Logout</button>
        </>
      ) : (
        <p>Loading user information...</p>
      )}
    </div>
  );
};

export default DashboardPage;
