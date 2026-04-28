use std::any::Any;

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn description(&self) -> &'static str;
    
    fn on_load(&self) {}
    fn on_unload(&self) {}
    
    fn execute(&self, args: &[String]) -> Result<(), String>;
}

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PLUGIN_DECLARATION: $crate::plugins::PluginDeclaration = $crate::plugins::PluginDeclaration {
            rustc_version: $crate::plugins::interface::RUSTC_VERSION,
            core_version: $crate::plugins::interface::CORE_VERSION,
            register: $register,
        };
    };
}

pub const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
