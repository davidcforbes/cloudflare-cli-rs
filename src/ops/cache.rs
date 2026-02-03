use crate::api::cache::{PurgeAll, PurgeFiles, PurgeHosts, PurgePrefixes, PurgeTags};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;

pub async fn purge_all(client: &CloudflareClient, zone_id: &str) -> Result<()> {
    let endpoint = format!("/zones/{}/purge_cache", zone_id);

    let payload = PurgeAll {
        purge_everything: true,
    };

    let _response: CfResponse<serde_json::Value> = client.post(&endpoint, payload).await?;

    println!("✓ Purged all cache for zone");
    Ok(())
}

pub async fn purge_files(
    client: &CloudflareClient,
    zone_id: &str,
    urls: Vec<String>,
) -> Result<()> {
    let endpoint = format!("/zones/{}/purge_cache", zone_id);

    let payload = PurgeFiles { files: urls };

    let _response: CfResponse<serde_json::Value> = client.post(&endpoint, &payload).await?;

    println!("✓ Purged {} URLs from cache", payload.files.len());
    Ok(())
}

pub async fn purge_tags(client: &CloudflareClient, zone_id: &str, tags: Vec<String>) -> Result<()> {
    let endpoint = format!("/zones/{}/purge_cache", zone_id);

    let payload = PurgeTags { tags };

    let _response: CfResponse<serde_json::Value> = client.post(&endpoint, &payload).await?;

    println!("✓ Purged cache by tags");
    Ok(())
}

pub async fn purge_hosts(
    client: &CloudflareClient,
    zone_id: &str,
    hosts: Vec<String>,
) -> Result<()> {
    let endpoint = format!("/zones/{}/purge_cache", zone_id);

    let payload = PurgeHosts { hosts };

    let _response: CfResponse<serde_json::Value> = client.post(&endpoint, &payload).await?;

    println!("✓ Purged cache by hosts");
    Ok(())
}

pub async fn purge_prefixes(
    client: &CloudflareClient,
    zone_id: &str,
    prefixes: Vec<String>,
) -> Result<()> {
    let endpoint = format!("/zones/{}/purge_cache", zone_id);

    let payload = PurgePrefixes { prefixes };

    let _response: CfResponse<serde_json::Value> = client.post(&endpoint, &payload).await?;

    println!("✓ Purged cache by prefixes");
    Ok(())
}
