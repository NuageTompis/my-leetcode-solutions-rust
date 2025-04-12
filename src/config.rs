use crate::args::{ConfigCommand, ConfigSubcommand};
use crate::read_write;

pub fn handle_config_command(config: ConfigCommand) {
    let (variable, value) = match config.command {
        ConfigSubcommand::Username(username_command) => ("username", username_command.username),
        ConfigSubcommand::Cookie(cookie_command) => ("cookie", cookie_command.cookie),
        ConfigSubcommand::Premium(premium_command) => {
            if premium_command.premium >= 2 {
                println!("The premium value {} is not valid, it should be either 0 or 1", {premium_command.premium});
                return;
            } else {
                ("premium", premium_command.premium.to_string())
            }
        },
    };

    let result = read_write::try_update_env_variable(variable, &value);
    match result {
        Ok(_) => {
            println!("Sucessfully wrote {} to .env", variable);
        },
        Err(e) => {
            println!("Unexpected error while handling config command: {}.", e);
        }
    }
}
