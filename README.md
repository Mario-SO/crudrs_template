# 🚀 crudrs_template: A Simple CRUD API with PostgreSQL made in Rust 🦀

## 📋 Table of Contents
-  [Features](#-features)
-  [Getting Started](#-getting-started)
-  [Running the Application](#-running-the-application)
-  [API Endpoints](#-api-endpoints)
-  [Contributing](#-contributing)
-  [License](#-license)

## ✨ Features
-  📦 Containerized with Docker
-  🗄️ Uses PostgreSQL for data storage
-  🔄 Basic CRUD operations
-  🦀 Built with Rust for speed and safety

## 🏁 Getting Started

### Prerequisites
Make sure you have the following installed:
-  🐋 [Docker](https://www.docker.com/)
-  🧳 [Docker Compose](https://docs.docker.com/compose/)
-  💻 [Rust](https://www.rust-lang.org/)

### Installation
Clone the repository:
```bash
git clone https://github.com/Mario-SO/crudrs_template.git
cd crudrs_template
```

## ▶️ Running the Application

### Using Docker Compose
Simply run the following command to start the application:
```bash
docker-compose up --build
```

This will build and start two containers:
-  🦀 `rustapp`: Your Rust application.
-  🗃️ `db`: A PostgreSQL database.

### Accessing the API
Once the containers are up, your API will be accessible at:
```
http://localhost:8080
```

## 🛠️ API Endpoints

### Create a User
```http
POST /users
```
**Request Body:**
```json
{
  "name": "John Doe",
  "email": "johndoe@example.com"
}
```

### Get All Users
```http
GET /users
```

### Get a User by ID
```http
GET /users/{id}
```

### Update a User
```http
PUT /users/{id}
```
**Request Body:**
```json
{
  "name": "John Doe",
  "email": "johndoe@example.com"
}
```

### Delete a User
```http
DELETE /users/{id}
```

---

Happy coding! 🎉✨