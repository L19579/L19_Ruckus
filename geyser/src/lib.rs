pub mod config;
pub mod client;
pub mod plugin;

use { 
    std::boxed::Box,
    plugin::GeyserRedisPlugin,
    solana_geyser_plugin_interface::
        geyser_plugin_interface::GeyserPlugin,
};

#[no_mangle]
#[allow(improper_ctype_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin{
    let geyser_redis_plugin: Box<dyn GeyserPlugin> = Box::new(GeyserRedisPlugin{
        redis_client: None,
    });
    
    return Box::into_raw(geyser_redis_plugin);
}
