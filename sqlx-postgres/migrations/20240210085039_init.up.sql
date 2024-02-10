-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS movies(
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(100) NOT NULL UNIQUE,
        description TEXT NOT NULL,
        genre TEXT[],
        actors TEXT[]
    );