use wasm_bindgen::prelude::*;
use std::io::Cursor;

#[wasm_bindgen]
pub fn binary_to_xml(binary: &[u8]) -> Result<String, JsValue> {
    let dom = rbx_binary::from_reader(Cursor::new(binary))
        .map_err(|e| JsValue::from_str(&format!("Failed to parse binary: {}", e)))?;
    
    let mut out = Vec::new();
    rbx_xml::to_writer_default(&mut out, &dom, dom.root().children())
        .map_err(|e| JsValue::from_str(&format!("Failed to write XML: {}", e)))?;
    
    String::from_utf8(out)
        .map_err(|e| JsValue::from_str(&format!("Invalid UTF-8 in XML: {}", e)))
}
