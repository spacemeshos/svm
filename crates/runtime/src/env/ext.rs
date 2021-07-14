use svm_types::{Account, SpawnAccount, SpawnerAddr, TemplateAddr};

pub struct ExtApp {
    base: Account,

    spawner: SpawnerAddr,
}

impl ExtApp {
    pub fn new(base: &Account, spawner: &SpawnerAddr) -> Self {
        Self {
            base: base.clone(),
            spawner: spawner.clone(),
        }
    }

    pub fn base(&self) -> &Account {
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
    base: SpawnAccount,

    spawner: SpawnerAddr,
}

impl ExtSpawnApp {
    pub fn new(base: SpawnAccount, spawner: &SpawnerAddr) -> Self {
        Self {
            base,
            spawner: spawner.clone(),
        }
    }

    pub fn base(&self) -> &SpawnAccount {
        &self.base
    }

    pub fn app(&self) -> &Account {
        self.base.account()
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        self.base().template_addr()
    }

    pub fn name(&self) -> &str {
        self.base().account_name()
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
