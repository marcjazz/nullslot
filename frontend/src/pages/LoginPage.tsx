import React, { useState } from 'react';
import { useMutation } from 'urql';
import { REQUEST_MAGIC_LINK } from '../graphql/mutations';

const LoginPage: React.FC = () => {
  const [email, setEmail] = useState('');
  const [success, setSuccess] = useState(false);
  const [result, executeMutation] = useMutation(REQUEST_MAGIC_LINK);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const { error } = await executeMutation({ input: { email } });
    if (!error) {
      setSuccess(true);
    }
  };

  if (success) {
    return (
      <div className="login-container">
        <h2>Check your email</h2>
        <p>A magic link has been sent to {email}. Please click the link to log in.</p>
      </div>
    );
  }

  return (
    <div className="login-container">
      <h2>Login</h2>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="email">Email</label>
          <input
            id="email"
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
            placeholder="Enter your email"
          />
        </div>
        <button type="submit" disabled={result.fetching}>
          {result.fetching ? 'Sending...' : 'Send Magic Link'}
        </button>
      </form>
      {result.error && (
        <div className="error-message">
          <p>Error: {result.error.message}</p>
        </div>
      )}
      <div className="sso-login" style={{ marginTop: '20px', borderTop: '1px solid #ccc', paddingTop: '20px' }}>
        <p>Or log in using your corporate account:</p>
        <a href="/auth/oidc/login" className="sso-button" style={{
          display: 'inline-block',
          padding: '10px 20px',
          backgroundColor: '#4285F4',
          color: 'white',
          textDecoration: 'none',
          borderRadius: '4px'
        }}>
          Login with SSO
        </a>
      </div>
    </div>
  );
};

export default LoginPage;
