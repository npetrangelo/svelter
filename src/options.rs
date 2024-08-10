#[derive(Default)]
pub struct RawOptions;

impl RawOptions {
    pub fn validate(self) -> ValidOptions {
        ValidOptions { modernAst: true }
    }
}

pub struct ValidOptions {
    pub modernAst: bool
}
