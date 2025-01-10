use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Project(ProjectCommands),
    #[command(subcommand)]
    User(UserCommands),
    #[command(subcommand)]
    Expense(ExpenseCommands),
    Balance {
        project_id: String
    }
}

#[derive(Subcommand)]
enum ProjectCommands {
    Add {
        name: String,
    },
    List,
}

#[derive(Subcommand)]
enum UserCommands {
    Add {
        project_id: String,
        name: String
    },
    List {
        project_id: String,
    },
}

#[derive(Subcommand)]
enum ExpenseCommands {
    Add {
        project_id: String,
        name: String,
        date: String,
        made_by: String,
        amount: u32,
    },
    List {
        project_id: String,
    }
}
