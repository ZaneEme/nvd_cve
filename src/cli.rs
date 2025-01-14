use clap::ArgMatches;
use nvd_cve::cache::{search_by_id, CacheConfig};
use nvd_cve::cache::{search_description, sync_blocking};
use nvd_cve::client::{BlockingHttpClient, ReqwestBlockingClient};

pub fn sync(matches: &ArgMatches) {
    let mut config = CacheConfig::new();

    if matches.get_flag("show-default") {
        println!("Default Config Values:\n{}", config);
        return;
    }

    if let Some(url) = matches.get_one::<String>("url").map(|s| s.as_str()) {
        config.url = String::from(url);
    }

    if let Some(feeds) = matches.get_one::<String>("feeds").map(|s| s.as_str()) {
        config.feeds = feeds.split(',').map(|feed| feed.to_string()).collect();
    }

    if let Some(db) = matches.get_one::<String>("db").map(|s| s.as_str()) {
        config.db = String::from(db);
    }

    if matches.get_flag("no-progress") {
        config.show_progress = false;
    }

    if matches.get_flag("force") {
        config.force_update = true;
    }

    if matches.get_flag("verbose") {
        env_logger::init();
    }

    let client = ReqwestBlockingClient::new(&config.url, None, None, None);

    if let Err(error) = sync_blocking(&config, client) {
        eprintln!("Fatal Error: {:?}", error);
        std::process::exit(1);
    }
}

pub fn search(matches: &ArgMatches) {
    let mut config = CacheConfig::new();

    if let Some(db) = matches.get_one::<String>("db").map(|s| s.as_str()) {
        config.db = String::from(db);
    }

    if let Some(text) = matches.get_one::<String>("text").map(|s| s.as_str()) {
        match search_description(&config, text) {
            Ok(cves) => {
                if cves.is_empty() {
                    eprintln!("No results found");
                    std::process::exit(1);
                } else {
                    for cve in cves {
                        println!("{}", cve);
                    }
                }
            }
            Err(error) => {
                eprintln!("Fatal Error: {:?}", error);
                std::process::exit(2);
            }
        }
    } else if let Some(cve) = matches.get_one::<String>("CVE").map(|s| s.as_str()) {
        match search_by_id(&config, cve) {
            Ok(cve_result) => println!("{}", serde_json::to_string_pretty(&cve_result).unwrap()),
            Err(error) => {
                eprintln!("Fatal Error: {:?}", error);
                std::process::exit(3);
            }
        }
    }
}
