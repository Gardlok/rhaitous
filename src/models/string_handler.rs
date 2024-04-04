use crate::conduit::Conduit;
use crate::executor::EngineConfigurationStrategy;
use rhai::{Dynamic, Engine};

#[derive(Debug, Clone)]
pub struct StringHandler {
    pub content: String,
}

impl StringHandler {
    pub fn new(content: &str) -> Self {
        StringHandler {
            content: content.into(),
        }
    }

    pub fn append(&mut self, other: &str) {
        self.content.push_str(other);
    }

    pub fn prepend(&mut self, other: &str) {
        self.content = format!("{}{}", other, self.content);
    }
}

impl Conduit for StringHandler {
    fn create_from_dynamic(dynamic: Dynamic) -> Option<Self> {
        dynamic.try_cast::<Self>()
    }
}

pub struct StringHandlerConfiguration;

impl EngineConfigurationStrategy for StringHandlerConfiguration {
    fn configure_engine(&self, engine: &mut Engine) {
        engine
            .register_type_with_name::<StringHandler>("StringHandler")
            .register_fn("new_string", StringHandler::new)
            .register_fn("append", StringHandler::append)
            .register_fn("prepend", StringHandler::prepend)
            .register_get("content", |s: &mut StringHandler| s.content.clone());
    }
}
