use std::collections::HashMap;
use std::sync::Mutex;

use serde_wasm_bindgen::to_value;
use url::Url;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

// Define a lazy_static Mutex-protected HashMap with keys of type usize and values of type Url.
lazy_static! {
    static ref URL_LIST: Mutex<HashMap<usize, Url>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn new_url(url: &str) -> Result<usize, JsValue> {
    let parsed_url: Url = url
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Failed to parse URL: {}", e)))?;

    let mut list = URL_LIST.lock().unwrap();
    let index = list.len();
    list.insert(index, parsed_url);
    Ok(index)
}

#[wasm_bindgen]
pub fn parse_url(url: &str) -> Result<String, JsError> {
    let parsed_url: Url = url
        .parse()
        .map_err(|e| JsError::new(&format!("Failed to parse URL: {}", e)))?;
    Ok(parsed_url.to_string())
}

#[wasm_bindgen]
pub fn parse(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let parsed_url = url.to_string();
    list.remove_entry(&index);
    Ok(parsed_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_host(url: &str) -> Result<String, JsError> {
    let parsed_url = Url::parse(url).map_err(|e| JsError::new(&format!("{}", e)))?;
    let host = parsed_url
        .host_str()
        .ok_or_else(|| JsError::new("No host found"))?;
    Ok(host.to_string())
}

#[wasm_bindgen]
pub fn get_host_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let host = url
        .host_str()
        .ok_or_else(|| JsValue::from_str("No host found"))
        .unwrap()
        .to_string();
    list.remove_entry(&index);
    Ok(host)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_hostname(url: &str) -> Result<String, JsError> {
    get_host(url)
}

#[wasm_bindgen]
pub fn get_hostname_url(index: usize) -> Result<String, JsError> {
    get_host_url(index)
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
pub fn get_port_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let port = url
      .port_or_known_default()
      .ok_or_else(|| JsValue::from_str("No port found"))
      .unwrap()
      .to_string();
    list.remove_entry(&index);
    Ok(port)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn get_domain_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let domain = url
      .domain()
      .ok_or_else(|| JsValue::from_str("No port found"))
      .unwrap()
      .to_string();
    list.remove_entry(&index);
    Ok(domain)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_scheme(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    Ok(parsed_url.scheme().to_string())
}

#[wasm_bindgen]
pub fn get_scheme_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let domain = url
      .scheme()
      .to_string();
    list.remove_entry(&index);
    Ok(domain)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_origin(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let origin = parsed_url.origin().unicode_serialization();
    Ok(origin.to_string())
}

#[wasm_bindgen]
pub fn get_origin_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let origin = url
      .origin()
      .unicode_serialization();
    list.remove_entry(&index);
    Ok(origin)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_fragment(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let fragment = parsed_url.fragment().unwrap();
    Ok(fragment.to_string())
}

#[wasm_bindgen]
pub fn get_fragment_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let fragment = url
      .fragment()
      .unwrap()
      .to_string();
    list.remove_entry(&index);
    Ok(fragment)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn join_url(url: &str, input: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let joined_url = parsed_url.join(input).unwrap();
    Ok(joined_url.to_string())
}

#[wasm_bindgen]
pub fn join(index: usize, input: &str) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let joined_url = url
      .join(input)
      .unwrap()
      .to_string();
    list.remove_entry(&index);
    Ok(joined_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn make_relative(base: &str, url: &str) -> Result<String, JsValue> {
    let parsed_base = Url::parse(base).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let relative_url = parsed_base.make_relative(&parsed_url).unwrap();
    Ok(relative_url.to_string())
}

#[wasm_bindgen]
pub fn relative_partial(index: usize, url: &str) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(parsed_base) = list.get(&index) { 
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)));
    let relative_url = parsed_base
      .make_relative(&(parsed_url.unwrap()))
      .unwrap()
      .to_string();
    list.remove_entry(&index);
    Ok(relative_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn relative(index: usize, url_index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();

  if let Some(parsed_base) = list.get(&index)  { 
    if let Some(parsed_url) = list.get(&url_index)  { 
      let relative_url = parsed_base
        .make_relative(&parsed_url)
        .unwrap()
        .to_string();
      list.remove_entry(&index);
      Ok(relative_url)
    } else {
      Err(JsError::new("No URL found with key"))
    }
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_password(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let password_url = parsed_url.password().unwrap();
    Ok(password_url.to_string())
}

#[wasm_bindgen]
pub fn get_password_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let password_url = url
      .password()
      .unwrap()
      .to_string();
    list.remove_entry(&index);
    Ok(password_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_pathname(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let pathname_url = parsed_url.path();
    Ok(pathname_url.to_string())
}

#[wasm_bindgen]
pub fn get_pathname_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let pathname_url = url
      .path()
      .to_string();
    list.remove_entry(&index);
    Ok(pathname_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn get_path_segments_url(index: usize) -> Result<JsValue, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let path_segments = url
        .path_segments()
        .map(|segments| segments.map(str::to_string).collect())
        .unwrap_or_else(Vec::new);
    let result = to_value(&path_segments).map_err(|_| JsValue::from_str("Serialization error"));
    list.remove_entry(&index);
    Ok(result.unwrap())
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_query(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let query_url = parsed_url.query().unwrap();
    Ok(query_url.to_string())
}

#[wasm_bindgen]
pub fn get_query_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let query_url = url
        .query()
        .unwrap()
        .to_string();
    list.remove_entry(&index);
    Ok(query_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn get_username(url: &str) -> Result<String, JsValue> {
    let parsed_url = Url::parse(url).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let username_url = parsed_url.username();
    Ok(username_url.to_string())
}

#[wasm_bindgen]
pub fn get_username_url(index: usize) -> Result<String, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let username_url = url
        .username()
        .to_string();
    list.remove_entry(&index);
    Ok(username_url)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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

#[wasm_bindgen]
pub fn get_query_pairs_url(index: usize) -> Result<JsValue, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get(&index) { 
    let query_pairs: Vec<Vec<String>> = url
        .query_pairs()
        .map(|(key, value)| vec![key.to_string(), value.to_string()])
        .collect();
    let result = to_value(&query_pairs).map_err(|_| JsValue::from_str("Serialization error"));
    list.remove_entry(&index);
    Ok(result.unwrap())
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_host_url(index: usize, host: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_host(Some(host))
      .map_err(|_| "host isn't valid")
      .unwrap();

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
}

#[wasm_bindgen]
pub fn set_hostname(url: &str, host: &str) -> Result<String, JsValue> {
    set_host(url, host)
}

#[wasm_bindgen]
pub fn set_hostname_url(index: usize, host: &str) -> Result<usize, JsError> {
  set_host_url(index, host)
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
pub fn set_port_url(index: usize, port: u16) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_port(Some(port))
      .map_err(|_| "port isn't valid")
      .unwrap();

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_scheme_url(index: usize, scheme: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_scheme(scheme)
      .map_err(|_| "scheme isn't valid")
      .unwrap();

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_fragment_url(index: usize, fragment: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_fragment(Some(fragment));

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_password_url(index: usize, password: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_password(Some(password))
      .map_err(|_| "password isn't valid")
      .unwrap();

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_pathname_url(index: usize, path: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_path(path);

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_path_segments_url(index: usize, path: &str) -> Result<JsValue, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_path(path);

    let path_segments: Vec<String> = url
      .path_segments()
      .map(|segments| segments.map(str::to_string).collect())
      .unwrap_or_else(Vec::new);
    let result = to_value(&path_segments).map_err(|_| JsValue::from_str("Serialization error"));
    list.remove_entry(&index);
    Ok(result.unwrap())
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_query_url(index: usize, query: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_query(Some(query));

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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
pub fn set_username_url(index: usize, username: &str) -> Result<usize, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_username(username)
      .map_err(|_| "username isn't valid")
      .unwrap();

    Ok(index)
  } else {
    Err(JsError::new("No URL found with key"))
  }
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

#[wasm_bindgen]
pub fn set_query_pairs_url(index: usize, query: &str) -> Result<JsValue, JsError> {
  let mut list = URL_LIST.lock().unwrap();
  if let Some(url) = list.get_mut(&index) { 
    url
      .set_query(Some(query));

    let query_pairs: Vec<Vec<String>> = url
        .query_pairs()
        .map(|(key, value)| vec![key.to_string(), value.to_string()])
        .collect();
    let result = to_value(&query_pairs).map_err(|_| JsValue::from_str("Serialization error"));
    list.remove_entry(&index);
    Ok(result.unwrap())
  } else {
    Err(JsError::new("No URL found with key"))
  }
}
