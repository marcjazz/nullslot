Perfect — let’s map out a **fast, modern, resilient timetable system** with **offline read-only mode**. I’ll break it down visually and explain each piece. 🗺️✨

---

# 🏗 University Timetable System Architecture (Fast + Modern + Offline Capable)

```
              ┌────────────────────┐
              │   Admin Portal     │
              │  (Web SPA, React)  │
              └─────────┬─────────┘
                        │
                        ▼
          ┌───────────────────────────┐
          │  GraphQL API       │
          │  Rust Backend (Actix)│
          │ - Auth & Roles            │
          │ - Timetable CRUD          │
          │ - Substitution workflow   │
          │ - Notifications           │
          └─────────┬─────────────────┘
                    │
      ┌─────────────┴─────────────┐
      ▼                           ▼
┌──────────────┐           ┌──────────────┐
│ PostgreSQL   │           │ Notification │
│  (Primary DB)│           │ Service      │
│ - Timetables │           │ - Email      │
│ - Users      │           │ - Calendar   │
│ - Substitutions│
└──────────────┘           └──────────────┘
       │
       ▼
┌─────────────────────────────┐
│  Offline Cache / IndexedDB  │
│  (Read-only mode)           │
│ - Latest timetable snapshot │
│ - Viewable without internet │
└─────────────────────────────┘
```

---

## 🧩 Component Details

### 1️⃣ **Frontend (SPA)**

* Modern SPA: React - Vite
* Features:

  * Drag-and-drop timetable editor (for admins)
  * Substitution alerts
  * Offline read-only cache:

    * Teachers/students can view the latest schedule without internet
    * Stored in **IndexedDB / localStorage**
  * Real-time conflict highlighting

---

### 2️⃣ **Backend (Rust)**

* **Rust + Actix Web / Axum**
* Handles:

  * CRUD for courses, rooms, teachers, time slots
  * Substitution workflow:

    1. Admin marks a class needing coverage
    2. System suggests available teachers
    3. Teacher accepts → notifications sent
    4. Update applied in DB
  * Role-based access (admin vs teacher vs student)
  * API to serve **offline snapshots** for SPA

---

### 3️⃣ **Database (PostgreSQL)**

* Single source of truth
* Tables:

  * Users (teachers, admins, students)
  * Courses & Rooms
  * Timetable entries
  * Substitution logs
  * Notifications history
* Optional read replica if load grows

---

### 4️⃣ **Notification Service**

* Sends **email + calendar invites** on substitution or schedule changes
* Optional SMS API for urgent alerts
* Can queue retries if offline temporarily

---

### 5️⃣ **Offline Mode**

* **Frontend stores read-only timetable snapshots** in **IndexedDB**
* Users can:

  * Browse schedules
  * Search courses and rooms
  * See the last known substitution state
* Admins / teachers cannot edit offline (read-only)
* Sync automatically when online

---

### 6️⃣ **Deployment & Performance**

* Dockerized backend + frontend
* Fast Rust backend handles real-time requests efficiently
* SPA frontend is static-host friendly (Vercel, Netlify, Fly.io)
* Optional caching layer (Redis) for real-time substitution checks

---

### ✅ Further Enhancements

* **WebSockets** for instant timetable updates
* **Push notifications** for mobile / desktop
* **Audit logs** for accountability
* **Multi-tenant support** if the university has multiple departments