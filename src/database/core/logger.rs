use env_logger::{Env, init_from_env};

pub fn logger_init() {
    let env = Env::default()
        .filter_or("NYXDB_LOG_LEVEL", "info")
        .write_style_or("NYXDB_LOG_STYLE", "always");

    init_from_env(env)
}
