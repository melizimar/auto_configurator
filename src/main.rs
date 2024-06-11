use std::fs::File;
use std::io::BufReader;
use std::env;

mod core {
    pub mod entities;
    pub mod use_cases;
    //pub mod repositories;
}

fn main() {
    // Obtém o caminho do arquivo de configuração da variável de ambiente
    // Defina com o comando Set-Item -Path Env:CONFIG_PATH -Value "./config.json"
    // Para remover utilize Remove-Item -Path Env:CONFIG_PATH
    let config_path = match env::var("CONFIG_PATH") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Variável de ambiente CONFIG_PATH não definida.");
            return;
        }
    };
    
    // Abra o arquivo JSON
    let file = File::open(config_path).expect("Falha ao abrir wingetApps.json");
    let reader = BufReader::new(file);
    
    // Deserializando o JSON para a estrutura Config
    let config: core::entities::config_entity::Config = serde_json::from_reader(reader).expect("Falha ao ler o arquivo JSON");

    // Executando a função para configurar a política de execução
    core::use_cases::set_execution_policy();
    
    // Cria usuários no Windows
    core::use_cases::create_user::create_system_users(config.users);

    // Iterar sobre a lista de aplicativos e instalar cada um
    core::use_cases::install_application::install_applications_winget(config.winget_apps);

    // Instala as fontes TTF
    core::use_cases::install_fonts::install_fonts(&config.font_folder);
}