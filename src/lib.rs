use skyline::libc::c_long;
use skyline::install_hook;
use skyline::hooks::getRegionAddress;
use std::ffi::c_int;
use skyline_config::{SdCardStorage, StorageHolder};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use once_cell::sync::Lazy;

const GBL_SAVE_DATA_OFFSET: usize = 0x229220;
const GET_MOVIE_DATA_OFFSET: usize = 0x114e20;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    get_config()
});

#[derive(Serialize, Deserialize)]
struct Config {
    is_enabled: bool,
    opening_type: OpeningType,
    is_specific_opening: bool,
    specific_opening: Opening
}

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy)]
enum OpeningType{
    Default,
    UminekoProject,
    Linear,
    SakuLinear
}

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy)]
enum Opening{
    KasaneawaseNoNekobako,
    UminekoNoNakuKoroNi,
    SenkyouNoIgreja,
    OcculticsNoMajo,
    KiriNoPithos,
    InnanaNoMitaYume,
    SakuPc, // I don't remember the name :O
    Ogon,
    OgonX,
    OgonCross,
}

#[skyline::main(name = "NekobakoOpeningMod")]
pub fn main() {
    if !CONFIG.is_enabled {
        return;
    }

    install_hook!(get_movie_data_hook);
}

#[skyline::hook(offset = GET_MOVIE_DATA_OFFSET)]
unsafe fn get_movie_data_hook(
    gbl_script: c_long,
    mut _movie_index: c_int,
) -> c_long {
    let base_adress = getRegionAddress(skyline::hooks::Region::Text) as usize;
    let save_offset = (base_adress + GBL_SAVE_DATA_OFFSET) as *const i64;

    let chapter_progress= persist_get(save_offset, 0) as c_int;

    if CONFIG.is_specific_opening{
        return call_original!(gbl_script, CONFIG.specific_opening as i32)
    }

    _movie_index = match CONFIG.opening_type {
        OpeningType::Default => 0,
        OpeningType::UminekoProject => { if chapter_progress < 15 { Opening::SenkyouNoIgreja as i32 } else { Opening::InnanaNoMitaYume as i32 } },
        OpeningType::Linear => todo!(),
        OpeningType::SakuLinear => todo!()
    };

    call_original!(gbl_script, _movie_index)
}

fn get_config() -> Config {
    let sd_storage = SdCardStorage::new("atmosphere/contents/01006A300BA2C000/romfs/skyline/config/nekobako_opening_mod");
    let mut storage_holder = StorageHolder::new(sd_storage);

    if !storage_holder.get_flag("config.yaml") {
        let default_config = Config { is_enabled: true, opening_type: OpeningType::Default, is_specific_opening: false, specific_opening: Opening::KasaneawaseNoNekobako };

        storage_holder.set_field_yaml("config.yaml", &default_config).unwrap();
    }

    storage_holder.get_field_yaml("config.yaml").unwrap()
}

//From decompiled main.nso, gets a value from persist_data array (i think it's an array) using loaded SAVEDATA
unsafe fn persist_get(gbl_save_data: *const i64, index: u32) -> u16 {
    if (index as usize) < ((*gbl_save_data.offset(1) - *gbl_save_data) >> 1) as usize {
        *((*gbl_save_data + index as i64 * 2) as *const u16)
    } else {
        0
    }
}

#[allow(dead_code)]
fn same_build_info() -> bool {
    todo!()
}




