//! Declarative Administrator Menu orchestrator (`grs_adminmenu`).

use goldsrc::prelude::*;

pub const ACTION_RESTART_1: &str = "admin:restart:1";
pub const ACTION_RESTART_3: &str = "admin:restart:3";
pub const ACTION_PAUSE: &str = "admin:pause";
pub const ACTION_CFG_CW: &str = "admin:cfg:cw";
pub const ACTION_CFG_WARMUP: &str = "admin:cfg:warmup";
pub const ACTION_MAP_CHANGE: &str = "admin:map";
pub const ACTION_STAFF_LIST: &str = "admin:staff:list";

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
        .item(MenuItem::action(item_slay, ACTION_RESTART_1).keep_open())
        .item(MenuItem::action(item_slap, ACTION_RESTART_3).keep_open())
        .item(MenuItem::action(item_teleport, ACTION_PAUSE).keep_open())
        .item(MenuItem::action(item_team, ACTION_CFG_CW).keep_open())
        .item(MenuItem::action(item_map, ACTION_MAP_CHANGE).keep_open())
        .build()
}

/// Builds the root administrator control menu with default language.
pub fn build_admin_main_menu() -> Menu {
    build_admin_main_menu_localized("common")
}
