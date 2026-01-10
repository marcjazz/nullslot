# Frontend Authentication Implementation Plan

This document outlines the strategy for implementing the frontend authentication system using React, Vite, and the Rust GraphQL backend.

## 1. Project Setup & Dependencies

We will use the following libraries to handle routing and GraphQL interactions.

*   **Routing:** `react-router-dom` (Standard for React SPAs)
*   **GraphQL Client:** `urql` (Lightweight, flexible, and easy to configure for auth) + `graphql`
*   **Utilities:** `clsx` or `tailwind-merge` (if styling needs it, optional), `jwt-decode` (useful for debugging or client-side expiry checks, though not strictly required if we rely on API errors).

**Action:** Install dependencies:
```bash
npm install react-router-dom urql graphql
```

## 2. Component & Directory Structure

We will organize the authentication logic into specific directories to maintain separation of concerns.

```text
frontend/src/
├── components/
│   └── AuthGuard.jsx       # Protects routes, redirects unauthenticated users
├── contexts/
│   └── AuthContext.jsx     # Manages global auth state (user, token)
├── graphql/
│   ├── client.js           # URQL client configuration
│   └── mutations.js        # GraphQL mutation definitions
├── pages/
│   ├── LoginPage.jsx       # Email input form
│   └── MagicLinkCallback.jsx # Handles token from URL
└── App.jsx                 # Main routing setup
```

## 3. Authentication Flow Logic

### A. Login Page (`src/pages/LoginPage.jsx`)
*   **UI:** A simple centered card with:
    *   Title: "Sign in to NullSlot"
    *   Input: Email address (type="email")
    *   Button: "Send Magic Link"
    *   Feedback area: Success message or error alert.
*   **Logic:**
    1.  User enters email and submits.
    2.  Call `requestMagicLink(email: $email)` mutation.
    3.  **On Success:** Replace form with a success message: "Check your email for the magic link."
    4.  **On Error:** Display error message (e.g., "Failed to send link").

### B. Magic Link Callback Page (`src/pages/MagicLinkCallback.jsx`)
*   **Route:** `/magic-link-callback` (e.g., `http://localhost:5173/magic-link-callback?token=...`)
*   **Logic:**
    1.  On mount, extract `token` from URL query parameters.
    2.  If no token, redirect to `/login`.
    3.  Call `loginWithMagicLink(token: $token)` mutation.
    4.  **On Success:**
        *   Receive `token` (JWT) and `user` object.
        *   Call `auth.login(token, user)` (from AuthContext).
        *   Redirect to the dashboard (`/`).
    5.  **On Error:**
        *   Display "Invalid or expired link".
        *   Provide a button to "Back to Login".

## 4. State Management (`src/contexts/AuthContext.jsx`)

The `AuthContext` will be the source of truth for the current user's session.

*   **State:**
    *   `user`: The User object (id, email, role) or `null`.
    *   `token`: The JWT string or `null`.
    *   `isLoading`: Boolean, true while checking localStorage on app start.
*   **Actions:**
    *   `login(token, user)`:
        *   Sets state.
        *   Saves `token` and `user` to `localStorage`.
    *   `logout()`:
        *   Clears state.
        *   Removes items from `localStorage`.
        *   Redirects to `/login`.
*   **Initialization:**
    *   `useEffect` on mount: Check `localStorage` for `auth_token` and `auth_user`.
    *   If found, restore state. (Optionally verify token validity, but for now, we assume validity until an API call fails).

## 5. GraphQL Client Configuration (`src/graphql/client.js`)

We need to inject the JWT into the `Authorization` header for every request if the user is logged in.

*   **Setup:** Create a `urql` Client instance.
*   **Fetch Options:**
    *   Use the `fetchOptions` callback.
    *   Read the token from `localStorage` (or a closure variable if we want to be fancy, but localStorage is reliable for the client config).
    *   If token exists, add header: `Authorization: Bearer <token>`.

```javascript
// Conceptual snippet
const client = createClient({
  url: 'http://localhost:8000/graphql',
  fetchOptions: () => {
    const token = localStorage.getItem('auth_token');
    return {
      headers: { authorization: token ? `Bearer ${token}` : '' },
    };
  },
});
```

## 6. Routing & AuthGuard (`src/App.jsx` & `src/components/AuthGuard.jsx`)

*   **AuthGuard Component:**
    *   Wraps protected routes.
    *   Consumes `AuthContext`.
    *   If `isLoading` is true, show a spinner/loading state.
    *   If `!user`, `<Navigate to="/login" />`.
    *   If `user`, render `children` (the protected page).

*   **Router Structure:**
    ```jsx
    <AuthProvider>
      <BrowserRouter>
        <Routes>
          {/* Public Routes */}
          <Route path="/login" element={<LoginPage />} />
          <Route path="/magic-link-callback" element={<MagicLinkCallback />} />

          {/* Protected Routes */}
          <Route path="/" element={
            <AuthGuard>
              <Dashboard /> {/* Placeholder for main app */}
            </AuthGuard>
          } />
        </Routes>
      </BrowserRouter>
    </AuthProvider>
    ```

## 7. Implementation Steps

1.  **Install Dependencies:** `npm install react-router-dom urql graphql`
2.  **Create Context:** Implement `AuthContext.jsx`.
3.  **Setup Client:** Create `graphql/client.js` and wrap App in `Provider`.
4.  **Create Pages:** Implement `LoginPage` and `MagicLinkCallback`.
5.  **Implement Guard:** Create `AuthGuard`.
6.  **Configure Routes:** Update `App.jsx` with the routing logic.
7.  **Test:** Verify the full flow from email request to login to dashboard access.
