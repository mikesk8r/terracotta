#![cfg(test)]

#[test]
fn legacy_config_values() {
    let config = "server-ip=localhost\n".to_string();
    let config = crate::config::legacy::get_legacy_config(config);
    assert_eq!(config.server_ip, "localhost".to_string())
}

#[test]
fn legacy_config_comments() {
    let config = "# This is a comment. Comments should not be read!\n".to_string();
    let config = crate::config::legacy::get_legacy_config(config);
    assert_eq!(config.server_ip, "".to_string())
}
