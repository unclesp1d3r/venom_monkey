table! {
    agents (id) {
        id -> Int4,
        machine_id -> Varchar,
        host_name -> Varchar,
        created_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        identity_public_key -> Bytea,
        public_prekey -> Bytea,
        public_prekey_signature -> Bytea,
    }
}

table! {
    jobs (id) {
        id -> Int8,
        encrypted_job -> Bytea,
        ephemeral_public_key -> Bytea,
        nonce -> Bytea,
        signature -> Bytea,
        encrypted_result -> Nullable<Bytea>,
        result_ephemeral_public_key -> Nullable<Bytea>,
        result_nonce -> Nullable<Bytea>,
        result_signature -> Nullable<Bytea>,
        agent_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

joinable!(jobs -> agents (agent_id));

allow_tables_to_appear_in_same_query!(
    agents,
    jobs,
    users,
);
