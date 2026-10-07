//! Declarative VIP and Privileges Menu orchestrator (`grs_privmenu` / `/vip`).

use goldsrc::prelude::*;

pub const ACTION_ARMOR: &str = "vip:item:armor";
pub const ACTION_GRENADES: &str = "vip:item:grenades";
pub const ACTION_M4A1: &str = "vip:item:m4a1";
pub const ACTION_AK47: &str = "vip:item:ak47";
pub const ACTION_AWP: &str = "vip:item:awp";
pub const ACTION_DEAGLE: &str = "vip:item:deagle";
pub const ACTION_TOGGLE_REGEN: &str = "vip:perk:regen";

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
            MenuItem::action(item_armor, ACTION_ARMOR)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(
            MenuItem::action(item_grenades, ACTION_GRENADES)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(
            MenuItem::action(item_m4a1, ACTION_M4A1)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(
            MenuItem::action(item_ak47, ACTION_AK47)
                .require_spec::<Alive>()
                .keep_open(),
        );

    // AWP sniper rifle restricted to round >= 3
    if round_number >= 3 {
        builder = builder.item(
            MenuItem::action(item_awp, ACTION_AWP)
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
        builder = builder.item(MenuItem::action(restricted_label, ACTION_AWP).keep_open());
    }

    builder
        .item(
            MenuItem::action(item_deagle, ACTION_DEAGLE)
                .require_spec::<Alive>()
                .keep_open(),
        )
        .item(MenuItem::action(item_toggle, ACTION_TOGGLE_REGEN).keep_open())
        .build()
}

/// Builds the interactive VIP privileges menu with default language.
pub fn build_vip_menu(round_number: u32) -> Menu {
    build_vip_menu_localized(round_number, "common")
}
