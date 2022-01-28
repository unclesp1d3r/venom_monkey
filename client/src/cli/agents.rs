use prettytable::{cell, format, row, Table};

use crate::{api, Error};

pub fn run(api_client: &api::Client) -> Result<(), Error> {
    let agents = api_client.list_agents()?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    table.set_titles(row![bc=>
        "Agent ID",
        "Host Name",
        "Created At",
        "Last Seen At",
        "Identity Public Key",
        "Public Prekey"
    ]);

    for agent in agents {
        let identity_public_key_base64 = base64::encode(agent.identity_public_key);
        let public_prekey = base64::encode(agent.public_prekey);

        table.add_row(row![
            agent.id.to_string().as_str(),
            agent.host_name.to_string().as_str(),
            agent.created_at.to_string().as_str(),
            agent.last_seen_at.to_string().as_str(),
            identity_public_key_base64.as_str(),
            public_prekey.as_str()
        ]);
    }

    table.printstd();

    Ok(())
}
