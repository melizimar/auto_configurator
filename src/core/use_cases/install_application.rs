use std::process::Command;

pub fn install_application_winget(app: String){
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

pub fn install_applications_winget(apps: Vec<String>){
    for app in apps {
        install_application_winget(app);
    }
}