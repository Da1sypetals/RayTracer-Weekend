use serde::{de::DeserializeOwned, Deserialize, Serialize};
use toml::Value;

pub fn value_get_into<T: DeserializeOwned>(value: &Value, key: &str) -> T {
    #[derive(Serialize, Deserialize, Debug)]
    struct Wrapper<T> {
        value: T,
    }

    let repr = value
        .get(key)
        .expect(&format!("Expect field {}", key))
        .to_string();
    let wrapper: Wrapper<T> = toml::from_str(&format!("value = {}", repr)).unwrap();
    wrapper.value
}

pub fn value_get_into_option<T: DeserializeOwned>(value: &Value, key: &str) -> Option<T> {
    #[derive(Serialize, Deserialize, Debug)]
    struct Wrapper<T> {
        value: T,
    }

    let repr = value.get(key)?.to_string();
    let wrapper: Wrapper<T> = toml::from_str(&format!("value = {}", repr)).unwrap();
    Some(wrapper.value)
}
