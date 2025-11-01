use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};

use crate::{api, Error};

pub fn run(api_client: &api::Client) -> Result<(), Error> {
    let agents = api_client.list_agents()?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Agent ID",
            "Host Name",
            "Created At",
            "Last Seen At",
            "Identity Public Key",
            "Public Prekey",
        ]);

    for agent in agents {
        let identity_public_key_base64 = base64::encode(agent.identity_public_key);
        let public_prekey = base64::encode(agent.public_prekey);

        table.add_row(vec![
            agent.id.to_string().as_str(),
            agent.host_name.to_string().as_str(),
            agent.created_at.to_string().as_str(),
            agent.last_seen_at.to_string().as_str(),
            identity_public_key_base64.as_str(),
            public_prekey.as_str(),
        ]);
    }

    println!("{}", table);

    Ok(())
}
