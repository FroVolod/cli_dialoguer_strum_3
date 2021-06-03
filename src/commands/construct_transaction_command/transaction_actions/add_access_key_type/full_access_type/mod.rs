use async_recursion::async_recursion;

/// данные для определения ключа с полным доступом
#[derive(Debug, Default, clap::Clap)]
pub struct CliFullAccessType {
    #[clap(subcommand)]
    next_action: Option<super::super::CliSkipNextAction>,
}

#[derive(Debug)]
pub struct FullAccessType {
    pub next_action: Box<super::super::NextAction>,
}

impl From<CliFullAccessType> for FullAccessType {
    fn from(item: CliFullAccessType) -> Self {
        let skip_next_action: super::super::NextAction = match item.next_action {
            Some(cli_skip_action) => super::super::NextAction::from(cli_skip_action),
            None => super::super::NextAction::input_next_action(),
        };
        Self {
            next_action: Box::new(skip_next_action),
        }
    }
}

impl FullAccessType {
    #[async_recursion(?Send)]
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
                public_key,
                access_key,
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        match *self.next_action {
            super::super::NextAction::AddAction(select_action) => {
                select_action
                    .process(unsigned_transaction, network_connection_config)
                    .await
            }
            super::super::NextAction::Skip(skip_action) => {
                skip_action
                    .process(unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}
