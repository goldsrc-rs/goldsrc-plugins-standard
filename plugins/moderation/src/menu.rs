//! Declarative Moderator Menu orchestrator (`grs_modmenu`).

use goldsrc::prelude::*;

pub const ACTION_SLAP: &str = "mod:action:slap";
pub const ACTION_SLAY: &str = "mod:action:slay";
pub const ACTION_FREEZE: &str = "mod:action:freeze";
pub const ACTION_GAG: &str = "mod:action:gag";
pub const ACTION_MUTE: &str = "mod:action:mute";
pub const ACTION_KICK: &str = "mod:action:kick";
pub const ACTION_BAN: &str = "mod:action:ban";
pub const ACTION_INSPECT: &str = "mod:action:inspect";

/// Builds the root moderator control panel using the player's preferred language.
pub fn build_moderator_main_menu_localized(lang: &str) -> Menu {
    let title = tr!("moderation", lang, "menus.title");
    let item_freeze = tr!("moderation", lang, "menus.freeze");
    let item_gag = tr!("moderation", lang, "menus.gag");
    let item_mute = tr!("moderation", lang, "menus.mute");
    let item_slap = tr!("moderation", lang, "menus.slap");
    let item_slay = tr!("moderation", lang, "menus.slay");
    let item_kick = tr!("moderation", lang, "menus.kick");
    let item_ban = tr!("moderation", lang, "menus.ban");

    Menu::builder(title)
        .style(MenuStyle::brackets())
        .item(MenuItem::action(item_slap, ACTION_SLAP).keep_open())
        .item(MenuItem::action(item_slay, ACTION_SLAY).keep_open())
        .item(MenuItem::action(item_freeze, ACTION_FREEZE).keep_open())
        .item(MenuItem::action(item_gag, ACTION_GAG).keep_open())
        .item(MenuItem::action(item_mute, ACTION_MUTE).keep_open())
        .item(MenuItem::action(item_kick, ACTION_KICK).keep_open())
        .item(MenuItem::action(item_ban, ACTION_BAN).keep_open())
        .item(MenuItem::action("8. Inspect", ACTION_INSPECT).keep_open())
        .build()
}

/// Builds the root moderator control panel with default language.
pub fn build_moderator_main_menu() -> Menu {
    build_moderator_main_menu_localized("common")
}

/// Builds an interactive target selection menu for the specified action ID.
pub fn build_target_selection_menu(action_title: &str) -> Menu {
    let mut builder = Menu::builder(format!("Модерация: Выбор цели ({action_title})"));
    builder = builder.style(MenuStyle::brackets());

    for i in 1..=32 {
        let p = Player::new(i);
        if p.is_valid() {
            let name = p.name().unwrap_or_else(|| format!("Player #{i}"));
            let item_label = format!("{name} (#{i})");
            builder = builder.item(MenuItem::action(item_label, format!("mod:target:{i}")));
        }
    }

    builder.build()
}
