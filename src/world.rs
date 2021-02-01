use std::{collections::HashMap, convert::Infallible};

use async_trait::async_trait;
use cucumber_rust::{World, WorldInit};
use fantoccini::{Client, ClientBuilder};
use uuid::Uuid;

use crate::entity::{Builder, Entity, Room};

#[derive(WorldInit)]
pub struct BrowserWorld {
    entity_factory: EntityFactory,
    rooms: HashMap<String, Entity<Room>>,
}

impl BrowserWorld {
    pub async fn new(mut client: Client) -> Self {
        client
            .execute("window.holders = new Map();", vec![])
            .await
            .unwrap();
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
        let mut c = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .unwrap();
        c.goto("localhost:30000/index.html").await.unwrap();

        Ok(Self::new(c).await)
    }
}

struct EntityFactory(Client);

impl EntityFactory {
    pub async fn new_entity<T>(&mut self, obj: T) -> Entity<T>
    where
        T: Builder,
    {
        let js_build = obj.build();
        let id = Uuid::new_v4().to_string();

        self.0
            .execute(
                &format!(
                    "{}\nwindow.holders.set('{}', ({})());",
                    js_build.get_js_for_objs(),
                    id,
                    js_build.expression
                ),
                js_build.args,
            )
            .await
            .unwrap();

        Entity::new(id, self.0.clone())
    }
}
