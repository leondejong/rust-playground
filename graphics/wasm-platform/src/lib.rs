mod logic;
mod state;
mod wasm;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use gloo::console::log;
use gloo::utils::document;

use wasm::{handle_key, request_frame};

use state::{State, TITLE};

// npm install
// npm run serve

#[wasm_bindgen(start)]
async fn start() {
    log!(TITLE);
    document().set_title(TITLE);
    let state = Rc::new(RefCell::new(State::new()));
    futures::join!(
        request_frame(state.clone()),
        handle_key("keydown", state.clone()),
        handle_key("keyup", state.clone())
    );
}
