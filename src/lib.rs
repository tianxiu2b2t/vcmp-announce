use vcmp_bindings::{func::ServerMethods, raw::{PluginCallbacks, PluginFuncs, PluginInfo}, utils::set_plugin_name, vcmp_func, VcmpFunctions};

use crate::{announce::announce, cfg::load_config, logger::init};

pub mod cfg;
pub mod announce;
pub mod logger;

pub fn run() {

}

#[unsafe(no_mangle)]
extern "C" fn VcmpPluginInit(
    plugin_functions: *mut PluginFuncs,
    plugin_callbacks: *mut PluginCallbacks,
    plugin_info: *mut PluginInfo,
) -> u32 {
    {
        // check null
        if plugin_functions.is_null() {
            println!("!!! plugin_functions is null !!!");
            return 0;
        }
        if plugin_callbacks.is_null() {
            println!("!!! plugin_callbacks is null !!!");
            return 0;
        }
        if plugin_info.is_null() {
            println!("!!! plugin_info is null !!!");
            return 0;
        }
    }

    let (callbacks, info) = unsafe { (&mut *plugin_callbacks, &mut *plugin_info) };

    let functions = VcmpFunctions::from(plugin_functions);

    vcmp_bindings::init_vcmp_func(functions);

    info.apiMajorVersion = 2;
    info.apiMinorVersion = 0; 
    info.pluginVersion = 64;

    let _ = set_plugin_name("vcmp-announce", info);
    // check announce toml
    if !std::fs::exists("announce.toml") {
        let cfg = cfg::Config::default();
        let content = toml::to_string_pretty(&cfg).unwrap();
        std::fs::write(config_path, content).unwrap();
    }
    load_config("announce.toml");
    init();


    // struct size check

    callbacks.OnServerInitialise = Some(on_server_initialise);

    1
}

#[unsafe(no_mangle)]
extern "C" fn on_server_initialise() -> u8 {
    announce(vcmp_func().get_server_name(), vcmp_func().server_settings().port() as u16);

    1
}