mod proto;
mod util;
mod handlers;
mod store;

use anyhow::Result;
use tokio::task::JoinSet;
use crate::handlers::create::create;
use crate::handlers::verify::verify;
use crate::store::Store;

#[tokio::main]
async fn main() -> Result<()> {

    // get the app name, used for group and such
    let app_name = match util::get_app_name() {
        Some(name) => name,
        None => { return Err(anyhow::anyhow!("Could not  determine application name.")); },
    };

    // Setup logging
    util::setup_logging(app_name.as_str());

    // connect to db
    let db = util::connect_to_database().await?;
    let store = Store::new(db.clone());

    // connect to nats
    let nc = util::connect_to_nats().await?;

    let mut set = JoinSet::new();

    let _nc = nc.clone();
    let _store = store.clone();
    set.spawn(async move {
        util::handle_requests(_nc, "accounts.access.create", move|_nc, msg| {
            create(_store.clone(), _nc, msg)
        }).await.expect("accounts.access.create");
    });

    let _nc = nc.clone();
    let _store = store.clone();
    set.spawn(async move {
        util::handle_requests(_nc, "accounts.access.verify", move|_nc, msg| {
            verify(_store.clone(), _nc, msg)
        }).await.expect("accounts.access.verify");
    });

    set.join_all().await;
    Ok(())
}
