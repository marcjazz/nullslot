import React from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import LoginPage from './pages/LoginPage';
import MagicLinkCallbackPage from './pages/MagicLinkCallbackPage';
import OidcCallbackPage from './pages/OidcCallbackPage';
import DashboardPage from './pages/DashboardPage';
import AuthGuard from './components/AuthGuard';
import './App.css';

const App: React.FC = () => {
  return (
    <div className="app-container">
      <Routes>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/magic-link-callback" element={<MagicLinkCallbackPage />} />
        <Route path="/oidc-callback" element={<OidcCallbackPage />} />
        <Route
          path="/dashboard"
          element={
            <AuthGuard>
              <DashboardPage />
            </AuthGuard>
          }
        />
        <Route path="/" element={<Navigate to="/login" replace />} />
      </Routes>
    </div>
  );
};

export default App;
