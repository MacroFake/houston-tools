use mlua::prelude::*;

use azur_lane::ship::*;

use crate::context;
use crate::convert_al;
use crate::model::*;

pub fn load_skin(set: &SkinSet) -> LuaResult<ShipSkin> {
    macro_rules! get {
        ($key:literal) => {
            set.template.get($key).with_context(context!("skin template {} for skin {}", $key, set.skin_id))?
        };
    }

    let mut skin = ShipSkin {
        skin_id: set.skin_id,
        image_key: get!("painting"),
        name: get!("name"),
        description: get!("desc"),
        words: load_words(set)?,
        words_extra: None, // loaded below
    };

    if let Some(ref extra) = set.words_extra {
        skin.words_extra = Some(Box::new(load_words_extra(set, extra, &skin.words)?));
    }

    Ok(skin)
}

fn load_words(set: &SkinSet) -> LuaResult<ShipSkinWords> {
    macro_rules! get {
        ($key:literal) => {{
            let text: String = set.words.get($key).with_context(context!("skin word {} for skin {}", $key, set.skin_id))?;
            if text.is_empty() { None } else { Some(text) }
        }};
    }

    Ok(ShipSkinWords {
        description: get!("drop_descrip"),
        introduction: get!("profile"),
        acquisition: get!("unlock"),
        login: get!("login"),
        details: get!("detail"),
        main_screen: to_main_screen(get!("main").as_deref()).collect(),
        touch: get!("touch"),
        special_touch: get!("touch2"),
        rub: get!("headtouch"),
        mission_reminder: get!("mission"),
        mission_complete: get!("mission_complete"),
        mail_reminder: get!("mail"),
        return_to_port: get!("home"),
        commission_complete: get!("expedition"),
        enhance: get!("upgrade"),
        flagship_fight: get!("battle"),
        victory: get!("win_mvp"),
        defeat: get!("lose"),
        skill: get!("skill"),
        low_health: get!("hp_warning"),
        disappointed: get!("feeling1"),
        stranger: get!("feeling2"),
        friendly: get!("feeling3"),
        crush: get!("feeling4"),
        love: get!("feeling5"),
        oath: get!("propose"),
        couple_encourage: {
            let tables: Option<Vec<LuaTable>> = set.words.get("couple_encourage").context("skin word couple_encourage").ok();
            tables.into_iter().flatten().map(|t| load_couple_encourage(set, t)).collect::<LuaResult<_>>()?
        }
    })
}

fn load_words_extra(set: &SkinSet, table: &LuaTable, base: &ShipSkinWords) -> LuaResult<ShipSkinWords> {
    macro_rules! get {
        ($key:literal) => {{
            let value: LuaValue = table.get($key).with_context(context!("skin word extra {} for skin {}", $key, set.skin_id))?;
            match value {
                LuaValue::Table(t) => {
                    let t: LuaTable = t.get(1)?;
                    let text: String = t.get(2)?;
                    if text.is_empty() { None } else { Some(text) }
                }
                _ => None
            }
        }};
    }

    let mut main_screen: Vec<ShipMainScreenLine> = to_main_screen(get!("main").as_deref()).collect();

    main_screen.extend(
        to_main_screen(get!("main_extra").as_deref())
            .map(|line| { let index = line.index(); line.with_index(index + base.main_screen.len()) })
    );

    Ok(ShipSkinWords {
        description: get!("drop_descrip"),
        introduction: get!("profile"),
        acquisition: get!("unlock"),
        login: get!("login"),
        details: get!("detail"),
        main_screen,
        touch: get!("touch"),
        special_touch: get!("touch2"),
        rub: get!("headtouch"),
        mission_reminder: get!("mission"),
        mission_complete: get!("mission_complete"),
        mail_reminder: get!("mail"),
        return_to_port: get!("home"),
        commission_complete: get!("expedition"),
        enhance: get!("upgrade"),
        flagship_fight: get!("battle"),
        victory: get!("win_mvp"),
        defeat: get!("lose"),
        skill: get!("skill"),
        low_health: get!("hp_warning"),
        disappointed: get!("feeling1"),
        stranger: get!("feeling2"),
        friendly: get!("feeling3"),
        crush: get!("feeling4"),
        love: get!("feeling5"),
        oath: get!("propose"),
        couple_encourage: Vec::new()
    })
}

fn to_main_screen<'a>(raw: Option<&'a str>) -> impl Iterator<Item = ShipMainScreenLine> + 'a {
    raw.into_iter().flat_map(|s| s.split('|')).enumerate()
        .filter(|(_, text)| !text.is_empty() && *text != "nil")
        .map(|(index, text)| ShipMainScreenLine::new(index, text.to_owned()))
}

fn load_couple_encourage(set: &SkinSet, table: LuaTable) -> LuaResult<ShipCoupleEncourage> {
    let filter: Vec<u32> = table.get(1).with_context(context!("couple_encourage 1 for skin {}", set.skin_id))?;
    let mode: Option<u32> = table.get(4).with_context(context!("couple_encourage 4 for skin {}", set.skin_id))?;

    Ok(ShipCoupleEncourage {
        amount: table.get(2).with_context(context!("couple_encourage 2 for skin {}", set.skin_id))?,
        line: table.get(3).with_context(context!("couple_encourage 3 for skin {}", set.skin_id))?,
        condition: match mode {
            Some(1) => ShipCouple::HullType(filter.into_iter().map(convert_al::to_hull_type).collect()),
            Some(2) => ShipCouple::Rarity(filter.into_iter().map(convert_al::to_rarity).collect()),
            Some(3) => ShipCouple::Faction(filter.into_iter().map(convert_al::to_faction).collect()),
            Some(4) => ShipCouple::Illustrator,
            _ => ShipCouple::ShipGroup(filter),
        }
    })
}