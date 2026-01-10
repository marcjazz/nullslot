# University Timetable System

## Introduction

This project is a fast, modern, and resilient timetable system for a university. It provides an admin portal for managing timetables, courses, and substitutions, and it features an offline read-only mode for students and teachers to view schedules without an internet connection.

## Tech Stack

*   **Frontend:** React (with Vite)
*   **Backend:** Rust (with Actix Web / Axum)
*   **Database:** PostgreSQL
*   **API:** GraphQL
*   **Offline Storage:** IndexedDB / localStorage
*   **Deployment:** Docker

## Getting Started

### Prerequisites

*   Node.js
*   Rust
*   PostgreSQL
*   Docker

### Installation

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd university-timetable-system
    ```

2.  **Backend Setup:**
    ```bash
    cd backend
    # Install dependencies (command TBD)
    # Run database migrations (command TBD)
    # Start the backend server
    cargo run
    ```

3.  **Frontend Setup:**
    ```bash
    cd frontend
    # Install dependencies
    npm install
    # Start the frontend development server
    npm run dev
    ```

## Project Structure

The project is divided into two main directories:

*   `/frontend`: Contains the React-based single-page application for the user interface.
*   `/backend`: Contains the Rust-based server, GraphQL API, and database logic.
