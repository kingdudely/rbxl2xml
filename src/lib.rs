use wasm_bindgen::prelude::*;
use std::io::Cursor;

// Set up panic hook for better error messages in JS
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Convert Roblox binary (RBXL) to XML (RBXMX)
#[wasm_bindgen]
pub fn binary_to_xml(binary: &[u8]) -> Result<Vec<u8>, JsValue> {
    // Parse binary into WeakDom
    let dom = rbx_binary::from_reader(Cursor::new(binary))
        .map_err(|e| JsValue::from_str(&format!("Failed to parse binary: {}", e)))?;

    let root_ids = dom.root().children();

    // Encode XML into a Vec<u8>
    let mut out = Vec::new();
    let options = rbx_xml::EncodeOptions::new()
        .property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown);

    rbx_xml::to_writer(&mut out, &dom, root_ids, options)
        .map_err(|e| JsValue::from_str(&format!("Failed to write XML: {}", e)))?;

    Ok(out)
}

/// Convert Roblox XML (RBXMX) to binary (RBXL)
#[wasm_bindgen]
pub fn xml_to_binary(xml: &[u8]) -> Result<Vec<u8>, JsValue> {
    // Parse XML into WeakDom
    let options = rbx_xml::DecodeOptions::new()
        .property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown);

    let dom = rbx_xml::from_reader(Cursor::new(xml), options)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse XML: {}", e)))?;

    let root_ids = dom.root().children();

    // Encode binary into a Vec<u8>
    let mut out = Vec::new();
    rbx_binary::to_writer(&mut out, &dom, root_ids)
        .map_err(|e| JsValue::from_str(&format!("Failed to write binary: {}", e)))?;

    Ok(out)
}
