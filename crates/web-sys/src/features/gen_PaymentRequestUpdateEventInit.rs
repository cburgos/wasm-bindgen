#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PaymentRequestUpdateEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentRequestUpdateEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub type PaymentRequestUpdateEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PaymentRequestUpdateEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &PaymentRequestUpdateEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PaymentRequestUpdateEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &PaymentRequestUpdateEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PaymentRequestUpdateEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &PaymentRequestUpdateEventInit, val: bool);
}
impl PaymentRequestUpdateEventInit {
    #[doc = "Construct a new `PaymentRequestUpdateEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
}
impl Default for PaymentRequestUpdateEventInit {
    fn default() -> Self {
        Self::new()
    }
}
