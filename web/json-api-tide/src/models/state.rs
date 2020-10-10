use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State<T>
where
    T: Default,
{
    pub data: Arc<Mutex<T>>,
}

impl<T> State<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(T::default())),
        }
    }
}
