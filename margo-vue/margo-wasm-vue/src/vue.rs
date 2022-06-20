use js_sys::{Object, Reflect};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "vue")]
extern "C" {
    pub fn h(s: JsValue, p: Object, c: &Closure<dyn Fn() -> Box<[JsValue]>>) -> JsValue;
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
        let obj = Object::new();
        for (k, v) in self.p {
            Reflect::set(&obj, &k.into(), &v).unwrap();
        }
        let f =
            Closure::wrap(Box::new(move || self.c.clone().into_boxed_slice())
                as Box<dyn Fn() -> Box<[JsValue]>>);
        let out = h(self.s, obj, &f);
        f.forget();
        out
    }
}
