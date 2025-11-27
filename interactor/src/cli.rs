use clap::{Args, Parser, Subcommand};

/// Subscription Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Subscription Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "upgrade", about = "Upgrade contract")]
    Upgrade,
    #[command(name = "add-plan", about = "Add a subscription plan")]
    AddSubscriptionPlan(SubscriptionPlanArgs),
    #[command(name = "add-sub", about = "Add a subscription")]
    AddSubscription(SubscriptionArgs),
    #[command(name = "plan", about = "Get a subscription plan")]
    GetPlan(GetPlanArgs),
    #[command(name = "sub", about = "Get a subscription")]
    GetSubscription(SubscriptionAddressArgs),
    #[command(name = "plans-ids", about = "Get all subscription plan IDs")]
    GetAllPlanIDs,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SubscriptionArgs {
    /// The egld amount for the subscription
    #[arg(short = 'a', long = "amount")]
    pub amount: u128,

    /// The id of the subscription plan
    #[arg(short = 'i', long = "id")]
    pub id: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GetPlanArgs {
    /// The id of the subscription plan
    #[arg(short = 'i', long = "id")]
    pub id: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SubscriptionPlanArgs {
    /// The title of the subscription plan
    #[arg(short = 't', long = "title")]
    pub title: String,

    /// The duration of the subscription plan in days
    #[arg(short = 'd', long = "duration")]
    pub duration_days: u64,

    /// The price of the subscription plan
    #[arg(short = 'p', long = "price")]
    pub price: u128,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SubscriptionAddressArgs {
    /// The number of contracts to deploy
    #[arg(short = 'a', long = "address")]
    pub address: String,
}
