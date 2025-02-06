# rust4u

## Description
`rust4u` is an open-source portal where Rust developers can showcase and centralize their projects, seek community collaboration, and even post paid bounties for issues. Built entirely in Rust, it uses the Axum framework for the backend, SQLx for database operations, and blockchain integration for escrow-based bounties.

## Features
- **User Authentication** (JWT-based login & signup)
- **Project Listings** (publish, edit, delete projects)
- **Issue Bounty System** (fund issues and claim rewards)
- **Blockchain Escrow Integration** (secure fund transfers for bounties)
- **Database Migrations using SQLx**
- **Fully Containerized with Docker**

---

## Tech Stack
- **Language:** Rust
- **Web Framework:** Axum
- **Database:** PostgreSQL
- **ORM / Querying:** SQLx
- **Authentication:** JWT
- **Blockchain:** Substrate / Ink! (optional)
- **Containerization:** Docker / Docker Compose

---

## Setup & Installation

### 1. Install Rust (if not installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### 2. Clone the Repository
```bash
git clone https://github.com/YOUR-USERNAME/rust4u.git
cd rust4u
```

### 3. Install Dependencies
```bash
cargo build
```

### 4. Set Up PostgreSQL Database
**Using Docker:**
```bash
docker run -d \
  --name rust4u-postgres \
  -e POSTGRES_USER=rust4u \
  -e POSTGRES_PASSWORD=rust4u \
  -e POSTGRES_DB=rust4u \
  -p 5432:5432 \
  postgres:15
```

### 5. Set Environment Variables
Copy `.env.example` and configure `.env`:
```bash
cp .env.example .env
```
Example `.env`:
```
DATABASE_URL=postgres://rust4u:rust4u@localhost:5432/rust4u
JWT_SECRET=your_super_secret_key
JWT_EXPIRATION_HOURS=24
```

### 6. Run Database Migrations
```bash
cargo install sqlx-cli
sqlx migrate run
```

### 7. Start the Server
```bash
cargo run
```
By default, the API will be accessible at `http://localhost:3000`.

---

## Running with Docker
You can also run the entire stack using **Docker Compose**:

```yaml
version: '3.8'
services:
  db:
    image: postgres:15
    container_name: rust4u-postgres
    environment:
      - POSTGRES_USER=rust4u
      - POSTGRES_PASSWORD=rust4u
      - POSTGRES_DB=rust4u
    ports:
      - '5432:5432'

  app:
    build: .
    container_name: rust4u-app
    depends_on:
      - db
    environment:
      - DATABASE_URL=postgres://rust4u:rust4u@db:5432/rust4u
      - JWT_SECRET=supersecretkey
    ports:
      - '3000:3000'
    command: ["bash", "-c", "sqlx migrate run && ./rust4u-backend"]
```

To start the services:
```bash
docker-compose up --build
```

---

## API Overview

### **User Authentication**
- **POST** `/users` → Register a user
- **POST** `/users/login` → Login and receive JWT
- **GET** `/users/:id` → Get user profile (requires auth)
- **PUT** `/users/:id` → Update user details
- **DELETE** `/users/:id` → Remove user

### **Project Management**
- **POST** `/projects` → Create a new project
- **GET** `/projects` → List all projects
- **GET** `/projects/:id` → Get project details
- **PUT** `/projects/:id` → Update project
- **DELETE** `/projects/:id` → Delete project

### **Issue Bounty System**
- **POST** `/issues` → Create an issue with an optional bounty
- **PUT** `/issues/:id/resolve` → Mark an issue as resolved and release funds
- **GET** `/issues` → List all open issues

---

## Testing
Run unit and integration tests with:
```bash
cargo test
```

---

## Contributing
1. Fork the repo
2. Create a branch: `git checkout -b feature/my-feature`
3. Commit changes: `git commit -m 'Add new feature'`
4. Push to branch: `git push origin feature/my-feature`
5. Open a pull request

---

## License
This project is licensed under the **MIT License**.

---

## Tags
```
#rust #axum #sqlx #postgresql #blockchain #web3 #open-source #docker #backend
```
