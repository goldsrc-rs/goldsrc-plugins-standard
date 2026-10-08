//! Interactive map voting and nomination menus.

use goldsrc::prelude::*;

pub mod actions {
    pub const VOTE_0: &str = "map_manager:vote_0";
    pub const VOTE_1: &str = "map_manager:vote_1";
    pub const VOTE_2: &str = "map_manager:vote_2";
    pub const VOTE_3: &str = "map_manager:vote_3";
    pub const VOTE_4: &str = "map_manager:vote_4";

    pub const VOTE_OPTIONS: [&str; 5] = [VOTE_0, VOTE_1, VOTE_2, VOTE_3, VOTE_4];
}

/// Builds the interactive map voting ballot menu localized for the specified language.
pub fn build_vote_menu_localized(options: &[String], lang: &str) -> Menu {
    let title = tr!("map_manager", lang, "menus.vote_title");
    let mut builder = Menu::builder(title).style(MenuStyle::brackets());

    for (idx, map) in options.iter().enumerate() {
        let label = format!("{}. {}", idx + 1, map);
        let action = actions::VOTE_OPTIONS
            .get(idx)
            .copied()
            .unwrap_or("map_manager:vote_overflow");
        builder = builder.item(MenuItem::action(label, action));
    }

    builder.build()
}

/// Builds the interactive map voting ballot menu.
pub fn build_vote_menu(options: &[String]) -> Menu {
    build_vote_menu_localized(options, "common")
}

/// Builds the map nomination selection menu localized for the specified language.
pub fn build_nomination_menu_localized(maps: &[String], lang: &str) -> Menu {
    let title = tr!("map_manager", lang, "menus.nomination_title");
    let mut builder = Menu::builder(title).style(MenuStyle::brackets());

    for (idx, map) in maps.iter().take(8).enumerate() {
        let label = format!("{}. {}", idx + 1, map);
        let action = format!("map_manager:nominate_{idx}");
        builder = builder.item(MenuItem::action(label, action));
    }

    builder.build()
}

/// Builds the map nomination selection menu.
pub fn build_nomination_menu(maps: &[String]) -> Menu {
    build_nomination_menu_localized(maps, "common")
}
