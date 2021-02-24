# 16. plugin system design

Date: 2021-02-08

## Status

2021-02-08 proposed

2021-02-24 done

## Context

For current, we need to support for plugins load.

in article [Dynamic Loading & Plugins](https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html) show a plugin
 lifecycle:
 
```rust
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn on_plugin_load(&self) {}
    fn pre_send(&self, _request: &mut Request) {}
    fn post_receive(&self, _response: &mut Response) {}
}
```

example 2:

```rust
// client/src/plugins.rs

use std::ffi::OsStr;
use std::any::Any;
use libloading::{Library, Symbol};

use errors::*;
use {Request, Response};


/// A plugin which allows you to add extra functionality to the REST client.
pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used 
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    /// Inspect (and possibly mutate) the request before it is sent.
    fn pre_send(&self, _request: &mut Request) {}
    /// Inspect and/or mutate the received response before it is displayed to
    /// the user.
    fn post_receive(&self, _response: &mut Response) {}
}
```

Loading Samples 

```rust
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<()> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        let lib = Library::new(filename.as_ref()).chain_err(|| "Unable to load the plugin")?;

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .chain_err(|| "The `_plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_load();
        self.plugins.push(plugin);


        Ok(())
    }
    
    /// Iterate over the plugins, running their `pre_send()` hook.
    pub fn pre_send(&mut self, request: &mut Request) {
        debug!("Firing pre_send hooks");

        for plugin in &mut self.plugins {
            trace!("Firing pre_send for {:?}", plugin.name());
            plugin.pre_send(request);
        }
    }

    /// Iterate over the plugins, running their `post_receive()` hook.
    pub fn post_receive(&mut self, response: &mut Response) {
        debug!("Firing post_receive hooks");

        for plugin in &mut self.plugins {
            trace!("Firing post_receive for {:?}", plugin.name());
            plugin.post_receive(response);
        }
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire 
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
```

a plugin manager: 

```rust
impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}
```

## Decision

1. use `coco_plugins`

## Consequences

Consequences here...
