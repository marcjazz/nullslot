import React, { useEffect } from 'react';
import { useSearchParams, useNavigate } from 'react-router-dom';
import { useQuery } from 'urql';
import { useAuth } from '../contexts/AuthContext';
import { ME_QUERY } from '../graphql/queries';

const OidcCallbackPage: React.FC = () => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const { login } = useAuth();
  const token = searchParams.get('token');

  const [result] = useQuery({
    query: ME_QUERY,
    pause: !token,
    context: React.useMemo(() => ({
      fetchOptions: {
        headers: {
          authorization: token ? `Bearer ${token}` : '',
        },
      },
    }), [token]),
  });

  const { data, fetching, error } = result;

  useEffect(() => {
    if (data?.me && token) {
      login(data.me, token);
      navigate('/dashboard');
    }
  }, [data, token, login, navigate]);

  if (!token) {
    return (
      <div className="callback-container">
        <h2>Error</h2>
        <p>No token found in the URL. SSO login failed.</p>
        <button onClick={() => navigate('/login')}>Back to Login</button>
      </div>
    );
  }

  if (fetching) {
    return (
      <div className="callback-container">
        <h2>Logging you in...</h2>
        <p>Please wait while we verify your credentials.</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="callback-container">
        <h2>Error</h2>
        <p>Failed to fetch user details: {error.message}</p>
        <button onClick={() => navigate('/login')}>Back to Login</button>
      </div>
    );
  }

  return null;
};

export default OidcCallbackPage;
