use std::fs;

use crate::models::{Role, User};



pub fn get_users() -> Vec<User> {
    vec![
        User {
            username: "admin".to_string(),
            password: "admin".to_string(),
            role: Role::Admin,
        },
        User {
            username: "Dave".to_string(),
            password: "Dave".to_string(),
            role: Role::User,
        },
        User {
            username: "user".to_string(),
            password: "user".to_string(),
            role: Role::User,
        },

    ]
}


pub fn login_success(role: &Role) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(".session", role.to_string())?;
    Ok(())
}

pub fn get_logged_in_role() -> Result<Option<Role>, Box<dyn std::error::Error>> {
    let role = fs::read_to_string(".session")?;
    match role.as_str()  {
        "Admin" => Ok(Some(Role::Admin)),
        "User" => Ok(Some(Role::User)),
        _ => Ok(None),
    }
}

pub fn logout() {
    fs::remove_file(".session").unwrap_or_else(|_|{
        println!("没有用户登陆");
    });
}