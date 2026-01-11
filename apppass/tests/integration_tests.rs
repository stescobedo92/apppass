//! Integration tests for AppPass
//! 
//! These tests verify the complete workflow of password management operations
//! including creating, retrieving, updating, and deleting passwords.
//!
//! Run with: cargo test --test integration_tests -- --test-threads=1

use std::process::Command;

/// Helper function to run apppass CLI command
fn run_apppass(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute apppass")
}

/// Helper function to cleanup test entries
fn cleanup_test_entry(app_name: &str) {
    let _ = run_apppass(&["--delete", app_name]);
}

/// Test unique app name generator
fn unique_app_name(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{}_{}", prefix, timestamp)
}

/// Extract password from CLI output
/// Output format is: "Application Name: xxx\nPassword: yyy"
fn extract_password(output: &str) -> Option<String> {
    for line in output.lines() {
        let line = line.trim();
        if line.starts_with("Password:") {
            return Some(line.replace("Password:", "").trim().to_string());
        }
    }
    None
}

/// Check if output indicates success (contains checkmark or success message)
fn is_success_output(output: &str) -> bool {
    output.contains("✓") || 
    output.contains("saved") || 
    output.contains("generated") ||
    output.contains("deleted") ||
    output.contains("updated") ||
    output.contains("Exported") ||
    output.contains("Imported")
}

/// Check if output indicates an error or not found
fn is_error_output(output: &str) -> bool {
    output.contains("✗") ||
    output.contains("not found") ||
    output.contains("NoEntry") ||
    output.contains("Error") ||
    output.contains("error")
}

#[test]
fn test_integration_create_and_get_password() {
    let app_name = unique_app_name("int_create");
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success(), "Failed to create password");
    
    // Get the password
    let output = run_apppass(&["--get", &app_name]);
    assert!(output.status.success(), "Failed to get password");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let password = extract_password(&stdout);
    assert!(password.is_some(), "Should extract password from output");
    assert!(!password.unwrap().is_empty(), "Password should not be empty");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_create_with_custom_length() {
    let app_name = unique_app_name("int_length");
    cleanup_test_entry(&app_name);
    
    // Create a password with custom length
    let output = run_apppass(&["--app", &app_name, "--length", "15"]);
    assert!(output.status.success(), "Failed to create password with custom length");
    
    // Get the password and verify length
    let output = run_apppass(&["--get", &app_name]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let password = extract_password(&stdout);
    assert!(password.is_some(), "Should extract password");
    assert_eq!(password.unwrap().len(), 15, "Password should be 15 characters");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_delete_password() {
    let app_name = unique_app_name("int_delete");
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success());
    
    // Delete the password
    let output = run_apppass(&["--delete", &app_name]);
    assert!(output.status.success(), "Failed to delete password");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(is_success_output(&stdout) || stdout.contains("deleted"), 
            "Should indicate successful deletion");
    
    // Try to get deleted password (should fail)
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should indicate not found
    assert!(
        is_error_output(&stdout) || 
        is_error_output(&stderr) ||
        extract_password(&stdout).is_none(),
        "Should indicate password not found"
    );
}

#[test]
fn test_integration_update_password_regenerate() {
    let app_name = unique_app_name("int_update");
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success());
    
    // Get original password
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let original_password = extract_password(&stdout).unwrap();
    
    // Update (regenerate) the password
    let output = run_apppass(&["--update", &app_name]);
    assert!(output.status.success(), "Failed to update password");
    
    // Get new password
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let new_password = extract_password(&stdout).unwrap();
    
    // Passwords should be different
    assert_ne!(original_password, new_password, "Password should have changed after update");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_update_password_custom() {
    let app_name = unique_app_name("int_update_custom");
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success());
    
    // Update with custom password
    let custom_password = "my_custom_password_123";
    let output = run_apppass(&["--update-custom", &app_name, "--password", custom_password]);
    assert!(output.status.success(), "Failed to update with custom password");
    
    // Get password and verify
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let retrieved = extract_password(&stdout).unwrap();
    assert_eq!(retrieved, custom_password, "Password should match custom value");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_list_passwords() {
    let app_name = unique_app_name("int_list");
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success());
    
    // List passwords
    let output = run_apppass(&["--list"]);
    assert!(output.status.success(), "Failed to list passwords");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&app_name), "List should contain the created app");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_memorizable_password() {
    let app_name = unique_app_name("int_memo");
    cleanup_test_entry(&app_name);
    
    // Create memorizable password
    let output = run_apppass(&["--memorizable", &app_name]);
    assert!(output.status.success(), "Failed to create memorizable password");
    
    // Get password
    let output = run_apppass(&["--get", &app_name]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let password = extract_password(&stdout).unwrap();
    
    // Should have dashes (word separators)
    assert!(password.contains('-'), "Memorizable password should contain dashes");
    
    // Should have 3 parts
    let parts: Vec<&str> = password.split('-').collect();
    assert_eq!(parts.len(), 3, "Memorizable password should have 3 parts");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_otp_generation() {
    let app_name = unique_app_name("int_otp");
    cleanup_test_entry(&app_name);
    
    // Generate OTP with 10 second TTL (long enough for test)
    let output = run_apppass(&["--otp", &app_name, "--ttl", "10"]);
    assert!(output.status.success(), "Failed to generate OTP");
    
    // Immediately get the OTP
    let output = run_apppass(&["--get", &app_name]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let otp = extract_password(&stdout);
    assert!(otp.is_some(), "OTP should be retrievable");
    assert!(!otp.unwrap().is_empty(), "OTP should not be empty");
    
    // Cleanup (or wait for TTL)
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_duplicate_app_fails() {
    let app_name = unique_app_name("int_dup");
    cleanup_test_entry(&app_name);
    
    // Create first password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success());
    
    // Try to create duplicate
    let output = run_apppass(&["--app", &app_name]);
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should indicate duplicate or already exists
    // Note: The app may print a message but still exit with success
    assert!(
        !output.status.success() || 
        is_error_output(&stderr) ||
        is_error_output(&stdout) ||
        stdout.contains("already exists") ||
        stderr.contains("already exists") ||
        stdout.contains("Use update") ||
        stderr.contains("Use update"),
        "Should indicate duplicate: stdout={}, stderr={}", stdout, stderr
    );
    
    // Cleanup
    cleanup_test_entry(&app_name);
}

#[test]
fn test_integration_export_import_workflow() {
    let app_name = unique_app_name("int_export");
    let export_file = format!("test_export_{}.csv", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos());
    
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name]);
    assert!(output.status.success());
    
    // Get original password
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let original_password = extract_password(&stdout).unwrap();
    
    // Export
    let output = run_apppass(&["--export", &export_file]);
    assert!(output.status.success(), "Failed to export passwords");
    
    // Delete original
    let output = run_apppass(&["--delete", &app_name]);
    assert!(output.status.success());
    
    // Import
    let output = run_apppass(&["--import", &export_file]);
    assert!(output.status.success(), "Failed to import passwords");
    
    // Get imported password
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let imported_password = extract_password(&stdout).unwrap();
    
    assert_eq!(imported_password, original_password, "Imported password should match original");
    
    // Cleanup
    cleanup_test_entry(&app_name);
    let _ = std::fs::remove_file(&export_file);
}

#[test]
fn test_integration_version_flag() {
    let output = run_apppass(&["--version"]);
    assert!(output.status.success(), "Version flag should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("apppass") || stdout.contains("0."), "Should display version info");
}

#[test]
fn test_integration_help_flag() {
    let output = run_apppass(&["--help"]);
    assert!(output.status.success(), "Help flag should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("password") || stdout.contains("Usage") || stdout.contains("USAGE"), 
            "Should display help info");
}

#[test]
fn test_integration_multiple_passwords_workflow() {
    let apps: Vec<String> = (0..3)
        .map(|i| unique_app_name(&format!("int_multi_{}", i)))
        .collect();
    
    // Cleanup
    for app in &apps {
        cleanup_test_entry(app);
    }
    
    // Create multiple passwords
    for app in &apps {
        let output = run_apppass(&["--app", app]);
        assert!(output.status.success(), "Failed to create password for {}", app);
    }
    
    // List and verify all appear
    let output = run_apppass(&["--list"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    for app in &apps {
        assert!(stdout.contains(app), "List should contain {}", app);
    }
    
    // Delete middle one
    let output = run_apppass(&["--delete", &apps[1]]);
    assert!(output.status.success());
    
    // List again
    let output = run_apppass(&["--list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains(&apps[0]), "First app should still exist");
    assert!(!stdout.contains(&apps[1]), "Middle app should be deleted");
    assert!(stdout.contains(&apps[2]), "Last app should still exist");
    
    // Cleanup remaining
    for app in &apps {
        cleanup_test_entry(app);
    }
}

#[test]
fn test_integration_password_is_alphanumeric() {
    let app_name = unique_app_name("int_alphanum");
    cleanup_test_entry(&app_name);
    
    // Create a password
    let output = run_apppass(&["--app", &app_name, "--length", "50"]);
    assert!(output.status.success());
    
    // Get password
    let output = run_apppass(&["--get", &app_name]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let password = extract_password(&stdout).unwrap();
    
    // All characters should be alphanumeric
    assert!(password.chars().all(|c| c.is_alphanumeric()), 
            "Password should only contain alphanumeric characters");
    
    // Cleanup
    cleanup_test_entry(&app_name);
}
