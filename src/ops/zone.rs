use crate::api::zone::{Zone, ZoneSettings};
use crate::client::{CfResponse, CloudflareClient};
use crate::error::Result;
use serde::Serialize;

pub async fn list_zones(client: &CloudflareClient, status: Option<&str>) -> Result<Vec<Zone>> {
    let mut endpoint = "/zones".to_string();

    if let Some(s) = status {
        endpoint.push_str(&format!("?status={}", s));
    }

    let response: CfResponse<Vec<Zone>> = client.get(&endpoint).await?;
    Ok(response.result.unwrap_or_default())
}

pub async fn get_zone(client: &CloudflareClient, zone_identifier: &str) -> Result<Zone> {
    // Try zone ID first, then zone name
    let endpoint = if zone_identifier.len() == 32 {
        format!("/zones/{}", zone_identifier)
    } else {
        format!("/zones?name={}", zone_identifier)
    };

    let response: CfResponse<Zone> = client.get(&endpoint).await?;

    response
        .result
        .ok_or_else(|| crate::error::CfadError::not_found("Zone", zone_identifier))
}

pub async fn create_zone(client: &CloudflareClient, name: &str, account_id: &str) -> Result<Zone> {
    #[derive(Serialize)]
    struct CreateZone<'a> {
        name: &'a str,
        account: Account<'a>,
    }

    #[derive(Serialize)]
    struct Account<'a> {
        id: &'a str,
    }

    let create = CreateZone {
        name,
        account: Account { id: account_id },
    };

    let response: CfResponse<Zone> = client.post("/zones", create).await?;

    let zone = response
        .result
        .ok_or_else(|| crate::error::CfadError::api("No result returned from create zone"))?;

    println!("✓ Created zone: {}", zone.name);
    Ok(zone)
}

pub async fn delete_zone(client: &CloudflareClient, zone_id: &str) -> Result<()> {
    let endpoint = format!("/zones/{}", zone_id);
    let _response: CfResponse<serde_json::Value> = client.delete(&endpoint).await?;

    println!("✓ Deleted zone");
    Ok(())
}

pub async fn update_zone_settings(
    client: &CloudflareClient,
    zone_id: &str,
    settings: ZoneSettings,
) -> Result<()> {
    // Update individual settings
    if let Some(security_level) = settings.security_level {
        update_setting(client, zone_id, "security_level", &security_level).await?;
    }

    if let Some(cache_level) = settings.cache_level {
        update_setting(client, zone_id, "cache_level", &cache_level).await?;
    }

    if let Some(dev_mode) = settings.development_mode {
        let value = if dev_mode { "on" } else { "off" };
        update_setting(client, zone_id, "development_mode", &value).await?;
    }

    if let Some(ipv6) = settings.ipv6 {
        let value = if ipv6 { "on" } else { "off" };
        update_setting(client, zone_id, "ipv6", &value).await?;
    }

    if let Some(ssl) = settings.ssl {
        update_setting(client, zone_id, "ssl", &ssl).await?;
    }

    if let Some(always_https) = settings.always_use_https {
        let value = if always_https { "on" } else { "off" };
        update_setting(client, zone_id, "always_use_https", &value).await?;
    }

    println!("✓ Updated zone settings");
    Ok(())
}

async fn update_setting<T: Serialize>(
    client: &CloudflareClient,
    zone_id: &str,
    setting: &str,
    value: &T,
) -> Result<()> {
    let endpoint = format!("/zones/{}/settings/{}", zone_id, setting);

    #[derive(Serialize)]
    struct SettingValue<'a, T> {
        value: &'a T,
    }

    let payload = SettingValue { value };
    let _response: CfResponse<serde_json::Value> = client.patch(&endpoint, payload).await?;
    Ok(())
}
