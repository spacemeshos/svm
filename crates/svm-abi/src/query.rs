use crate::render::VarRenderer;
use crate::schema::Var;

pub struct AppStorageCmd {
    var_id: usize,

    keyword: String,

    params: Vec<String>,
}

pub struct AppStorageQuery {
    cmds: Vec<AppStorageCmd>,
}

pub trait AppStorageReader {
    fn read(cmd: &AppStorageCmd) -> Vec<Var>;
}

impl AppStorageQuery {
    pub fn new() -> Self {
        Self { cmds: Vec::new() }
    }

    pub fn add_cmd(&mut self, cmd: AppStorageCmd) {
        self.cmds.push(cmd);
    }

    // pub fn run(&self, storage: &impl AppStorageReader) -> Vec<Var> {
    //     self.cmds.iter().map(|cmd| )
    // }
}
