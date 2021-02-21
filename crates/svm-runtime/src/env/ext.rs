use svm_layout::Layout;
use svm_types::{App, AuthorAddr, SpawnApp, SpawnerAddr, Template, TemplateAddr};

pub struct ExtTemplate {
    base: Template,

    author: AuthorAddr,
}

impl ExtTemplate {
    pub fn new(base: Template, author: &AuthorAddr) -> Self {
        Self {
            base,
            author: author.clone(),
        }
    }

    pub fn base(&self) -> &Template {
        &self.base
    }

    pub fn name(&self) -> &str {
        &self.base.name
    }

    pub fn code(&self) -> &[u8] {
        &self.base.code
    }

    pub fn layout(&self) -> &Layout {
        &self.base.layout
    }

    pub fn author(&self) -> &AuthorAddr {
        &self.author
    }

    pub fn is_ctor(&self, func_name: &str) -> bool {
        let base = self.base();

        base.ctors.iter().any(|ctor| ctor == func_name)
    }
}

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
