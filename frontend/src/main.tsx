import React from 'react';
import ReactDOM from 'react-dom/client';
import { Provider } from 'urql';
import { BrowserRouter } from 'react-router-dom';
import client from './graphql/client';
import { AuthProvider } from './contexts/AuthContext';
import App from './App';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Provider value={client}>
      <AuthProvider>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </AuthProvider>
    </Provider>
  </React.StrictMode>,
);
