mod controllers;
mod models;

use controllers::crud::{add, all, info, one, remove, update};
use models::state::State;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let state = State::new();

    let mut app = tide::with_state(state);
    app.at("/").get(all);
    app.at("/list").get(all);
    app.at("/info").all(info);

    let mut item = app.at("/item");
    item.post(add);
    item.at("/:id").get(one).put(update).delete(remove);

    app.listen("127.0.0.1:8433").await?;

    Ok(())
}
