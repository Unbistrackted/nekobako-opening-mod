use skyline::install_hook;
use once_cell::sync::Lazy;
use skyline::libc::{c_long, c_int};
use skyline::hooks::{Region, getRegionAddress};
use skyline_config::{SdCardStorage, StorageHolder};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

const GBL_SAVE_DATA_OFFSET: usize = 0x229220;
const GET_MOVIE_DATA_OFFSET: usize = 0x114e20;
const BUILD_INFO: [u8; 20] = [
    0x76, 0x16, 0xf8, 0x96, 0x3d,
    0xac, 0xcd, 0x70, 0xe2, 0x0f,
    0xf3, 0x90, 0x4e, 0x13, 0x36,
    0x7f, 0x96, 0xf2, 0xd9, 0xb3,
];

static CONFIG: Lazy<Config> = Lazy::new(|| {
    get_config()
});

static IS_DIFFERENT_BUILD: Lazy<bool> = Lazy::new(|| {
    unsafe  {
        !is_same_build_info()
    }
});

#[derive(Serialize, Deserialize)]
struct Config {
    is_enabled: bool,
    opening_type: OpeningType,
    is_specific_opening: bool,
    specific_opening: OpeningMovie
}

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy)]
enum OpeningType {
    Default,
    UminekoProject,
    Linear,
    SakuLinear
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

#[skyline::main(name = "NekobakoOpeningMod")]
pub fn main() {
    if *IS_DIFFERENT_BUILD{
        return;
    }

    if !CONFIG.is_enabled {
        return;
    }

    install_hook!(get_movie_data_hook);
}

fn get_config() -> Config {
    let sd_storage = SdCardStorage::new("atmosphere/contents/01006A300BA2C000/romfs/skyline/config/nekobako_opening_mod");
    let mut storage_holder = StorageHolder::new(sd_storage);

    if !storage_holder.get_flag("config.yaml") {
        let default_config = Config { is_enabled: true, opening_type: OpeningType::Default, is_specific_opening: false, specific_opening: OpeningMovie::KasaneawaseNoNekobako };

        storage_holder.set_field_yaml("config.yaml", &default_config).unwrap();
    }

    storage_holder.get_field_yaml("config.yaml").unwrap()
}

#[skyline::hook(offset = GET_MOVIE_DATA_OFFSET)]
unsafe fn get_movie_data_hook(
    gbl_script: c_long,
    mut _movie_index: c_int,
) -> c_long {
    let base_adress = getRegionAddress(Region::Text) as usize;

    let save_offset = (base_adress + GBL_SAVE_DATA_OFFSET) as *const i64;

    let chapter_progress= persist_get(save_offset, 0) as c_int;

    if CONFIG.is_specific_opening{
        return call_original!(gbl_script, CONFIG.specific_opening as i32)
    }

    _movie_index = match CONFIG.opening_type {
        OpeningType::Default => 0,
        OpeningType::UminekoProject => { if chapter_progress < 15 { OpeningMovie::SenkyouNoIgreja as i32 } else { OpeningMovie::InnanaNoMitaYume as i32 } },
        OpeningType::Linear => todo!(),
        OpeningType::SakuLinear => todo!()
    };

    call_original!(gbl_script, _movie_index)
}

//From decompiled main.nso, gets a value from persist_data array (i think it's an array) using loaded SAVEDATA
unsafe fn persist_get(gbl_save_data: *const i64, index: u32) -> u16 {
    if (index as usize) < ((*gbl_save_data.offset(1) - *gbl_save_data) >> 1) as usize {
        *((*gbl_save_data + index as i64 * 2) as *const u16)
    } else {
        0
    }
}

//From ReSwitched's Discord Server, last 0x1000 bytes in .rodata contains build info just after "GNU\x00" and, .rodata is located before .data (Thanks DCNick3 and Masa!)
unsafe fn is_same_build_info() -> bool {
     let data_adress   = getRegionAddress(Region::Data) as usize;

     let scan = core::slice::from_raw_parts((data_adress - 0x1000) as *const u8,0x1000);

     let gnu_end_pos = match scan.windows(4).position(|w| w == b"GNU\x00"){
          Some(pos) => pos + 4,
          None => return false
     };

     let build_info = &scan[gnu_end_pos..gnu_end_pos + 20]; // In the decompilation BUILD INFO had 20 bytes

     build_info == BUILD_INFO.as_slice()
}



