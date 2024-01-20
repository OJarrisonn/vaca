use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct Settings {
    #[envconfig(from = "VACA_HOME")]
    pub vaca_home: String,
    #[envconfig(from = "VACA_REPL_HIST_LEN", default = "100")]
    pub repl_history_len: usize,
    #[envconfig(default = "unknown")]
    pub version: String 
}