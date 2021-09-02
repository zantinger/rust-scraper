use std::io::prelude::*;
use headless_chrome::Browser;

#[allow(unused_must_use)]
fn main() {
    get_cookies_from_site();
}

fn get_cookies_from_site() -> Result<(), failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://google.de")?;
    tab.wait_until_navigated()?;
    let cookies = tab.get_cookies()?;

    let mut buffer = std::fs::File::create("cookies.txt")?;

    for i in &cookies {
        writeln!(buffer, "{} {}", i.name, i.value)?;
    }
    Ok(())
}
