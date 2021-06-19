use dialoguer::Input;

/// аргументы, необходимые для создания транзакции в offline mode
#[derive(Debug, Default, clap::Clap)]
pub struct CliOfflineArgs {
    #[clap(long)]
    signer_public_key: Option<near_crypto::PublicKey>,
    #[clap(long)]
    nonce: Option<u64>,
    #[clap(long)]
    block_hash: Option<crate::common::BlockHashAsBase58>,
    #[clap(subcommand)]
    pub send_from: Option<super::online_mode::select_server::server::CliSendFrom>,
}

#[derive(Debug)]
pub struct OfflineArgs {
    signer_public_key: near_crypto::PublicKey,
    nonce: u64,
    block_hash: near_primitives::hash::CryptoHash,
    send_from: super::online_mode::select_server::server::SendFrom,
}

impl From<CliOfflineArgs> for OfflineArgs {
    fn from(item: CliOfflineArgs) -> Self {
        let signer_public_key = match item.signer_public_key {
            Some(cli_public_key) => cli_public_key,
            None => OfflineArgs::input_signer_public_key(),
        };
        let nonce: u64 = match item.nonce {
            Some(cli_nonce) => cli_nonce,
            None => OfflineArgs::input_nonce(),
        };
        let block_hash = match item.block_hash {
            Some(cli_block_hash) => cli_block_hash.inner,
            None => OfflineArgs::input_block_hash(),
        };
        let send_from = match item.send_from {
            Some(cli_send_from) => {
                super::online_mode::select_server::server::SendFrom::from(cli_send_from)
            }
            None => super::online_mode::select_server::server::SendFrom::choose_send_from(),
        };
        Self {
            signer_public_key,
            nonce,
            block_hash,
            send_from,
        }
    }
}

impl OfflineArgs {
    fn input_signer_public_key() -> near_crypto::PublicKey {
        Input::new()
            .with_prompt("Enter signer public key")
            .interact_text()
            .unwrap()
    }
    fn input_nonce() -> u64 {
        Input::new()
            .with_prompt(
                "Enter transaction nonce (query the access key information with \
                `near-cli utils view-access-key frol4.testnet ed25519:...` incremented by 1)",
            )
            .interact_text()
            .unwrap()
    }

    fn input_block_hash() -> near_primitives::hash::CryptoHash {
        let input_block_hash: crate::common::BlockHashAsBase58 = Input::new()
            .with_prompt("Enter recent block hash")
            .interact_text()
            .unwrap();
        input_block_hash.inner
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let nonce = self.nonce.clone();
        let block_hash = self.block_hash.clone();
        let signer_public_key = self.signer_public_key.clone();
        let unsigned_transaction = near_primitives::transaction::Transaction {
            block_hash,
            nonce,
            public_key: signer_public_key,
            ..prepopulated_unsigned_transaction
        };
        let selected_server_url = None;
        self.send_from
            .process(unsigned_transaction, selected_server_url)
            .await
    }
}
