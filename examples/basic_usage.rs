use cliparser::{App, Command, Flag, FlagType, ParsedArgs};

fn main() {
    let app = App::new("exemplo-basico", "1.0.0")
        .description("Exemplo básico de uso da biblioteca cliparser")
        .add_command(
            Command::new("hello")
                .description("Saúda o usuário")
                .add_flag(
                    Flag::new("name", FlagType::String)
                        .short('n')
                        .description("Nome da pessoa para saudar")
                        .required(true)
                )
                .add_flag(
                    Flag::new("times", FlagType::Integer)
                        .short('t')
                        .description("Quantas vezes repetir a saudação")
                        .default_value(cliparser::flag::FlagValue::Integer(1))
                )
                .add_flag(
                    Flag::new("greeting", FlagType::String)
                        .short('g')
                        .description("Tipo de saudação")
                        .possible_values(vec![
                            "oi".to_string(),
                            "olá".to_string(),
                            "hello".to_string(),
                            "hola".to_string()
                        ])
                        .default_value(cliparser::flag::FlagValue::String("olá".to_string()))
                )

        )
        .add_command(
            Command::new("calc")
                .description("Calculadora simples")
                .add_subcommand(
                    Command::new("add")
                        .description("Soma números")
                        .add_flag(
                            Flag::new("numbers", FlagType::IntegerList)
                                .description("Números a serem somados")
                                .required(true)
                        )
                )
                .add_subcommand(
                    Command::new("multiply")
                        .description("Multiplica números")
                        .add_flag(
                            Flag::new("numbers", FlagType::IntegerList)
                                .description("Números a serem multiplicados")
                                .required(true)
                        )
                )
        );

    match app.run_from_env() {
        Ok(parsed) => {
            if parsed.help_requested {
                return;
            }


            match parsed.subcommand.as_deref() {
                Some("hello") => handle_hello_command(&parsed),
                Some("calc") => handle_calc_command(&parsed),
                _ => {
                    cliparser::ui::ColoredUI::show_warning("Nenhum comando especificado");
                    cliparser::ui::ColoredUI::show_info("Use --help para ver os comandos disponíveis");
                }
            }
        }
        Err(_) => {
            std::process::exit(1);
        }
    }
}

fn handle_hello_command(parsed: &cliparser::ParsedArgs) {
    let name = parsed.get_flag("name")
        .and_then(|v| v.as_string())
        .unwrap_or("Mundo");

    let times = parsed.get_flag("times")
        .and_then(|v| v.as_integer())
        .unwrap_or(1);

    let greeting = parsed.get_flag("greeting")
    .and_then(|v| v.as_string())
        .unwrap_or("olá");

    for i in 1..=times {
        let message = format!("{}, {}!", greeting, name);
        if times > 1 {
            cliparser::ui::ColoredUI::show_success(&format!("({}/{}) {}", i, times, message));
        }else {
            cliparser::ui::ColoredUI::show_success(&message);
        }
    }
}

pub fn handle_calc_command(parsed: &ParsedArgs) {
    if let Some(numbers) = parsed.get_flag("numbers") {
        if let Some(number_list) = numbers.as_integer_list() {
            match parsed.command.as_str() {
                "add" => {
                    let sum: i64 = number_list.iter().sum();
                    cliparser::ui::ColoredUI::show_success(&format!("Soma: {}", sum));
                }
                "multiply" => {
                    let product: i64 = number_list.iter().product();
                    cliparser::ui::ColoredUI::show_success(&format!("Produto: {}", product));
                }
                _ => {}
            }
        }
    }
}