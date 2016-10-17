//  Copyright 2017 RustVR
//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Plugin {
}

/// plugin for graphics language
pub trait PluginGl: Plugin {
    fn run_terrain(&self);
}

/// plugin for human interface
pub trait PluginHi: Plugin {
}

pub struct Session<'a>  {
    pub plugin_gl: &'a PluginGl,
    pub plugin_hi: &'a PluginHi
}

impl<'a> Session<'a> {
    pub fn new(gl: &'a PluginGl, hi: &'a PluginHi) -> Session<'a> {
        Session { plugin_gl: gl, plugin_hi: hi }
    }
    pub fn run_terrain(&self) {
        self.plugin_gl.run_terrain();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
