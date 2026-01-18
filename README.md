# NVIDIA Overclock Monitor

🛡️ **Automatic protection against overclock reset to prevent 12VHPWR connector melting**

> 🇧🇷 [Versão em Português](README.pt-BR.md)

## 🔥 Why does this project exist?

The **12VHPWR** connector on NVIDIA RTX 4000/5000 cards is known for having melting issues when subjected to power spikes. The best way to prevent this is through:

1. **Undervolt with custom curve** in MSI Afterburner
2. **Memory overclock** to compensate performance
3. **Constant monitoring** to ensure settings don't reset

### The Problem

MSI Afterburner can reset overclock settings in various situations:
- ✗ Windows reboot
- ✗ NVIDIA driver updates
- ✗ Afterburner crashes
- ✗ Accidental manual changes
- ✗ Conflicts with other software

When this happens, the GPU returns to stock settings, allowing **dangerous voltage and power spikes** that can melt the 12VHPWR connector.

## ✅ The Solution

This program continuously monitors the GPU's memory clock to detect if the overclock profile has been reset. If it detects a return to stock:

1. 🔔 **Sends alert via Telegram**
2. 🔄 **Automatically reapplies the Afterburner profile**
3. ⏱️ **Waits for stabilization and continues monitoring**

### 🔍 Why monitor memory clock?

**The problem:** `nvidia-smi` doesn't expose information about custom voltage/frequency curves. There's no way to directly query if the undervolt is applied.

**The solution:** Use **memory overclock as a detection proxy**:

- When you apply a profile in Afterburner with memory OC, the NVIDIA driver allows higher clocks
- `nvidia-smi` **can** query current memory clock via `--query-gpu=clocks.current.memory`
- If the profile resets, memory clock returns to stock values (lower)
- **Indirect detection:** If memory clock dropped = entire profile (including undervolt) was reset

**Practical example:**
```
With profile applied:    17001 MHz (overclocked memory)
After profile reset:     10501 MHz (stock memory)
```

When we detect that memory returned to stock, we know that the **voltage curve also reset**, and we can automatically reapply everything.

**That's why it's important to have memory overclock in your profile**, even if it's just +100 MHz - it serves as a "canary" to detect resets.

### 🔋 Alternative: Use Power Limit instead of Memory OC

**If you don't want to overclock memory**, you can use Power Limit as the detection method:

- Set Power Limit to **99%** or **101%** in MSI Afterburner
- Use `nvidia-smi --query-gpu=power.limit --format=csv,noheader,nounits` to query
- When the profile resets, Power Limit returns to 100% (stock)

**Advantages:**
- ✅ Doesn't touch memory clocks
- ✅ Works equally well as a "canary"
- ✅ 99% can even slightly improve temperatures

**To implement:** Replace the `get_max_mem_clock()` function with one that queries `power.limit` and adjust `MEM_CLOCK_TARGET` to 99.0 or 101.0.

## 📊 Features

- ⚡ **Extremely lightweight**: Only ~2.1 MB of RAM
- 🔇 **Runs silently in background** (no window/console)
- 📱 **Telegram alerts** when problems detected
- 🔄 **Automatic profile reapplication**
- ⏰ **Checks every 1 hour**
- 🚀 **Zero impact on games/applications**

## 🛠️ Installation

### Prerequisites

1. **MSI Afterburner** installed
2. **NVIDIA GPU** with drivers installed
3. **Telegram Bot** (for alerts)

### Configuration

#### 1. Clone the repository
```bash
git clone https://github.com/your-username/check_nvidia.git
cd check_nvidia
```

#### 2. Configure MSI Afterburner

- Create your undervolt/overclock profile in **Profile 1**
- Enable "Apply overclocking at system startup"
- Enable "Start with Windows"

#### 3. Find your target clock

Run in PowerShell **with overclock applied**:
```powershell
nvidia-smi --query-gpu=clocks.current.memory --format=csv,noheader,nounits
```

Note the value (e.g., 11501 MHz) and subtract a small margin (~100 MHz).

#### 4. Edit the code

In `src/main.rs`, adjust the target value:
```rust
const MEM_CLOCK_TARGET: f64 = 11400.0; // Your value here
```

#### 5. Configure environment variables

**Create a Telegram bot:**
1. Chat with [@BotFather](https://t.me/botfather)
2. Use `/newbot` and follow instructions
3. Copy the **token** provided

**Get your Chat ID:**
1. Chat with [@userinfobot](https://t.me/userinfobot)
2. Copy your **ID**

**Configure on Windows (PowerShell as Administrator):**
```powershell
[System.Environment]::SetEnvironmentVariable('TELEGRAM_BOT_TOKEN', 'your_token_here', 'User')
[System.Environment]::SetEnvironmentVariable('TELEGRAM_CHAT_ID', 'your_chat_id_here', 'User')
```

**Restart the terminal** for variables to take effect.

#### 6. Build the project

```bash
cargo build --release
```

The executable will be at: `target/release/check_nvidia.exe`

#### 7. Configure automatic startup

1. Open **Task Scheduler**
2. Click "Create Task"
3. **General**: Name "NVIDIA Overclock Monitor"
4. **Triggers**: "At log on" + **Delay: 30 minutes**
5. **Actions**: Path to `check_nvidia.exe`
6. **Conditions**: Uncheck "Start only if on AC power"
7. **Settings**: Check "Run task as soon as possible after a scheduled start is missed"

## 📱 Alert Example

When detected, you'll receive on Telegram:

```
⚠️ NVIDIA OVERCLOCK ALERT

Detected clock: 10501 MHz
Expected target: 11400 MHz

Profile automatically reapplied.
```

## 🔧 Advanced Settings

### Change check interval

In `src/main.rs`, final line:
```rust
tokio::time::sleep(Duration::from_secs(3600)).await; // 3600 = 1 hour
```

Recommended values:
- `3600` - 1 hour (default, ideal for daily monitoring)
- `1800` - 30 minutes
- `600` - 10 minutes

### Change Afterburner profile

In `src/main.rs`:
```rust
const AB_PROFILE_ARG: &str = "-profile1"; // -profile2, -profile3, etc.
```

### Custom Afterburner path

```rust
const AB_PATH: &str = r"C:\Custom\Path\MSIAfterburner.exe";
```

## 🚨 Security

- ✅ Telegram credentials in environment variables (not in code)
- ✅ Doesn't expose sensitive information
- ✅ Runs with user permissions (doesn't need admin)

## 🤝 Contributing

Contributions are welcome! Feel free to:
- 🐛 Report bugs
- 💡 Suggest features
- 🔧 Submit pull requests

## 📄 License

MIT License - use freely!

## ⚠️ Legal Disclaimer

This software is provided "as is". Use of overclock/undervolt is at your own risk. Always monitor temperatures and system stability.

---

**Developed to protect NVIDIA RTX 4000/5000 GPUs against 12VHPWR connector issues** 🛡️🔥
