# NekobakoOpeningMod ![Version](https://img.shields.io/github/v/release/Unbistrackted/NekobakoOpeningMod?style=plastic&label=Version&color=dc3e3e) ![Downloads](https://img.shields.io/github/downloads/Unbistrackted/NekobakoOpeningMod/total?style=plastic&label=Downloads&color=50f63f) 

A plugin for [Skyline](https://github.com/skyline-dev/skyline) that allows you change the opening in ``Umineko no Naku Koro ni Saku ~Nekobako to Musou no Koukyoukyoku~``

``Title ID:01006A300BA2C000``


## Configuration Settings

 Setting Key                   | Value Type | Default Value | Description                                                                                    
-------------------------------|------------|---------------|------------------------------------------------------------------------------------------------
 is_enabled                    | bool       | true          | Enables or disables the plugin     
 opening_type                  | i32        | 0             | Which opening type should the plugin use? ( See OpeningType ) 
 is_specific_opening           | bool       | false         | Should the plugin force a specific opening?
 specific_opening              | i32        | 0             | ( See Opening )                       

 ### OpeningType 

 Config                        | Value      | Description                                                                                    
-------------------------------|------------|---------------------------------------------------------------------------------------------------------------
 Default                       | 0          | Plays the default intro (Kasaneawase No Nekobako)            
 UminekoProject                | 1          | Plays ``Senkyou No Igreja`` if you completed epsiodes 1-4, then ``Mita Yume`` for episodes 5-8            
 Linear                        | 2          | To do! - Don't use this value, will most likely crash your game!!!      
 SakuLinear                    | 3          | To do! - Don't use this value, will most likely crash your game!!!    
 
### Opening

 Config                        | Value                                                                                        
-------------------------------|---------------------------------------------------------------------------------------------------------------------------
 KasaneawaseNoNekobako         | 0                    
 UminekoNoNakuKoroNi           | 1                   
 SenkyouNoIgreja               | 2           
 OcculticsNoMajo               | 3  
 KiriNoPithos                  | 4
 InnanaNoMitaYume              | 5
 SakuPc                        | 6
 Ogon                          | 7
 OgonX                         | 8
 OgonC                         | 9
 
 

## Download

❗️Do **NOT** install if the game's UPDATE VERSION is different from ``0.0.3.0/v196608``, this will most likely crash your game!!!

You can download the latest version of NekobakoOpeningMod [here](https://github.com/Unbistrackted/NekobakoOpeningMod/releases/latest).

After downloading, drop the contents of the .zip file into the root of your SD card.

You can find the config file at ``/atmosphere/contents/01006A300BA2C000/romfs/skyline/config/nekobako_opening_mod/config.yaml) ``

(**Generates after loading the plugin**)

## Building from source

To build from source you would need to have [Rust](https://www.rust-lang.org/) and [cargo-skyline](https://github.com/jam1garner/cargo-skyline) installed.

Then you can build the plugin with:

```bash
cargo skyline build
```

Or generate the plugin package with:

```bash
cargo skyline package
```

## Special thanks to:

https://github.com/DCNick3 for the tools he created, the time explaining how the basics works, and all sorts of things that would not have made this possible without his help ( I can't really express how grateful I am, never thought I could pull this off)
