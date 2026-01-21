import React, { createContext, useState, useEffect, useContext, type ReactNode } from 'react';

interface User {
  id: string;
  email: string;
  role: string;
}

interface AuthContextType {
  user: User | null;
  token: string | null;
  currentWorkspaceId: string | null;
  login: (userData: User, userToken: string) => void;
  logout: () => void;
  switchWorkspace: (id: string | null) => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [currentWorkspaceId, setCurrentWorkspaceId] = useState<string | null>(null);

  useEffect(() => {
    const storedToken = localStorage.getItem('token');
    const storedUser = localStorage.getItem('user');
    const storedWorkspaceId = localStorage.getItem('workspace_id');

    if (storedToken && storedUser) {
      setToken(storedToken);
      try {
        setUser(JSON.parse(storedUser));
      } catch (e) {
        console.error('Failed to parse stored user', e);
        localStorage.removeItem('user');
        localStorage.removeItem('token');
      }
    }

    if (storedWorkspaceId) {
      setCurrentWorkspaceId(storedWorkspaceId);
    }
  }, []);

  const login = (userData: User, userToken: string) => {
    setUser(userData);
    setToken(userToken);
    localStorage.setItem('token', userToken);
    localStorage.setItem('user', JSON.stringify(userData));
  };

  const logout = () => {
    setUser(null);
    setToken(null);
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    localStorage.removeItem('workspace_id');
    setCurrentWorkspaceId(null);
  };

  const switchWorkspace = (id: string | null) => {
    setCurrentWorkspaceId(id);
    if (id) {
      localStorage.setItem('workspace_id', id);
    } else {
      localStorage.removeItem('workspace_id');
    }
  };

  return (
    <AuthContext.Provider value={{ user, token, currentWorkspaceId, login, logout, switchWorkspace }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
