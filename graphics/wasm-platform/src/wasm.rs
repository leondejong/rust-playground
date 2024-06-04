use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use futures::channel::mpsc;
use futures::stream::Stream;
use futures::StreamExt;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, EventTarget, HtmlCanvasElement, KeyboardEvent};

use gloo::events::EventListener;
use gloo::render::{request_animation_frame, AnimationFrame};
use gloo::utils::{document, window};

use crate::state::State;

pub trait Update {
    fn update(&mut self, delta: f64) -> ();
}

pub struct KeyHandler {
    receiver: mpsc::UnboundedReceiver<KeyboardEvent>,
    #[allow(unused)]
    listener: EventListener, // cancelled on drop
}

impl KeyHandler {
    pub fn new(kind: &str, target: &EventTarget) -> Self {
        let (sender, receiver) = mpsc::unbounded();

        let listener = EventListener::new(&target, kind.to_owned(), move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw();
            sender.unbounded_send(event.to_owned()).unwrap_throw();
        });

        Self { receiver, listener }
    }
}

impl Stream for KeyHandler {
    type Item = KeyboardEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

pub struct RequestAnimationFrame<T: Update> {
    animation_frame: Option<AnimationFrame>, // cancelled on drop
    state: Rc<RefCell<T>>,
}

impl<T: Update> RequestAnimationFrame<T> {
    pub fn new(state: Rc<RefCell<T>>) -> Self {
        Self {
            animation_frame: None,
            state,
        }
    }
    pub async fn start(&mut self) {
        let (sender, mut receiver) = mpsc::unbounded();

        sender.unbounded_send(0.0).unwrap_throw();

        loop {
            let send = sender.clone();

            self.animation_frame = Some(request_animation_frame(move |time| {
                let _ = send.unbounded_send(time);
            }));

            let time = receiver.next().await.unwrap_throw();

            self.state.clone().borrow_mut().update(time);
        }
    }
}

pub async fn request_frame(state: Rc<RefCell<State>>) {
    let mut frame = RequestAnimationFrame::new(state);
    frame.start().await;
}

pub async fn handle_key(kind: &str, state: Rc<RefCell<State>>) {
    let mut handler = KeyHandler::new(kind, &window());
    loop {
        let event = handler.next().await.unwrap_throw();
        state
            .clone()
            .borrow_mut()
            .events(&event.type_(), &event.key());
    }
}

pub fn setup() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

pub fn draw_rectangle(
    context: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: &str,
) {
    context.set_fill_style(&JsValue::from_str(color));
    context.fill_rect(x, y, width, height);
}

pub fn draw_text(
    context: &CanvasRenderingContext2d,
    text: &str,
    x: f64,
    y: f64,
    font: &str,
    color: &str,
) {
    context.set_font(font);
    context.set_fill_style(&JsValue::from_str(color));
    context.fill_text(text, x, y).unwrap();
}
