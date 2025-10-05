# Documentação da API - cliparser

Esta documentação detalha todos os módulos, structs, enums e funções da biblioteca `cliparser`.

## Módulos

### `cli`
Contém a aplicação CLI principal (`CliApp`) e funcionalidades de alto nível.

### `command`
Define a estrutura de comandos e subcomandos.

### `flag`
Sistema de flags/opções com diferentes tipos e validações.

### `parser`
Engine de parsing que processa argumentos da linha de comando.

### `ui`
Interface de usuário colorida usando ratatui para output amigável.

### `error`
Sistema de erros específicos para operações CLI.

---

## `CliApp`

### Descrição
Struct principal que representa uma aplicação CLI completa.

### Campos
```rust
pub struct CliApp {
    pub name: String,        // Nome da aplicação
    pub version: String,     // Versão da aplicação  
    pub description: String, // Descrição da aplicação
    pub root_command: Command, // Comando raiz contendo subcomandos
}
```

### Métodos

#### `new(name: impl Into<String>, version: impl Into<String>) -> Self`
Cria uma nova aplicação CLI.

**Parâmetros:**
- `name`: Nome da aplicação
- `version`: Versão da aplicação

**Exemplo:**
```rust
let app = CliApp::new("minha-app", "1.2.3");
```

#### `description(self, description: impl Into<String>) -> Self`
Define a descrição da aplicação (builder pattern).

#### `add_command(self, command: Command) -> Self`  
Adiciona um comando à aplicação.

#### `add_global_flag(self, flag: Flag) -> Self`
Adiciona uma flag que estará disponível globalmente.

#### `parse<I, S>(&self, args: I) -> Result<ParsedArgs>`
Parseia argumentos fornecidos como iterador.

#### `run<I, S>(&self, args: I) -> Result<ParsedArgs>`
Executa o parsing com tratamento automático de erros e help.

#### `run_from_env(&self) -> Result<ParsedArgs>`
Executa usando argumentos do `std::env::args()`.

#### `validate(&self) -> Result<()>`
Valida a configuração da aplicação (flags duplicadas, etc).

#### `get_info(&self) -> AppInfo`
Retorna informações estruturadas sobre a aplicação.

---

## `Command`

### Descrição
Representa um comando ou subcomando CLI.

### Campos
```rust
pub struct Command {
    pub name: String,
    pub description: String,
    pub flags: HashMap<String, Flag>,
    pub subcommands: HashMap<String, Command>,
    pub positional_args: Vec<PositionalArg>,
    pub show_help_on_empty: bool,
}
```

### Métodos

#### `new(name: impl Into<String>) -> Self`
Cria um novo comando.

#### `description(self, description: impl Into<String>) -> Self`
Define a descrição do comando.

#### `add_flag(self, flag: Flag) -> Self`
Adiciona uma flag ao comando.

#### `add_subcommand(self, subcommand: Command) -> Self`
Adiciona um subcomando.

#### `add_positional_arg(self, arg: PositionalArg) -> Self`
Adiciona um argumento posicional.

#### `show_help_on_empty(self, show: bool) -> Self`
Define se deve mostrar help quando nenhum argumento é fornecido.

#### `get_flag(&self, name: &str) -> Option<&Flag>`
Busca uma flag pelo nome longo ou curto.

#### `get_subcommand(&self, name: &str) -> Option<&Command>`
Busca um subcomando pelo nome.

#### `has_subcommands(&self) -> bool`
Verifica se o comando tem subcomandos.

#### `has_flags(&self) -> bool`
Verifica se o comando tem flags.

---

## `Flag`

### Descrição
Representa uma flag/opção CLI com tipo e validações.

### Campos
```rust
pub struct Flag {
    pub name: String,
    pub short: Option<char>,
    pub flag_type: FlagType,
    pub description: String,
    pub required: bool,
    pub default_value: Option<FlagValue>,
    pub possible_values: Option<Vec<String>>,
}
```

### Métodos

#### `new(name: impl Into<String>, flag_type: FlagType) -> Self`
Cria uma nova flag.

#### `short(self, short: char) -> Self`
Define nome curto da flag (ex: 'v' para -v).

#### `description(self, description: impl Into<String>) -> Self`
Define a descrição da flag.

#### `required(self, required: bool) -> Self`
Define se a flag é obrigatória.

#### `default_value(self, value: FlagValue) -> Self`
Define valor padrão.

#### `possible_values(self, values: Vec<String>) -> Self`
Define lista de valores válidos.

#### `parse_value(&self, value: &str) -> Result<FlagValue>`
Parseia um valor string para o tipo da flag.

#### `parse_values(&self, values: &[String]) -> Result<FlagValue>`
Parseia múltiplos valores (para listas).

---

## `FlagType`

### Descrição
Enum que define os tipos de valores que uma flag pode aceitar.

### Variantes
```rust
pub enum FlagType {
    Bool,        // Flag booleana (--verbose)
    String,      // String (--name "João")  
    Integer,     // Inteiro i64 (--count 42)
    Float,       // Float f64 (--ratio 3.14)
    StringList,  // Lista de strings (--files a.txt b.txt)
    IntegerList, // Lista de inteiros (--numbers 1 2 3)
}
```

### Métodos

#### `description(&self) -> &'static str`
Retorna descrição legível do tipo.

---

## `FlagValue`

### Descrição
Enum que representa o valor parseado de uma flag.

### Variantes
```rust
pub enum FlagValue {
    Bool(bool),
    String(String),
    Integer(i64),
    Float(f64),
    StringList(Vec<String>),
    IntegerList(Vec<i64>),
}
```

### Métodos

#### `as_string(&self) -> Option<&str>`
Converte para string se o tipo for compatível.

#### `as_bool(&self) -> Option<bool>`
Converte para bool se o tipo for compatível.

#### `as_integer(&self) -> Option<i64>`
Converte para inteiro se o tipo for compatível.

#### `as_float(&self) -> Option<f64>`
Converte para float se o tipo for compatível.

#### `as_string_list(&self) -> Option<&Vec<String>>`
Converte para lista de strings se o tipo for compatível.

#### `as_integer_list(&self) -> Option<&Vec<i64>>`
Converte para lista de inteiros se o tipo for compatível.

---

## `ParsedArgs`

### Descrição
Resultado do parsing contendo comandos, flags e argumentos processados.

### Campos
```rust
pub struct ParsedArgs {
    pub command: String,
    pub subcommand: Option<String>,
    pub flags: HashMap<String, FlagValue>,
    pub positional_args: Vec<String>,
    pub help_requested: bool,
}
```

### Métodos

#### `get_flag(&self, name: &str) -> Option<&FlagValue>`
Obtém valor de uma flag pelo nome.

#### `has_flag(&self, name: &str) -> bool`
Verifica se uma flag está presente.

#### `get_arg(&self, index: usize) -> Option<&String>`
Obtém argumento posicional por índice.

#### `get_args(&self) -> &Vec<String>`
Obtém todos os argumentos posicionais.

---

## `ColoredUi`

### Descrição
Utilitários para interface colorida no terminal.

### Métodos Estáticos

#### `show_help(app_name: &str, version: &str, description: &str, command: &Command)`
Exibe help formatado e colorido.

#### `show_error(error: &CliError)`
Exibe mensagem de erro em vermelho.

#### `show_success(message: &str)`
Exibe mensagem de sucesso em verde.

#### `show_warning(message: &str)`
Exibe mensagem de aviso em amarelo.

#### `show_info(message: &str)`
Exibe mensagem informativa em azul.

#### `show_progress(current: usize, total: usize, message: &str)`
Exibe barra de progresso colorida.

**Exemplo:**
```rust
for i in 1..=100 {
    ColoredUi::show_progress(i, 100, "Processando...");
    // ... processamento
}
```

#### `confirm(message: &str) -> bool`
Solicita confirmação do usuário (s/n).

**Retorna:** `true` se usuário confirmar, `false` caso contrário.

#### `show_table(headers: &[&str], rows: &[Vec<String>])`
Exibe tabela formatada com bordas.

**Exemplo:**
```rust
ColoredUi::show_table(
    &["Nome", "Status", "Tempo"],
    &[
        vec!["teste1".to_string(), "OK".to_string(), "50ms".to_string()],
        vec!["teste2".to_string(), "ERRO".to_string(), "120ms".to_string()],
    ]
);
```

#### `show_interactive_help(...) -> io::Result<()>`
Exibe help em interface interativa usando ratatui (pressione 'q' para sair).

---

## `CliError`

### Descrição
Enum de erros específicos para operações CLI.

### Variantes

#### `CommandNotFound { command: String }`
Comando especificado não existe.

#### `SubcommandNotFound { command: String, subcommand: String }`
Subcomando não encontrado para o comando.

#### `RequiredFlagMissing { flag: String }`
Flag obrigatória não foi fornecida.

#### `UnknownFlag { flag: String }`
Flag desconhecida foi especificada.

#### `InvalidFlagValue { flag: String, value: String, expected: String }`
Valor da flag é inválido para o tipo esperado.

#### `FlagValueMissing { flag: String }`
Flag requer um valor mas nenhum foi fornecido.

#### `TooManyArguments`
Muitos argumentos posicionais foram fornecidos.

#### `InsufficientArguments { expected: usize, provided: usize }`
Argumentos posicionais obrigatórios não foram fornecidos.

#### `IoError(String)`
Erro de I/O durante operação.

#### `ParseError { message: String }`
Erro genérico de parsing.

#### `ConfigurationError { message: String }`
Erro na configuração da aplicação.

### Conversões

Implementa `From<std::io::Error>` para conversão automática de erros de I/O.

---

## `PositionalArg`

### Descrição
Representa um argumento posicional (não-flag).

### Campos
```rust
pub struct PositionalArg {
    pub name: String,
    pub description: String,
    pub required: bool,
}
```

### Métodos

#### `new(name: impl Into<String>) -> Self`
Cria novo argumento posicional.

#### `description(self, description: impl Into<String>) -> Self`
Define descrição do argumento.

#### `required(self, required: bool) -> Self`
Define se o argumento é obrigatório.

---

## `CliParser`

### Descrição
Engine de parsing que converte argumentos em `ParsedArgs`.

### Métodos Estáticos

#### `parse(command: &Command, args: Vec<String>) -> Result<ParsedArgs>`
Parseia argumentos usando um comando como template.

**Lógica:**
1. Identifica flags (--long, -short)
2. Parseia valores de flags
3. Identifica subcomandos
4. Coleta argumentos posicionais
5. Valida flags obrigatórias
6. Aplica valores padrão

---

## Fluxo de Parsing

```
Argumentos CLI
    ↓
CliParser::parse()
    ↓
┌──────────────────┐
│ Identifica Flags │
└──────────────────┘
    ↓
┌──────────────────┐
│ Parseia Valores  │
└──────────────────┘
    ↓
┌──────────────────┐
│ Busca Subcomando │
└──────────────────┘
    ↓
┌──────────────────┐
│ Valida Tipos     │
└──────────────────┘
    ↓
┌──────────────────┐
│ Aplica Padrões   │
└──────────────────┘
    ↓
ParsedArgs
```

---

## Exemplos Completos

### 1. Aplicação Simples

```rust
use cliparser::{CliApp, Command, Flag, FlagType};

fn main() {
    let app = CliApp::new("git-clone", "1.0.0")
        .description("Clona repositórios")
        .add_command(
            Command::new("clone")
                .description("Clona um repositório")
                .add_flag(
                    Flag::new("url", FlagType::String)
                        .required(true)
                        .description("URL do repositório")
                )
                .add_flag(
                    Flag::new("depth", FlagType::Integer)
                        .description("Profundidade do clone")
                        .default_value(cliparser::flag::FlagValue::Integer(1))
                )
        );

    match app.run_from_env() {
        Ok(parsed) if !parsed.help_requested => {
            let url = parsed.get_flag("url").unwrap().as_string().unwrap();
            let depth = parsed.get_flag("depth").unwrap().as_integer().unwrap();
            
            cliparser::ui::ColoredUi::show_info(
                &format!("Clonando {} com profundidade {}", url, depth)
            );
        }
        Err(_) => std::process::exit(1),
        _ => {}
    }
}
```

### 2. Múltiplos Subcomandos

```rust
let app = CliApp::new("docker", "1.0.0")
    .add_command(
        Command::new("container")
            .add_subcommand(
                Command::new("list")
                    .add_flag(Flag::new("all", FlagType::Bool).short('a'))
            )
            .add_subcommand(
                Command::new("stop")
                    .add_flag(Flag::new("id", FlagType::String).required(true))
            )
    )
    .add_command(
        Command::new("image")
            .add_subcommand(
                Command::new("pull")
                    .add_flag(Flag::new("name", FlagType::String).required(true))
            )
    );
```

### 3. Validação com Valores Possíveis

```rust
let app = CliApp::new("deploy", "1.0.0")
    .add_command(
        Command::new("deploy")
            .add_flag(
                Flag::new("env", FlagType::String)
                    .required(true)
                    .possible_values(vec![
                        "dev".to_string(),
                        "staging".to_string(),
                        "prod".to_string(),
                    ])
            )
    );

// deploy --env invalid  -> Erro: InvalidFlagValue
// deploy --env prod     -> OK
```

### 4. Listas de Valores

```rust
let app = CliApp::new("compiler", "1.0.0")
    .add_command(
        Command::new("build")
            .add_flag(
                Flag::new("sources", FlagType::StringList)
                    .required(true)
                    .description("Arquivos fonte")
            )
            .add_flag(
                Flag::new("optimization", FlagType::Integer)
                    .possible_values(vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()])
            )
    );

// Uso: compiler build --sources main.rs lib.rs utils.rs --optimization 2
```

### 5. Interface Rica

```rust
use cliparser::ui::ColoredUi;

fn deploy(env: &str) {
    ColoredUi::show_info(&format!("Iniciando deploy para {}", env));
    
    if env == "prod" {
        if !ColoredUi::confirm("Você tem certeza que deseja fazer deploy em PRODUÇÃO?") {
            ColoredUi::show_warning("Deploy cancelado");
            return;
        }
    }
    
    let steps = vec![
        "Validando configuração",
        "Fazendo backup",
        "Enviando arquivos",
        "Instalando dependências",
        "Reiniciando serviços",
    ];
    
    for (i, step) in steps.iter().enumerate() {
        ColoredUi::show_progress(i + 1, steps.len(), step);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    ColoredUi::show_success("Deploy concluído com sucesso!");
    
    // Mostra tabela de resultados
    let results = vec![
        vec!["Serviço".to_string(), "Status".to_string(), "Porta".to_string()],
        vec!["API".to_string(), "Running".to_string(), "8080".to_string()],
        vec!["Frontend".to_string(), "Running".to_string(), "3000".to_string()],
        vec!["Database".to_string(), "Running".to_string(), "5432".to_string()],
    ];
    
    println!("\nServiços:");
    ColoredUi::show_table(
        &["Serviço", "Status", "Porta"],
        &results[1..].to_vec()
    );
}
```

---

## Padrões de Uso Recomendados

### 1. Validação na Criação

```rust
let app = create_app();
if let Err(e) = app.validate() {
    eprintln!("Erro na configuração: {}", e);
    std::process::exit(1);
}
```

### 2. Tratamento de Erros Granular

```rust
match app.run_from_env() {
    Ok(parsed) if parsed.help_requested => return,
    Ok(parsed) => handle_command(parsed),
    Err(CliError::RequiredFlagMissing { flag }) => {
        eprintln!("Flag obrigatória não fornecida: --{}", flag);
        eprintln!("Use --help para mais informações");
        std::process::exit(1);
    }
    Err(e) => {
        ColoredUi::show_error(&e);
        std::process::exit(1);
    }
}
```

### 3. Logging Configurável

```rust
fn setup_logging(parsed: &ParsedArgs) {
    let verbose = parsed.get_flag("verbose")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    if verbose {
        // Configura log detalhado
    }
}
```

### 4. Testes Unitários

```rust
#[test]
fn test_my_command() {
    let app = create_app();
    let result = app.parse(vec!["my-cmd", "--flag", "value"]);
    
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(
        parsed.get_flag("flag").unwrap().as_string().unwrap(),
        "value"
    );
}
```

---

## Performance

### Complexidade
- Parsing: O(n) onde n é número de argumentos
- Validação: O(f) onde f é número de flags
- Busca de comando: O(1) usando HashMap

### Otimizações
- HashMap para lookups O(1) de comandos e flags
- Parsing single-pass sempre que possível
- Validação lazy (apenas quando necessário)

---

## Limitações Conhecidas

1. **Não suporta flags encadeadas**: `-abc` não expande para `-a -b -c`
2. **Subcomandos não herdam flags do pai**: Cada comando tem suas próprias flags
3. **Sem suporte a argumentos variadicos nativos**: Use `StringList` ou `IntegerList`
4. **Help sempre em português**: Internacionalização não implementada

---

## Roadmap Futuro

- [ ] Suporte a internacionalização (i18n)
- [ ] Flags encadeadas (`-abc`)
- [ ] Autocompletion para shells
- [ ] Geração de man pages
- [ ] Serialização/desserialização de configuração
- [ ] Modo interativo (REPL)
- [ ] Temas de cores personalizáveis
- [ ] Plugins/extensões

---

## Troubleshooting

### Erro: "Flag duplicada encontrada"
**Causa:** Duas flags com o mesmo nome ou short name.  
**Solução:** Use nomes únicos para cada flag.

### Erro: "Flag não pode ser obrigatória e ter valor padrão"
**Causa:** Flag marcada como `required(true)` e com `default_value()`.  
**Solução:** Remova um dos dois - flags obrigatórias não precisam de padrão.

### Help não aparece
**Causa:** `show_help_on_empty(false)` configurado.  
**Solução:** Remova essa configuração ou chame `--help` explicitamente.

### Cores não aparecem no Windows
**Causa:** Terminal do Windows pode não suportar códigos ANSI.  
**Solução:** Use Windows Terminal ou habilite suporte ANSI.

---

## Recursos Adicionais

- [README.md](../README.md) - Visão geral e quickstart
- [examples/](../examples/) - Exemplos funcionais
- [tests/](../tests/) - Suite de testes completa
- [Cargo.toml](../Cargo.toml) - Dependências e metadata

---
