use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Args, Parser, Subcommand, ValueEnum};
use firetv_core::{
    AdbTransport, BackupOptions, ConnectionState, LauncherCatalog, ProfileStore, RsAdbTransport,
    TargetAddress, capture_backup, inspect_device, restore_backup, verify_backup, verify_device,
};
use sha2::{Digest, Sha256};

#[derive(Parser)]
#[command(
    name = "firetvctl",
    version,
    about = "Safe Fire TV backup, launcher, recovery, and restore tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Connect(Target),
    Inspect(OutputTarget),
    Profiles(OutputTarget),
    Launchers {
        #[arg(long)]
        json: bool,
    },
    Backup(BackupCommand),
    Plan(PlanCommand),
    Install(InstallCommand),
    Repair(MutationCommand),
    Health(OutputTarget),
    Uninstall(MutationCommand),
    Restore(RestoreCommand),
}

#[derive(Args)]
struct Target {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
}

#[derive(Args)]
struct OutputTarget {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct BackupCommand {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
    #[arg(short, long)]
    output: PathBuf,
    #[arg(long)]
    full: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum PlannedAction {
    Install,
    Repair,
    Uninstall,
    Restore,
}

#[derive(Args)]
struct PlanCommand {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
    #[arg(value_enum)]
    action: PlannedAction,
    #[arg(long)]
    backup: PathBuf,
    #[arg(long)]
    payload: Option<PathBuf>,
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct Confirmation {
    #[arg(long)]
    backup: Option<PathBuf>,
    #[arg(long, requires = "plan_hash")]
    yes: bool,
    #[arg(long, requires = "yes")]
    plan_hash: Option<String>,
}

#[derive(Args)]
struct InstallCommand {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
    #[arg(long)]
    apk: PathBuf,
    #[command(flatten)]
    confirmation: Confirmation,
}

#[derive(Args)]
struct MutationCommand {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
    #[command(flatten)]
    confirmation: Confirmation,
}

#[derive(Args)]
struct RestoreCommand {
    #[arg(value_parser = parse_target)]
    target: TargetAddress,
    #[arg(long)]
    backup: Option<PathBuf>,
    #[arg(long, requires = "plan_hash")]
    yes: bool,
    #[arg(long, requires = "yes")]
    plan_hash: Option<String>,
}

fn parse_target(value: &str) -> Result<TargetAddress, String> {
    TargetAddress::parse(value)
        .map_err(|_| "target must be a numeric IPv4 or IPv6 address with an optional port".into())
}

fn plan_hash(
    action: PlannedAction,
    target: TargetAddress,
    backup: &Path,
    payload: Option<&Path>,
) -> String {
    let action = match action {
        PlannedAction::Install => "install",
        PlannedAction::Repair => "repair",
        PlannedAction::Uninstall => "uninstall",
        PlannedAction::Restore => "restore",
    };
    let text = format!(
        "{action}|{target}|{}|{}",
        backup.display(),
        payload.map_or_else(String::new, |path| path.display().to_string())
    );
    hex_sha256(text.as_bytes())
}

fn hex_sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

async fn connected(target: TargetAddress) -> Result<RsAdbTransport, String> {
    let transport = RsAdbTransport::default();
    match transport
        .connect(target)
        .await
        .map_err(|error| error.to_string())?
    {
        ConnectionState::Connected => Ok(transport),
        ConnectionState::Unauthorized => Err(
            "ADB authorization required: approve this computer on the Fire TV, then retry".into(),
        ),
        ConnectionState::Offline => {
            Err("Fire TV is offline or unreachable; verify its IP and ADB debugging setting".into())
        }
    }
}

fn confirmation_backup(confirmation: &Confirmation) -> Result<&PathBuf, String> {
    let backup = confirmation
        .backup
        .as_ref()
        .ok_or("--backup is required before every mutation")?;
    if !confirmation.yes || confirmation.plan_hash.is_none() {
        return Err("mutation requires --yes and --plan-hash from the plan command".into());
    }
    Ok(backup)
}

fn validate_hash(expected: &str, actual: Option<&String>) -> Result<(), String> {
    if actual.is_some_and(|value| value == expected) {
        Ok(())
    } else {
        Err("plan hash does not match; run firetvctl plan again".into())
    }
}

#[allow(clippy::too_many_lines)]
async fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Launchers { json } => {
            let catalog = LauncherCatalog::bundled().map_err(|error| error.to_string())?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&catalog).map_err(|error| error.to_string())?
                );
            } else {
                for launcher in catalog.launchers {
                    println!(
                        "{}\t{}\t{}",
                        launcher.id, launcher.name, launcher.source_url
                    );
                }
            }
        }
        Command::Connect(command) => {
            connected(command.target).await?;
            println!("connected to {}", command.target);
        }
        Command::Inspect(command) => {
            let transport = connected(command.target).await?;
            let facts = inspect_device(&transport)
                .await
                .map_err(|error| error.to_string())?;
            if command.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&facts).map_err(|error| error.to_string())?
                );
            } else {
                println!(
                    "{} ({}) SDK {}",
                    facts.identity.model, facts.identity.codename, facts.identity.android_sdk
                );
                println!(
                    "Home: {}",
                    facts.home_component.as_deref().unwrap_or("not resolved")
                );
            }
        }
        Command::Profiles(command) => {
            let transport = connected(command.target).await?;
            let facts = inspect_device(&transport)
                .await
                .map_err(|error| error.to_string())?;
            let selection = ProfileStore::bundled()
                .map_err(|error| error.to_string())?
                .select(&facts);
            if command.json {
                println!(
                    "{{\"support\":\"{}\",\"base\":\"{}\",\"reason\":{}}}",
                    selection.level,
                    selection.base.id,
                    serde_json::to_string(&selection.reason).unwrap_or_default()
                );
            } else {
                println!("{}: {}", selection.level, selection.reason);
            }
        }
        Command::Backup(command) => {
            let transport = connected(command.target).await?;
            let facts = inspect_device(&transport)
                .await
                .map_err(|error| error.to_string())?;
            let manifest = capture_backup(
                &transport,
                &facts,
                &command.output,
                &BackupOptions {
                    full: command.full,
                    settings: Vec::new(),
                },
            )
            .await
            .map_err(|error| error.to_string())?;
            println!(
                "backup captured at {} ({} files)",
                command.output.display(),
                manifest.files.len()
            );
        }
        Command::Health(command) => {
            let transport = connected(command.target).await?;
            let facts = inspect_device(&transport)
                .await
                .map_err(|error| error.to_string())?;
            let report = verify_device(&transport, &facts).await;
            if command.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
                );
            } else {
                for (name, result) in report.checks {
                    println!("{name}: {:?} - {}", result.status, result.detail);
                }
            }
        }
        Command::Plan(command) => {
            let hash = plan_hash(
                command.action,
                command.target,
                &command.backup,
                command.payload.as_deref(),
            );
            if command.json {
                println!("{{\"plan_hash\":\"{hash}\",\"backup_required\":true,\"dry_run\":true}}");
            } else {
                println!("Dry run only; no device changes made.\nPlan hash: {hash}");
            }
        }
        Command::Install(command) => {
            let backup = confirmation_backup(&command.confirmation)?;
            let hash = plan_hash(
                PlannedAction::Install,
                command.target,
                backup,
                Some(command.apk.as_path()),
            );
            validate_hash(&hash, command.confirmation.plan_hash.as_ref())?;
            let transport = connected(command.target).await?;
            let facts = inspect_device(&transport)
                .await
                .map_err(|error| error.to_string())?;
            verify_backup(backup, &facts.identity).map_err(|error| error.to_string())?;
            transport
                .install(&command.apk)
                .await
                .map_err(|error| error.to_string())?;
            println!("APK installed; run firetvctl health {}", command.target);
        }
        Command::Restore(command) => {
            let backup = command
                .backup
                .as_ref()
                .ok_or("--backup is required before restore")?;
            if !command.yes || command.plan_hash.is_none() {
                return Err("restore requires --yes and --plan-hash from the plan command".into());
            }
            let hash = plan_hash(PlannedAction::Restore, command.target, backup, None);
            validate_hash(&hash, command.plan_hash.as_ref())?;
            let transport = connected(command.target).await?;
            let facts = inspect_device(&transport)
                .await
                .map_err(|error| error.to_string())?;
            let verified =
                verify_backup(backup, &facts.identity).map_err(|error| error.to_string())?;
            restore_backup(&transport, &verified, &facts)
                .await
                .map_err(|error| error.to_string())?;
            println!("backup settings restored");
        }
        Command::Repair(command) | Command::Uninstall(command) => {
            let _ = confirmation_backup(&command.confirmation)?;
            return Err("this mutation needs a package-specific profile and is unavailable in base-safe mode".into());
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run(Cli::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
