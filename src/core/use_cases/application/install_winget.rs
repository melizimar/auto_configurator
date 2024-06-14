use crate::core::entities::application_entity::Application;

use std::process::Command;

pub fn install_winget_application(app: Application){
    let command = format!("winget install -e --id {}", app.name);
    println!("Executando: {}", command);

    let status = Command::new("powershell")
        .args(&["/C", &command])
        .status()
        .expect("Falha ao executar o comando");

    if status.success() {
        println!("{} instalado com sucesso!", app.name);
    } else {
        eprintln!("Falha ao instalar {}", app.name);
    }
}

pub fn install_winget_applications(apps: Vec<Application>){
    for app in apps {
        install_winget_application(app);
    }
}