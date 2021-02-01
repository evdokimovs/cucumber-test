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
                () => {
                    const [id] = arguments;

                    return { id: id };
                }
            "#,
            vec![self.id.into()],
        )
    }
}

impl Entity<Room> {
    pub async fn get_id(&mut self) -> String {
        self.execute(JsExecutable::with_objs(
            r#"
            (room) => {
                const [objRoom] = objs;

                return objRoom.id;
            }
        "#,
            vec![],
            vec![&self],
        ))
        .await
        .as_str()
        .unwrap()
        .to_string()
    }
}
