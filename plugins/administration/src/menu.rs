//! Declarative Administrator Menu orchestrator (`grs_adminmenu`).

use goldsrc::prelude::*;

pub mod actions {
    pub const RESTART_1: &str = "admin:restart_1";
    pub const RESTART_3: &str = "admin:restart_3";
    pub const PAUSE: &str = "admin:pause";
    pub const CFG_CW: &str = "admin:cfg_cw";
    pub const CFG_WARMUP: &str = "admin:cfg_warmup";
    pub const MAP_CHANGE: &str = "admin:map_change";
    pub const STAFF_LIST: &str = "admin:staff_list";
}

/// Builds the root administrator control menu using localized dictionary.
pub fn build_admin_main_menu_localized(lang: &str) -> Menu {
    let title = tr!("administration", lang, "menus.title");
    let item_slay = tr!("administration", lang, "menus.slay");
    let item_slap = tr!("administration", lang, "menus.slap");
    let item_teleport = tr!("administration", lang, "menus.teleport");
    let item_team = tr!("administration", lang, "menus.team");
    let item_map = tr!("administration", lang, "menus.map");

    Menu::builder(title)
        .style(MenuStyle::brackets())
        .item(MenuItem::action(item_slay, actions::RESTART_1).keep_open())
        .item(MenuItem::action(item_slap, actions::RESTART_3).keep_open())
        .item(MenuItem::action(item_teleport, actions::PAUSE).keep_open())
        .item(MenuItem::action(item_team, actions::CFG_CW).keep_open())
        .item(MenuItem::action(item_map, actions::MAP_CHANGE).keep_open())
        .build()
}

/// Builds the root administrator control menu with default language.
pub fn build_admin_main_menu() -> Menu {
    build_admin_main_menu_localized("common")
}
