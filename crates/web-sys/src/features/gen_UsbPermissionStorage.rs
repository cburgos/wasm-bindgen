#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = USBPermissionStorage)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbPermissionStorage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbPermissionStorage;
    #[wasm_bindgen(method, setter = "allowedDevices")]
    fn allowed_devices_shim(this: &UsbPermissionStorage, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
impl UsbPermissionStorage {
    #[doc = "Construct a new `UsbPermissionStorage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allowed_devices(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.allowed_devices_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UsbPermissionStorage {
    fn default() -> Self {
        Self::new()
    }
}
