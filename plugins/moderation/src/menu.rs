//! Declarative Moderator Menu orchestrator (`grs_modmenu`).

use goldsrc::prelude::*;

pub mod actions {
    pub const SLAP: &str = "mod:slap";
    pub const SLAY: &str = "mod:slay";
    pub const FREEZE: &str = "mod:freeze";
    pub const GAG: &str = "mod:gag";
    pub const MUTE: &str = "mod:mute";
    pub const KICK: &str = "mod:kick";
    pub const BAN: &str = "mod:ban";
    pub const INSPECT: &str = "mod:inspect";
}

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
        .item(MenuItem::action(item_slap, actions::SLAP).keep_open())
        .item(MenuItem::action(item_slay, actions::SLAY).keep_open())
        .item(MenuItem::action(item_freeze, actions::FREEZE).keep_open())
        .item(MenuItem::action(item_gag, actions::GAG).keep_open())
        .item(MenuItem::action(item_mute, actions::MUTE).keep_open())
        .item(MenuItem::action(item_kick, actions::KICK).keep_open())
        .item(MenuItem::action(item_ban, actions::BAN).keep_open())
        .item(MenuItem::action("8. Inspect", actions::INSPECT).keep_open())
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

    for p in Players::all() {
        let i = p.index();
        let name = p.name().unwrap_or_else(|| format!("Player #{i}"));
        let item_label = format!("{name} (#{i})");
        builder = builder.item(MenuItem::target(item_label, p.slot(), "moderation:target"));
    }

    builder.build()
}
