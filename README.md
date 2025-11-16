# # EzyTutor - Online Tutoring Platform

EzyTutor is a comprehensive online tutoring platform built with Rust (backend) and React (frontend), following a modern microservices architecture.

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

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials and JWT secret
   ```

3. **Set up the database**
   ```bash
   # Create database
   createdb ezytutor
   
   # Run migrations
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. **Run the backend**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:8080`

### API Endpoints

#### Authentication
- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - Login user

#### Courses
- `GET /api/v1/courses` - List all courses
- `POST /api/v1/courses` - Create a new course (tutor only)
- `GET /api/v1/courses/{id}` - Get course details
- `PUT /api/v1/courses/{id}` - Update course (tutor only)
- `DELETE /api/v1/courses/{id}` - Delete course (tutor only)

#### Tutors
- `GET /api/v1/tutors` - List all tutors
- `POST /api/v1/tutors/profile` - Create tutor profile
- `PUT /api/v1/tutors/profile` - Update tutor profile
- `GET /api/v1/tutors/{id}` - Get tutor details
- `POST /api/v1/tutors/{id}/reviews` - Add review for tutor

#### Health Check
- `GET /api/v1/health` - Health check endpoint

## Database Schema

The application uses PostgreSQL with the following main tables:
- `users` - User accounts and authentication
- `tutors` - Tutor profiles and specializations
- `courses` - Course information and pricing
- `tutor_reviews` - Reviews and ratings for tutors

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

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