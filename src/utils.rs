use fantoccini::{Client, ClientBuilder};
use serde_json::Value as Json;

use crate::entity::Entity;

#[derive(Clone, Debug)]
pub struct WebClient(Client);

impl WebClient {
    pub async fn new() -> Self {
        let mut c = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .unwrap();
        c.goto("localhost:30000/index.html").await.unwrap();

        Self(c)
    }

    pub async fn execute(&mut self, executable: JsExecutable) -> Json {
        let mut final_js = r#"
            let lastResult;
            let objs;
            let args;
        "#
        .to_string();
        let mut args = Vec::new();

        let mut executable = Some(Box::new(executable));
        while let Some(mut e) = executable.take() {
            final_js.push_str(&e.get_js());
            args.push(std::mem::take(&mut e.args).into());
            executable = e.pop();
        }
        final_js.push_str("return lastResult;\n");

        self.0.execute(&final_js, args).await.unwrap()
    }
}

pub struct JsExecutable {
    pub expression: String,
    pub args: Vec<Json>,
    pub objs: Vec<String>,
    pub and_then: Option<Box<JsExecutable>>,
    pub depth: u32,
}

impl JsExecutable {
    pub fn new(expression: &str, args: Vec<Json>) -> Self {
        Self {
            expression: expression.to_string(),
            args,
            objs: Vec::new(),
            and_then: None,
            depth: 0,
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
            and_then: None,
            depth: 0,
        }
    }

    pub fn and_then(mut self, mut another: Self) -> Self {
        if let Some(e) = self.and_then {
            self.and_then = Some(Box::new(e.and_then(another)));
            self
        } else {
            another.depth = self.depth + 1;
            self.and_then = Some(Box::new(another));
            self
        }
    }

    fn get_js_for_objs(&self) -> String {
        let mut objs = String::new();
        objs.push_str("objs = [];\n");
        for obj in &self.objs {
            objs.push_str(&format!(
                "objs.push(window.holders.get('{}'));\n",
                obj
            ));
        }

        objs
    }

    fn get_js(&self) -> String {
        let args = format!("args = arguments[{}];\n", self.depth);
        let objs = self.get_js_for_objs();
        let expr = format!("lastResult = ({})(lastResult);\n", self.expression);

        let mut out = String::new();
        out.push_str(&args);
        out.push_str(&objs);
        out.push_str(&expr);

        out
    }

    fn pop(self) -> Option<Box<Self>> {
        self.and_then
    }
}
