import React, { useEffect, useRef } from 'react';
import { useSearchParams, useNavigate } from 'react-router-dom';
import { useMutation } from 'urql';
import { LOGIN_WITH_MAGIC_LINK } from '../graphql/mutations';
import { useAuth } from '../contexts/AuthContext';

const MagicLinkCallbackPage: React.FC = () => {
  const [searchParams] = useSearchParams();
  const token = searchParams.get('token');
  const navigate = useNavigate();
  const { login } = useAuth();
  const [result, executeMutation] = useMutation(LOGIN_WITH_MAGIC_LINK);
  const attempted = useRef(false);

  useEffect(() => {
    const performLogin = async () => {
      if (token && !attempted.current) {
        attempted.current = true;
        const { data, error } = await executeMutation({ input: { token } });
        if (!error && data?.loginWithMagicLink) {
          const { user, token: authToken } = data.loginWithMagicLink;
          login(user, authToken);
          navigate('/dashboard');
        }
      }
    };

    performLogin();
  }, [token, executeMutation, login, navigate]);

  if (!token) {
    return (
      <div className="callback-container">
        <h2>Invalid Link</h2>
        <p>No token found in the URL. Please make sure you followed the link correctly.</p>
      </div>
    );
  }

  if (result.fetching) {
    return (
      <div className="callback-container">
        <h2>Logging you in...</h2>
        <p>Please wait while we verify your magic link.</p>
      </div>
    );
  }

  if (result.error) {
    return (
      <div className="callback-container">
        <h2>Login Failed</h2>
        <p>Error: {result.error.message}</p>
        <button onClick={() => navigate('/login')}>Back to Login</button>
      </div>
    );
  }

  return (
    <div className="callback-container">
      <h2>Processing...</h2>
    </div>
  );
};

export default MagicLinkCallbackPage;
