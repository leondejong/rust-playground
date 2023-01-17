mod controllers;
mod models;
mod utils;

use controllers::{add, all, one, remove, update};
use models::State;

// http http://127.0.0.1:8433
// http http://127.0.0.1:8433/1
// http post http://127.0.0.1:8433 name=name content=content active:=true
// http put http://127.0.0.1:8433/1 name=name content=content active:=true
// http delete http://127.0.0.1:8433/1

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let state = State::new();

    let mut app = tide::with_state(state);

    app.at("/").get(all).post(add);
    app.at("/:id").get(one).put(update).delete(remove);

    app.listen("127.0.0.1:8433").await?;

    Ok(())
}
