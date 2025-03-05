mod common;

use common::setup_test_config;
use fleur_lib::{
    app::{self, APP_REGISTRY_CACHE},
    environment,
};
use serde_json::json;

#[test]
fn test_full_app_lifecycle() {
    // Enable test mode first
    environment::set_test_mode(true);

    let (_config_path, temp_dir) = setup_test_config();

    // Mock home directory
    let original_home = std::env::var("HOME").ok();
    std::env::set_var("HOME", temp_dir.path());

    // Set up test app registry
    let test_registry = json!([{
        "name": "Browser",
        "description": "Web browser",
        "icon": {
            "type": "url",
            "url": {
                "light": "browser.svg",
                "dark": "browser.svg"
            }
        },
        "category": "Utilities",
        "price": "Free",
        "developer": "Test Developer",
        "config": {
            "mcpKey": "puppeteer",
            "runtime": "npx",
            "args": ["-y", "@modelcontextprotocol/server-puppeteer", "--debug"]
        }
    }]);

    // Set the test registry in the cache
    {
        let mut cache = APP_REGISTRY_CACHE.lock().unwrap();
        *cache = Some(test_registry);
    }

    // Test installation
    let install_result = app::install("Browser", None);
    assert!(
        install_result.is_ok(),
        "Install failed: {:?}",
        install_result
    );
    assert!(app::is_installed("Browser").unwrap());

    // Test uninstallation
    let uninstall_result = app::uninstall("Browser");
    assert!(uninstall_result.is_ok());
    assert!(!app::is_installed("Browser").unwrap());

    // Cleanup
    {
        let mut cache = APP_REGISTRY_CACHE.lock().unwrap();
        *cache = None;
    }
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    // Disable test mode after test
    environment::set_test_mode(false);
}
