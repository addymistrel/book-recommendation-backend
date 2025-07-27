#!/bin/bash

# Book Recommendation Backend Setup Script

set -e

echo "🚀 Setting up Book Recommendation Backend..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first: https://rustup.rs/"
    exit 1
fi

# Check if SurrealDB is installed
if ! command -v surreal &> /dev/null; then
    echo "📦 Installing SurrealDB..."
    curl --proto '=https' --tlsv1.2 -sSf https://install.surrealdb.com | sh
fi

# Create necessary directories
echo "📁 Creating project directories..."
mkdir -p models/
mkdir -p python_models/
mkdir -p config/
mkdir -p logs/
mkdir -p uploads/

# Copy environment file
if [ ! -f .env ]; then
    echo "📝 Creating environment file..."
    cp .env.example .env
    echo "⚠️  Please update the .env file with your configuration!"
else
    echo "✅ Environment file already exists"
fi

# Install Python dependencies (for ML model)
if command -v python3 &> /dev/null; then
    echo "🐍 Installing Python dependencies..."
    pip3 install numpy pandas scikit-learn pickle5 2>/dev/null || echo "⚠️  Some Python packages may need manual installation"
fi

# Build the project
echo "🔨 Building the project..."
cargo build

# Run database migrations
echo "🗄️  Setting up database..."
if pgrep -f "surreal" > /dev/null; then
    echo "✅ SurrealDB is already running"
else
    echo "🚀 Starting SurrealDB..."
    surreal start --log trace --user root --pass root memory &
    SURREAL_PID=$!
    sleep 3
    
    # Run migrations
    surreal sql --conn http://localhost:8000 --user root --pass root --ns book_rec --db main migrations/001_initial_schema.surql
    
    echo "🎉 Database setup complete!"
fi

# Generate JWT secret if not exists
if grep -q "your-super-secret-jwt-key-change-in-production" .env; then
    JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || date | md5sum | cut -d' ' -f1)
    sed -i.bak "s/your-super-secret-jwt-key-change-in-production/$JWT_SECRET/g" .env
    echo "🔐 Generated JWT secret"
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Update your .env file with your Cloudinary credentials"
echo "2. Place your ML model file in the models/ directory"
echo "3. Run 'cargo run' to start the server"
echo "4. Visit http://localhost:8080/swagger-ui/ for API documentation"
echo ""