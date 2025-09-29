CREATE TABLE jobs (
    id BIGSERIAL PRIMARY KEY,
    encrypted_job BYTEA NOT NULL,
    ephemeral_public_key BYTEA NOT NULL,
    nonce BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    encrypted_result BYTEA,
    result_ephemeral_public_key BYTEA,
    result_nonce BYTEA,
    result_signature BYTEA,
    agent_id INT NOT NULL REFERENCES agents(id) ON DELETE CASCADE
);
