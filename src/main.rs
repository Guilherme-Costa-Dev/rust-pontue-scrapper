use headless_chrome::{Browser, LaunchOptionsBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[derive(Deserialize)]
struct Config {
    nome: String,
    email: String,
    login: String,
    senha: String,
    google_app_key: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = load_config()?;
    loop {
        let id = get_id(&config)?;
        let novo = check_old(&id)?;

        if novo {
            send_email(&config)?;
            break;
        }
        sleep(Duration::from_mins(30));
    }
    Ok(())
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let user = env::var("USER")?;
    let path = format!("/home/{user}/rust-pontue-scrapper/config.json");
    let config_str = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}

fn send_email(config: &Config) -> Result<(), Box<dyn Error>> {
    let nome = &config.nome;
    let email = &config.email;
    let key = &config.google_app_key;
    let email_struct = Message::builder()
        .from(format!("{nome} <{email}>").parse()?)
        .to(format!("{nome} <{email}>").parse()?)
        .subject("rust-pontue-scrapper")
        .body(("Redação nova corrigida").to_string())?;

    let creds = Credentials::new(email.to_string(), key.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email_struct)?;

    Ok(())
}

fn check_old(id: &str) -> Result<bool, Box<dyn Error>> {
    let user = env::var("USER")?;
    let path = format!("/home/{user}/rust-pontue-scrapper/last.txt");
    let ultimo_id = fs::read_to_string(&path).unwrap_or_default();

    if ultimo_id != id {
        fs::write(&path, id)?;
        return Ok(true);
    } else {
        Ok(false)
    }
}

fn get_id(config: &Config) -> Result<String, Box<dyn Error>> {
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .args(vec![
            std::ffi::OsStr::new("--disable-gpu"),
            std::ffi::OsStr::new("--no-sandbox"),
            std::ffi::OsStr::new("--disable-dev-shm-usage"),
        ])
        .build()?;

    let browser = Browser::new(options)?;
    let url = "https://app.pontue.com.br/login";
    let tab = browser.new_tab()?;

    tab.navigate_to(&url)?;

    tab.wait_for_element("input[name='email']")?.click()?;
    tab.type_str(&config.login)?;

    tab.wait_for_element("input[type='password']")?.click()?;
    tab.type_str(&config.senha)?;

    tab.press_key("Enter")?;
    sleep(Duration::from_secs(2));

    tab.wait_for_element("#menu-left-student-show > a > span > i")?
        .click()?;

    tab.wait_for_element(
        "#entity-content > div > div > div > nav > ul > li:nth-child(2) > a > span.icon > i",
    )?
    .click()?;
    sleep(Duration::from_secs(10));

    let num = tab
        .wait_for_element("td[data-label='Nº'")?
        .get_inner_text()
        .unwrap_or_default();

    Ok(num.to_string())
}
