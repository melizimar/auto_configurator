use auto_configurator::adapters;
use auto_configurator::core::entities;
use auto_configurator::core::use_cases;
use auto_configurator::adapters::config_dto;

use std::fs::File;
use std::io::BufReader;
use std::env;

fn main() { 
    // Obtém o caminho do arquivo de configuração da variável de ambiente
    // Defina com o comando Set-Item -Path Env:CONFIG_PATH -Value "./config.json"
    // Para remover utilize Remove-Item -Path Env:CONFIG_PATH
    /* 
    let config_path = match env::var("CONFIG_PATH") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Variável de ambiente CONFIG_PATH não definida.");
            return;
        }
    };
    */

    let config_path = "config.json";

    // Abra o arquivo JSON
    let file = File::open(config_path).expect("Falha ao abrir wingetApps.json");
    let reader = BufReader::new(file);
    
    // Deserializando o JSON para a estrutura Config
    let config: adapters::config_dto::Config = serde_json::from_reader(reader).expect("Falha ao ler o arquivo JSON");

    // Executando a função para configurar a política de execução
    //core::use_cases::set_execution_policy();

    // Cria usuários no Windows
    use_cases::user::create_system::create_system_users(config.users);

    // Iterar sobre a lista de aplicativos e instalar cada um
    use_cases::application::install_winget::install_winget_applications(config.apps);

}