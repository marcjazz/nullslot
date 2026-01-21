# NullSlot Timetable System

NullSlot is a fast, modern, and resilient timetable management system designed for collaborative scheduling. It is built around the concept of **Workspaces**, allowing different teams or departments to manage their schedules in an isolated, multi-tenant environment.

For a detailed explanation of the system's design and data flow, please see [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## ✨ Features

-   **Workspace Management**: Create or join workspaces to manage schedules independently.
-   **Collaborative Scheduling**: Invite users to workspaces with role-based permissions (Owner, Editor, Viewer).
-   **Simplified Authentication**: Secure login using Google (OIDC) or passwordless Magic Links.
-   **Timetable Management**: A clear, coordinator-driven workflow for drafting, resolving conflicts, and publishing timetables.
-   **Offline-First Viewing**: Published timetables are cached on the client, allowing for read-only access without an internet connection.

---

## 🛠️ Tech Stack

| Component  | Technology                  |
| :--------- | :-------------------------- |
| **Frontend** | React (Vite), TypeScript, URQL |
| **Backend**  | Rust (Axum), GraphQL (async-graphql) |
| **Database** | PostgreSQL (with SQLx)      |
| **Deployment**| Docker                      |

---

## 🚀 Getting Started

### Prerequisites

-   [Node.js](https://nodejs.org/) (with `pnpm`)
-   [Rust](https://www.rust-lang.org/)
-   [Docker](https://www.docker.com/) and Docker Compose
-   [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)

### Installation & Running

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd nullslot
    ```

2.  **Set up Environment Variables:**
    *   Copy `.env.example` in the `backend` directory to `.env`.
    *   Fill in the required secrets (Google OIDC, database URL, etc.).

3.  **Launch the Database:**
    ```bash
    docker-compose up -d db
    ```

4.  **Run Backend Migrations:**
    ```bash
    cd backend
    sqlx database create
    sqlx migrate run
    cd ..
    ```

5.  **Run the Application:**
    You can run the frontend and backend separately or using the main Docker Compose file.

    *   **Docker (Recommended):**
        ```bash
        docker-compose up --build
        ```
        The app will be available at `http://localhost:5173`.

    *   **Locally:**
        -   **Backend**: `cd backend && cargo run`
        -   **Frontend**: `cd frontend && pnpm install && pnpm dev`

---

## Project Structure

The project is a monorepo containing two main packages:

-   `./frontend`: A React Single-Page Application that provides the user interface.
-   `./backend`: A Rust server that exposes the GraphQL API and handles all business logic.
