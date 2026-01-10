use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use anyhow::{Context, Result};
use crate::config::Config;

pub type OidcClient = CoreClient;

pub async fn discover_oidc_client(config: &Config) -> Result<OidcClient> {

    let issuer_url = IssuerUrl::new(config.oidc_issuer_url.clone())
        .context("Invalid issuer URL")?;

    let provider_metadata = CoreProviderMetadata::discover_async(
        issuer_url,
        openidconnect::reqwest::async_http_client,
    )
    .await
    .context("Failed to discover OIDC provider")?;

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(config.oidc_client_id.clone()),
        Some(ClientSecret::new(config.oidc_client_secret.clone())),
    )
    .set_redirect_uri(
        RedirectUrl::new(config.oidc_redirect_uri.clone())
            .context("Invalid redirect URI")?
    );

    Ok(client)
}
