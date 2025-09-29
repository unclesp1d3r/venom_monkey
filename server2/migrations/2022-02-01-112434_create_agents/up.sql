CREATE TABLE agents (
    id SERIAL PRIMARY KEY,
    machine_id varchar(255) NOT NULL UNIQUE,
    host_name varchar(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_seen_at TIMESTAMP WITH TIME ZONE NOT NULL,
    identity_public_key BYTEA NOT NULL,
    public_prekey BYTEA NOT NULL,
    public_prekey_signature BYTEA NOT NULL
);
