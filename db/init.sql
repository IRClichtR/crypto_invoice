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

CREATE TYPE event_type AS ENUM (
    'login',
    'failedlogin',
    'walletconnected',
    'walletdisconnected',
    'passwordchanged',
    'accountlocked',
    'accountunlocked'
);

-- CREATE TYPE dispute_decision AS ENUM (
--     'accepted',
--     'rejected'
-- );

-- CREATE TYPE dispute_reason AS ENUM (
--     'fraud',
--     'non_payment',
--     'other'
-- );

-- CREATE TYPE tx_type AS ENUM (
--     'invoice_payment',
--     'invoice_dispute',
--     'invoice_dispute_resolution'
-- );


CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    ethereum_address VARCHAR(42) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    is_admin BOOLEAN DEFAULT FALSE, 
    is_verified BOOLEAN DEFAULT FALSE,
    metadata JSONB DEFAULT '{}'::JSONB
);

CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY,
    on_chain_id VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    amount NUMERIC(20, 8) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    due_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status invoice_status DEFAULT 'pending',
    created_by UUID REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS auth_challenges (
    id UUID PRIMARY KEY,
    ethereum_address VARCHAR(42) NOT NULL,
    challenge VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS security_events (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    event_type event_type NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    client_ip INET,
    user_agent VARCHAR(255),
    metadata JSONB DEFAULT '{}'::JSONB
);

CREATE TABLE IF NOT EXISTS token_blacklist (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    jti VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    issued_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    blacklisted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    reason VARCHAR(255) NOT NULL
);