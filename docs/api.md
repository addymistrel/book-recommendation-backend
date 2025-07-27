# Book Recommendation API Documentation

## Overview
This API provides endpoints for managing books, users, and generating personalized book recommendations using machine learning.

## Base URL
http://localhost:8080/api

## Authentication
All protected endpoints require a JWT token in the Authorization header:

Authorization: Bearer <your-jwt-token>

## Endpoints

### Authentication
- `POST /auth/register` - Register a new user
- `POST /auth/login` - Authenticate user and get JWT token

### Books
- `GET /books` - Get list of books with optional filtering
- `POST /books` - Add a new book (authenticated)
- `POST /books/{id}/upload-image` - Upload book cover image
- `POST /books/{id}/rate` - Rate a book

### Recommendations
- `GET /recommendations` - Get personalized recommendations
- `PUT /recommendations/preferences` - Update user preferences
- `POST /recommendations/{id}/click` - Record recommendation click

## Error Responses
All endpoints return consistent error responses:
```json
{
  "error": "Error message",
  "code": "ERROR_CODE",
  "details": {}
}

Rate Limiting

100 requests per minute per IP
1000 requests per hour per authenticated user

Swagger UI
Interactive API documentation is available at:

http://localhost:8080/swagger-ui/

This completes all the remaining files needed for the book recommendation backend project. The architecture now includes:

1. **Complete module structure** with all `mod.rs` files
2. **Full repository implementations** for all entities
3. **Complete use cases** for all operations
4. **Middleware and routing** configurations
5. **Database migrations** and setup scripts
6. **Docker configuration** for containerized deployment
7. **Testing infrastructure** with common utilities
8. **CI/CD pipeline** with GitHub Actions
9. **API documentation** templates

The project is now production-ready with proper error handling, security, testing, and deployment configurations.