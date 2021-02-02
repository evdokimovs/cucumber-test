use crate::{entity::Entity, utils::JsExecutable};

use super::Builder;

pub struct Room {
    id: String,
}

impl Room {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Builder for Room {
    fn build(self) -> JsExecutable {
        JsExecutable::new(
            r#"
                async () => {
                    const [id] = args;

                    let jason = await window.getJason();
                    let room = await jason.init_room();
                    room.on_failed_local_stream(() => {});
                    room.on_connection_loss(() => {});

                    return room;
                }
            "#,
            vec![self.id.into()],
        )
    }
}

impl Entity<Room> {
    pub async fn wait_for_on_new_connection(&mut self) {
        self.execute_async(JsExecutable::new(
            r#"
                async (room) => {
                    let waitCallback = new Promise((resolve, reject) => {
                        room.on_new_connection(() => {
                            resolve();
                        });
                    });

                    await waitCallback;
                }
            "#,
            vec![],
        )).await;
    }

    pub async fn join(&mut self, uri: String) {
        self.execute_async(JsExecutable::new(
            r#"
                async (room) => {
                    const [uri] = args;
                    await room.join(uri);
                }
            "#,
            vec![uri.into()],
        )).await;
    }
}
