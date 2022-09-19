use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use chrono::Local;
use clap::ArgMatches;
use rand::Rng;
use reqwest::StatusCode;
use std::time::Duration;
use tokio::task::JoinHandle;

const ENC_TOKEN:&str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2OTQ1MzQxMzcsIm5iZiI6MTY2MzQzMTkzNywiaWF0IjoxNjYzNDMwMTM3LCJqdGkiOiJDTTpjYXRfbWF0Y2g6bHQxMjM0NTYiLCJvcGVuX2lkIjoiIiwidWlkIjoxMzU5Njk1MiwiZGVidWciOiIiLCJsYW5nIjoiIn0.rxNp69Cy_UmYZt1uzsGkIKFBOZehW3vXzo3kltJtybY";

#[derive(Debug)]
pub struct FuckSheep {
    parallel: usize,
    token: Option<String>,
    uid: String,
    times: usize,
}

impl Default for FuckSheep {
    fn default() -> Self {
        Self {
            parallel: num_cpus::get(),
            token: None,
            times: 1000,
            uid: "".to_string(),
        }
    }
}

impl FuckSheep {
    pub fn new() -> Self {
        FuckSheep::default()
    }

    pub fn parse_args(
        &mut self,
        args: ArgMatches,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let parallel: usize = args
            .value_of("parallel")
            .unwrap_or(&num_cpus::get().to_string())
            .parse()?;
        let times: usize = args.value_of("times").unwrap_or("1000").parse()?;
        let user_id = args.value_of("uid").expect("UID不得为空");

        self.parallel = parallel;
        self.times = times;
        self.token = None;
        self.uid = user_id.to_string();
        Ok(())
    }

    pub async fn get_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .build()?;

        let user_info = client
            .get("https://cat-match.easygame2021.com/sheep/v1/game/user_info")
            .query(&[("uid", &self.uid), ("t", &ENC_TOKEN.to_string())])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let uuid = user_info["data"]["wx_open_id"].as_str();

        let user_info = client
            .post("https://cat-match.easygame2021.com/sheep/v1/user/login_oppo")
            .json(&serde_json::json!({
                "uid":uuid.unwrap_or(""),
                "avatar":"1",
                "nick_name":"1",
                "sex":1
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        self.token = Some(
            user_info["data"]["token"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        );
        Ok(())
    }

    pub async fn start(&self) -> Result<usize, Box<dyn std::error::Error + 'static>> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()?;

        let total = Arc::new(AtomicUsize::new(0));
        let mut task_vec: Vec<JoinHandle<()>> = Vec::new();

        for _ in 0..self.parallel {
            let client = client.to_owned();
            let total = total.to_owned();
            let times = self.times.to_owned();
            let token = self.token.to_owned();
            let mut rng = rand::thread_rng();
            let rand_time = rng.gen_range(1..3600);
            let handle = tokio::spawn(async move {
                let user_token = token.to_owned().unwrap_or("".to_string());
                loop {
                    let result = client.post(format!("https://cat-match.easygame2021.com/sheep/v1/game/game_over_ex?rank_score=1&rank_state=1&rank_time={}&rank_role=1&skin=1",rand_time))
                    .header("t", &user_token)
                    .json(&serde_json::json!({
                        "rank_score":1,
                        "rank_state":1,
                        "rank_time":rand_time,
                        "rank_role":1,
                        "skin":1,
                        "MatchPlayInfo":"TpjYXRfbWF0Y2g6bHQxMjM0NTYiLCJvcGVuX2lkIjoiIiwidWlkIjo5MzIxNTgsImR"
                    }))
                    .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.28(0x18001c25) NetType/WIFI Language/zh_CN")
                    .header("Referer", "https://servicewechat.com/wx141bfb9b73c970a9/15/page-frame.html")
                    .send()
                .await;

                    match result {
                        Ok(r) => {
                            if r.status() != StatusCode::OK {
                                eprintln!(
                                    "请求时发生错误，状态码：{}，本次请求请忽略。",
                                    r.status()
                                );
                                continue;
                            }
                            let result = client.get(format!("https://cat-match.easygame2021.com/sheep/v1/game/topic_game_over?rank_score=1&rank_state=1&rank_time={}&rank_role=2&skin=1",rand_time))
                            .header("t", &user_token)
                            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.28(0x18001c25) NetType/WIFI Language/zh_CN")
                            .header("Referer", "https://servicewechat.com/wx141bfb9b73c970a9/15/page-frame.html")
                            .send()
                            .await;

                            match result {
                                Ok(r) => {
                                    if r.status() != StatusCode::OK {
                                        eprintln!(
                                            "请求时发生错误，状态码：{}，本次请求请忽略。",
                                            r.status()
                                        );
                                        continue;
                                    }
                                    total.fetch_add(1, Ordering::SeqCst);
                                    if total.load(Ordering::SeqCst) >= times {
                                        break;
                                    }
                                }
                                Err(e) => eprint!("{:#?}", e),
                            }
                        }
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }
            });

            task_vec.push(handle);
        }

        let total_print = total.to_owned();

        let print_task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
                println!(
                    "当前时间：{}，已刷次数：{}",
                    Local::now().format("%Y年%m月%d日 %H:%M:%S"),
                    total_print.load(Ordering::SeqCst)
                );
            }
        });

        for i in task_vec {
            i.await?
        }

        print_task.abort();

        Ok(total.load(Ordering::SeqCst))
    }

    pub async fn start_once(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()?;

        let mut rand_time = rand::thread_rng();
        let rand_time = rand_time.gen_range(1..3600);
        let result = client.post(format!("https://cat-match.easygame2021.com/sheep/v1/game/game_over_ex?rank_score=1&rank_state=1&rank_time={}&rank_role=1&skin=1",rand_time))
        .header("t", self.token.as_ref().unwrap_or(&"".to_string()))
        .json(&serde_json::json!({
            "rank_score":1,
            "rank_state":1,
            "rank_time":rand_time,
            "rank_role":1,
            "skin":1,
            "MatchPlayInfo":"TpjYXRfbWF0Y2g6bHQxMjM0NTYiLCJvcGVuX2lkIjoiIiwidWlkIjo5MzIxNTgsImR"
        }))
        .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.28(0x18001c25) NetType/WIFI Language/zh_CN")
        .header("Referer", "https://servicewechat.com/wx141bfb9b73c970a9/15/page-frame.html")
        .send()
    .await;

        match result {
            Ok(r) => {
                if r.status() != StatusCode::OK {
                    eprintln!("请求时发生错误，状态码：{}，本次请求请忽略。", r.status());
                    std::process::exit(1);
                }
                let result = client.get(format!("https://cat-match.easygame2021.com/sheep/v1/game/topic_game_over?rank_score=1&rank_state=1&rank_time={}&rank_role=2&skin=1",rand_time))
                            .header("t", self.token.as_ref().unwrap_or(&"".to_string()))
                            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.28(0x18001c25) NetType/WIFI Language/zh_CN")
                            .header("Referer", "https://servicewechat.com/wx141bfb9b73c970a9/15/page-frame.html")
                            .send()
                            .await;

                match result {
                    Ok(r) => {
                        if r.status() != StatusCode::OK {
                            eprintln!("请求时发生错误，状态码：{}，本次请求请忽略。", r.status());
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprint!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("{:#?}", e);
                std::process::exit(1)
            }
        }

        Ok(())
    }
}
