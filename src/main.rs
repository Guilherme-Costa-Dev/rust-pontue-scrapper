use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[derive(serde::Deserialize)]
struct Config {
    nome: String,
    email: String,
    login: String,
    senha: String,
    google_app_key: String,
}

fn main() {
    let config: Config = load_config().expect("Falha ao carregar config.json");
    loop {
        {
            match get_id(&config) {
                Ok((id, nota)) => {
                    let novo = check_old(&id).expect("Falha ao checar last.txt");
                    if novo {
                        println!("Redação nova encontrada");
                        match send_email(&config, &nota) {
                            Ok(_) => {
                                println!("Email enviado com sucesso");
                                break;
                            }
                            Err(e) => {
                                println!("Falha ao enviar o email: {e}");
                                break;
                            }
                        }                         
                    } 
                    else {
                        println!("Nenhuma redação nova encontrada");
                        println!("Procurando novamente em 1h");
                    }
                }
                Err(e) => {
                    eprintln!("Falha ao pegar o ID da redação: {e}");
                    match send_err_email(&config, e) {
                        Ok(_) => {
                            println!("Email enviado com sucesso");
                            break;
                        },
                        Err(e) => {
                            println!("Falha ao enviar o email: {e}");
                        }
                    }
                println!("Tentando novamente em 1h");
                }
            }
        }
        sleep(Duration::from_mins(60));
    }
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("config.json");
    let config_str = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}

fn send_err_email(config: &Config, e: Box<dyn Error>) -> Result<(), Box<dyn Error>> {
    let nome = &config.nome;
    let email = &config.email;
    let key = &config.google_app_key;
    let email_struct = Message::builder()
        .from(format!("{nome} <{email}>").parse()?)
        .to(format!("{nome} <{email}>").parse()?)
        .subject("Erro no rust-pontue-scrapper")
        .body(format!("Erro ao pegar o ID da redação: {e}").to_string())?;

    let creds = Credentials::new(email.to_string(), key.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email_struct)?;

    Ok(())
}

fn send_email(config: &Config, nota: &String) -> Result<(), Box<dyn Error>> {
    let nome = &config.nome;
    let email = &config.email;
    let key = &config.google_app_key;
    let email_struct = Message::builder()
        .from(format!("{nome} <{email}>").parse()?)
        .to(format!("{nome} <{email}>").parse()?)
        .subject("rust-pontue-scrapper")
        .body(format!("Redação nova corrigida. Nota: {nota}").to_string())?;

    let creds = Credentials::new(email.to_string(), key.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email_struct)?;

    Ok(())
}


fn check_old(id: &str) -> Result<bool, Box<dyn Error>> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("last.txt");
    let ultimo_id = fs::read_to_string(&path).unwrap_or_default();
    if ultimo_id != id {
        fs::write(path, id)?;
        return Ok(true);
    } else {
        Ok(false)
    }
}

fn get_id(config: &Config) -> Result<(String, String), Box<dyn Error>> {
    let login = &config.login;
    let senha = &config.senha;

    let mut headers = HeaderMap::new();
    headers.insert(ORIGIN, HeaderValue::from_static("https://app.pontue.com.br"));
    headers.insert(REFERER, HeaderValue::from_static("https://app.pontue.com.br/"));
    headers.insert(
        USER_AGENT, 
        HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36")
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let login = json!({
        "email": login,
        "password": senha
    });

    let client = Client::builder().default_headers(headers).build()?;

    let res = client
        .post("https://api.pontue.com.br/auth/login")
        .json(&login)
        .send()?;

    let json_response: Value = res.json()?;
    let id = json_response["aluno"]["id"].as_str().ok_or("Falha ao pegar o ID do aluno")?;
    let token = json_response["access_token"].as_str().ok_or("Falha ao pegar o token do login")?;
    let url = format!("https://api.pontue.com.br/alunos/{id}/redacaos/corrected");

    let red = client.get(url).bearer_auth(token).send()?;
    let json_redacoes: Value = red.json()?;
    let redacao = &json_redacoes["data"][0];
    let id = redacao["numero"].to_string();
    let nota = redacao["correcao"]["nota_final"].to_string();

    Ok((id, nota))
}
