use url::Url;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub fn parse_url(url: &str) -> Result<String, JsValue> {
    let parsed_url: Url = url.parse().map_err(|e| JsValue::from_str(&format!("Failed to parse URL: {}", e)))?;
    Ok(parsed_url.to_string())
}

#[wasm_bindgen]
pub fn get_host(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let host = parsed_url.host_str().ok_or_else(|| JsValue::from_str("No host found"))?;
    Ok(host.to_string())
}

#[wasm_bindgen]
pub fn get_port(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let port = parsed_url.port_or_known_default().ok_or_else(|| JsValue::from_str("No port found"))?;
    Ok(port.to_string())
}

#[wasm_bindgen]
pub fn get_domain(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let domain = parsed_url.domain().ok_or_else(|| JsValue::from_str("No port found"))?;
    Ok(domain.to_string())
}

#[wasm_bindgen]
pub fn get_scheme(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    Ok(parsed_url.scheme().to_string())
}

#[wasm_bindgen]
pub fn get_origin(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let origin = parsed_url.origin().unicode_serialization(); 
    Ok(origin.to_string())
}

#[wasm_bindgen]
pub fn get_fragment(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let fragment = parsed_url.fragment().unwrap(); 
    Ok(fragment.to_string())
}

#[wasm_bindgen]
pub fn join_url(url: &str, input: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let joined_url = parsed_url.join(input).unwrap(); 
    Ok(joined_url.to_string())
}

#[wasm_bindgen]
pub fn make_relative(base: &str, url: &str) -> Result<String, JsValue> {
    let parsed_base = Url::parse(base).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let relative_url = parsed_base.make_relative(&parsed_url).unwrap(); 
    Ok(relative_url.to_string())
}

#[wasm_bindgen]
pub fn get_password(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let password_url = parsed_url.password().unwrap(); 
    Ok(password_url.to_string())
}

#[wasm_bindgen]
pub fn get_pathname(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let pathname_url = parsed_url.path(); 
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn get_path_segments(url: &str) -> Result<JsValue, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let path_segments: Vec<String> = parsed_url
        .path_segments()
        .map(|segments| segments.map(str::to_string).collect())
        .unwrap_or_else(Vec::new);
    let result = to_value(&path_segments).map_err(|_| JsValue::from_str("Serialization error"))?;
    Ok(result)
}

#[wasm_bindgen]
pub fn get_query(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let pathname_url = parsed_url.query().unwrap(); 
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn get_username(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let pathname_url = parsed_url.username(); 
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn get_query_pairs(url: &str) -> Result<JsValue, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let query_pairs: Vec<Vec<String>> = parsed_url
        .query_pairs()
        .map(|(key, value)| vec![key.to_string(), value.to_string()])
        .collect();
    let result = to_value(&query_pairs).map_err(|_| JsValue::from_str("Serialization error"))?;
    Ok(result)
}