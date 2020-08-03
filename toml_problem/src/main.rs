use toml;

fn has_cert(toml_str: &str) -> Option<bool> {
    let root_value: toml::Value = toml::from_str(toml_str).unwrap();
    root_value.get("whereami")?.get("cert")?.as_bool()
}


fn main() {
    let toml_str_1 = String::from("[whereami]\ncert=true");
    let has_cert1 = has_cert(toml_str_1.as_str()).unwrap_or(false);

    let toml_str_2 = String::from("[whereami]\ncert=false");
    let has_cert2 = has_cert(toml_str_2.as_str()).unwrap_or(false);

    let toml_str_3 = String::from("[whereami]\n");
    let has_cert3 = has_cert(toml_str_3.as_str()).unwrap_or(false);

    println!("cert1={}, cert2={}, cert3={}", has_cert1, has_cert2, has_cert3);
}
