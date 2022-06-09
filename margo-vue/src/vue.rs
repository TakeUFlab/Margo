use js_sys::Map;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "vue")]
extern "C" {
    pub fn h(s: JsValue, p: JsValue, c: Box<[JsValue]>) -> JsValue;
}

#[derive(Default)]
pub struct H {
    s: JsValue,
    p: HashMap<String, JsValue>,
    c: Vec<JsValue>,
}

impl H {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn component(mut self, c: JsValue) -> Self {
        self.s = c;
        self
    }

    pub fn add_property<K, T>(mut self, key: K, c: T) -> Self
    where
        K: Into<String>,
        T: Into<JsValue>,
    {
        self.p.insert(key.into(), c.into());
        self
    }

    pub fn add_child<T>(mut self, c: T) -> Self
    where
        T: Into<JsValue>,
    {
        self.c.push(c.into());
        self
    }

    pub fn h(self) -> JsValue {
        h(
            self.s,
            self.p
                .into_iter()
                .fold(Map::new(), |acc, (key, value)| acc.set(&key.into(), &value))
                .into(),
            self.c.into_boxed_slice(),
        )
    }
}
