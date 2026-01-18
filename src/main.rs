#![windows_subsystem = "windows"]

use std::process::Command;
use std::time::Duration;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const MEM_CLOCK_TARGET: f64 = 16901.0;
const AB_PATH: &str = r"C:\Program Files (x86)\MSI Afterburner\MSIAfterburner.exe";
const AB_PROFILE_ARG: &str = "-profile1";
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn get_max_mem_clock() -> Option<f64> {
    let mut command = Command::new("nvidia-smi");
    command.args(&["--query-gpu=clocks.current.memory", "--format=csv,noheader,nounits"]);

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command.output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let stdout = String::from_utf8_lossy(&o.stdout);
                if let Ok(val) = stdout.trim().parse::<f64>() {
                    return Some(val);
                }
            }
        }
        Err(_) => {}
    }
    None
}

fn force_apply_profile() {
    let mut command = Command::new(AB_PATH);
    command.arg(AB_PROFILE_ARG);

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    let _ = command.spawn();
}

async fn send_telegram_alert(current_clock: f64) {
    let bot_token = match std::env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(_) => return,
    };

    let chat_id = match std::env::var("TELEGRAM_CHAT_ID") {
        Ok(id) => id,
        Err(_) => return,
    };

    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        bot_token
    );

    let message = format!(
        "⚠️ ALERTA NVIDIA OVERCLOCK\n\n\
        Clock detectado: {} MHz\n\
        Alvo esperado: {} MHz\n\n\
        Perfil reaplicado automaticamente.",
        current_clock, MEM_CLOCK_TARGET
    );

    let client = reqwest::Client::new();
    let _ = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": message,
            "parse_mode": "HTML"
        }))
        .send()
        .await;
}

#[tokio::main]
async fn main() {
    loop {
        if let Some(current_max) = get_max_mem_clock() {
            if current_max < MEM_CLOCK_TARGET {
                send_telegram_alert(current_max).await;
                force_apply_profile();
                tokio::time::sleep(Duration::from_secs(15)).await;
            }
        }

        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
