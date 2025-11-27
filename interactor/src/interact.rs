#![allow(non_snake_case)]

pub mod cli;
pub mod config;
pub mod state;
mod subscription_proxy;

use clap::Parser;
use config::Config;
use multiversx_sc_snippets::imports::*;

use crate::state::State;

const SUBSCRIPTION_CODE_PATH: MxscPath = MxscPath::new("../output/subscription.mxsc.json");

pub async fn subscription_cli() {
    env_logger::init();

    let config = Config::new();

    let mut interact = ContractInteract::new(config).await;

    let cli = cli::InteractCli::parse();
    match &cli.command {
        Some(cli::InteractCliCommand::Deploy) => interact.deploy().await,
        Some(cli::InteractCliCommand::Upgrade) => interact.upgrade().await,
        Some(cli::InteractCliCommand::AddSubscriptionPlan(args)) => {
            interact
                .add_subscription_plan(&args.title, args.duration_days, args.price)
                .await
        }
        Some(cli::InteractCliCommand::AddSubscription(args)) => {
            interact.add_new_subscription(args.amount, args.id).await
        }
        Some(cli::InteractCliCommand::GetPlan(args)) => interact.get_plan(args.id).await,
        Some(cli::InteractCliCommand::GetSubscription(args)) => {
            interact.get_subscription(&args.address).await
        }
        Some(cli::InteractCliCommand::GetAllPlanIDs) => interact.get_all_plan_ids().await,
        _ => (),
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("subscription-sc");

        // TODO: add your pem wallet
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;

        ContractInteract {
            interactor,
            wallet_address,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(subscription_proxy::SubscriptionContractProxy)
            .init()
            .code(SUBSCRIPTION_CODE_PATH)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {new_address_bech32}");
        self.state.set_address(new_address_bech32);
    }

    pub async fn upgrade(&mut self) {
        self.interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(subscription_proxy::SubscriptionContractProxy)
            .upgrade()
            .code(SUBSCRIPTION_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .run()
            .await;

        println!("Successfully upgraded contract");
    }

    pub async fn add_subscription_plan(&mut self, title: &String, duration_days: u64, price: u128) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(subscription_proxy::SubscriptionContractProxy)
            .add_subscription_plan(title, duration_days, price)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!(
            "Result [{response}] for title: {title} | duration days: {duration_days} | price: {price}"
        );
    }

    pub async fn add_new_subscription(&mut self, amount: u128, plan_id: u32) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(subscription_proxy::SubscriptionContractProxy)
            .add_new_subscription(plan_id)
            .egld(amount)
            .run()
            .await;

        println!("Plan ID {plan_id} | Amount {amount}: Subscription added");
    }

    pub async fn get_plan(&mut self, plan_id: u32) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(subscription_proxy::SubscriptionContractProxy)
            .get_plan(plan_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        match result_value {
            OptionalValue::Some(plan) => {
                println!(
                    "Plan ID [{plan_id}]: Title: {} | Duration (days): {} | Price: {}",
                    plan.title,
                    plan.duration_days,
                    plan.price.to_display()
                );
            }
            OptionalValue::None => {
                println!("Plan ID [{plan_id}] not found");
            }
        }
    }

    pub async fn get_subscription(&mut self, address: &String) {
        let user = Bech32Address::from_bech32_string(address.to_string());

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(subscription_proxy::SubscriptionContractProxy)
            .get_subscription(user)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        match result_value {
            OptionalValue::Some(subscription) => {
                println!(
                    "Subscription for [{address}]: Plan ID: {} | Started At: {} | Expires At: {} | Paid Amount: {}",
                    subscription.plan_id,
                    subscription.started_at,
                    subscription.expires_at,
                    subscription.paid_amount.to_display()
                );
            }
            OptionalValue::None => {
                println!("Subscription for [{address}] not found");
            }
        }
    }

    pub async fn get_all_plan_ids(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(subscription_proxy::SubscriptionContractProxy)
            .get_all_plan_ids()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        print!("Plan IDs: [");
        for id in result_value.into_vec().iter() {
            print!("{}, ", id);
        }
        println!("]");
    }
}
