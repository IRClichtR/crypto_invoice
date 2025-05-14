CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; 

CREATE TYPE user_role AS ENUM (
    'emitter',
    'client',
    'arbitrator'
);

CREATE TYPE invoice_status AS ENUM (
    'pending',
    'paid',
    'disputed'
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    user_role user_role NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE, -- temporary field before we have a proper admin system
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS invoice (
    invoice_id UUID PRIMARY KEY,
    emitter_id UUID NOT NULL,
    client_id UUID NOT NULL,
    arbitrator_id UUID,
    amount NUMERIC(10, 2) NOT NULL,
    status invoice_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,    
    dispute_reason TEXT,
    dispute_decision TIMESTAMP DEFAULT NULL
);

