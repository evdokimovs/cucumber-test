mod room;

use std::marker::PhantomData;

use fantoccini::Client;
use serde_json::Value as Json;

use crate::utils::JsExecutable;

pub use self::room::Room;

pub struct Entity<T> {
    pub id: String,
    pub client: Client,
    _entity_type: PhantomData<T>,
}

impl<T> Entity<T> {
    pub fn new(uri: String, client: Client) -> Self {
        Self {
            id: uri,
            client,
            _entity_type: PhantomData::default(),
        }
    }

    async fn execute(&mut self, js: JsExecutable) -> Json {
        self.client
            .execute(
                &format!(
                    "{}\nreturn ({})(window.holders.get('{}'));",
                    js.get_js_for_objs(),
                    js.expression,
                    self.id
                ),
                js.args,
            )
            .await
            .unwrap()
    }
}

pub trait Builder {
    fn build(self) -> JsExecutable;
}
