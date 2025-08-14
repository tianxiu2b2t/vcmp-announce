use clap::{self, Parser};
use tracing::event;

use crate::cfg::load_config;

pub mod cfg;
pub mod announce;
pub mod logger;

#[derive(Parser, Debug)]
#[clap(author, version, about = "配置处理程序", long_about = None)]
struct Args {
    /// 创建默认配置文件
    #[clap(long)]
    default_config: Option<String>,

    /// 使用指定的配置文件
    #[clap(long)]
    config: Option<String>,
}

fn main() {    
    let args = Args::parse();

    // 处理创建默认配置文件
    if let Some(config_path) = args.default_config.clone() && !std::fs::exists(&config_path).unwrap_or(false) {
        let cfg = cfg::Config::default();
        let content = toml::to_string_pretty(&cfg).unwrap();
        std::fs::write(config_path, content).unwrap();
        println!("default config file created");
    }
    
    let config_path = match args.config {
        Some(path) => path,
        None => {
            match args.default_config {
                Some(path) => path,
                None => {
                    println!("no config file specified");
                    return;
                }
            }
        }
    };

    load_config(&config_path);
    logger::init();


    let servers = match cfg::get_servers() {
        None => {
            event!(tracing::Level::ERROR, "No servers found");
            return;
        },
        Some(servers) => {
            for (id, server) in servers.clone() {
                event!(tracing::Level::INFO, "Server ({id}): {}", server.port)
            }
            servers
        }
    };
    announce::start_announce(servers);
}
