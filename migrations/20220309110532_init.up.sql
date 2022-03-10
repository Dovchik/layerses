CREATE EXTENSION "uuid-ossp";

CREATE TABLE users 
( 
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL
);
