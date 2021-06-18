/// данные для определения ключа с полным доступом
#[derive(Debug, Default, clap::Clap)]
pub struct CliFullAccessType {
    #[clap(subcommand)]
    sign_option: Option<
        crate::commands::construct_transaction_command::sign_transaction::CliSignTransaction,
    >,
}

#[derive(Debug)]
pub struct FullAccessType {
    pub sign_option:
        crate::commands::construct_transaction_command::sign_transaction::SignTransaction,
}

impl From<CliFullAccessType> for FullAccessType {
    fn from(item: CliFullAccessType) -> Self {
        let sign_option = match item.sign_option {
            Some(cli_sign_transaction) => cli_sign_transaction.into(),
            None => crate::commands::construct_transaction_command::sign_transaction::SignTransaction::choose_sign_option(),
        };
        Self { sign_option }
    }
}

impl FullAccessType {
    pub async fn process(
        self,
        nonce: near_primitives::types::Nonce,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
        public_key: near_crypto::PublicKey,
    ) -> crate::CliResult {
        let access_key: near_primitives::account::AccessKey = near_primitives::account::AccessKey {
            nonce,
            permission: near_primitives::account::AccessKeyPermission::FullAccess,
        };
        let action = near_primitives::transaction::Action::AddKey(
            near_primitives::transaction::AddKeyAction {
                public_key: public_key.clone(),
                access_key,
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        println!("\nunsigned transaction: {:?}", unsigned_transaction);
        println!(
            "\nAdding full access key = {:?} to {:?}.",
            public_key, unsigned_transaction.signer_id
        );
        match self
            .sign_option
            .process(unsigned_transaction.clone(), network_connection_config)
            .await?
        {
            Some(transaction_info) => {
                match transaction_info.status {
                    near_primitives::views::FinalExecutionStatus::NotStarted => {
                        println!("NotStarted")
                    }
                    near_primitives::views::FinalExecutionStatus::Started => println!("Started"),
                    near_primitives::views::FinalExecutionStatus::Failure(tx_execution_error) => {
                        crate::common::print_transaction_error(tx_execution_error).await
                    }
                    near_primitives::views::FinalExecutionStatus::SuccessValue(_) => {
                        match transaction_info.transaction.actions[0].clone() {
                            near_primitives::views::ActionView::AddKey {
                                public_key,
                                access_key: _,
                            } => {
                                println!(
                                    "Added full access key = {:?} to {}.",
                                    public_key, transaction_info.transaction.signer_id,
                                );
                            }
                            _ => unreachable!("Error")
                        }
                    }
                }
                println!("\nTransaction Id {id} .\n\nTo see the transaction in the transaction explorer, please open this url in your browser:
                        \nhttps://explorer.testnet.near.org/transactions/{id}\n", id=transaction_info.transaction_outcome.id);
            }
            None => {}
        };
        Ok(())
    }
}
