use protobuf::Message;
use async_nats::Client;
use crate::proto::account_access_validate::{ValidateAccountAccessToken, ValidateAccountAccessTokenResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn verify(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let mut request = ValidateAccountAccessToken::parse_from_bytes(&msg.payload)?;

    if let Some(reply) = msg.reply {

        // create the token
        let account_id = db.token_to_account(&request.token).await?;

        // Build and Send Response
        let mut resp = ValidateAccountAccessTokenResponse::new();
        resp.account_id = account_id;
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish(reply, encoded.into()).await?;
    }

    Ok(())
}