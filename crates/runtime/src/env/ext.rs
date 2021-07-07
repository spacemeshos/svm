use std::collections::HashMap;

use svm_layout::{FixedLayout, SymbolicVar};
use svm_types::{App, DeployerAddr, SpawnApp, SpawnerAddr, Template, TemplateAddr};

pub struct ExtApp {
    base: App,

    spawner: SpawnerAddr,
}

impl ExtApp {
    pub fn new(base: &App, spawner: &SpawnerAddr) -> Self {
        Self {
            base: base.clone(),
            spawner: spawner.clone(),
        }
    }

    pub fn base(&self) -> &App {
        &self.base
    }

    pub fn name(&self) -> &str {
        self.base().name()
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        self.base().template_addr()
    }

    pub fn spawner(&self) -> &SpawnerAddr {
        &self.spawner
    }
}

pub struct ExtSpawnApp {
    base: SpawnApp,

    spawner: SpawnerAddr,
}

impl ExtSpawnApp {
    pub fn new(base: SpawnApp, spawner: &SpawnerAddr) -> Self {
        Self {
            base,
            spawner: spawner.clone(),
        }
    }

    pub fn base(&self) -> &SpawnApp {
        &self.base
    }

    pub fn app(&self) -> &App {
        self.base.app()
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        self.base().template_addr()
    }

    pub fn name(&self) -> &str {
        self.base().app_name()
    }

    pub fn ctor_name(&self) -> &str {
        self.base().ctor_name()
    }

    pub fn ctor_data(&self) -> &[u8] {
        self.base().ctor_data()
    }

    pub fn spawner(&self) -> &SpawnerAddr {
        &self.spawner
    }
}
