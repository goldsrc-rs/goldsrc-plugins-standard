//! Declarative VIP and Privileges Menu orchestrator (`grs_privmenu` / `/vip`).

use goldsrc::prelude::*;

pub mod actions {
    pub const VIP_ARMOR: &str = "vip:armor";
    pub const VIP_GRENADES: &str = "vip:grenades";
    pub const VIP_M4A1: &str = "vip:m4a1";
    pub const VIP_AK47: &str = "vip:ak47";
    pub const VIP_AWP: &str = "vip:awp";
    pub const VIP_DEAGLE: &str = "vip:deagle";
    pub const VIP_TOGGLE_REGEN: &str = "vip:toggle_regen";
}

/// Builds the interactive VIP privileges menu using player's preferred language.
pub fn build_vip_menu_localized(round_number: u32, lang: &str) -> Menu {
    let title = tr!("privileges", lang, "menus.title");
    let item_armor = tr!("privileges", lang, "menus.armor");
    let item_grenades = tr!("privileges", lang, "menus.grenades");
    let item_m4a1 = tr!("privileges", lang, "menus.m4a1");
    let item_ak47 = tr!("privileges", lang, "menus.ak47");
    let item_awp = tr!("privileges", lang, "menus.awp");
    let item_deagle = tr!("privileges", lang, "menus.deagle");
    let item_toggle = tr!("privileges", lang, "menus.toggle_regen");

    let mut builder = Menu::builder(title)
        .style(MenuStyle::brackets())
        .item(
            MenuItem::action(item_armor, actions::VIP_ARMOR)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(
            MenuItem::action(item_grenades, actions::VIP_GRENADES)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(
            MenuItem::action(item_m4a1, actions::VIP_M4A1)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(
            MenuItem::action(item_ak47, actions::VIP_AK47)
                .require_spec::<Alive>()
                .keep_open(),
        );

    // AWP sniper rifle restricted to round >= 3
    if round_number >= 3 {
        builder = builder.item(
            MenuItem::action(item_awp, actions::VIP_AWP)
                .require_spec::<Alive>()
                .keep_open(),
        );
    } else {
        let restricted_label = tr!(
            "privileges",
            lang,
            "awp_restricted",
            round = 3,
            cur = round_number
        );
        builder = builder.item(MenuItem::action(restricted_label, actions::VIP_AWP).keep_open());
    }

    builder
        .item(
            MenuItem::action(item_deagle, actions::VIP_DEAGLE)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(MenuItem::action(item_toggle, actions::VIP_TOGGLE_REGEN).keep_open())
        .build()
}

/// Builds the interactive VIP privileges menu with default language.
pub fn build_vip_menu(round_number: u32) -> Menu {
    build_vip_menu_localized(round_number, "common")
}
