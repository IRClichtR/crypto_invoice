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
    'challengecreated',
    'challengeused',
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
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE, 
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB
);

CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY,
    on_chain_id VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    due_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status invoice_status DEFAULT 'pending',
    created_by UUID REFERENCES users(id),
    secure_url VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS auth_challenges (
    id UUID PRIMARY KEY,
    ethereum_address VARCHAR(42) NOT NULL,
    nonce VARCHAR(255) NOT NULL,
    challenge_message VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    domain VARCHAR(255) NOT NULL,
    chal_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS security_events (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
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

CREATE TABLE IF NOT EXISTS rate_limits (
    id UUID PRIMARY KEY,
    identifier VARCHAR(255) NOT NULL,
    action_type VARCHAR(100) NOT NULL,
    attempts_count INTEGER NOT NULL DEFAULT 0,
    window_start TIMESTAMP NOT NULL,
    last_attempt TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_rate_limits_identifier_action ON rate_limits(identifier, action_type);
CREATE INDEX idx_rate_limits_window_start ON rate_limits(window_start);
CREATE INDEX idx_rate_limits_last_attempt ON rate_limits(last_attempt);