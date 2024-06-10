use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use std::env;

#[derive(Deserialize)]
struct Config {
    winget_apps: Vec<String>,
    users: Vec<User>,
    font_folder: String
}

#[derive(Deserialize)]
struct User {
    user_type: String,
    username: String,
    password: String,
}

fn set_execution_policy(){
    // Executando o comando PowerShell para configurar a política de execução
    let ps_command = "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser";
    let status = Command::new("powershell")
    .args(&["-Command", ps_command])
    .status()
    .expect("Falha ao executar o comando PowerShell");

    if status.success() {
        println!("Política de execução do PowerShell configurada com sucesso!");
    } else {
        eprintln!("Falha ao configurar a política de execução do PowerShell");
    }
}

fn create_system_users(users: Vec<User>){
    // Criando usuários conforme configurado no JSON
    for user in users {
        let username = &user.username;
        let password = &user.password;

        // Comando PowerShell para criar um novo usuário
        let create_user_command = format!(
            "New-LocalUser -Name '{}' -Password (ConvertTo-SecureString '{}' -AsPlainText -Force) -FullName '{}' -Description 'Usuário criado pelo script Rust'",
            username, password, username
        );

        // Executando o comando PowerShell para criar o novo usuário
        let status = Command::new("powershell")
            .args(&["-Command", &create_user_command])
            .status()
            .expect("Falha ao executar o comando PowerShell para criar usuário");

        if status.success() {
            println!("Usuário {} criado com sucesso!", username);
        } else {
            eprintln!("Falha ao criar o usuário {}", username);
        }

        // Se o usuário for administrador, adicioná-lo ao grupo de administradores
        if user.user_type == "admin" {
            let add_to_admins_command = format!(
                "Add-LocalGroupMember -Group 'Administrators' -Member '{}'",
                username
            );

            // Executando o comando PowerShell para adicionar o usuário ao grupo de administradores
            let status = Command::new("powershell")
                .args(&["-Command", &add_to_admins_command])
                .status()
                .expect("Falha ao executar o comando PowerShell para adicionar usuário ao grupo de administradores");

            if status.success() {
                println!("Usuário {} adicionado ao grupo de administradores com sucesso!", username);
            } else {
                eprintln!("Falha ao adicionar o usuário {} ao grupo de administradores", username);
            }
        }
    }
}

fn install_applications_winget(apps: Vec<String>){
    // Iterar sobre a lista de aplicativos e instalar cada um
    // Entender o impacto que o argumento -g traz ao winget install
        for app in apps {
            let command = format!("winget install -e --id {}", app);
            println!("Executando: {}", command);
    
            let status = Command::new("powershell")
                .args(&["/C", &command])
                .status()
                .expect("Falha ao executar o comando");
    
            if status.success() {
                println!("{} instalado com sucesso!", app);
            } else {
                eprintln!("Falha ao instalar {}", app);
            }
        }
    }

fn install_fonts(font_folder: &str) {
    // Comando PowerShell para instalar as fontes TTF
    let install_fonts_command = format!(
        r#"Get-ChildItem "{}\*.ttf" | ForEach-Object {{ Install-Font $_.FullName -Scope AllUsers }}"#,
        font_folder
    );

    // Executando o comando PowerShell para instalar as fontes
    let status = Command::new("powershell")
        .args(&["-Command", &install_fonts_command])
        .status()
        .expect("Falha ao executar o comando PowerShell para instalar fontes");

    if status.success() {
        println!("Fontes instaladas com sucesso!");
    } else {
        eprintln!("Falha ao instalar fontes");
    }
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
    let config: Config = serde_json::from_reader(reader).expect("Falha ao ler o arquivo JSON");

    // Executando a função para configurar a política de execução
    set_execution_policy();
    
    // Cria usuários no Windows
    create_system_users(config.users);

    // Iterar sobre a lista de aplicativos e instalar cada um
    install_applications_winget(config.winget_apps);

    // Instala as fontes TTF
    install_fonts(&config.font_folder);
}