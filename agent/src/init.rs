use crate::consts;
use crate::Error;
use common::api;
use std::{fs, path::PathBuf};
use uuid::Uuid;
pub fn init(api_client: &ureq::Agent) -> Result<Uuid, Error> {
    let saved_agent_id = get_saved_agent_id()?;

    let agent_id = match saved_agent_id {
        Some(agent_id) => agent_id,
        None => {
            let agent_id = register(api_client)?;
            save_agent_id(agent_id)?;
            agent_id
        }
    };

    Ok(agent_id)
}

pub fn register(api_client: &ureq::Agent) -> Result<Uuid, Error> {
    let register_agent_route = format!("{}/api/agents", consts::SERVER_URL);

    let api_res: api::Response<api::AgentRegistered> = api_client
        .post(register_agent_route.as_str())
        .call()?
        .into_json()?;

    let agent_id = match (api_res.data, api_res.error) {
        (Some(data), None) => Ok(data.id),
        (None, Some(err)) => Err(Error::Api(err.message)),
        (None, None) => Err(Error::Api(
            "Received invalid api response: data and error are both null.".to_string(),
        )),
        (Some(_), Some(_)) => Err(Error::Api(
            "Received invalid api response: data and error are both non null.".to_string(),
        )),
    }?;

    Ok(agent_id)
}

pub fn get_saved_agent_id() -> Result<Option<Uuid>, Error> {
    let agent_id_file = get_agent_id_file_path()?;

    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;

        let agent_id = Uuid::from_slice(&agent_file_content)?;
        Ok(Some(agent_id))
    } else {
        Ok(None)
    }
}

pub fn save_agent_id(agent_id: Uuid) -> Result<(), Error> {
    let agent_id_file = get_agent_id_file_path()?;
    fs::write(agent_id_file, agent_id.as_bytes())?;
    Ok(())
}

fn get_agent_id_file_path() -> Result<PathBuf, Error> {
    let mut cache_dir = match dirs::cache_dir() {
        Some(cache_dir) => cache_dir,
        None => return Err(Error::Internal("Error getting cache directory".to_string())),
    };

    cache_dir.push(consts::AGENT_ID_FILE);

    Ok(cache_dir)
}
