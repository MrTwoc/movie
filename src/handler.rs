

use crate::services::{get_users, login_success, logout};

pub fn handler_login(username: &str) -> Result<(),Box<dyn std::error::Error>> {
    
    if let Some(user) = get_users().iter().find(|u|u.username.eq_ignore_ascii_case(username)) {
        // 此处区分大小写
        println!("Welcome {}!,Please input your password:", user.username);
        match rpassword::read_password()  {
            Ok(password)=> {
                if user.password == password {
                    login_success(&user.role)?;
                    println!("Login successful! You are logged in as {}.", user.username);
                } else {
                    println!("Incorrect password for user {}!", user.username);
                }
            }
            Err(_) => {
                println!("Failed to read password. Please try again.");
            }
        }
    } else {
        println!("User {} not found!", username);
    }
    Ok(())

}

pub fn handler_logout() {
    logout();
    println!("Logging out...");
}