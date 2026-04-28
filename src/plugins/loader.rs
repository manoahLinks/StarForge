use crate::plugins::interface::{Plugin, PluginDeclaration, PluginRegistrar, CORE_VERSION, RUSTC_VERSION};
use anyhow::{Context, Result};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::rc::Rc;

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    libraries: Vec<Rc<Library>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            libraries: Vec::new(),
        }
    }

    /// # Safety
    /// The caller must ensure the plugin at `path` is a valid starforge plugin
    /// compiled with a compatible Rust toolchain and ABI.
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, path: P) -> Result<()> {
        let library = Rc::new(Library::new(path).context("Failed to load library")?);

        let decl: Symbol<*mut PluginDeclaration> = library
            .get(b"PLUGIN_DECLARATION")
            .context("Failed to find PLUGIN_DECLARATION symbol")?;

        let decl = &**decl;

        if decl.rustc_version != RUSTC_VERSION {
            anyhow::bail!(
                "Plugin rustc version mismatch: expected {}, found {}",
                RUSTC_VERSION,
                decl.rustc_version
            );
        }

        if decl.core_version != CORE_VERSION {
             // We could be more lenient here, but let's be strict for now
             println!("Warning: Plugin core version mismatch: core={}, plugin={}", CORE_VERSION, decl.core_version);
        }

        let mut registrar = ProxyRegistrar::new();
        (decl.register)(&mut registrar);

        for plugin in registrar.plugins {
            let name = plugin.name().to_string();
            plugin.on_load();
            self.plugins.insert(name, plugin);
        }

        self.libraries.push(library);

        Ok(())
    }

    pub fn list_plugins(&self) -> Vec<(&str, &str)> {
        self.plugins.iter().map(|(n, p)| (n.as_str(), p.description())).collect()
    }
    
    pub fn execute(&self, name: &str, args: &[String]) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get(name) {
            plugin.execute(args)
        } else {
            Err(format!("Plugin '{}' not found", name))
        }
    }
}

struct ProxyRegistrar {
    plugins: Vec<Box<dyn Plugin>>,
}

impl ProxyRegistrar {
    fn new() -> Self {
        Self { plugins: Vec::new() }
    }
}

impl PluginRegistrar for ProxyRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
}
