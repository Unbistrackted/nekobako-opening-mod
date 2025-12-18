use nekobako_plugin_utils::get_or_generate_config;
use nekobako_plugin_utils::is_enabled;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use skyline::hooks::{getRegionAddress, Region};
use skyline::install_hook;
use skyline::libc::{c_int, c_long};
use smart_default::SmartDefault;

const GBL_SAVE_DATA_OFFSET: usize = 0x229220;
const GET_MOVIE_DATA_OFFSET: usize = 0x114e20;

static CONFIG: Lazy<Config> =
    Lazy::new(|| get_or_generate_config::<Config>(env!("CARGO_PKG_NAME")));

#[derive(Serialize, Deserialize, SmartDefault)]
struct Config {
    #[default = true]
    is_enabled: bool,

    #[default(OpeningType::UminekoProject)]
    opening_type: OpeningType,

    #[default = false]
    is_specific_opening: bool,

    #[default(OpeningMovie::SenkyouNoIgreja)]
    specific_opening: OpeningMovie,
}

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy)]
enum OpeningType {
    Default,
    UminekoProject,
    Linear,
    SakuLinear,
}

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy)]
enum OpeningMovie {
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

#[skyline::from_offset(0xd1350)]
unsafe fn persist_get(gbl_script: *const i64, index: u32) -> u16;

#[skyline::hook(offset = GET_MOVIE_DATA_OFFSET)]
unsafe fn get_movie_data_hook(gbl_script: c_long, mut _movie_index: c_int) -> c_long {
    let base_adress = getRegionAddress(Region::Text) as usize;

    let save_offset = (base_adress + GBL_SAVE_DATA_OFFSET) as *const i64;

    let chapter_progress = persist_get(save_offset, 0) as c_int;

    if CONFIG.is_specific_opening {
        return call_original!(gbl_script, CONFIG.specific_opening as i32);
    }

    _movie_index = match CONFIG.opening_type {
        OpeningType::Default => 0,
        OpeningType::UminekoProject => {
            if chapter_progress < 15 {
                OpeningMovie::SenkyouNoIgreja as i32
            } else {
                OpeningMovie::InnanaNoMitaYume as i32
            }
        }
        OpeningType::Linear => todo!(),
        OpeningType::SakuLinear => todo!(),
    };

    call_original!(gbl_script, _movie_index)
}

#[skyline::main(name = "NekobakoOpeningMod")]
pub fn main() {
    if is_enabled!(*CONFIG) {
        install_hook!(get_movie_data_hook);
    }
}
