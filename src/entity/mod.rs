mod room;

use std::marker::PhantomData;

use serde_json::Value as Json;

use crate::utils::{JsExecutable, WebClient};

pub use self::room::Room;

pub struct Entity<T> {
    id: String,
    client: WebClient,
    _entity_type: PhantomData<T>,
}

impl<T> Entity<T> {
    pub fn new(uri: String, client: WebClient) -> Self {
        Self {
            id: uri,
            client,
            _entity_type: PhantomData::default(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    async fn execute(&mut self, js: JsExecutable) -> Json {
        self.client
            .execute_async(
                JsExecutable::new(
                    r#"
                    async () => {
                        const [id] = args;
                        return window.holders.get(id);
                    }
                "#,
                    vec![self.id.clone().into()],
                )
                .and_then(js),
            )
            .await
            .unwrap()
    }

    async fn execute_async(&mut self, js: JsExecutable) -> Json {
        self.client
            .execute_async(
                JsExecutable::new(
                    r#"
                    async () => {
                        const [id] = args;
                        return window.holders.get(id);
                    }
                "#,
                    vec![self.id.clone().into()],
                )
                .and_then(js),
            )
            .await
            .unwrap()
    }
}

pub trait Builder {
    fn build(self) -> JsExecutable;
}
