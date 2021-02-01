use std::{collections::HashMap, convert::Infallible};

use async_trait::async_trait;
use cucumber_rust::{World, WorldInit};
use uuid::Uuid;

use crate::{
    entity::{Builder, Entity, Room},
    utils::{JsExecutable, WebClient},
};

#[derive(WorldInit)]
pub struct BrowserWorld {
    entity_factory: EntityFactory,
    rooms: HashMap<String, Entity<Room>>,
}

impl BrowserWorld {
    pub async fn new(mut client: WebClient) -> Self {
        client
            .execute(JsExecutable::new(
                r#"
                () => {
                    window.holders = new Map();
                }
            "#,
                vec![],
            ))
            .await;
        Self {
            entity_factory: EntityFactory(client),
            rooms: HashMap::new(),
        }
    }

    pub async fn create_room(&mut self, id: &str) {
        let room = self
            .entity_factory
            .new_entity(Room::new(id.to_string()))
            .await;
        self.rooms.insert(id.to_string(), room);
    }

    pub fn get_room(&mut self, id: &str) -> Option<&mut Entity<Room>> {
        self.rooms.get_mut(id)
    }
}

#[async_trait(?Send)]
impl World for BrowserWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::new(WebClient::new().await).await)
    }
}

struct EntityFactory(WebClient);

impl EntityFactory {
    pub async fn new_entity<T>(&mut self, obj: T) -> Entity<T>
    where
        T: Builder,
    {
        let id = Uuid::new_v4().to_string();
        self.0
            .execute(obj.build().and_then(JsExecutable::new(
                r#"
                    (obj) => {
                        const [id] = args;
                        window.holders.set(id, obj);
                    }
                "#,
                vec![id.clone().into()],
            )))
            .await;

        Entity::new(id, self.0.clone())
    }
}
