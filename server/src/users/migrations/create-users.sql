CREATE TABLE users (
    id TEXT PRIMARY KEY,
    auth_type SMALLINT,
    oauth_id TEXT,
    name TEXT NOT NULL,
    email TEXT NOT NULL, -- don't allow users who don't provide an email
    password TEXT,
    created_on TIMESTAMP NOT NULL,
    CONSTRAINT email_unique UNIQUE (email)
);

CREATE INDEX idx_users_email ON users(email);

-- Create the Sessions table
CREATE TABLE user_sessions (
    token TEXT PRIMARY KEY,
    timeout BIGINT NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_on TIMESTAMP NOT NULL,
    oauth_token TEXT
);

-- Indexes for better performance
CREATE INDEX idx_sessions_user_id ON user_sessions(user_id);
