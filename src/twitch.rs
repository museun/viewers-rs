pub fn get_viewers(client_id: &str, channel: &str) -> Option<u32> {
    const EP: &str = "https://api.twitch.tv/helix/streams?user_login=";

    let body = attohttpc::get(&[EP, channel].concat())
        .header("Client-ID", client_id)
        .send()
        .map_err(|err| {
            log::warn!("cannot get stream info for: {} -> {}", channel, err);
            err
        })
        .ok()?;

    #[derive(serde::Deserialize)]
    struct Data {
        data: Vec<Inner>,
    }

    #[derive(serde::Deserialize)]
    struct Inner {
        viewer_count: u32,
    }

    let resp = body
        .json::<Data>()
        .map_err(|err| {
            log::warn!(
                "cannot deserialize body for channel: {} -> {}",
                channel,
                err
            );
            err
        })
        .ok()?;

    resp.data.get(0).map(|v| v.viewer_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_viewer_count() {
        let api_key = crate::util::get_api_key(crate::util::CLIENT_ID_ENV_VAR);
        // this is always online
        assert!(get_viewers(&api_key, "rifftrax").expect("viewers") > 0);
        // this is always offline
        assert!(get_viewers(&api_key, "shaken_bot").is_none());
    }
}
