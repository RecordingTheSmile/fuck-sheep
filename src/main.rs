use clap::{Arg, Command};
use fuck_sheep::FuckSheep;
use num_cpus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = Command::new("羊了个羊刷通关")
        .version("0.1")
        .author("RTSmile")
        .about("羊了个羊刷关器，采用Rust+Tokio编写的高性能多核刷关器，仅供学习交流使用")
        .arg(
            Arg::new("parallel")
                .help("并行线程数量，默认为CPU核心数，如无特殊需要建议不修改")
                .short('p')
                .takes_value(true)
                .allow_hyphen_values(true)
                .default_value(&num_cpus::get().to_string()),
        )
        .arg(
            Arg::new("times")
                .takes_value(true)
                .short('s')
                .allow_hyphen_values(true)
                .help("需要刷得的次数，默认为1000"),
        )
        .arg(
            Arg::new("uid")
                .help("用户UID，必填")
                .takes_value(true)
                .short('u')
                .allow_hyphen_values(true)
                .required(true),
        )
        .subcommand(Command::new("once").about("只运行一次刷通关"))
        .get_matches();

    let mut fuck_sheep = FuckSheep::new();

    fuck_sheep.parse_args(matches.to_owned())?;
    println!("开始获取用户Token");
    fuck_sheep.get_token().await?;

    println!("开始刷次数！");

    if matches.to_owned().subcommand_name().unwrap_or("") == "once" {
        fuck_sheep.start_once().await?;
        println!("刷次数完成，感谢使用！");
    } else {
        let start_time = chrono::Local::now().timestamp();
        let total_count = fuck_sheep.start().await?;
        let end_time = chrono::Local::now().timestamp();
        println!(
            "刷次数完成！一共刷完成次数为：{}，用时为：{}秒,感谢使用！",
            total_count,
            end_time - start_time
        );
    }
    Ok(())
}
