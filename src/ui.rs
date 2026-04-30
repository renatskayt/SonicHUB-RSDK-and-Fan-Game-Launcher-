use crate::config::{AppConfig, EngineVersion, GameProfile};
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, CssProvider,
    Dialog, DropDown, Entry, FileDialog, FileFilter,
    Label, ListBox, ListBoxRow, Orientation, Picture,
    ResponseType, ScrolledWindow, SearchEntry, Separator, StringList, Switch,
    gdk,
};
use std::cell::RefCell;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;

const CSS: &str = r#"
window {
    background-color: #1a1a2e;
}
.sidebar {
    background-color: #16213e;
    border-right: 1px solid #0f3460;
    padding: 0;
}
.sidebar-header {
    padding: 16px 20px;
    background: linear-gradient(180deg, #0f3460, #16213e);
}
.sidebar-title {
    font-size: 18px;
    font-weight: 800;
    color: #e94560;
    letter-spacing: 2px;
}
.game-row {
    padding: 12px 20px;
    border-bottom: 1px solid rgba(15, 52, 96, 0.5);
    transition: background-color 200ms ease;
}
.game-row:hover {
    background-color: rgba(233, 69, 96, 0.1);
}
.game-row:selected {
    background-color: rgba(233, 69, 96, 0.25);
}
.game-name {
    font-size: 14px;
    font-weight: 600;
    color: #eee;
}
.game-engine {
    font-size: 11px;
    color: #888;
    margin-top: 2px;
}
.detail-panel {
    padding: 32px;
}
.detail-title {
    font-size: 28px;
    font-weight: 800;
    color: #fff;
    margin-bottom: 8px;
}
.detail-subtitle {
    font-size: 13px;
    color: #aaa;
    margin-bottom: 24px;
}
.info-label {
    font-size: 12px;
    color: #888;
    margin-top: 12px;
}
.info-value {
    font-size: 13px;
    color: #ccc;
    margin-top: 2px;
}
.launch-button {
    background: linear-gradient(135deg, #e94560, #c0392b);
    color: white;
    font-size: 13px;
    font-weight: 700;
    padding: 8px 20px;
    border-radius: 8px;
    border: none;
    transition: all 200ms ease;
}
.launch-button:hover {
    background: linear-gradient(135deg, #ff6b81, #e94560);
    box-shadow: 0 4px 15px rgba(233, 69, 96, 0.4);
}
.add-button {
    background-color: rgba(233, 69, 96, 0.15);
    color: #e94560;
    font-size: 13px;
    font-weight: 600;
    border-radius: 6px;
    border: 1px solid rgba(233, 69, 96, 0.3);
    padding: 8px 20px;
}
.add-button:hover {
    background-color: rgba(233, 69, 96, 0.3);
}
.delete-button {
    background-color: rgba(255, 50, 50, 0.1);
    color: #ff4444;
    font-weight: 600;
    border-radius: 6px;
    border: 1px solid rgba(255, 50, 50, 0.2);
    padding: 8px 16px;
    margin-top: 16px;
}
.delete-button:hover {
    background-color: rgba(255, 50, 50, 0.25);
}
.empty-state {
    color: #555;
    font-size: 16px;
}
.empty-icon {
    font-size: 48px;
    color: #333;
    margin-bottom: 12px;
}
.cover-placeholder {
    background: linear-gradient(135deg, #0f3460, #1a1a2e);
    border-radius: 12px;
    border: 1px solid #0f3460;
    min-height: 180px;
    min-width: 320px;
    margin-bottom: 20px;
}
.cover-text {
    color: #e94560;
    font-size: 40px;
    font-weight: 800;
}
.dialog-content {
    padding: 20px;
}
.dialog-content entry {
    margin-top: 4px;
    margin-bottom: 12px;
    padding: 8px;
    background-color: #16213e;
    color: #eee;
    border: 1px solid #0f3460;
    border-radius: 6px;
}
.dialog-label {
    color: #aaa;
    font-size: 12px;
    font-weight: 600;
}
.version-badge {
    background-color: rgba(233, 69, 96, 0.2);
    color: #e94560;
    font-size: 11px;
    font-weight: 700;
    padding: 4px 10px;
    border-radius: 4px;
}
"#;

pub fn build_ui(app: &Application) {
    // Load CSS
    let provider = CssProvider::new();
    provider.load_from_string(CSS);
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("No display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let config = Rc::new(RefCell::new(AppConfig::load()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SonicHub Launcher")
        .default_width(960)
        .default_height(640)
        .build();

    let main_box = GtkBox::new(Orientation::Horizontal, 0);

    // === SIDEBAR ===
    let sidebar = GtkBox::new(Orientation::Vertical, 0);
    sidebar.add_css_class("sidebar");
    sidebar.set_width_request(260);

    let sidebar_header = GtkBox::new(Orientation::Vertical, 4);
    sidebar_header.add_css_class("sidebar-header");
    let title_label = Label::new(Some("SONICHUB"));
    title_label.add_css_class("sidebar-title");
    title_label.set_halign(Align::Start);
    sidebar_header.append(&title_label);
    sidebar.append(&sidebar_header);

    let separator = Separator::new(Orientation::Horizontal);
    sidebar.append(&separator);

    let list_scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .build();
    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::Single);
    list_scroll.set_child(Some(&listbox));
    sidebar.append(&list_scroll);

    let add_btn = Button::with_label("＋  Add Game");
    add_btn.add_css_class("add-button");
    sidebar.append(&add_btn);

    main_box.append(&sidebar);

    // === DETAIL PANEL ===
    let detail_scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    let detail_panel = GtkBox::new(Orientation::Vertical, 0);
    detail_panel.add_css_class("detail-panel");
    detail_panel.set_halign(Align::Start);
    detail_panel.set_valign(Align::Start);

    // Empty state
    let empty_box = GtkBox::new(Orientation::Vertical, 8);
    empty_box.set_halign(Align::Center);
    empty_box.set_valign(Align::Center);
    empty_box.set_vexpand(true);
    empty_box.set_hexpand(true);
    let empty_icon = Label::new(Some("🎮"));
    empty_icon.add_css_class("empty-icon");
    let empty_label = Label::new(Some("No game selected"));
    empty_label.add_css_class("empty-state");
    empty_box.append(&empty_icon);
    empty_box.append(&empty_label);

    // Detail content (hidden initially)
    let detail_content = GtkBox::new(Orientation::Vertical, 0);
    detail_content.set_visible(false);

    // Cover: Picture for banner or placeholder
    let cover_box = GtkBox::new(Orientation::Vertical, 0);
    cover_box.add_css_class("cover-placeholder");
    cover_box.set_halign(Align::Start);

    let cover_picture = Picture::new();
    cover_picture.set_content_fit(gtk4::ContentFit::Cover);
    cover_picture.set_size_request(320, 180);
    cover_picture.set_visible(false);
    cover_box.append(&cover_picture);

    let cover_label = Label::new(Some(""));
    cover_label.add_css_class("cover-text");
    cover_label.set_halign(Align::Center);
    cover_label.set_valign(Align::Center);
    cover_label.set_vexpand(true);
    cover_box.append(&cover_label);
    detail_content.append(&cover_box);

    let detail_title = Label::new(Some(""));
    detail_title.add_css_class("detail-title");
    detail_title.set_halign(Align::Start);
    detail_content.append(&detail_title);

    let detail_engine = Label::new(Some(""));
    detail_engine.add_css_class("detail-subtitle");
    detail_engine.set_halign(Align::Start);
    detail_content.append(&detail_engine);

    let lbl_data = Label::new(Some("DATA FILE"));
    lbl_data.add_css_class("info-label");
    lbl_data.set_halign(Align::Start);
    detail_content.append(&lbl_data);
    let val_data = Label::new(Some(""));
    val_data.add_css_class("info-value");
    val_data.set_halign(Align::Start);
    val_data.set_wrap(true);
    detail_content.append(&val_data);

    let lbl_exe = Label::new(Some("EXECUTABLE"));
    lbl_exe.add_css_class("info-label");
    lbl_exe.set_halign(Align::Start);
    detail_content.append(&lbl_exe);
    let val_exe = Label::new(Some(""));
    val_exe.add_css_class("info-value");
    val_exe.set_halign(Align::Start);
    val_exe.set_wrap(true);
    detail_content.append(&val_exe);

    // Buttons row
    let btn_row = GtkBox::new(Orientation::Horizontal, 8);
    btn_row.set_halign(Align::Start);
    btn_row.set_margin_top(16);
    btn_row.set_margin_bottom(4);
    let launch_btn = Button::with_label("▶  Launch Game");
    launch_btn.add_css_class("launch-button");
    btn_row.append(&launch_btn);
    let banner_btn = Button::with_label("🖼  Set Banner");
    banner_btn.add_css_class("add-button");
    btn_row.append(&banner_btn);
    detail_content.append(&btn_row);

    // === MODS SECTION ===
    let mods_separator = Separator::new(Orientation::Horizontal);
    mods_separator.set_margin_top(12);
    mods_separator.set_margin_bottom(8);
    detail_content.append(&mods_separator);

    let mods_header = GtkBox::new(Orientation::Horizontal, 6);
    mods_header.set_valign(Align::Center);
    let mods_title = Label::new(Some("MODS"));
    mods_title.add_css_class("info-label");
    mods_title.set_halign(Align::Start);
    mods_header.append(&mods_title);
    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    mods_header.append(&spacer);
    let add_mod_btn = Button::with_label("＋ Add Mod");
    add_mod_btn.add_css_class("add-button");
    mods_header.append(&add_mod_btn);
    let gb_browse_btn = Button::with_label("🌐 GameBanana");
    gb_browse_btn.add_css_class("launch-button");
    mods_header.append(&gb_browse_btn);
    let refresh_mods_btn = Button::with_label("↻ Refresh");
    refresh_mods_btn.add_css_class("add-button");
    mods_header.append(&refresh_mods_btn);
    detail_content.append(&mods_header);

    let mods_scroll = ScrolledWindow::builder()
        .vexpand(false)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_height(120)
        .max_content_height(250)
        .build();
    let mods_list = ListBox::new();
    mods_list.set_selection_mode(gtk4::SelectionMode::None);
    mods_list.add_css_class("sidebar");
    mods_scroll.set_child(Some(&mods_list));
    detail_content.append(&mods_scroll);

    let mods_empty = Label::new(Some("No mods folder configured"));
    mods_empty.add_css_class("empty-state");
    mods_empty.set_margin_top(8);
    detail_content.append(&mods_empty);

    let delete_btn = Button::with_label("Delete Profile");
    delete_btn.add_css_class("delete-button");
    delete_btn.set_halign(Align::Start);
    delete_btn.set_margin_top(16);
    detail_content.append(&delete_btn);

    detail_panel.append(&empty_box);
    detail_panel.append(&detail_content);
    detail_scroll.set_child(Some(&detail_panel));
    main_box.append(&detail_scroll);

    window.set_child(Some(&main_box));

    // === STATE ===
    let selected_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    // Populate list
    let populate = {
        let listbox = listbox.clone();
        let config = config.clone();
        move || {
            // Remove all rows
            while let Some(child) = listbox.first_child() {
                listbox.remove(&child);
            }
            let cfg = config.borrow();
            for game in &cfg.games {
                let row = ListBoxRow::new();
                let row_box = GtkBox::new(Orientation::Vertical, 2);
                row_box.add_css_class("game-row");

                let name = Label::new(Some(&format!(
                    "{} {}", crate::gamebanana::engine_icon(game.engine_version.as_str()), game.name
                )));
                name.add_css_class("game-name");
                name.set_halign(Align::Start);

                let engine_box = GtkBox::new(Orientation::Horizontal, 6);
                let engine = Label::new(Some(game.engine_version.as_str()));
                engine.add_css_class("version-badge");
                engine_box.append(&engine);

                row_box.append(&name);
                row_box.append(&engine_box);
                row.set_child(Some(&row_box));

                // Store game ID in widget name
                row.set_widget_name(&game.id);
                listbox.append(&row);
            }
        }
    };

    let populate = Rc::new(populate);
    populate();

    // Show detail for selected game
    let show_detail = {
        let config = config.clone();
        let detail_content = detail_content.clone();
        let empty_box = empty_box.clone();
        let detail_title = detail_title.clone();
        let detail_engine = detail_engine.clone();
        let val_data = val_data.clone();
        let val_exe = val_exe.clone();
        let cover_label = cover_label.clone();
        let cover_picture = cover_picture.clone();
        let selected_id = selected_id.clone();
        let mods_list = mods_list.clone();
        let mods_empty = mods_empty.clone();
        move |game_id: &str| {
            let cfg = config.borrow();
            if let Some(game) = cfg.get_game(game_id) {
                *selected_id.borrow_mut() = Some(game.id.clone());
                detail_title.set_text(&game.name);
                detail_engine.set_text(&format!("Engine: {}", game.engine_version.as_str()));
                val_data.set_text(&game.data_path);
                val_exe.set_text(&game.executable_path);

                // Cover image
                if !game.cover_image.is_empty() && Path::new(&game.cover_image).exists() {
                    cover_picture.set_filename(Some(&game.cover_image));
                    cover_picture.set_visible(true);
                    cover_label.set_visible(false);
                } else {
                    cover_picture.set_visible(false);
                    cover_label.set_visible(true);
                    let initials: String = game.name.chars().take(2).collect();
                    cover_label.set_text(&initials.to_uppercase());
                }

                // Populate mods
                populate_mods(&mods_list, &mods_empty, &game.mods_folder);

                empty_box.set_visible(false);
                detail_content.set_visible(true);
            }
        }
    };
    let show_detail = Rc::new(show_detail);

    // List selection
    {
        let show_detail = show_detail.clone();
        listbox.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                let id = row.widget_name().to_string();
                show_detail(&id);
            }
        });
    }

    // Launch button
    {
        let config = config.clone();
        let selected_id = selected_id.clone();
        let window_weak = window.downgrade();
        launch_btn.connect_clicked(move |_| {
            let sel = selected_id.borrow().clone();
            if let Some(id) = sel {
                let cfg = config.borrow();
                if let Some(game) = cfg.get_game(&id) {
                    let exe = game.executable_path.clone();
                    let data = game.data_path.clone();
                    let engine = game.engine_version.clone();
                    drop(cfg);

                    let show_error = |win: &ApplicationWindow, msg: &str| {
                        let dlg = gtk4::AlertDialog::builder()
                            .message(msg)
                            .build();
                        dlg.show(Some(win));
                    };

                    let win_ref = window_weak.upgrade();

                    if exe.is_empty() {
                        if let Some(ref w) = win_ref {
                            show_error(w, "Executable path is empty!");
                        }
                        return;
                    }

                    // Determine launch mode
                    let use_wine = matches!(engine, EngineVersion::Sonic1Forever | EngineVersion::Sonic2Absolute);
                    let is_command = exe.starts_with("flatpak ") || exe.contains(' ');
                    let is_s3air = matches!(engine, EngineVersion::Sonic3AIR);

                    // Validate executable exists (skip for commands like "flatpak run ...")
                    if !is_command {
                        let exe_path = Path::new(&exe);
                        if !exe_path.exists() {
                            if let Some(ref w) = win_ref {
                                show_error(w, &format!("Executable not found:\n{}", exe));
                            }
                            return;
                        }

                        // Set execute permission
                        use std::os::unix::fs::PermissionsExt;
                        if let Ok(meta) = std::fs::metadata(&exe) {
                            let mut perms = meta.permissions();
                            let mode = perms.mode() | 0o111;
                            perms.set_mode(mode);
                            let _ = std::fs::set_permissions(&exe, perms);
                        }
                    }

                    // Deploy Data.rsdk (only for RSDK engines and S1 Forever)
                    let needs_data = matches!(engine,
                        EngineVersion::RSDKv3 | EngineVersion::RSDKv4 |
                        EngineVersion::RSDKv5 | EngineVersion::Sonic1Forever
                    );

                    if needs_data && !data.is_empty() {
                        let data_path = Path::new(&data);
                        if !data_path.exists() {
                            if let Some(ref w) = win_ref {
                                show_error(w, &format!("Data file not found:\n{}", data));
                            }
                            return;
                        }
                        let exe_path = Path::new(&exe);
                        if let Some(exe_dir) = exe_path.parent() {
                            let target = exe_dir.join("Data.rsdk");
                            let same = std::fs::canonicalize(&data)
                                .and_then(|d| std::fs::canonicalize(&target).map(|t| d == t))
                                .unwrap_or(false);
                            if !same {
                                let _ = std::fs::remove_file(&target);
                                if let Err(e) = std::os::unix::fs::symlink(&data, &target) {
                                    if let Some(ref w) = win_ref {
                                        show_error(w, &format!("Symlink failed: {}", e));
                                    }
                                    return;
                                }
                            }
                        }
                    }

                    // Build and run command
                    let mut cmd;
                    if is_command {
                        // Command mode: "flatpak run org.sonic3air.Sonic3AIR" etc.
                        let parts: Vec<&str> = exe.split_whitespace().collect();
                        cmd = Command::new(parts[0]);
                        for arg in &parts[1..] {
                            cmd.arg(arg);
                        }
                    } else if use_wine {
                        // Wine mode for S1 Forever / S2 Absolute
                        cmd = Command::new("wine");
                        cmd.arg(&exe);
                        if let Some(dir) = Path::new(&exe).parent() {
                            cmd.current_dir(dir);
                        }
                    } else {
                        // Native RSDK executable
                        cmd = Command::new(&exe);
                        if let Some(dir) = Path::new(&exe).parent() {
                            cmd.current_dir(dir);
                        }
                    };

                    cmd.stdin(std::process::Stdio::null());
                    cmd.stdout(std::process::Stdio::null());
                    cmd.stderr(std::process::Stdio::null());

                    match cmd.spawn() {
                        Ok(_) => eprintln!("Launched: {}", exe),
                        Err(e) => {
                            if let Some(ref w) = win_ref {
                                show_error(w, &format!("Launch failed:\n{}\n\nError: {}", exe, e));
                            }
                        }
                    }
                }
            }
        });
    }

    // Delete button
    {
        let config = config.clone();
        let selected_id = selected_id.clone();
        let populate = populate.clone();
        let detail_content = detail_content.clone();
        let empty_box = empty_box.clone();
        delete_btn.connect_clicked(move |_| {
            let sel = selected_id.borrow().clone();
            if let Some(id) = sel {
                {
                    let mut cfg = config.borrow_mut();
                    cfg.remove_game(&id);
                    let _ = cfg.save();
                }
                *selected_id.borrow_mut() = None;
                populate();
                detail_content.set_visible(false);
                empty_box.set_visible(true);
            }
        });
    }

    // Banner button
    {
        let config = config.clone();
        let selected_id = selected_id.clone();
        let cover_picture = cover_picture.clone();
        let cover_label = cover_label.clone();
        let window_weak = window.downgrade();
        banner_btn.connect_clicked(move |_| {
            let Some(win) = window_weak.upgrade() else { return };
            let sel = selected_id.borrow().clone();
            let Some(id) = sel else { return };
            let config = config.clone();
            let cover_picture = cover_picture.clone();
            let cover_label = cover_label.clone();
            let filter = FileFilter::new();
            filter.add_mime_type("image/*");
            filter.set_name(Some("Image files"));
            let filters = gtk4::gio::ListStore::new::<FileFilter>();
            filters.append(&filter);
            let file_dlg = FileDialog::builder()
                .title("Select Banner Image")
                .filters(&filters)
                .build();
            file_dlg.open(Some(&win), gtk4::gio::Cancellable::NONE, move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        let path_str = path.to_string_lossy().to_string();
                        let mut cfg = config.borrow_mut();
                        if let Some(game) = cfg.games.iter_mut().find(|g| g.id == id) {
                            game.cover_image = path_str.clone();
                        }
                        let _ = cfg.save();
                        cover_picture.set_filename(Some(&path_str));
                        cover_picture.set_visible(true);
                        cover_label.set_visible(false);
                    }
                }
            });
        });
    }

    // Refresh mods button
    {
        let config = config.clone();
        let selected_id = selected_id.clone();
        let mods_list = mods_list.clone();
        let mods_empty = mods_empty.clone();
        refresh_mods_btn.connect_clicked(move |_| {
            let sel = selected_id.borrow().clone();
            if let Some(id) = sel {
                let cfg = config.borrow();
                if let Some(game) = cfg.get_game(&id) {
                    populate_mods(&mods_list, &mods_empty, &game.mods_folder);
                }
            }
        });
    }

    // Add mod button - copy mod folder into mods dir
    {
        let config = config.clone();
        let selected_id = selected_id.clone();
        let mods_list = mods_list.clone();
        let mods_empty = mods_empty.clone();
        let window_weak = window.downgrade();
        add_mod_btn.connect_clicked(move |_| {
            let Some(win) = window_weak.upgrade() else { return };
            let sel = selected_id.borrow().clone();
            let Some(id) = sel else { return };
            let cfg = config.borrow();
            let Some(game) = cfg.get_game(&id) else { return };
            let mods_folder = game.mods_folder.clone();
            drop(cfg);

            if mods_folder.is_empty() {
                let dlg = gtk4::AlertDialog::builder()
                    .message("Mods folder not configured for this game")
                    .build();
                dlg.show(Some(&win));
                return;
            }

            // Create mods folder if it doesn't exist
            let _ = std::fs::create_dir_all(&mods_folder);

            let config = config.clone();
            let mods_list = mods_list.clone();
            let mods_empty = mods_empty.clone();
            let id = id.clone();
            let file_dlg = FileDialog::builder()
                .title("Select Mod Folder")
                .build();
            file_dlg.select_folder(Some(&win), gtk4::gio::Cancellable::NONE, move |result| {
                if let Ok(file) = result {
                    if let Some(src) = file.path() {
                        let mod_name = src.file_name().unwrap_or_default();
                        let dest = Path::new(&mods_folder).join(mod_name);
                        if dest.exists() {
                            eprintln!("Mod already exists: {:?}", dest);
                            return;
                        }
                        // Copy recursively
                        if let Err(e) = copy_dir_recursive(&src, &dest) {
                            eprintln!("Copy failed: {}", e);
                            return;
                        }
                        // Enable mod in modconfig
                        let mod_name_str = mod_name.to_string_lossy().to_string();
                        let mut mcfg = read_modconfig(&mods_folder);
                        mcfg.insert(mod_name_str, true);
                        write_modconfig(&mods_folder, &mcfg);
                        // Refresh
                        let cfg = config.borrow();
                        if let Some(game) = cfg.get_game(&id) {
                            populate_mods(&mods_list, &mods_empty, &game.mods_folder);
                        }
                    }
                }
            });
        });
    }

    // Add button
    {
        let config = config.clone();
        let populate = populate.clone();
        let window_weak = window.downgrade();
        add_btn.connect_clicked(move |_| {
            let Some(win) = window_weak.upgrade() else { return };
            show_add_dialog(&win, config.clone(), populate.clone());
        });
    }

    // GameBanana browse button
    {
        let config = config.clone();
        let selected_id = selected_id.clone();
        let mods_list = mods_list.clone();
        let mods_empty = mods_empty.clone();
        let window_weak = window.downgrade();
        gb_browse_btn.connect_clicked(move |_| {
            let Some(win) = window_weak.upgrade() else { return };
            let sel = selected_id.borrow().clone();
            let Some(id) = sel else { return };
            let cfg = config.borrow();
            let Some(game) = cfg.get_game(&id) else { return };
            let engine = game.engine_version.as_str().to_string();
            let mods_folder = game.mods_folder.clone();
            drop(cfg);
            show_gb_browser(&win, &engine, &mods_folder, mods_list.clone(), mods_empty.clone());
        });
    }

    window.present();
}

fn read_modconfig(mods_folder: &str) -> std::collections::HashMap<String, bool> {
    let path = Path::new(mods_folder).join("modconfig.ini");
    let mut map = std::collections::HashMap::new();
    if let Ok(content) = std::fs::read_to_string(&path) {
        let mut in_mods = false;
        for line in content.lines() {
            let line = line.trim();
            if line.eq_ignore_ascii_case("[mods]") {
                in_mods = true;
                continue;
            }
            if line.starts_with('[') {
                in_mods = false;
                continue;
            }
            if in_mods {
                if let Some((name, val)) = line.split_once('=') {
                    let v = val.trim();
                    map.insert(
                        name.trim().to_string(),
                        v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("y") || v == "1",
                    );
                }
            }
        }
    }
    map
}

fn write_modconfig(mods_folder: &str, mods: &std::collections::HashMap<String, bool>) {
    let path = Path::new(mods_folder).join("modconfig.ini");

    // Read existing file, preserve non-[Mods] sections
    let mut other_sections = String::new();
    if let Ok(content) = std::fs::read_to_string(&path) {
        let mut in_mods = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.eq_ignore_ascii_case("[mods]") {
                in_mods = true;
                continue;
            }
            if trimmed.starts_with('[') {
                in_mods = false;
            }
            if !in_mods {
                other_sections.push_str(line);
                other_sections.push('\n');
            }
        }
    }

    let mut content = other_sections;
    content.push_str("[Mods]\n");
    for (name, enabled) in mods {
        content.push_str(&format!("{}={}\n", name, if *enabled { "y" } else { "n" }));
    }
    let _ = std::fs::write(&path, content);
}

fn read_mod_display_name(mod_path: &Path) -> Option<String> {
    let ini = mod_path.join("mod.ini");
    if let Ok(content) = std::fs::read_to_string(&ini) {
        for line in content.lines() {
            let line = line.trim().trim_start_matches('\u{feff}');
            if let Some(name) = line.strip_prefix("Name=") {
                let n = name.trim().to_string();
                if !n.is_empty() {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn populate_mods(mods_list: &ListBox, mods_empty: &Label, mods_folder: &str) {
    while let Some(child) = mods_list.first_child() {
        mods_list.remove(&child);
    }

    if mods_folder.is_empty() || !Path::new(mods_folder).is_dir() {
        mods_empty.set_text(if mods_folder.is_empty() {
            "No mods folder configured"
        } else {
            "Mods folder not found"
        });
        mods_empty.set_visible(true);
        return;
    }

    let mod_dirs: Vec<_> = match std::fs::read_dir(mods_folder) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir() && e.file_name() != "." && e.file_name() != "..")
            .collect(),
        Err(_) => {
            mods_empty.set_text("Cannot read mods folder");
            mods_empty.set_visible(true);
            return;
        }
    };

    if mod_dirs.is_empty() {
        mods_empty.set_text("No mods found");
        mods_empty.set_visible(true);
        return;
    }

    mods_empty.set_visible(false);
    let modconfig = read_modconfig(mods_folder);

    for entry in &mod_dirs {
        let folder_name = entry.file_name().to_string_lossy().to_string();
        if folder_name == "modconfig.ini" { continue; }

        let display_name = read_mod_display_name(&entry.path())
            .unwrap_or_else(|| folder_name.clone());
        let is_enabled = modconfig.get(&folder_name).copied().unwrap_or(false);

        let row = ListBoxRow::new();
        let hbox = GtkBox::new(Orientation::Horizontal, 8);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);
        hbox.set_margin_top(6);
        hbox.set_margin_bottom(6);

        let label = Label::new(Some(&display_name));
        label.add_css_class("game-name");
        label.set_halign(Align::Start);
        label.set_hexpand(true);

        let switch = Switch::new();
        switch.set_active(is_enabled);
        switch.set_valign(Align::Center);

        let mf = mods_folder.to_string();
        let fn_clone = folder_name.clone();
        switch.connect_state_set(move |_, enabled| {
            let mut cfg = read_modconfig(&mf);
            cfg.insert(fn_clone.clone(), enabled);
            write_modconfig(&mf, &cfg);
            gtk4::glib::Propagation::Proceed
        });

        // Delete button
        let del_btn = Button::with_label("🗑");
        del_btn.add_css_class("delete-button");
        del_btn.set_valign(Align::Center);
        del_btn.set_tooltip_text(Some("Delete mod"));

        let mf2 = mods_folder.to_string();
        let fn2 = folder_name.clone();
        let dn2 = display_name.clone();
        let ml = mods_list.clone();
        let me = mods_empty.clone();
        del_btn.connect_clicked(move |btn| {
            let mod_path = Path::new(&mf2).join(&fn2);
            let mf_ref = mf2.clone();
            let fn_ref = fn2.clone();
            let ml = ml.clone();
            let me = me.clone();

            // Confirm deletion
            let dlg = gtk4::AlertDialog::builder()
                .message(&format!("Delete \"{}\"?\n\nThis will permanently remove the mod folder.", dn2))
                .buttons(["Cancel", "Delete"])
                .build();

            let btn_weak = btn.downgrade();
            dlg.choose(
                btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok()).as_ref(),
                gtk4::gio::Cancellable::NONE,
                move |result| {
                    if result == Ok(1) {
                        // Delete folder
                        let _ = std::fs::remove_dir_all(&mod_path);
                        // Remove from modconfig
                        let mut cfg = read_modconfig(&mf_ref);
                        cfg.remove(&fn_ref);
                        write_modconfig(&mf_ref, &cfg);
                        // Refresh list
                        populate_mods(&ml, &me, &mf_ref);
                    }
                },
            );
        });

        hbox.append(&label);
        hbox.append(&switch);
        hbox.append(&del_btn);
        row.set_child(Some(&hbox));
        mods_list.append(&row);
    }
}

fn show_add_dialog(
    parent: &ApplicationWindow,
    config: Rc<RefCell<AppConfig>>,
    populate: Rc<dyn Fn()>,
) {
    let dialog = Dialog::builder()
        .title("Add Game")
        .transient_for(parent)
        .modal(true)
        .default_width(420)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Add", ResponseType::Accept);

    let content = dialog.content_area();
    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.add_css_class("dialog-content");

    let lbl_name = Label::new(Some("GAME NAME"));
    lbl_name.add_css_class("dialog-label");
    lbl_name.set_halign(Align::Start);
    let entry_name = Entry::new();
    entry_name.set_placeholder_text(Some("Sonic the Hedgehog"));

    let lbl_data = Label::new(Some("DATA.RSDK PATH"));
    lbl_data.add_css_class("dialog-label");
    lbl_data.set_halign(Align::Start);
    let data_box = GtkBox::new(Orientation::Horizontal, 4);
    let entry_data = Entry::builder().hexpand(true).build();
    entry_data.set_placeholder_text(Some("/path/to/Data.rsdk"));
    let browse_data = Button::with_label("...");
    data_box.append(&entry_data);
    data_box.append(&browse_data);

    let lbl_exe = Label::new(Some("EXECUTABLE PATH"));
    lbl_exe.add_css_class("dialog-label");
    lbl_exe.set_halign(Align::Start);
    let exe_box = GtkBox::new(Orientation::Horizontal, 4);
    let entry_exe = Entry::builder().hexpand(true).build();
    entry_exe.set_placeholder_text(Some("/path/to/RSDKv5U"));
    let browse_exe = Button::with_label("...");
    exe_box.append(&entry_exe);
    exe_box.append(&browse_exe);

    let lbl_ver = Label::new(Some("ENGINE VERSION"));
    lbl_ver.add_css_class("dialog-label");
    lbl_ver.set_halign(Align::Start);
    let versions = StringList::new(&["RSDKv3", "RSDKv4", "RSDKv5", "Sonic 1 Forever", "Sonic 2 Absolute", "Sonic 3 AIR"]);
    let dropdown = DropDown::new(Some(versions), gtk4::Expression::NONE);
    dropdown.set_selected(2);

    // Dynamic labels based on engine selection
    {
        let lbl_data = lbl_data.clone();
        let lbl_exe = lbl_exe.clone();
        let entry_data = entry_data.clone();
        let entry_exe = entry_exe.clone();
        let data_box = data_box.clone();
        dropdown.connect_selected_notify(move |dd| {
            match dd.selected() {
                3 => { // Sonic 1 Forever (needs Data.rsdk, wine)
                    lbl_data.set_text("DATA.RSDK PATH");
                    entry_data.set_placeholder_text(Some("/path/to/Data.rsdk"));
                    lbl_exe.set_text("EXECUTABLE (.exe for Wine)");
                    entry_exe.set_placeholder_text(Some("/path/to/Sonic1Forever.exe"));
                    data_box.set_visible(true);
                    lbl_data.set_visible(true);
                }
                4 => { // Sonic 2 Absolute (no data.rsdk, wine)
                    lbl_data.set_text("GAME FOLDER");
                    entry_data.set_placeholder_text(Some("(not needed)"));
                    lbl_exe.set_text("EXECUTABLE (.exe for Wine)");
                    entry_exe.set_placeholder_text(Some("/path/to/Sonic2Absolute.exe"));
                    data_box.set_visible(false);
                    lbl_data.set_visible(false);
                }
                5 => { // Sonic 3 AIR (flatpak or native)
                    lbl_data.set_text("GAME FOLDER");
                    entry_data.set_placeholder_text(Some("(not needed)"));
                    lbl_exe.set_text("LAUNCH COMMAND");
                    entry_exe.set_placeholder_text(Some("flatpak run org.sonic3air.Sonic3AIR"));
                    data_box.set_visible(false);
                    lbl_data.set_visible(false);
                }
                _ => { // RSDK engines
                    lbl_data.set_text("DATA.RSDK PATH");
                    entry_data.set_placeholder_text(Some("/path/to/Data.rsdk"));
                    lbl_exe.set_text("EXECUTABLE PATH");
                    entry_exe.set_placeholder_text(Some("/path/to/RSDKv5U"));
                    data_box.set_visible(true);
                    lbl_data.set_visible(true);
                }
            }
        });
    }

    let lbl_mods = Label::new(Some("MODS FOLDER (optional)"));
    lbl_mods.add_css_class("dialog-label");
    lbl_mods.set_halign(Align::Start);
    let entry_mods = Entry::new();
    entry_mods.set_placeholder_text(Some("/path/to/mods/"));

    vbox.append(&lbl_name);
    vbox.append(&entry_name);
    vbox.append(&lbl_ver);
    vbox.append(&dropdown);
    vbox.append(&lbl_data);
    vbox.append(&data_box);
    vbox.append(&lbl_exe);
    vbox.append(&exe_box);
    vbox.append(&lbl_mods);
    vbox.append(&entry_mods);
    content.append(&vbox);

    // Browse Data.rsdk
    {
        let entry = entry_data.clone();
        let dialog_weak = dialog.downgrade();
        browse_data.connect_clicked(move |_| {
            let entry = entry.clone();
            if let Some(dlg) = dialog_weak.upgrade() {
                let filter = FileFilter::new();
                filter.add_pattern("*.rsdk");
                filter.set_name(Some("RSDK Data files"));
                let filters = gtk4::gio::ListStore::new::<FileFilter>();
                filters.append(&filter);
                let file_dlg = FileDialog::builder()
                    .title("Select Data.rsdk")
                    .filters(&filters)
                    .build();
                file_dlg.open(Some(&dlg), gtk4::gio::Cancellable::NONE, move |result| {
                    if let Ok(file) = result {
                        if let Some(path) = file.path() {
                            entry.set_text(&path.to_string_lossy());
                        }
                    }
                });
            }
        });
    }

    // Browse executable
    {
        let entry = entry_exe.clone();
        let dialog_weak = dialog.downgrade();
        browse_exe.connect_clicked(move |_| {
            let entry = entry.clone();
            if let Some(dlg) = dialog_weak.upgrade() {
                let file_dlg = FileDialog::builder()
                    .title("Select Executable")
                    .build();
                file_dlg.open(Some(&dlg), gtk4::gio::Cancellable::NONE, move |result| {
                    if let Ok(file) = result {
                        if let Some(path) = file.path() {
                            entry.set_text(&path.to_string_lossy());
                        }
                    }
                });
            }
        });
    }

    // Response
    {
        let entry_name = entry_name.clone();
        let entry_data = entry_data.clone();
        let entry_exe = entry_exe.clone();
        let entry_mods = entry_mods.clone();
        let dropdown = dropdown.clone();
        dialog.connect_response(move |dlg, resp| {
            if resp == ResponseType::Accept {
                let name = entry_name.text().to_string();
                let data = entry_data.text().to_string();
                let exe = entry_exe.text().to_string();
                let ver = EngineVersion::from_index(dropdown.selected());

                if !name.is_empty() {
                    let mut game = GameProfile::new(name, data, exe, ver);
                    game.mods_folder = entry_mods.text().to_string();
                    let mut cfg = config.borrow_mut();
                    cfg.add_game(game);
                    let _ = cfg.save();
                    drop(cfg);
                    populate();
                }
            }
            dlg.close();
        });
    }

    dialog.present();
}

fn show_error(parent: &impl IsA<gtk4::Window>, msg: &str) {
    let dlg = gtk4::AlertDialog::builder().message(msg).build();
    dlg.show(Some(parent));
}

fn show_gb_browser(
    parent: &ApplicationWindow,
    engine: &str,
    mods_folder: &str,
    mods_list_ref: ListBox,
    mods_empty_ref: Label,
) {
    use crate::gamebanana;

    let game_id = match gamebanana::game_id_for_engine(engine) {
        Some(id) => id,
        None => {
            show_error(parent, &format!("No GameBanana ID for {}", engine));
            return;
        }
    };

    let win = gtk4::Window::builder()
        .title(&format!("GameBanana — {} mods", engine))
        .transient_for(parent)
        .modal(true)
        .default_width(700)
        .default_height(600)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.add_css_class("detail-panel");

    let header = Label::new(Some(&format!("🌐 GameBanana — {} Mods", engine)));
    header.add_css_class("detail-title");
    header.set_halign(Align::Start);
    vbox.append(&header);

    // Search bar
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("🔍 Search mods..."));
    search_entry.set_margin_bottom(6);
    vbox.append(&search_entry);

    // Toolbar: sort + status
    let toolbar = GtkBox::new(Orientation::Horizontal, 8);
    toolbar.set_margin_bottom(8);
    let sort_label = Label::new(Some("Sort:"));
    sort_label.add_css_class("info-label");
    toolbar.append(&sort_label);
    let sort_list = StringList::new(&["Newest", "Most Liked", "Most Downloaded"]);
    let sort_dropdown = DropDown::new(Some(sort_list), gtk4::Expression::NONE);
    sort_dropdown.set_selected(0);
    toolbar.append(&sort_dropdown);
    let status = Label::new(Some("Loading..."));
    status.add_css_class("info-value");
    status.set_hexpand(true);
    status.set_halign(Align::Start);
    toolbar.append(&status);
    vbox.append(&toolbar);

    let search_query: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

    // Results
    let scroll = ScrolledWindow::builder().vexpand(true).hscrollbar_policy(gtk4::PolicyType::Never).build();
    let results_list = ListBox::new();
    results_list.set_selection_mode(gtk4::SelectionMode::None);
    results_list.add_css_class("sidebar");
    scroll.set_child(Some(&results_list));
    vbox.append(&scroll);

    // Pagination
    let page_bar = GtkBox::new(Orientation::Horizontal, 12);
    page_bar.set_halign(Align::Center);
    page_bar.set_valign(Align::Center);
    page_bar.set_margin_top(12);
    page_bar.set_margin_bottom(8);
    let prev_btn = Button::with_label("← Prev");
    prev_btn.add_css_class("add-button");
    prev_btn.set_sensitive(false);
    prev_btn.set_size_request(100, -1);
    page_bar.append(&prev_btn);
    let page_label = Label::new(Some("1 / 1"));
    page_label.add_css_class("game-name");
    page_label.set_margin_start(20);
    page_label.set_margin_end(20);
    page_label.set_halign(Align::Center);
    page_label.set_width_chars(10);
    page_bar.append(&page_label);
    let next_btn = Button::with_label("Next →");
    next_btn.add_css_class("add-button");
    next_btn.set_size_request(100, -1);
    page_bar.append(&next_btn);
    vbox.append(&page_bar);

    win.set_child(Some(&vbox));
    win.present();

    let current_page: Rc<RefCell<u32>> = Rc::new(RefCell::new(1));
    let total_count: Rc<RefCell<u64>> = Rc::new(RefCell::new(0));
    let cache_dir = dirs::cache_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp")).join("rsdk-launcher/thumbs").to_string_lossy().to_string();

    let load_page = {
        let results_list = results_list.clone();
        let status = status.clone();
        let page_label = page_label.clone();
        let prev_btn = prev_btn.clone();
        let next_btn = next_btn.clone();
        let current_page = current_page.clone();
        let total_count = total_count.clone();
        let mods_folder = mods_folder.to_string();
        let mods_list_ref = mods_list_ref.clone();
        let mods_empty_ref = mods_empty_ref.clone();
        let cache_dir = cache_dir.clone();
        let win_weak = win.downgrade();

        let search_query = search_query.clone();

        move |page: u32, sort_idx: u32| {
            while let Some(c) = results_list.first_child() { results_list.remove(&c); }
            status.set_text("Loading...");
            prev_btn.set_sensitive(false);
            next_btn.set_sensitive(false);

            let query = search_query.borrow().clone();
            let sort = match sort_idx { 1 => "Generic_MostLiked", 2 => "Generic_MostDownloaded", _ => "" }.to_string();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let result = if query.is_empty() {
                    gamebanana::fetch_mods_list(game_id, page, &sort)
                } else {
                    gamebanana::search_mods(game_id, &query, page)
                };
                let _ = tx.send(result);
            });

            let results_list = results_list.clone();
            let status = status.clone();
            let page_label = page_label.clone();
            let prev_btn = prev_btn.clone();
            let next_btn = next_btn.clone();
            let current_page = current_page.clone();
            let total_count = total_count.clone();
            let mf = mods_folder.clone();
            let ml = mods_list_ref.clone();
            let me = mods_empty_ref.clone();
            let cd = cache_dir.clone();
            let ww = win_weak.clone();

            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                match rx.try_recv() {
                    Ok(Ok((mods, count))) => {
                        *current_page.borrow_mut() = page;
                        *total_count.borrow_mut() = count;
                        let tp = (count as f64 / 15.0).ceil() as u32;
                        status.set_text(&format!("{} mods • Page {}/{}", count, page, tp));
                        page_label.set_text(&format!("{} / {}", page, tp));
                        prev_btn.set_sensitive(page > 1);
                        next_btn.set_sensitive(page < tp);

                        for gm in &mods {
                            let row = ListBoxRow::new();
                            let hb = GtkBox::new(Orientation::Horizontal, 10);
                            hb.set_margin_start(8); hb.set_margin_end(8);
                            hb.set_margin_top(4); hb.set_margin_bottom(4);

                            let thumb = Picture::new();
                            thumb.set_size_request(110, 62);
                            thumb.set_content_fit(gtk4::ContentFit::Cover);
                            hb.append(&thumb);

                            if let Some(url) = gm.thumb_url() {
                                let tw = thumb.downgrade();
                                let cdc = cd.clone();
                                let (ttx, trx) = std::sync::mpsc::channel::<Option<String>>();
                                std::thread::spawn(move || { let _ = ttx.send(gamebanana::download_thumbnail(&url, &cdc)); });
                                gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
                                    match trx.try_recv() {
                                        Ok(Some(p)) => { if let Some(t) = tw.upgrade() { t.set_filename(Some(&p)); } gtk4::glib::ControlFlow::Break }
                                        Ok(None) => gtk4::glib::ControlFlow::Break,
                                        Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                                        Err(_) => gtk4::glib::ControlFlow::Break,
                                    }
                                });
                            }

                            let ib = GtkBox::new(Orientation::Vertical, 2);
                            ib.set_hexpand(true);
                            let nm = Label::new(Some(&gm.name));
                            nm.add_css_class("game-name"); nm.set_halign(Align::Start);
                            nm.set_wrap(true); nm.set_max_width_chars(40);
                            ib.append(&nm);
                            let au = gm.submitter.as_ref().map(|s| s.name.as_str()).unwrap_or("?");
                            let mt = Label::new(Some(&format!("by {} • 👁{} • ❤{}", au, gm.views, gm.likes)));
                            mt.add_css_class("game-engine"); mt.set_halign(Align::Start);
                            ib.append(&mt);

                            // Mod description (truncated)
                            if let Some(ref desc) = gm.description {
                                let clean = desc.replace("<br>", " ").replace("<br/>", " ");
                                let clean = clean.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">");
                                // Strip HTML tags
                                let mut result = String::new();
                                let mut in_tag = false;
                                for ch in clean.chars() {
                                    match ch {
                                        '<' => in_tag = true,
                                        '>' => in_tag = false,
                                        _ if !in_tag => result.push(ch),
                                        _ => {}
                                    }
                                }
                                let short: String = result.chars().take(120).collect();
                                if !short.trim().is_empty() {
                                    let dl = Label::new(Some(&format!("{}…", short.trim())));
                                    dl.add_css_class("info-value");
                                    dl.set_halign(Align::Start);
                                    dl.set_wrap(true);
                                    dl.set_max_width_chars(50);
                                    dl.set_opacity(0.6);
                                    ib.append(&dl);
                                }
                            }

                            // Check if already installed (check both config AND folder existence)
                            let already_installed = {
                                let cfg = read_modconfig(&mf);
                                let mods_path = Path::new(&mf);
                                cfg.keys().any(|k| {
                                    // Only count as installed if the folder still exists
                                    if !mods_path.join(k).is_dir() {
                                        return false;
                                    }
                                    let kl = k.to_lowercase();
                                    let mod_name_lower = gm.name.to_lowercase().replace(' ', "_");
                                    kl == mod_name_lower || gm.name.to_lowercase().contains(&kl) || kl.contains(&mod_name_lower)
                                })
                            };

                            let ibtn = if already_installed {
                                let b = Button::with_label("✓ Installed");
                                b.add_css_class("add-button");
                                b.set_sensitive(false);
                                b
                            } else {
                                let b = Button::with_label("⬇ Install");
                                b.add_css_class("launch-button");
                                b
                            };
                            ibtn.set_valign(Align::Center);
                            let mid = gm.id;
                            let mf1 = mf.clone(); let mf2 = mf.clone();
                            let mlc = ml.clone(); let mec = me.clone(); let wwc = ww.clone();
                            ibtn.connect_clicked(move |btn| {
                                btn.set_label("⏳"); btn.set_sensitive(false);
                                let mft = mf1.clone(); let mfc = mf2.clone();
                                let mlc = mlc.clone(); let mec = mec.clone();
                                let wwc = wwc.clone(); let bw = btn.downgrade();
                                let (t2, r2) = std::sync::mpsc::channel();
                                std::thread::spawn(move || {
                                    let r = gamebanana::fetch_mod_details(mid).and_then(|d|
                                        d.files.first().map(|f| gamebanana::download_and_install_mod(&f.download_url, &mft))
                                            .unwrap_or(Err("No files".into())));
                                    let _ = t2.send(r);
                                });
                                gtk4::glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
                                    match r2.try_recv() {
                                        Ok(Ok(mn)) => {
                                            if let Some(b) = bw.upgrade() { b.set_label("✓ Done"); }
                                            let mut c = read_modconfig(&mfc); c.insert(mn, true); write_modconfig(&mfc, &c);
                                            populate_mods(&mlc, &mec, &mfc);
                                            gtk4::glib::ControlFlow::Break
                                        }
                                        Ok(Err(e)) => {
                                            if let Some(b) = bw.upgrade() { b.set_label("✗ Err"); }
                                            if let Some(w) = wwc.upgrade() { show_error(&w, &e); }
                                            gtk4::glib::ControlFlow::Break
                                        }
                                        Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                                        Err(_) => gtk4::glib::ControlFlow::Break,
                                    }
                                });
                            });

                            hb.append(&ib); hb.append(&ibtn);
                            row.set_child(Some(&hb));
                            results_list.append(&row);
                        }
                        gtk4::glib::ControlFlow::Break
                    }
                    Ok(Err(e)) => { status.set_text(&format!("Error: {}", e)); gtk4::glib::ControlFlow::Break }
                    Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                    Err(_) => gtk4::glib::ControlFlow::Break,
                }
            });
        }
    };
    let load_page = Rc::new(load_page);

    load_page(1, sort_dropdown.selected());

    { let lp = load_page.clone(); sort_dropdown.connect_selected_notify(move |dd| { lp(1, dd.selected()); }); }
    { let lp = load_page.clone(); let cp = current_page.clone(); let sd = sort_dropdown.clone();
      prev_btn.connect_clicked(move |_| { let p = *cp.borrow(); if p > 1 { lp(p-1, sd.selected()); } }); }
    { let lp = load_page.clone(); let cp = current_page.clone(); let tc = total_count.clone(); let sd = sort_dropdown.clone();
      next_btn.connect_clicked(move |_| { let p = *cp.borrow(); let t = *tc.borrow();
        let mp = (t as f64 / 15.0).ceil() as u32; if p < mp { lp(p+1, sd.selected()); } }); }
    // Search activation
    { let lp = load_page.clone(); let sq = search_query.clone(); let sd = sort_dropdown.clone();
      search_entry.connect_activate(move |se| {
          *sq.borrow_mut() = se.text().to_string();
          lp(1, sd.selected());
      });
    }
}
