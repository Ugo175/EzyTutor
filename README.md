# EzyTutor - Online Tutoring Platform

EzyTutor is a comprehensive online tutoring platform built with Rust (backend) and React (frontend), featuring a complete REST API with JWT authentication, role-based access control, and comprehensive business logic.

## 🚀 Project Status

**Backend: ✅ COMPLETE** - Fully implemented and tested
- ✅ Complete REST API with 15+ endpoints
- ✅ JWT authentication and role-based access control
- ✅ PostgreSQL database with migrations
- ✅ Comprehensive business logic and validation
- ✅ Error handling and logging
- ✅ API testing suite included

**Frontend: 🚧 IN PROGRESS** - React components implemented
**Mobile: 📋 PLANNED** - React Native applications

## Architecture

The application follows a layered architecture with:

- **Rust Web Services**: Course and Tutor APIs built with Actix-web
- **Application Modules**: User authentication, templates, error handling, database access
- **Infrastructure**: PostgreSQL database, logging, configuration management
- **Frontend**: React web application with modern UI components
- **Mobile Support**: React Native mobile applications (iOS/Android)

## Features

### Core Features
- **User Management**: Registration, authentication, and role-based access (Student, Tutor, Admin)
- **Course Management**: Create, update, and manage tutoring courses
- **Tutor Profiles**: Comprehensive tutor profiles with ratings and reviews
- **Search & Discovery**: Find tutors and courses by category, difficulty, and rating
- **Review System**: Student reviews and ratings for tutors

### Technical Features
- JWT-based authentication
- RESTful API design
- Database migrations with SQLx
- Input validation and error handling
- CORS support for web clients
- Structured logging

## Getting Started

### Prerequisites
- Rust (latest stable version)
- PostgreSQL 12+
- Node.js 18+ (for frontend)
- npm or yarn

### Backend Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd EzyTutor
   ```

2. **Install dependencies**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install PostgreSQL (macOS)
   brew install postgresql
   brew services start postgresql
   ```

3. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials and JWT secret
   ```

4. **Set up the database**
   ```bash
   # Create database
   createdb ezytutor
   
   # The migrations will run automatically when you start the server
   ```

5. **Run the backend**
   ```bash
   # Full server (requires PostgreSQL)
   cargo run --bin ezytutor
   
   # Or test server (no database required)
   cargo run --bin test_server
   ```

The API will be available at `http://localhost:8080`

### Quick Testing

Test the API without setting up a database:
```bash
# Start test server
cargo run --bin test_server

# Run basic tests
./test_simple.sh
```

For full API testing with database:
```bash
# Start full server
cargo run --bin ezytutor

# Run comprehensive tests
./test_api.sh
```

### API Endpoints

#### Public Endpoints
- `GET /api/v1/health` - Health check endpoint
- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - Login user and get JWT token
- `GET /api/v1/courses` - List all active courses
- `GET /api/v1/courses/{id}` - Get specific course details
- `GET /api/v1/tutors` - List all available tutors
- `GET /api/v1/tutors/{id}` - Get specific tutor profile
- `GET /api/v1/tutors/{id}/reviews` - Get reviews for a tutor
- `GET /api/v1/tutors/search?specialization=Math` - Search tutors by specialization

#### Protected Endpoints (Require JWT Token)

**Course Management (Tutors Only)**
- `POST /api/v1/courses` - Create a new course
- `PUT /api/v1/courses/{id}` - Update course details
- `DELETE /api/v1/courses/{id}` - Delete a course
- `GET /api/v1/my/courses` - Get tutor's own courses

**Tutor Profile Management**
- `POST /api/v1/tutors/profile` - Create tutor profile
- `PUT /api/v1/tutors/profile` - Update tutor profile

**Review System (Students Only)**
- `POST /api/v1/tutors/{id}/reviews` - Add review for a tutor

#### Authentication
All protected endpoints require a Bearer token in the Authorization header:
```
Authorization: Bearer <jwt_token>
```

## Database Schema

The application uses PostgreSQL with the following main tables:

- **`users`** - User accounts with email, password, role (student/tutor/admin)
- **`tutors`** - Tutor profiles with bio, specializations, hourly rates, ratings
- **`courses`** - Course information with title, description, price, difficulty level
- **`tutor_reviews`** - Student reviews and ratings (1-5 stars) for tutors

### Key Features:
- **Custom PostgreSQL enums** for user roles and difficulty levels
- **Automatic timestamps** with triggers for created_at/updated_at
- **Comprehensive indexes** for performance optimization
- **Foreign key constraints** ensuring data integrity
- **Rating aggregation** automatically calculated from reviews

## Development

### Running Tests
```bash
# Unit and integration tests
cargo test

# API endpoint testing (requires database)
./test_api.sh

# Basic health check testing (no database)
./test_simple.sh
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check compilation
cargo check
```

### Development Workflow
1. **Make changes** to source code
2. **Test locally** with `cargo run --bin test_server`
3. **Run tests** with `./test_simple.sh`
4. **Set up database** for full testing
5. **Run full tests** with `./test_api.sh`

## Deployment

The application can be deployed using Docker or directly on a server with PostgreSQL.

### Environment Variables
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT token signing
- `HOST` - Server host (default: 127.0.0.1)
- `PORT` - Server port (default: 8080)
- `RUST_LOG` - Logging level (default: info)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.