use crate::core::entities::user_entity::User;
use std::process::Command;

pub fn create_system_user(user: User){
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

pub fn create_system_users(users: Vec<User>){
    for user in users {
        create_system_user(user);
    }
}