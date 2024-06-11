pub mod create_user;
pub mod install_application;
pub mod install_fonts;

use std::process::Command;

pub fn set_execution_policy(){
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