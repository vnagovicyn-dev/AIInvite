#[derive(Clone, Debug)]
pub struct AppState {
    pub service_name: String,
}

impl AppState {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}
