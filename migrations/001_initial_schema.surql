-- Create users table
DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD id ON users TYPE record<users>;
DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
DEFINE FIELD username ON users TYPE string;
DEFINE FIELD password_hash ON users TYPE string;
DEFINE FIELD first_name ON users TYPE string;
DEFINE FIELD last_name ON users TYPE string;
DEFINE FIELD preferences ON users TYPE array<string>;
DEFINE FIELD created_at ON users TYPE datetime;
DEFINE FIELD updated_at ON users TYPE datetime;
DEFINE FIELD is_active ON users TYPE bool DEFAULT true;

-- Create unique indexes
DEFINE INDEX unique_email ON users COLUMNS email UNIQUE;
DEFINE INDEX unique_username ON users COLUMNS username UNIQUE;

-- Create books table
DEFINE TABLE books SCHEMAFULL;
DEFINE FIELD id ON books TYPE record<books>;
DEFINE FIELD title ON books TYPE string;
DEFINE FIELD author ON books TYPE string;
DEFINE FIELD isbn ON books TYPE option<string>;
DEFINE FIELD description ON books TYPE string;
DEFINE FIELD genre ON books TYPE array<string>;
DEFINE FIELD tags ON books TYPE array<string>;
DEFINE FIELD publication_year ON books TYPE int;
DEFINE FIELD publisher ON books TYPE string;
DEFINE FIELD language ON books TYPE string;
DEFINE FIELD page_count ON books TYPE int;
DEFINE FIELD cover_image_url ON books TYPE option<string>;
DEFINE FIELD average_rating ON books TYPE float DEFAULT 0.0;
DEFINE FIELD ratings_count ON books TYPE int DEFAULT 0;
DEFINE FIELD created_at ON books TYPE datetime;
DEFINE FIELD updated_at ON books TYPE datetime;
DEFINE FIELD created_by ON books TYPE record<users>;

-- Create indexes for books
DEFINE INDEX books_title ON books COLUMNS title;
DEFINE INDEX books_author ON books COLUMNS author;
DEFINE INDEX books_genre ON books COLUMNS genre;
DEFINE INDEX books_rating ON books COLUMNS average_rating;

-- Create book_ratings table
DEFINE TABLE book_ratings SCHEMAFULL;
DEFINE FIELD id ON book_ratings TYPE record<book_ratings>;
DEFINE FIELD book_id ON book_ratings TYPE record<books>;
DEFINE FIELD user_id ON book_ratings TYPE record<users>;
DEFINE FIELD rating ON book_ratings TYPE float ASSERT $value >= 1.0 AND $value <= 5.0;
DEFINE FIELD review ON book_ratings TYPE option<string>;
DEFINE FIELD created_at ON book_ratings TYPE datetime;

-- Create unique constraint for user-book rating
DEFINE INDEX unique_user_book_rating ON book_ratings COLUMNS [user_id, book_id] UNIQUE;

-- Create recommendations table
DEFINE TABLE recommendations SCHEMAFULL;
DEFINE FIELD id ON recommendations TYPE record<recommendations>;
DEFINE FIELD user_id ON recommendations TYPE record<users>;
DEFINE FIELD book_id ON recommendations TYPE record<books>;
DEFINE FIELD score ON recommendations TYPE float;
DEFINE FIELD reason ON recommendations TYPE string;
DEFINE FIELD algorithm_version ON recommendations TYPE string;
DEFINE FIELD created_at ON recommendations TYPE datetime;
DEFINE FIELD is_clicked ON recommendations TYPE bool DEFAULT false;
DEFINE FIELD is_purchased ON recommendations TYPE bool DEFAULT false;

-- Create indexes for recommendations
DEFINE INDEX recommendations_user ON recommendations COLUMNS user_id;
DEFINE INDEX recommendations_score ON recommendations COLUMNS score;

-- Create user_preferences table
DEFINE TABLE user_preferences SCHEMAFULL;
DEFINE FIELD id ON user_preferences TYPE record<user_preferences>;
DEFINE FIELD user_id ON user_preferences TYPE record<users>;
DEFINE FIELD genre ON user_preferences TYPE string;
DEFINE FIELD preference_score ON user_preferences TYPE float ASSERT $value >= 0.0 AND $value <= 1.0;
DEFINE FIELD last_updated ON user_preferences TYPE datetime;

-- Create unique constraint for user-genre preference
DEFINE INDEX unique_user_genre_preference ON user_preferences COLUMNS [user_id, genre] UNIQUE;

-- Create reading_sessions table
DEFINE TABLE reading_sessions SCHEMAFULL;
DEFINE FIELD id ON reading_sessions TYPE record<reading_sessions>;
DEFINE FIELD user_id ON reading_sessions TYPE record<users>;
DEFINE FIELD book_id ON reading_sessions TYPE record<books>;
DEFINE FIELD start_time ON reading_sessions TYPE datetime;
DEFINE FIELD end_time ON reading_sessions TYPE option<datetime>;
DEFINE FIELD pages_read ON reading_sessions TYPE int DEFAULT 0;
DEFINE FIELD session_duration_minutes ON reading_sessions TYPE option<int>;

-- Create indexes for reading sessions
DEFINE INDEX reading_sessions_user ON reading_sessions COLUMNS user_id;
DEFINE INDEX reading_sessions_book ON reading_sessions COLUMNS book_id;