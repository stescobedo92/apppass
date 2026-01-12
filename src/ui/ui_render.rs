use crate::ui::app::{App, Mode};
use crate::app::keyring::{has_auto_passwords, has_custom_passwords};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Main render function
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    
    match app.mode {
        Mode::Menu => render_menu(f, chunks[1], app),
        Mode::Create => render_create(f, chunks[1], app),
        Mode::CreateCustom => render_create_custom(f, chunks[1], app),
        Mode::List => render_list(f, chunks[1], app),
        Mode::UpdateAuto => render_update_auto(f, chunks[1], app),
        Mode::UpdateCustom => render_update_custom(f, chunks[1], app),
        Mode::Delete => render_delete(f, chunks[1], app),
        Mode::View => render_view(f, chunks[1], app),
        Mode::GenerateOTP => render_generate_otp(f, chunks[1], app),
        Mode::Memorizable => render_memorizable(f, chunks[1], app),
        Mode::Export => render_export(f, chunks[1], app),
        Mode::Import => render_import(f, chunks[1], app),
        Mode::Settings => render_settings(f, chunks[1], app),
    }

    render_footer(f, chunks[2], app);
}

/// Renders the header
fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("🔒 AppPass - Interactive Password Manager")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::White)));
    f.render_widget(title, area);
}

/// Renders the footer with help text
fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.mode {
        Mode::Menu => "↑↓: Navigate | Enter: Select | q/Esc: Quit",
        Mode::Create => "Enter: Create | Esc: Back",
        Mode::CreateCustom => "Tab: Switch Field | Enter: Create | Esc: Back",
        Mode::List => "↑↓: Navigate | Enter: View | r: Refresh | Esc: Back",
        Mode::View => "Enter/Esc: Back",
        Mode::UpdateAuto => "↑↓: Navigate | Enter: Select/Save | r: Refresh | Esc: Back",
        Mode::UpdateCustom => "↑↓: Navigate | Enter: Select | Tab: Switch Field | Esc: Back",
        Mode::Delete => "↑↓: Navigate | Enter: Delete | r: Refresh | Esc: Back",
        Mode::GenerateOTP => "Tab: Switch Field | Enter: Generate | Esc: Back",
        Mode::Memorizable => "Enter: Generate | Esc: Back",
        Mode::Export => "Enter: Export | Esc: Back",
        Mode::Import => "Enter: Import | Esc: Back",
        Mode::Settings => "Enter: Save | Esc: Cancel",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}

/// Renders the settings form (password length configuration)
fn render_settings(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    // Password length input
    let length_input = Paragraph::new(app.length_input.value.as_str())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("Default Password Length (8-128 characters)")
                .borders(Borders::ALL),
        );
    f.render_widget(length_input, chunks[0]);

    // Info section
    let info_text = format!(
        "ℹ️  Settings - Password Length Configuration\n\
         \n\
         Set the default length for auto-generated passwords.\n\
         Current: {} characters\n\
         Range: 8-128 characters\n\
         \n\
         This setting is persistent and saved to keyring.\n\
         It will be remembered across app restarts.\n\
         \n\
         Affects: Create New Password, Generate OTP, Generate Memorizable Password",
        app.default_password_length
    );
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }

    // Set cursor position
    let cursor_x = chunks[0].x + (app.length_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
    let cursor_y = chunks[0].y + 1;
    f.set_cursor_position((cursor_x, cursor_y));
}

/// Renders the main menu
fn render_menu(f: &mut Frame, area: Rect, app: &App) {
    let has_passwords = app.has_passwords();
    let has_auto = has_auto_passwords();
    let has_custom = has_custom_passwords();
    
    let menu_items = vec![
        ("Create New Password (Auto-generated)", true),
        ("Create Custom Password", true),
        ("List All Passwords", has_passwords),
        ("Update Auto-generated Password", has_auto),
        ("Update Custom Password", has_custom),
        ("Delete Password", has_passwords),
        ("Generate OTP (One-Time Password)", has_passwords),
        ("Generate Memorizable Password", has_passwords),
        ("Export Passwords to CSV", has_passwords),
        ("Import Passwords from CSV", true),
        ("Settings (Password Length)", true),
        ("Set Auto-Lock", true),
        ("Exit", true),
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, (item, enabled))| {
            let text = if *enabled {
                format!("  {}  ", item)
            } else {
                // Customize message based on menu item
                let suffix = match i {
                    2 => "(No passwords)",           // List All Passwords
                    3 => "(No auto passwords)",      // Update Auto-generated Password
                    4 => "(No custom passwords)",    // Update Custom Password
                    5 => "(No passwords)",           // Delete Password
                    6 => "(No passwords)",           // Generate OTP
                    7 => "(No passwords)",           // Generate Memorizable Password
                    8 => "(No passwords)",          // Export Passwords to CSV
                    _ => "(Unavailable)",
                };
                // Format disabled item text - keep it concise
                format!("  {} {}  ", item, suffix)
            };
            
            let style = if i == app.selected_menu {
                if *enabled {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::DarkGray)
                        .bg(Color::Gray)
                }
            } else if *enabled {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(text).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(
            Block::default()
                .title("Main Menu")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(menu, area);
}

/// Renders the create password form (auto-generated)
fn render_create(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    // App name input
    let app_name_input = Paragraph::new(app.app_name_input.value.as_str())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("Application Name")
                .borders(Borders::ALL),
        );
    f.render_widget(app_name_input, chunks[0]);

    // Info section
    let info_text = format!(
        "ℹ️  Create New Password (Auto-generated)\n\
         Generates a secure {}-character password automatically.\n\
         Example: Enter 'gmail' to create a password for Gmail.",
        app.default_password_length
    );
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }

    // Set cursor position
    let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
    let cursor_y = chunks[0].y + 1;
    f.set_cursor_position((cursor_x, cursor_y));
}

/// Renders the create custom password form
fn render_create_custom(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    // App name input
    let app_name_style = if app.active_input == 0 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let app_name_input = Paragraph::new(app.app_name_input.value.as_str())
        .style(app_name_style)
        .block(
            Block::default()
                .title("Application Name")
                .borders(Borders::ALL),
        );
    f.render_widget(app_name_input, chunks[0]);

    // Password input
    let password_style = if app.active_input == 1 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let password_input = Paragraph::new(app.password_input.value.as_str())
        .style(password_style)
        .block(
            Block::default()
                .title("Custom Password (any length)")
                .borders(Borders::ALL),
        );
    f.render_widget(password_input, chunks[1]);

    // Info section
    let info_text = "ℹ️  Create Custom Password\n\
                     Save your own password of any length to the keyring.\n\
                     Example: Enter 'github' and your custom password.";
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[2]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[3]);
    }

    // Set cursor position
    if app.active_input == 0 {
        let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
        let cursor_y = chunks[0].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    } else {
        let cursor_x = chunks[1].x + (app.password_input.cursor_position as u16).min(chunks[1].width.saturating_sub(2)) + 1;
        let cursor_y = chunks[1].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

/// Renders the password list
fn render_list(f: &mut Frame, area: Rect, app: &App) {
    if app.password_list.is_empty() {
        let empty_msg = Paragraph::new("No passwords stored yet.\nCreate one from the main menu!")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Password List")
                    .borders(Borders::ALL),
            );
        f.render_widget(empty_msg, area);
        return;
    }

    let items: Vec<ListItem> = app
        .password_list
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if i == app.selected_list_item {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let masked_pwd = "*".repeat(entry.password.len().min(20));
            ListItem::new(format!("  {} - {}", entry.app_name, masked_pwd)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!("Password List ({} entries)", app.password_list.len()))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}

/// Renders the view single password
fn render_view(f: &mut Frame, area: Rect, app: &App) {
    if app.password_list.is_empty() || app.selected_list_item >= app.password_list.len() {
        return;
    }

    let entry = &app.password_list[app.selected_list_item];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    let app_name = Paragraph::new(entry.app_name.as_str())
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("Application Name")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(app_name, chunks[0]);

    let password = Paragraph::new(entry.password.as_str())
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().title("Password").borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    f.render_widget(password, chunks[1]);

    let info = Paragraph::new("Press Enter or Esc to go back to the list")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(info, chunks[2]);
}

/// Renders the update password form
/// Renders the update auto-generated password form (list selection + name change)
fn render_update_auto(f: &mut Frame, area: Rect, app: &App) {
    if app.password_list.is_empty() {
        let empty_msg = Paragraph::new("No passwords stored yet.\nCreate some passwords first!")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Update Auto-generated Password"),
            );
        f.render_widget(empty_msg, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .margin(2)
        .split(area);

    // If editing app name, show input field
    if app.is_editing {
        let app_name_input = Paragraph::new(app.app_name_input.value.as_str())
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .title("Edit Application Name (Enter to save, Esc to cancel)")
                    .borders(Borders::ALL),
            );
        f.render_widget(app_name_input, chunks[0]);
        
        let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
        let cursor_y = chunks[0].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    } else {
        // Show list for selection
        let items: Vec<ListItem> = app
            .password_list
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let content = format!("  {}  ", entry.app_name);
                let style = if i == app.selected_list_item {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!("Select Password to Update ({} total)", app.password_list.len()))
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[0]);
    }

    // Info section
    let info_text = format!(
        "ℹ️  Update Auto-generated Password\n\
        Select an application, change its name if needed, and a new {}-character password will be generated.\n\
        Example: Select 'gmail', press Enter, change name to 'gmail-work', press Enter again.",
        app.default_password_length
    );
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }
}

/// Renders the update custom password form (list selection + name and/or password change)
fn render_update_custom(f: &mut Frame, area: Rect, app: &App) {
    if app.password_list.is_empty() {
        let empty_msg = Paragraph::new("No passwords stored yet.\nCreate some passwords first!")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Update Custom Password"),
            );
        f.render_widget(empty_msg, area);
        return;
    }

    // If editing, show input fields
    if app.is_editing {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(3),
            ])
            .margin(2)
            .split(area);

        // App name input
        let app_name_style = if app.active_input == 0 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let app_name_input = Paragraph::new(app.app_name_input.value.as_str())
            .style(app_name_style)
            .block(
                Block::default()
                    .title("Application Name")
                    .borders(Borders::ALL),
            );
        f.render_widget(app_name_input, chunks[0]);

        // Password input
        let password_style = if app.active_input == 1 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let password_input = Paragraph::new(app.password_input.value.as_str())
            .style(password_style)
            .block(
                Block::default()
                    .title("Password")
                    .borders(Borders::ALL),
            );
        f.render_widget(password_input, chunks[1]);

        // Info section
        let info_text = "ℹ️  Update Custom Password\n\
                         Edit the application name and/or password, then press Enter to save.\n\
                         Example: Change 'gmail' to 'gmail-work' or update the password value.";
        let info = Paragraph::new(info_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Info"))
            .wrap(Wrap { trim: false });
        f.render_widget(info, chunks[2]);

        // Status message
        if !app.status_message.is_empty() {
            let status_color = if app.status_message.starts_with('✓') {
                Color::Green
            } else {
                Color::Red
            };
            let status = Paragraph::new(app.status_message.as_str())
                .style(Style::default().fg(status_color))
                .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(status, chunks[3]);
        }

        // Set cursor position
        if app.active_input == 0 {
            let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
            let cursor_y = chunks[0].y + 1;
            f.set_cursor_position((cursor_x, cursor_y));
        } else {
            let cursor_x = chunks[1].x + (app.password_input.cursor_position as u16).min(chunks[1].width.saturating_sub(2)) + 1;
            let cursor_y = chunks[1].y + 1;
            f.set_cursor_position((cursor_x, cursor_y));
        }
    } else {
        // Show list for selection
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(5),
                Constraint::Length(3),
            ])
            .margin(2)
            .split(area);

        let items: Vec<ListItem> = app
            .password_list
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let content = format!("  {}  ", entry.app_name);
                let style = if i == app.selected_list_item {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!("Select Password to Update ({} total)", app.password_list.len()))
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[0]);

        // Info section
        let info_text = "ℹ️  Update Custom Password\n\
                         Select an application to edit both its name and password.\n\
                         Example: Select 'gmail', then edit the name and/or password values.";
        let info = Paragraph::new(info_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Info"))
            .wrap(Wrap { trim: false });
        f.render_widget(info, chunks[1]);

        // Status message
        if !app.status_message.is_empty() {
            let status_color = if app.status_message.starts_with('✓') {
                Color::Green
            } else {
                Color::Red
            };
            let status = Paragraph::new(app.status_message.as_str())
                .style(Style::default().fg(status_color))
                .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(status, chunks[2]);
        }
    }
}

/// Renders the delete password form
fn render_delete(f: &mut Frame, area: Rect, app: &App) {
    if app.password_list.is_empty() {
        let empty_msg = Paragraph::new("No passwords stored yet.\nNothing to delete!")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Delete Password"),
            );
        f.render_widget(empty_msg, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .margin(2)
        .split(area);

    // Password list for selection - matching menu style
    let items: Vec<ListItem> = app
        .password_list
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if i == app.selected_list_item {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("  {}  ", entry.app_name)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Select Application to Delete")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, chunks[0]);

    // Info section
    let info_text = "ℹ️  Delete Password\n\
                     Select an application and press Enter to delete.\n\
                     Use ↑↓ to navigate, 'r' to refresh, Esc to cancel.";
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }
}

/// Renders the OTP generation form
fn render_generate_otp(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    // App name input
    let app_name_style = if app.active_input == 0 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let app_name_input = Paragraph::new(app.app_name_input.value.as_str())
        .style(app_name_style)
        .block(
            Block::default()
                .title("Application Name")
                .borders(Borders::ALL),
        );
    f.render_widget(app_name_input, chunks[0]);

    // TTL input
    let ttl_style = if app.active_input == 1 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let ttl_input = Paragraph::new(app.length_input.value.as_str())
        .style(ttl_style)
        .block(
            Block::default()
                .title("TTL in seconds (default: 300 seconds = 5 minutes)")
                .borders(Borders::ALL),
        );
    f.render_widget(ttl_input, chunks[1]);

    // Info section
    let info_text = format!(
        "ℹ️  Generate OTP (One-Time Password)\n\
         Creates a temporary {}-character password saved to keyring with automatic expiration.\n\
         The OTP will be automatically deleted from keyring after TTL expires.\n\
         Example: 'Gmail' with TTL '12' → password saved for 12 seconds, then auto-deleted.",
        app.default_password_length
    );
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[2]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[3]);
    }

    // Set cursor position
    if app.active_input == 0 {
        let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
        let cursor_y = chunks[0].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    } else {
        let cursor_x = chunks[1].x + (app.length_input.cursor_position as u16).min(chunks[1].width.saturating_sub(2)) + 1;
        let cursor_y = chunks[1].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

/// Renders the memorizable password generation form
fn render_memorizable(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    let app_name_input = Paragraph::new(app.app_name_input.value.as_str())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("Application Name")
                .borders(Borders::ALL),
        );
    f.render_widget(app_name_input, chunks[0]);

    // Info section
    let info_text = "ℹ️  Generate Memorizable Password\n\
                     Creates easy-to-remember passwords (e.g., Tiger-42-Cloud).\n\
                     Example: Enter 'blog' to generate a memorable password.";
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }

    // Set cursor position
    let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
    let cursor_y = chunks[0].y + 1;
    f.set_cursor_position((cursor_x, cursor_y));
}

/// Renders the export passwords form
fn render_export(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    let file_path_input = Paragraph::new(app.app_name_input.value.as_str())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("File Path (e.g., passwords.csv)")
                .borders(Borders::ALL),
        );
    f.render_widget(file_path_input, chunks[0]);

    // Info section
    let info_text = "ℹ️  Export Passwords to CSV\n\
                     Exports all stored passwords to a CSV file for backup.\n\
                     Example: Enter 'my_passwords.csv' to create an export file.";
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }

    // Set cursor position
    let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
    let cursor_y = chunks[0].y + 1;
    f.set_cursor_position((cursor_x, cursor_y));
}

/// Renders the import passwords form
fn render_import(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    let file_path_input = Paragraph::new(app.app_name_input.value.as_str())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("File Path (e.g., passwords.csv)")
                .borders(Borders::ALL),
        );
    f.render_widget(file_path_input, chunks[0]);

    // Info section
    let info_text = "ℹ️  Import Passwords from CSV\n\
                     Imports passwords from a CSV file into the keyring.\n\
                     Example: Enter 'my_passwords.csv' to import from that file.";
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: false });
    f.render_widget(info, chunks[1]);

    // Status message
    if !app.status_message.is_empty() {
        let status_color = if app.status_message.starts_with('✓') {
            Color::Green
        } else {
            Color::Red
        };
        let status = Paragraph::new(app.status_message.as_str())
            .style(Style::default().fg(status_color))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }

    // Set cursor position
    let cursor_x = chunks[0].x + (app.app_name_input.cursor_position as u16).min(chunks[0].width.saturating_sub(2)) + 1;
    let cursor_y = chunks[0].y + 1;
    f.set_cursor_position((cursor_x, cursor_y));
}
