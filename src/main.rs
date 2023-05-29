use serde::{Deserialize, Serialize}; // for serialization and deserialization of JSON
use rust_twitter_bot_lib;


const CONFIG_FILENAME: &str = "config.json"; // use the config.json as constant
const SUBGRAPH_OUTPUT_FILENAME: &str = "output.csv";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
    pub interval_hours: u64, 
}

fn main() {
    let config_path: std::path::PathBuf = dirs2::config_dir()
    .expect("Failed to get config file")
    .join("subgraph_output"); // under the subgraph_output direction

    let config: Config = serde_json::from_str(
        &std::fs::read_to_string(&config_path.join(CONFIG_FILENAME))
        .expect("Could not open the config file.")
    )
    .expect("Could not parse the config file.");

    let output: Vec<String> = std::fs::read_to_string(&config_path.
        join(SUBGRAPH_OUTPUT_FILENAME))
        .expect("Could not find the Subgraph output file.")
        .lines()
        .map(|s| s.to_owned())
        .collect(); 

    let twitter_bot = rust_twitter_bot_lib::TwitterBot::new() //new instance of Twitter bot
        .consumer_key(&config.consumer_key)
        .consumer_secret_key(&config.consumer_secret)
        .access_token(&config.access_key)
        .secret_access_token(&config.access_secret);

    for o in output {
        match twitter_bot.tweet(&o, None) { // tweet every line without parameters
            Ok(_) => println!("Tweeted the line \"{}\"", o),
            Err(e) => panic!("{:?}", e), 
        };

        std::thread::sleep(std::time::Duration::from_millis(
            1000 * 60 * 60 * config.interval_hours, // will tweeet a single line from the output
            // with every 1 hour
        ));
    }
}
