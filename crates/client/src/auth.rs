use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user: ReadSignal<Option<User>>,
    pub set_user: WriteSignal<Option<User>>,
    pub is_loading: ReadSignal<bool>,
    pub set_loading: WriteSignal<bool>,
}

impl AuthContext {
    pub async fn login(&self, email: String, password: String) -> Result<(), String> {
        self.set_loading.set(true);
        
        let login_data = LoginRequest { email, password };
        
        let request = Request::post("http://localhost:3000/login")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("email={}&password={}", 
                urlencoding::encode(&login_data.email),
                urlencoding::encode(&login_data.password)
            ));
            
        let response = match request {
            Ok(req) => req.send().await,
            Err(_) => {
                self.set_loading.set(false);
                return Err("Failed to create request".to_string());
            }
        };

        match response {
            Ok(resp) => {
                if resp.ok() {
                    // After successful login, fetch user info
                    match self.fetch_user().await {
                        Ok(user) => {
                            self.set_user.set(Some(user));
                            self.set_loading.set(false);
                            Ok(())
                        }
                        Err(e) => {
                            self.set_loading.set(false);
                            Err(e)
                        }
                    }
                } else {
                    self.set_loading.set(false);
                    match resp.status() {
                        401 => Err("Invalid email or password".to_string()),
                        403 => Err("Account not verified or inactive".to_string()),
                        _ => Err("Login failed".to_string()),
                    }
                }
            }
            Err(_) => {
                self.set_loading.set(false);
                Err("Network error".to_string())
            }
        }
    }

    pub async fn logout(&self) -> Result<(), String> {
        let response = Request::post("http://localhost:3000/logout")
            .send()
            .await;

        match response {
            Ok(_) => {
                self.set_user.set(None);
                Ok(())
            }
            Err(_) => Err("Logout failed".to_string()),
        }
    }

    pub async fn fetch_user(&self) -> Result<User, String> {
        let response = Request::get("http://localhost:3000/me")
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.ok() {
                    match resp.json::<User>().await {
                        Ok(user) => Ok(user),
                        Err(_) => Err("Failed to parse user data".to_string()),
                    }
                } else {
                    Err("Not authenticated".to_string())
                }
            }
            Err(_) => Err("Network error".to_string()),
        }
    }

    pub async fn check_auth(&self) {
        match self.fetch_user().await {
            Ok(user) => self.set_user.set(Some(user)),
            Err(_) => self.set_user.set(None),
        }
    }
}

pub fn provide_auth_context() -> AuthContext {
    let (user, set_user) = signal(None::<User>);
    let (is_loading, set_loading) = signal(false);
    
    let context = AuthContext {
        user: user.into(),
        set_user,
        is_loading: is_loading.into(),
        set_loading,
    };
    
    provide_context(context.clone());
    context
}

pub fn use_auth() -> AuthContext {
    expect_context::<AuthContext>()
}
