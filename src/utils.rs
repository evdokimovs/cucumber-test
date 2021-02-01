use serde_json::Value as Json;

use crate::entity::Entity;

pub struct JsExecutable {
    pub expression: String,
    pub args: Vec<Json>,
    pub objs: Vec<String>,
}

impl JsExecutable {
    pub fn new(expression: &str, args: Vec<Json>) -> Self {
        Self {
            expression: expression.to_string(),
            args,
            objs: Vec::new(),
        }
    }

    pub fn with_objs<T>(
        expression: &str,
        args: Vec<Json>,
        objs: Vec<&Entity<T>>,
    ) -> Self {
        Self {
            expression: expression.to_string(),
            args,
            objs: objs.into_iter().map(|o| o.id.clone()).collect(),
        }
    }

    pub fn get_js_for_objs(&self) -> String {
        let mut objs = String::new();
        objs.push_str("let objs = [];");
        for obj in &self.objs {
            objs.push_str(&format!(
                "objs.push(window.holders.get('{}'));",
                obj
            ));
        }

        objs
    }
}
