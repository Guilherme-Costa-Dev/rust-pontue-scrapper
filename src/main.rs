use headless_chrome::{Browser, LaunchOptionsBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let id = get_id()?;
        let novo = check_old(&id)?;

        if novo {
            send_email()?;
            break;
        }
        sleep(Duration::from_mins(10));
    }

    Ok(())
}

fn send_email() -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from("Guilherme Cordeiro Costa <guilhermecosta030409@gmail.com>".parse()?)
        .to("Guilherme Cordeiro Costa <guilhermecosta030409@gmail.com>".parse()?)
        .subject("pontue-scrapper")
        .body(("Redação corrigida mano").to_string())?;

    let creds = Credentials::new(
        "guilhermecosta030409@gmail.com".to_string(),
        "igkm fwgd abuu unaq".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}

fn check_old(id: &str) -> Result<bool, Box<dyn Error>> {
    const FILE: &str = "last.txt";

    let ultimo_id = fs::read_to_string(FILE).unwrap_or_default();

    if ultimo_id != id {
        fs::write(FILE, id)?;
        return Ok(true);
    } else {
        Ok(false)
    }
}

fn get_id() -> Result<String, Box<dyn Error>> {
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
    tab.type_str("2618175862700")?;

    tab.wait_for_element("input[type='password']")?.click()?;
    tab.type_str("12345")?;

    tab.press_key("Enter")?;
    sleep(Duration::from_secs(2));

    tab.wait_for_element("#menu-left-student-show > a > span > i")?
        .click()?;

    tab.wait_for_element(
        "#entity-content > div > div > div > nav > ul > li:nth-child(2) > a > span.icon > i",
    )?
    .click()?;
    sleep(Duration::from_secs(10));

    let js_script = r#"
        Array.from(document.querySelectorAll('td[data-label="Nº"]'))
             .find(td => td.offsetParent !== null)
             ?.textContent.trim() || ""
    "#;

    let result = tab.evaluate(js_script, false)?;

    let num = result
        .value
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();

    Ok(num)
}
