mod room;

use std::marker::PhantomData;

use serde_json::Value as Json;

use crate::utils::{JsExecutable, WebClient};

pub use self::room::Room;

pub struct Entity<T> {
    pub id: String,
    pub client: WebClient,
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

    async fn execute(&mut self, js: JsExecutable) -> Json {
        self.client
            .execute(
                JsExecutable::new(
                    r#"
                    () => {
                        const [id] = args;
                        return window.holders.get(id);
                    }
                "#,
                    vec![self.id.clone().into()],
                )
                .and_then(js),
            )
            .await
    }

    async fn execute_async(&mut self, js: JsExecutable) -> Json {
        self.client
            .execute_async(
                JsExecutable::new(
                    r#"
                    () => {
                        const [id] = args;
                        return window.holders.get(id);
                    }
                "#,
                    vec![self.id.clone().into()],
                )
                .and_then(js),
            )
            .await
    }
}

pub trait Builder {
    fn build(self) -> JsExecutable;
}
