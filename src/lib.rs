use serde_wasm_bindgen::to_value;
use url::Url;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_url(url: &str) -> Result<String, JsValue> {
    let parsed_url: Url = url
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Failed to parse URL: {}", e)))?;
    Ok(parsed_url.to_string())
}

#[wasm_bindgen]
pub fn get_host(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let host = parsed_url
        .host_str()
        .ok_or_else(|| JsValue::from_str("No host found"))?;
    Ok(host.to_string())
}

#[wasm_bindgen]
pub fn get_hostname(url: &str) -> Result<String, JsValue> {
    get_host(url)
}

#[wasm_bindgen]
pub fn get_port(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let port = parsed_url
        .port_or_known_default()
        .ok_or_else(|| JsValue::from_str("No port found"))?;
    Ok(port.to_string())
}

#[wasm_bindgen]
pub fn get_domain(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let domain = parsed_url
        .domain()
        .ok_or_else(|| JsValue::from_str("No port found"))?;
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

// #[wasm_bindgen]
// pub fn is_absolute(url: &str) -> Result<bool, JsValue> {
//     let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
//     let file_paths = parsed_url.to_file_path().unwrap();
//     Ok(file_paths.is_absolute())
// }

// #[wasm_bindgen]
// pub fn is_dir(url: &str) -> Result<bool, JsValue> {
//     let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
//     let file_paths = parsed_url.to_file_path().unwrap();
//     Ok(file_paths.is_dir())
// }

// #[wasm_bindgen]
// pub fn is_relative(url: &str) -> Result<bool, JsValue> {
//     let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
//     let file_paths = parsed_url.to_file_path().unwrap();
//     Ok(file_paths.is_relative())
// }

// #[wasm_bindgen]
// pub fn ends_with(url: &str, suffix: &str) -> Result<bool, JsValue> {
//     let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
//     let file_paths = parsed_url.to_file_path().unwrap();
//     Ok(file_paths.ends_with(suffix))
// }

#[wasm_bindgen]
pub fn set_host(url: &str, host: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
        .set_host(Some(host))
        .map_err(|_| "host isn't valid")?;

    let host = parsed_url
        .host_str()
        .ok_or_else(|| JsValue::from_str("No host found"))?;
    Ok(host.to_string())
}

#[wasm_bindgen]
pub fn set_hostname(url: &str, host: &str) -> Result<String, JsValue> {
    set_host(url, host)
}

#[wasm_bindgen]
pub fn set_port(url: &str, port: u16) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
        .set_port(Some(port))
        .map_err(|_| "port isn't valid")?;

    let port = parsed_url
        .port_or_known_default()
        .ok_or_else(|| JsValue::from_str("No port found"))?;
    Ok(port.to_string())
}

#[wasm_bindgen]
pub fn set_scheme(url: &str, scheme: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_scheme(scheme)
      .map_err(|_| "scheme isn't valid")?;

    Ok(parsed_url.scheme().to_string())
}

#[wasm_bindgen]
pub fn set_fragment(url: &str, fragment: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_fragment(Some(fragment));

    let fragment = parsed_url.fragment().unwrap();
    Ok(fragment.to_string())
}

#[wasm_bindgen]
pub fn set_password(url: &str, password: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_password(Some(password))
      .map_err(|_| "password isn't valid")?;

    let password_url = parsed_url.password().unwrap();
    Ok(password_url.to_string())
}

#[wasm_bindgen]
pub fn set_pathname(url: &str, path: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_path(path);

    let pathname_url = parsed_url.path();
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn set_path_segments(url: &str, path: &str) -> Result<JsValue, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_path(path);

    let path_segments: Vec<String> = parsed_url
        .path_segments()
        .map(|segments| segments.map(str::to_string).collect())
        .unwrap_or_else(Vec::new);
    let result = to_value(&path_segments).map_err(|_| JsValue::from_str("Serialization error"))?;
    Ok(result)
}

#[wasm_bindgen]
pub fn set_query(url: &str, query: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_query(Some(query));

    let pathname_url = parsed_url.query().unwrap();
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn set_username(url: &str, username: &str) -> Result<String, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_username(username)
      .map_err(|_| "username isn't valid")?;

    let pathname_url = parsed_url.username();
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn set_query_pairs(url: &str, query: &str) -> Result<JsValue, JsValue> {
    let mut parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    parsed_url
      .set_query(Some(query));

    let query_pairs: Vec<Vec<String>> = parsed_url
        .query_pairs()
        .map(|(key, value)| vec![key.to_string(), value.to_string()])
        .collect();
    let result = to_value(&query_pairs).map_err(|_| JsValue::from_str("Serialization error"))?;
    Ok(result)
}
