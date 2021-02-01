use std::{cell::RefCell, convert::Infallible};
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{dev::Server, web, App, HttpRequest, HttpServer};
use async_trait::async_trait;
use cucumber_rust::{given, then, when, World, WorldInit};
use fantoccini::{Client, ClientBuilder};
use serde_json::{Value as Json, Value};
use uuid::Uuid;

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
                    "window.holders.set('{}', ({})());",
                    id, js_build.expression
                ),
                js_build.args,
            )
            .await
            .unwrap();

        Entity::new(id, self.0.clone())
    }
}

struct JsExecutable {
    expression: String,
    args: Vec<Json>,
}

impl JsExecutable {
    pub fn new<T>(expression: &str, args: Vec<Json>) -> Self {
        Self {
            expression: expression.to_string(),
            args,
        }
    }
}

trait Builder {
    fn build(self) -> JsExecutable;
}

struct Room {
    id: String,
}

impl Builder for Room {
    fn build(self) -> JsExecutable {
        JsExecutable::new(
            r#"
                () => {
                    const [id] = arguments;

                    return { id: id };
                }
            "#,
            vec![self.id.into()],
        )
    }
}

#[derive(WorldInit)]
struct BrowserWorld {
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
            .new_entity(Room { id: id.to_string() })
            .await;
        self.rooms.insert(id.to_string(), room);
    }

    pub fn get_room(&mut self, id: &str) -> Option<&mut Entity<Room>> {
        self.rooms.get_mut(id)
    }
}

impl Entity<Room> {
    pub async fn get_id(&mut self) -> String {
        self.execute(JsExecutable::new(
            r#"
            (room) => {
                return room.id;
            }
        "#,
            vec![],
        ))
        .await
        .as_str()
        .unwrap()
        .to_string()
    }
}

struct Entity<T> {
    id: String,
    client: Client,
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
                    "return ({})(window.holders.get('{}'));",
                    js.expression, self.id
                ),
                js.args,
            )
            .await
            .unwrap()
    }
}

#[async_trait(?Send)]
impl World for BrowserWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let c = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .unwrap();

        Ok(Self::new(c).await)
    }
}

#[given(regex = "Room with ID '(.*)'")]
async fn given_room_with_id(world: &mut BrowserWorld, id: String) {
    world.create_room(&id).await;
}

#[then(regex = "Room with ID '(.*)' should exist in the BrowserWorld")]
async fn then_room_should_exist(world: &mut BrowserWorld, id: String) {
    let room = world.get_room(&id).unwrap();
    let js_id = room.get_id().await;
    assert_eq!(id, js_id);
}

#[tokio::main]
async fn main() {
    let runner = BrowserWorld::init(&["./features"]);
    runner.run_and_exit().await;
}
