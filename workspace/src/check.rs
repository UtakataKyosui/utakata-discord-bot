use anyhow::Context;
use shuttle_runtime::SecretStore;

pub fn token_check(secret_store: &SecretStore) -> String {
    secret_store
        .get("DISCORD_TOKEN")
        .expect("DISCORD TOKEN Not Found")
}