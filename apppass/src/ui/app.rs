use crate::app::keyring::{delete_from_keyring, get_from_keyring, save_to_keyring};
use crossterm::event::{KeyCode, KeyEvent};
use keyring::Entry;
use std::io;

/// Application modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    /// Main menu
    Menu,
    /// Create new password (auto-generated)
    Create,
    /// Create custom password (user-specified)
    CreateCustom,
    /// List all passwords
    List,
    /// Update password
    Update,
    /// Delete password
    Delete,
    /// View single password
    View,
    /// Generate OTP
    GenerateOTP,
    /// Generate memorizable password
    Memorizable,
    /// Export passwords
    Export,
    /// Import passwords
    Import,
    /// Settings (password length configuration)
    Settings,
}

/// Input field for forms
#[derive(Debug, Clone)]
pub struct InputField {
    pub value: String,
    pub cursor_position: usize,
}

impl InputField {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor_position: 0,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.value.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;
        }
    }
}

/// Password entry
#[derive(Debug, Clone)]
pub struct PasswordEntry {
    pub app_name: String,
    pub password: String,
}

/// Main application state
pub struct App {
    /// Current mode
    pub mode: Mode,
    /// Should quit flag
    pub should_quit: bool,
    /// Selected menu item
    pub selected_menu: usize,
    /// Input field for app name
    pub app_name_input: InputField,
    /// Input field for password
    pub password_input: InputField,
    /// Input field for password length
    pub length_input: InputField,
    /// List of passwords
    pub password_list: Vec<PasswordEntry>,
    /// Selected item in list
    pub selected_list_item: usize,
    /// Status message
    pub status_message: String,
    /// Active input field (0 = app name, 1 = password/length)
    pub active_input: usize,
    /// Default password length for auto-generation
    pub default_password_length: usize,
}

impl App {
    /// Creates a new App instance
    pub fn new() -> Self {
        // Load default password length from keyring (persistent setting)
        let default_password_length = Self::load_password_length_setting().unwrap_or(30);

        Self {
            mode: Mode::Menu,
            should_quit: false,
            selected_menu: 0,
            app_name_input: InputField::new(),
            password_input: InputField::new(),
            length_input: InputField::new(),
            password_list: Vec::new(),
            selected_list_item: 0,
            default_password_length,
            status_message: String::new(),
            active_input: 0,
        }
    }

    /// Load password length setting from keyring
    fn load_password_length_setting() -> Option<usize> {
        match get_from_keyring(crate::app::PASSWORD_LENGTH_KEY) {
            Ok(value) => {
                value.parse::<usize>().ok().filter(|&len| len >= 8 && len <= 128)
            }
            Err(_) => {
                // No saved setting or keyring access failed - use default
                None
            }
        }
    }

    /// Save password length setting to keyring
    fn save_password_length_setting(length: usize) -> Result<(), String> {
        save_to_keyring(crate::app::PASSWORD_LENGTH_KEY, &length.to_string())
            .map_err(|e| format!("Failed to save setting: {}", e))
    }

    /// Delete password length setting from keyring (resets to default)
    /// Note: Currently unused - user can change value instead of deleting.
    /// Kept for potential future use (e.g., explicit "Reset to Default" option).
    #[allow(dead_code)]
    fn delete_password_length_setting() -> Result<(), String> {
        delete_from_keyring(crate::app::PASSWORD_LENGTH_KEY)
            .map_err(|e| format!("Failed to delete setting: {}", e))
    }

    /// Handles keyboard input
    pub fn handle_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.mode {
            Mode::Menu => self.handle_menu_key(key),
            Mode::Create => self.handle_create_key(key),
            Mode::CreateCustom => self.handle_create_custom_key(key),
            Mode::List => self.handle_list_key(key),
            Mode::Update => self.handle_update_key(key),
            Mode::Delete => self.handle_delete_key(key),
            Mode::View => self.handle_view_key(key),
            Mode::GenerateOTP => self.handle_generate_otp_key(key),
            Mode::Memorizable => self.handle_memorizable_key(key),
            Mode::Export => self.handle_export_key(key),
            Mode::Import => self.handle_import_key(key),
            Mode::Settings => self.handle_settings_key(key),
        }
    }

    /// Handles keys in menu mode
    fn handle_menu_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Up => {
                if self.selected_menu > 0 {
                    self.selected_menu -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_menu < 11 {  // Updated for 12 menu items (0-11)
                    self.selected_menu += 1;
                }
            }
            KeyCode::Enter => {
                self.status_message.clear();
                match self.selected_menu {
                    0 => {
                        // Create New Password (auto-generated)
                        self.mode = Mode::Create;
                        self.app_name_input.clear();
                        self.length_input.clear();
                        self.active_input = 0;
                    }
                    1 => {
                        // Create Custom Password
                        self.mode = Mode::CreateCustom;
                        self.app_name_input.clear();
                        self.password_input.clear();
                        self.active_input = 0;
                    }
                    2 => {
                        // List All Passwords
                        self.mode = Mode::List;
                        self.load_passwords();
                    }
                    3 => {
                        // Update Password
                        self.mode = Mode::Update;
                        self.app_name_input.clear();
                        self.password_input.clear();
                        self.active_input = 0;
                    }
                    4 => {
                        // Delete Password
                        self.mode = Mode::Delete;
                        self.app_name_input.clear();
                        self.load_passwords();  // Load passwords for selection
                        self.selected_list_item = 0;
                    }
                    5 => {
                        // Generate OTP
                        self.mode = Mode::GenerateOTP;
                        self.app_name_input.clear();
                        self.length_input.clear();  // Use for TTL
                        self.active_input = 0;
                    }
                    6 => {
                        // Generate Memorizable Password
                        self.mode = Mode::Memorizable;
                        self.app_name_input.clear();
                    }
                    7 => {
                        // Export Passwords
                        self.mode = Mode::Export;
                        self.app_name_input.clear();  // Use for file path
                    }
                    8 => {
                        // Import Passwords
                        self.mode = Mode::Import;
                        self.app_name_input.clear();  // Use for file path
                    }
                    9 => {
                        // Settings (Password Length)
                        self.mode = Mode::Settings;
                        self.length_input.clear();
                        self.length_input.value = self.default_password_length.to_string();
                        self.length_input.cursor_position = self.length_input.value.len();
                    }
                    10 => {
                        // Set Auto-Lock
                        self.status_message = "Auto-lock not implemented in UI yet".to_string();
                    }
                    11 => {
                        // Exit
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys in create mode (auto-generate password)
    fn handle_create_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty() {
                    // Auto-generate password with configured default length
                    match crate::app::password::generate_save_safety_password(
                        &self.app_name_input.value,
                        Some(self.default_password_length),
                    ) {
                        Ok(_) => {
                            self.status_message = format!(
                                "✓ Password auto-generated ({} chars) for '{}'",
                                self.default_password_length,
                                self.app_name_input.value
                            );
                            self.app_name_input.clear();
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.app_name_input.insert_char(c);
            }
            KeyCode::Backspace => {
                self.app_name_input.delete_char();
            }
            KeyCode::Left => {
                self.app_name_input.move_cursor_left();
            }
            KeyCode::Right => {
                self.app_name_input.move_cursor_right();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys in create custom mode (user-specified password)
    fn handle_create_custom_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Tab => {
                self.active_input = (self.active_input + 1) % 2;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty() && !self.password_input.value.is_empty() {
                    match save_to_keyring(
                        &self.app_name_input.value,
                        &self.password_input.value,
                    ) {
                        Ok(_) => {
                            self.status_message = format!(
                                "✓ Custom password saved for '{}'",
                                self.app_name_input.value
                            );
                            self.app_name_input.clear();
                            self.password_input.clear();
                            self.active_input = 0;
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                if self.active_input == 0 {
                    self.app_name_input.insert_char(c);
                } else {
                    self.password_input.insert_char(c);
                }
            }
            KeyCode::Backspace => {
                if self.active_input == 0 {
                    self.app_name_input.delete_char();
                } else {
                    self.password_input.delete_char();
                }
            }
            KeyCode::Left => {
                if self.active_input == 0 {
                    self.app_name_input.move_cursor_left();
                } else {
                    self.password_input.move_cursor_left();
                }
            }
            KeyCode::Right => {
                if self.active_input == 0 {
                    self.app_name_input.move_cursor_right();
                } else {
                    self.password_input.move_cursor_right();
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Loads all passwords from keyring
    fn load_passwords(&mut self) {
        self.password_list.clear();
        self.selected_list_item = 0;

        if let Ok(entry) = Entry::new(crate::app::APP_SERVICE, crate::app::APP_INDEX) {
            if let Ok(data) = entry.get_password() {
                let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
                for app_name in app_names {
                    if let Ok(password) = get_from_keyring(app_name) {
                        self.password_list.push(PasswordEntry {
                            app_name: app_name.to_string(),
                            password,
                        });
                    }
                }
            } else {
                self.status_message = "No passwords found in keyring".to_string();
            }
        } else {
            self.status_message = "Failed to access keyring".to_string();
        }
    }

    /// Handles keys in list mode
    fn handle_list_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Up => {
                if self.selected_list_item > 0 {
                    self.selected_list_item -= 1;
                }
            }
            KeyCode::Down => {
                if !self.password_list.is_empty()
                    && self.selected_list_item < self.password_list.len() - 1
                {
                    self.selected_list_item += 1;
                }
            }
            KeyCode::Enter => {
                if !self.password_list.is_empty()
                    && self.selected_list_item < self.password_list.len()
                {
                    self.mode = Mode::View;
                }
            }
            KeyCode::Char('r') => {
                self.load_passwords();
                self.status_message = "✓ List refreshed".to_string();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys in view mode
    fn handle_view_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.mode = Mode::List;
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys in update mode
    fn handle_update_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Tab => {
                self.active_input = (self.active_input + 1) % 2;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty()
                    && !self.password_input.value.is_empty()
                {
                    match save_to_keyring(
                        &self.app_name_input.value,
                        &self.password_input.value,
                    ) {
                        Ok(_) => {
                            self.status_message = format!(
                                "✓ Password updated for '{}'",
                                self.app_name_input.value
                            );
                            self.app_name_input.clear();
                            self.password_input.clear();
                            self.active_input = 0;
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                if self.active_input == 0 {
                    self.app_name_input.insert_char(c);
                } else {
                    self.password_input.insert_char(c);
                }
            }
            KeyCode::Backspace => {
                if self.active_input == 0 {
                    self.app_name_input.delete_char();
                } else {
                    self.password_input.delete_char();
                }
            }
            KeyCode::Left => {
                if self.active_input == 0 {
                    self.app_name_input.move_cursor_left();
                } else {
                    self.password_input.move_cursor_left();
                }
            }
            KeyCode::Right => {
                if self.active_input == 0 {
                    self.app_name_input.move_cursor_right();
                } else {
                    self.password_input.move_cursor_right();
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys in delete mode
    fn handle_delete_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Up => {
                if self.selected_list_item > 0 {
                    self.selected_list_item -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_list_item < self.password_list.len().saturating_sub(1) {
                    self.selected_list_item += 1;
                }
            }
            KeyCode::Enter => {
                if !self.password_list.is_empty() && self.selected_list_item < self.password_list.len() {
                    let app_name = self.password_list[self.selected_list_item].app_name.clone();
                    match delete_from_keyring(&app_name) {
                        Ok(_) => {
                            self.status_message = format!("✓ Password deleted for '{}'", app_name);
                            self.load_passwords();  // Reload the list
                            if self.selected_list_item >= self.password_list.len() && self.selected_list_item > 0 {
                                self.selected_list_item -= 1;
                            }
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char('r') => {
                self.load_passwords();
                self.status_message = "✓ List refreshed".to_string();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys for OTP generation
    fn handle_generate_otp_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Tab => {
                self.active_input = (self.active_input + 1) % 2;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty() {
                    // Parse TTL in seconds, default to 300 seconds (5 minutes) if not provided
                    let ttl = if !self.length_input.value.is_empty() {
                        self.length_input.value.parse::<u64>().unwrap_or(300)
                    } else {
                        300
                    };
                    
                    // Use the configured default password length for OTP
                    match crate::app::otp::generate_otp(&self.app_name_input.value, ttl, self.default_password_length) {
                        Ok(otp) => {
                            self.status_message = format!(
                                "✓ OTP saved for '{}' (expires in {} seconds): {}",
                                self.app_name_input.value, ttl, otp
                            );
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                    self.app_name_input.clear();
                    self.length_input.clear();
                    self.active_input = 0;
                }
            }
            KeyCode::Char(c) => {
                if self.active_input == 0 {
                    self.app_name_input.insert_char(c);
                } else {
                    self.length_input.insert_char(c);
                }
            }
            KeyCode::Backspace => {
                if self.active_input == 0 {
                    self.app_name_input.delete_char();
                } else {
                    self.length_input.delete_char();
                }
            }
            KeyCode::Left => {
                if self.active_input == 0 {
                    self.app_name_input.move_cursor_left();
                } else {
                    self.length_input.move_cursor_left();
                }
            }
            KeyCode::Right => {
                if self.active_input == 0 {
                    self.app_name_input.move_cursor_right();
                } else {
                    self.length_input.move_cursor_right();
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys for memorizable password generation
    fn handle_memorizable_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty() {
                    match crate::app::password::generate_memorizable_password(&self.app_name_input.value) {
                        Ok(_) => {
                            self.status_message = format!(
                                "✓ Memorizable password generated for '{}'",
                                self.app_name_input.value
                            );
                            self.app_name_input.clear();
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.app_name_input.insert_char(c);
            }
            KeyCode::Backspace => {
                self.app_name_input.delete_char();
            }
            KeyCode::Left => {
                self.app_name_input.move_cursor_left();
            }
            KeyCode::Right => {
                self.app_name_input.move_cursor_right();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys for export passwords
    fn handle_export_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty() {
                    match crate::app::password::export_passwords(&self.app_name_input.value) {
                        Ok(_) => {
                            self.status_message = format!(
                                "✓ Passwords exported to '{}'",
                                self.app_name_input.value
                            );
                            self.app_name_input.clear();
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.app_name_input.insert_char(c);
            }
            KeyCode::Backspace => {
                self.app_name_input.delete_char();
            }
            KeyCode::Left => {
                self.app_name_input.move_cursor_left();
            }
            KeyCode::Right => {
                self.app_name_input.move_cursor_right();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys for import passwords
    fn handle_import_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Enter => {
                if !self.app_name_input.value.is_empty() {
                    match crate::app::password::import_passwords(&self.app_name_input.value) {
                        Ok(_) => {
                            self.status_message = format!(
                                "✓ Passwords imported from '{}'",
                                self.app_name_input.value
                            );
                            self.app_name_input.clear();
                        }
                        Err(e) => {
                            self.status_message = format!("✗ Error: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.app_name_input.insert_char(c);
            }
            KeyCode::Backspace => {
                self.app_name_input.delete_char();
            }
            KeyCode::Left => {
                self.app_name_input.move_cursor_left();
            }
            KeyCode::Right => {
                self.app_name_input.move_cursor_right();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keys in settings mode
    fn handle_settings_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
            }
            KeyCode::Enter => {
                // Parse and save the new default password length
                if let Ok(length) = self.length_input.value.parse::<usize>() {
                    if length >= 8 && length <= 128 {
                        self.default_password_length = length;
                        // Save to keyring for persistence
                        if let Err(e) = Self::save_password_length_setting(length) {
                            self.status_message = format!("✗ Failed to save setting: {}", e);
                        } else {
                            self.status_message = format!("✓ Default password length set to {} characters (saved)", length);
                        }
                        self.mode = Mode::Menu;
                    } else {
                        self.status_message = "✗ Length must be between 8 and 128 characters".to_string();
                    }
                } else {
                    self.status_message = "✗ Please enter a valid number".to_string();
                }
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.length_input.insert_char(c);
            }
            KeyCode::Backspace => {
                self.length_input.delete_char();
            }
            KeyCode::Left => {
                self.length_input.move_cursor_left();
            }
            KeyCode::Right => {
                self.length_input.move_cursor_right();
            }
            _ => {}
        }
        Ok(())
    }
}
