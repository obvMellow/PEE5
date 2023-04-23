pub mod afk;
pub mod automod;
pub mod chat;
pub mod config;
pub mod logging;
pub mod xp;

#[macro_export]
macro_rules! error_constructor {
    ($error_msg:expr) => {
        format!("```\nerror: {}\n\n", $error_msg)
    };
    (config $argument:expr, $error_msg:expr, $explanation:expr) => {
        error_constructor!("!config", $argument, $error_msg, $explanation)
    };
    (config set $argument:expr, $error_msg:expr, $explanation:expr) => {
        error_constructor!("!config set", $argument, $error_msg, $explanation)
    };
    (config enable plugin $argument:expr, $error_msg:expr, $explanation:expr) => {
        error_constructor!("!config enable-plugin", $argument, $error_msg, $explanation)
    };
    (config disable plugin $argument:expr, $error_msg:expr, $explanation:expr) => {
        error_constructor!(
            "!config disable-plugin",
            $argument,
            $error_msg,
            $explanation
        )
    };
    ($command:expr, $argument:expr, $error_msg:expr, $explanation:expr) => {{
        let mut base = error_constructor!($error_msg);

        let arrows = "^".repeat($argument.len());
        let spaces = " ".repeat($command.len());

        base.push_str(&format!("    | {} {}\n", $command, $argument));
        base.push_str(&format!(
            "    | {} {} {}\n```",
            spaces, arrows, $explanation
        ));

        base
    }};
}
