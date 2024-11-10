/*
A Trait to all possible VM types to output something we can use with crosvm
 */
pub enum CrosVmParam{
    Value(String),
    Bool(bool),
    Map(Vec<(String, CrosVmParam)>),
    List(Vec<CrosVmParam>)
}

impl From<String> for CrosVmParam{
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}
impl From<bool> for CrosVmParam{
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub struct CrosVmCmdLine{
    // Name of the argument
    pub name: String,

    // Either it's value (i.e. kernel path) or it's values (i.e. GPU command line)
    pub params: CrosVmParam
}
pub trait CrosVmConfig{
    fn generate_config(&self) -> Vec<CrosVmCmdLine>;
}