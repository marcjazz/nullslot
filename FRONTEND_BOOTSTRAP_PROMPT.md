# Greenfield Frontend Project Bootstrap Prompt

You are an expert frontend developer tasked with building a modern, robust, and scalable frontend application from scratch. The backend is already implemented and running. Your mission is to create a complete frontend experience, from project setup to a functional user interface.

## 1. Backend API Details

The backend is a Rust application using Axum and exposing a GraphQL API.

*   **GraphQL Endpoint**: The primary API is available at `/graphql`.
*   **Authentication**: The application uses a two-pronged authentication strategy:
    1.  **Magic Links**: Users can sign in using an email-based magic link.
    2.  **Google OIDC**: Users can authenticate via their Google accounts.
*   **Multitenancy**: The backend is multi-tenant. After a user authenticates, they must select a "workspace" to operate within. All subsequent API requests **must** include the active workspace ID in the `X-Workspace-ID` header. Without this header, requests will be rejected.

## 2. Your Task: Build the Frontend

Your task is to build the entire frontend application. This involves the following steps:

### Step 1: Project Scaffolding

*   Set up a new project using **Vite**.
*   The project should use **React** and **TypeScript**.
*   Ensure the project is initialized with a clean, well-organized file structure.

### Step 2: Install Core Dependencies

Install and configure the following essential libraries:

*   **GraphQL Client**: `urql` for handling GraphQL queries and mutations.
*   **Routing**: `react-router-dom` for managing client-side routing.
*   **Styling**: `tailwindcss` for utility-first CSS styling. Configure it with a basic theme.

### Step 3: Implement the Authentication Layer

This is a critical part of the application. You need to re-implement the full authentication flow.

*   **Authentication Context**: Create a React context to manage authentication state (e.g., user information, token, current workspace).
*   **Login Page**:
    *   Implement a login page that offers both Magic Link and Google OIDC sign-in options.
    *   For Magic Links, provide a form to enter an email address, which will trigger a `sendMagicLink` mutation.
    *   For Google OIDC, redirect the user to the backend's Google login endpoint.
*   **Callback Pages**:
    *   Create a callback page for Magic Links (`/auth/magic-link/callback`) that receives the token, exchanges it for a session, and stores it securely.
    *   Create a callback page for Google OIDC (`/auth/google/callback`) that handles the redirect from Google, stores the session, and manages user state.
*   **Workspace Selection**:
    *   After a user successfully logs in, they must be presented with a list of their available workspaces.
    *   Implement a workspace selector component that allows the user to choose a workspace.
    *   Once a workspace is selected, store the `workspaceId` globally (e.g., in your Auth context and local storage) and ensure the `X-Workspace-ID` header is added to all subsequent GraphQL requests.

### Step 4: Create the Core UI Shell

*   **Authenticated Routes**: Set up a routing structure where certain routes are protected and only accessible to authenticated users. Unauthenticated users trying to access these routes should be redirected to the login page.
*   **Main Application Layout**:
    *   Create a main layout component that includes a simple sidebar for navigation and a header.
    *   The header should display the current user's information and the currently selected workspace.
    *   It should also provide a "Log Out" button.
*   **Dashboard Page**:
    *   Create a basic "Dashboard" page as the default page after login. This can be a simple placeholder for now.

## 3. Final Deliverables

The final result should be a fully functional frontend application with a complete authentication flow and a basic UI shell, ready for feature development. Ensure the code is clean, well-commented, and follows best practices for React and TypeScript development.
