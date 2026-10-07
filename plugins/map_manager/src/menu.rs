//! Interactive map voting and nomination menus.

use goldsrc::prelude::*;

/// Builds the interactive map voting ballot menu localized for the specified language.
pub fn build_vote_menu_localized(options: &[String], lang: &str) -> Menu {
    let title = tr!("map_manager", lang, "menus.vote_title");
    let mut builder = Menu::builder(title).style(MenuStyle::brackets());

    for (idx, map) in options.iter().enumerate() {
        let label = format!("{}. {}", idx + 1, map);
        let action_name = format!("map:vote:{idx}");
        builder = builder.item(MenuItem::action(label, action_name));
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
        let action_name = format!("map:nominate:{idx}");
        builder = builder.item(MenuItem::action(label, action_name));
    }

    builder.build()
}

/// Builds the map nomination selection menu.
pub fn build_nomination_menu(maps: &[String]) -> Menu {
    build_nomination_menu_localized(maps, "common")
}
