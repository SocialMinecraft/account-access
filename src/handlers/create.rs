use protobuf::Message;
use async_nats::Client;
use chrono::Duration;
use crate::proto::account_access_create::{CreateAccountAccessToken, CreateAccountAccessTokenResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn create(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let mut request = CreateAccountAccessToken::parse_from_bytes(&msg.payload)?;

    if let Some(reply) = msg.reply {

        // create the token
        let token = db.create_token(&request.account_id, Duration::minutes(15)).await?;
        // todo - should check account_id is 16chars
        // todo - duration should be an env var.

        // Build and Send Response
        let mut resp = CreateAccountAccessTokenResponse::new();
        resp.token = Some(token);
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish(reply, encoded.into()).await?;
    }

    Ok(())
}