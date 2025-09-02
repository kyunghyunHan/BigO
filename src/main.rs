use clap::{Parser, Subcommand};
use colored::*;
use log::{info, error, warn};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};
use chrono::{DateTime, Utc};
use std::io::Write;

#[derive(Parser)]
#[command(name = "kpm")]
#[command(about = "Kubernetes Process Manager - systemd based PM2 alternative")]
#[command(version = "0.1.0")]
#[command(author = "Your Name")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project configuration
    New {
        /// Project name
        name: String,
        /// Script command to run
        #[arg(short, long)]
        script: Option<String>,
        /// Working directory
        #[arg(short = 'd', long)]
        cwd: Option<PathBuf>,
        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,
    },
    /// Start application by name
    Start {
        /// Application name
        name: String,
    },
    /// Stop application(s)
    Stop {
        /// Application name or 'all'
        name: String,
    },
    /// Restart application(s)
    Restart {
        /// Application name or 'all'
        name: String,
    },
    /// Delete application from process list
    Delete {
        /// Application name
        name: String,
    },
    /// List all applications
    List,
    /// Show application logs
    Logs {
        /// Application name
        name: String,
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
        /// Number of lines to show
        #[arg(short, long, default_value = "15")]
        lines: u32,
    },
    /// Show detailed application info
    Info {
        /// Application name
        name: String,
    },
    /// Monitor applications
    Monitor,
    /// Reload application
    Reload {
        /// Application name
        name: String,
    },
    /// Show RPM status
    Status,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    name: String,
    script: String,
    cwd: Option<PathBuf>,
    env: HashMap<String, String>,
    created_at: DateTime<Utc>,
}

#[derive(Clone)]
enum AppStatus {
    Running,
    Stopped,
    Error,
}

impl std::fmt::Display for AppStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppStatus::Running => write!(f, "{}", "running".green()),
            AppStatus::Stopped => write!(f, "{}", "stopped".red()),
            AppStatus::Error => write!(f, "{}", "error".yellow()),
        }
    }
}

#[derive(Tabled)]
struct AppDisplay {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Script")]
    script: String,
    #[tabled(rename = "Created")]
    created: String,
}

struct RPM {
    config_dir: PathBuf,
    apps_file: PathBuf,
}

impl RPM {
    fn new() -> Result<Self> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not find home directory"))?
            .join(".kpm");
            
        fs::create_dir_all(&config_dir)?;
        
        let apps_file = config_dir.join("apps.json");
        
        Ok(RPM {
            config_dir,
            apps_file,
        })
    }
    
    fn load_apps(&self) -> Result<HashMap<String, AppConfig>> {
        if !self.apps_file.exists() {
            return Ok(HashMap::new());
        }
        
        let content = fs::read_to_string(&self.apps_file)?;
        let apps: HashMap<String, AppConfig> = serde_json::from_str(&content)
            .unwrap_or_else(|_| HashMap::new());
        
        Ok(apps)
    }
    
    fn save_apps(&self, apps: &HashMap<String, AppConfig>) -> Result<()> {
        let content = serde_json::to_string_pretty(apps)?;
        fs::write(&self.apps_file, content)?;
        Ok(())
    }
    
    fn generate_service_content(&self, app: &AppConfig) -> String {
        let mut env_vars = String::new();
        for (key, value) in &app.env {
            env_vars.push_str(&format!("Environment={}={}\n", key, value));
        }
        
        let working_dir = app.cwd
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());
            
        let username = get_current_user();
        
        format!(
            r#"[Unit]
            Description=KPM managed service - {}
After=network.target

[Service]
Type=simple
User={}
Group={}
WorkingDirectory={}
ExecStart={}
Restart=always
RestartSec=5
{}Environment=PATH=/usr/local/bin:/usr/bin:/bin:/root/.cargo/bin
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
"#,
            app.name,
            username,
            username,
            working_dir,
            app.script,
            env_vars
        )
    }
    
    fn get_service_name(&self, app_name: &str) -> String {
        format!("kpm-{}", app_name)
    }
    
    async fn create_service_file(&self, app: &AppConfig) -> Result<()> {
        let service_name = self.get_service_name(&app.name);
        let service_file = format!("/etc/systemd/system/{}.service", service_name);
        
        // Generate service file content
        let service_content = self.generate_service_content(app);
        
        // Write service file (requires sudo)
        let mut cmd = Command::new("sudo");
        cmd.args(&["tee", &service_file]);
        
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .spawn()?;
            
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(service_content.as_bytes())?;
        }
        
        let exit_status = child.wait()?;
        if !exit_status.success() {
            return Err(anyhow!("Failed to create service file"));
        }
        
        // Reload systemd daemon
        self.run_systemctl(&["daemon-reload"]).await?;
        
        println!("{} Service file created: {}", "✓".green(), service_file);
        Ok(())
    }
    
    async fn start_app(&self, app_name: &str) -> Result<()> {
        let service_name = self.get_service_name(app_name);
        
        // Enable and start service
        self.run_systemctl(&["enable", &service_name]).await?;
        self.run_systemctl(&["start", &service_name]).await?;
        
        println!("{} Started application: {}", "✓".green(), app_name.bold());
        Ok(())
    }
    
    async fn stop_app(&self, app_name: &str) -> Result<()> {
        let service_name = self.get_service_name(app_name);
        
        self.run_systemctl(&["stop", &service_name]).await?;
        println!("{} Stopped application: {}", "✓".green(), app_name.bold());
        Ok(())
    }
    
    async fn restart_app(&self, app_name: &str) -> Result<()> {
        let service_name = self.get_service_name(app_name);
        
        self.run_systemctl(&["restart", &service_name]).await?;
        println!("{} Restarted application: {}", "✓".green(), app_name.bold());
        Ok(())
    }
    
    async fn delete_app(&self, app_name: &str) -> Result<()> {
        let service_name = self.get_service_name(app_name);
        let service_file = format!("/etc/systemd/system/{}.service", service_name);
        
        // Stop and disable service
        let _ = self.run_systemctl(&["stop", &service_name]).await;
        let _ = self.run_systemctl(&["disable", &service_name]).await;
        
        // Remove service file
        let _ = Command::new("sudo")
            .args(&["rm", "-f", &service_file])
            .output();
            
        self.run_systemctl(&["daemon-reload"]).await?;
        
        // Remove from apps config
        let mut apps = self.load_apps()?;
        apps.remove(app_name);
        self.save_apps(&apps)?;
        
        println!("{} Deleted application: {}", "✓".green(), app_name.bold());
        Ok(())
    }
    
    async fn run_systemctl(&self, args: &[&str]) -> Result<()> {
        let output = Command::new("sudo")
            .arg("systemctl")
            .args(args)
            .output()?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("systemctl failed: {}", stderr));
        }
        
        Ok(())
    }
    
    async fn get_app_status(&self, app_name: &str) -> AppStatus {
        let service_name = self.get_service_name(app_name);
        
        let output = Command::new("systemctl")
            .args(&["is-active", &service_name])
            .output();
            
        match output {
            Ok(out) => {
                let status_string = String::from_utf8_lossy(&out.stdout);
                let status = status_string.trim();
                match status {
                    "active" => AppStatus::Running,
                    "inactive" | "failed" => AppStatus::Stopped,
                    _ => AppStatus::Error,
                }
            }
            Err(_) => AppStatus::Error,
        }
    }
    
    async fn show_logs(&self, app_name: &str, follow: bool, lines: u32) -> Result<()> {
        let service_name = self.get_service_name(app_name);
        
        let mut args = vec!["journalctl", "-u", &service_name, "--no-pager"];
        
        if follow {
            args.push("-f");
        }
        
        let lines_str = lines.to_string();
        args.push("-n");
        args.push(&lines_str);
        
        let mut cmd = Command::new("sudo");
        cmd.args(&args);
        
        let status = cmd.status()?;
        
        if !status.success() {
            return Err(anyhow!("Failed to show logs for {}", app_name));
        }
        
        Ok(())
    }
}

fn get_current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "root".to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    let pm = RPM::new()?;
    
    match cli.command {
        Commands::New { name, script, cwd, env } => {
            // Check if app already exists
            let apps = pm.load_apps()?;
            if apps.contains_key(&name) {
                println!("{} Project '{}' already exists!", "✗".red(), name.red());
                return Ok(());
            }
            
            let mut env_map = HashMap::new();
            for env_var in env {
                if let Some((key, value)) = env_var.split_once('=') {
                    env_map.insert(key.to_string(), value.to_string());
                }
            }
            
            let default_script = format!("echo 'Please configure script for {}'", name);
            let app_config = AppConfig {
                name: name.clone(),
                script: script.unwrap_or(default_script),
                cwd,
                env: env_map,
                created_at: Utc::now(),
            };
            
            // Create systemd service file
            pm.create_service_file(&app_config).await?;
            
            // Save to apps config
            let mut apps = pm.load_apps()?;
            apps.insert(name.clone(), app_config);
            pm.save_apps(&apps)?;
            
            println!("{} Created project: {}", "✓".green(), name.bold());
            println!("Next steps:");
            println!("  kpm start {}  # Start the service", name);
            println!("  kpm logs {}   # View logs", name);
        }
        
        Commands::Start { name } => {
            let apps = pm.load_apps()?;
            if let Some(_app) = apps.get(&name) {
                pm.start_app(&name).await?;
            } else {
                println!("{} Project '{}' not found. Create it first with: kpm new {}", 
                    "✗".red(), name.red(), name);
                return Ok(());
            }
        }
        
        Commands::Stop { name } => {
            if name == "all" {
                let apps = pm.load_apps()?;
                for app_name in apps.keys() {
                    let _ = pm.stop_app(app_name).await;
                }
            } else {
                pm.stop_app(&name).await?;
            }
        }
        
        Commands::Restart { name } => {
            if name == "all" {
                let apps = pm.load_apps()?;
                for app_name in apps.keys() {
                    let _ = pm.restart_app(app_name).await;
                }
            } else {
                pm.restart_app(&name).await?;
            }
        }
        
        Commands::Delete { name } => {
            pm.delete_app(&name).await?;
        }
        
        Commands::List => {
            let apps = pm.load_apps()?;
            
            if apps.is_empty() {
                println!("No applications found");
                println!("Create one with: rpm new <name> --script '<command>'");
                return Ok(());
            }
            
            let mut app_displays = Vec::new();
            
            for (name, app) in &apps {
                let status = pm.get_app_status(name).await;
                
                app_displays.push(AppDisplay {
                    name: name.clone(),
                    status: status.to_string(),
                    script: app.script.clone(),
                    created: app.created_at.format("%Y-%m-%d %H:%M").to_string(),
                });
            }
            
            let table = Table::new(app_displays);
            println!("{}", table);
        }
        
        Commands::Logs { name, follow, lines } => {
            let apps = pm.load_apps()?;
            if apps.contains_key(&name) {
                pm.show_logs(&name, follow, lines).await?;
            } else {
                println!("{} Project '{}' not found", "✗".red(), name.red());
            }
        }
        
        Commands::Info { name } => {
            let apps = pm.load_apps()?;
            if let Some(app) = apps.get(&name) {
                let status = pm.get_app_status(&name).await;
                
                println!("Application: {}", name.bold());
                println!("Script: {}", app.script);
                println!("Status: {}", status);
                if let Some(cwd) = &app.cwd {
                    println!("Working Directory: {}", cwd.display());
                }
                println!("Created: {}", app.created_at.format("%Y-%m-%d %H:%M:%S"));
                
                if !app.env.is_empty() {
                    println!("Environment Variables:");
                    for (key, value) in &app.env {
                        println!("  {}={}", key, value);
                    }
                }
            } else {
                println!("Application '{}' not found", name.red());
            }
        }
        
        Commands::Monitor => {
            println!("KPM Monitor - Press Ctrl+C to exit");
            let apps = pm.load_apps()?;
            
            if apps.is_empty() {
                println!("No applications to monitor");
                return Ok(());
            }
            
            for (name, _app) in &apps {
                let status = pm.get_app_status(name).await;
                println!("{}: {}", name.bold(), status);
            }
        }
        
        Commands::Reload { name } => {
            pm.restart_app(&name).await?;
        }
        
        Commands::Status => {
            println!("KPM v{}", env!("CARGO_PKG_VERSION"));
            let apps = pm.load_apps()?;
            println!("Managed applications: {}", apps.len());
            
            if !apps.is_empty() {
                println!("\nApplications:");
                for (name, _app) in &apps {
                    let status = pm.get_app_status(name).await;
                    println!("  {}: {}", name, status);
                }
            }
        }
    }
    
    Ok(())
}