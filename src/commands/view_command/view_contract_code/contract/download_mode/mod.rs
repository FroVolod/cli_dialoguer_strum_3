use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod download_contract;
mod hash_contract;

#[derive(Debug, clap::Clap)]
pub enum CliDownloadMode {
    /// Download a contract file
    Download(self::download_contract::CliContractFile),
    /// View a contract hash
    Hash(self::hash_contract::CliContractHash),
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum DownloadMode {
    #[strum_discriminants(strum(message = "Download a contract file"))]
    Download(self::download_contract::ContractFile),
    #[strum_discriminants(strum(message = "View a contract hash"))]
    Hash(self::hash_contract::ContractHash),
}

impl DownloadMode {
    pub fn from(item: CliDownloadMode, contract_id: &str) -> Self {
        match item {
            CliDownloadMode::Download(cli_contract_file) => DownloadMode::Download(
                self::download_contract::ContractFile::from(cli_contract_file, contract_id),
            ),
            CliDownloadMode::Hash(cli_contract_hash) => {
                DownloadMode::Hash(self::hash_contract::ContractHash::from(cli_contract_hash))
            }
        }
    }
}

impl DownloadMode {
    pub fn choose_download_mode(contract_id: &str) -> Self {
        println!();
        let variants = DownloadModeDiscriminants::iter().collect::<Vec<_>>();
        let modes = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let selected_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("To view contract code you will need to choose next action")
            .items(&modes)
            .default(0)
            .interact()
            .unwrap();
        let cli_mode = match variants[selected_mode] {
            DownloadModeDiscriminants::Download => CliDownloadMode::Download(Default::default()),
            DownloadModeDiscriminants::Hash => CliDownloadMode::Hash(Default::default()),
        };
        Self::from(cli_mode, contract_id)
    }

    pub async fn process(
        self,
        contract_id: String,
        network_connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        match self {
            DownloadMode::Download(contract_file) => {
                contract_file
                    .process(contract_id, network_connection_config)
                    .await
            }
            DownloadMode::Hash(contract_hash) => {
                contract_hash
                    .process(contract_id, network_connection_config)
                    .await
            }
        }
    }
}
