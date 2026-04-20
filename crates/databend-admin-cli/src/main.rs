use clap::{Parser, Subcommand, ValueEnum};
use databend_admin_core::{
    AdminError, RoleGrant, SecurityFinding, UserAccount, WarehouseHealth, load_grants, load_users,
    load_warehouses, run_security_audit,
};

#[derive(Parser)]
#[command(name = "databend-admin")]
#[command(about = "Databend admin and governance CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Rbac {
        #[command(subcommand)]
        command: RbacCommand,
    },
    Security {
        #[command(subcommand)]
        command: SecurityCommand,
    },
    Warehouse {
        #[command(subcommand)]
        command: WarehouseCommand,
    },
}

#[derive(Subcommand)]
enum RbacCommand {
    Snapshot {
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum SecurityCommand {
    Audit {
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum WarehouseCommand {
    Health {
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Markdown,
}

#[tokio::main]
async fn main() -> Result<(), AdminError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Rbac { command } => match command {
            RbacCommand::Snapshot { format } => render_rbac(format).await?,
        },
        Commands::Security { command } => match command {
            SecurityCommand::Audit { format } => render_security(format).await?,
        },
        Commands::Warehouse { command } => match command {
            WarehouseCommand::Health { format } => render_warehouse(format).await?,
        },
    }
    Ok(())
}

async fn render_rbac(format: OutputFormat) -> Result<(), AdminError> {
    let users = load_users().await?;
    let grants = load_grants().await?;
    match format {
        OutputFormat::Text => {
            println!("users: {}", users.len());
            println!("grants: {}", grants.len());
            for user in users {
                println!(
                    "user={} default_role={:?} disabled={}",
                    user.name, user.default_role, user.disabled
                );
            }
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&(users, grants)).expect("json output")
            );
        }
        OutputFormat::Markdown => render_rbac_markdown(&users, &grants),
    }
    Ok(())
}

fn render_rbac_markdown(users: &[UserAccount], grants: &[RoleGrant]) {
    println!("# RBAC Snapshot");
    println!();
    println!("## Users");
    println!("| name | default role | disabled |");
    println!("| --- | --- | --- |");
    for user in users {
        println!(
            "| {} | {} | {} |",
            user.name,
            user.default_role.as_deref().unwrap_or("none"),
            user.disabled
        );
    }
    println!();
    println!("## Grants");
    println!("| role | object | privilege |");
    println!("| --- | --- | --- |");
    for grant in grants {
        println!(
            "| {} | {} | {} |",
            grant.role, grant.object, grant.privilege
        );
    }
}

async fn render_security(format: OutputFormat) -> Result<(), AdminError> {
    let users = load_users().await?;
    let grants = load_grants().await?;
    let findings = run_security_audit(&users, &grants);
    match format {
        OutputFormat::Text => {
            for finding in findings {
                println!(
                    "{:?}: {} - {}",
                    finding.severity, finding.title, finding.detail
                );
            }
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&findings).expect("json output")
            );
        }
        OutputFormat::Markdown => render_security_markdown(&findings),
    }
    Ok(())
}

fn render_security_markdown(findings: &[SecurityFinding]) {
    println!("# Security Audit");
    println!();
    println!("| severity | title | detail |");
    println!("| --- | --- | --- |");
    for finding in findings {
        println!(
            "| {:?} | {} | {} |",
            finding.severity, finding.title, finding.detail
        );
    }
}

async fn render_warehouse(format: OutputFormat) -> Result<(), AdminError> {
    let warehouses = load_warehouses().await?;
    match format {
        OutputFormat::Text => {
            for warehouse in warehouses {
                println!(
                    "warehouse={} size={} running={} auto_suspend_secs={:?} auto_resume={}",
                    warehouse.warehouse,
                    warehouse.size,
                    warehouse.running,
                    warehouse.auto_suspend_secs,
                    warehouse.auto_resume
                );
            }
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&warehouses).expect("json output")
            );
        }
        OutputFormat::Markdown => render_warehouse_markdown(&warehouses),
    }
    Ok(())
}

fn render_warehouse_markdown(warehouses: &[WarehouseHealth]) {
    println!("# Warehouse Health");
    println!();
    println!("| warehouse | size | running | auto suspend secs | auto resume |");
    println!("| --- | --- | --- | ---: | --- |");
    for warehouse in warehouses {
        println!(
            "| {} | {} | {} | {} | {} |",
            warehouse.warehouse,
            warehouse.size,
            warehouse.running,
            warehouse
                .auto_suspend_secs
                .map(|v| v.to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            warehouse.auto_resume
        );
    }
}
