use failure::Fallible;

use headless_chrome::{Browser, LaunchOptions};

use std::time::Duration;
use std::thread;

fn query(input: &str) -> Fallible<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
                    .headless(false)
                    .user_data_dir(Some(&std::path::Path::new("/Users/cedric/Library/Application Support/Google/Chrome")))
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://en.wikipedia.org")?;
    /*
    tab.type_str(&input)?.press_key("Enter")?;
    match tab.wait_for_element("div.shortdescription") {
        Err(e) => eprintln!("Query failed: {:?}", e),
        Ok(e) => match e.get_description()?.find(|n| n.node_name == "#text") {
            Some(n) => println!("Result for `{}`: {}", &input, n.node_value),
            None => eprintln!("No shortdescription-node found on page"),
        },
    }*/
    thread::sleep(Duration::from_secs(1000));
    Ok(())
}

fn main() -> Fallible<()> {

    env_logger::try_init().unwrap_or(());

    let input = "Elvis Aaron Presley";
    query(input)
}
