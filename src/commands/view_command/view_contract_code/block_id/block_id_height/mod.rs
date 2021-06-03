use dialoguer::Input;
use std::io::Write;

/// Specify the block_id height for this contract to view
#[derive(Debug, Default, clap::Clap)]
pub struct CliBlockIdHeight {
    block_id_height: Option<near_primitives::types::BlockHeight>,
}

#[derive(Debug)]
pub struct BlockIdHeight {
    block_id_height: near_primitives::types::BlockHeight,
}

impl From<CliBlockIdHeight> for BlockIdHeight {
    fn from(item: CliBlockIdHeight) -> Self {
        let block_id_height: near_primitives::types::BlockHeight = match item.block_id_height {
            Some(cli_block_id_hash) => cli_block_id_hash,
            None => BlockIdHeight::input_block_id_height(),
        };
        Self { block_id_height }
    }
}

impl BlockIdHeight {
    pub fn input_block_id_height() -> near_primitives::types::BlockHeight {
        Input::new()
            .with_prompt("Type the block ID height for this contract")
            .interact_text()
            .unwrap()
    }

    fn rpc_client(&self, selected_server_url: &str) -> near_jsonrpc_client::JsonRpcClient {
        near_jsonrpc_client::new_client(&selected_server_url)
    }

    pub async fn process(
        self,
        contract_id: String,
        network_connection_config: crate::common::ConnectionConfig,
        file_path: Option<std::path::PathBuf>,
    ) -> crate::CliResult {
        let query_view_method_response = self
            .rpc_client(network_connection_config.archival_rpc_url().as_str())
            .query(near_jsonrpc_primitives::types::query::RpcQueryRequest {
                block_reference: near_primitives::types::BlockReference::BlockId(
                    near_primitives::types::BlockId::Height(self.block_id_height.clone()),
                ),
                request: near_primitives::views::QueryRequest::ViewCode {
                    account_id: contract_id,
                },
            })
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to fetch query for view contract: {:?}",
                    err
                ))
            })?;
        let call_access_view =
            if let near_jsonrpc_primitives::types::query::QueryResponseKind::ViewCode(result) =
                query_view_method_response.kind
            {
                result
            } else {
                return Err(color_eyre::Report::msg(format!("Error call result")));
            };
        match &file_path {
            Some(file_path) => {
                std::fs::File::create(file_path)
                    .map_err(|err| {
                        color_eyre::Report::msg(format!("Failed to create file: {:?}", err))
                    })?
                    .write(&call_access_view.code)
                    .map_err(|err| {
                        color_eyre::Report::msg(format!("Failed to write to file: {:?}", err))
                    })?;
                println!("\nThe file {:?} was downloaded successfully", file_path);
            }
            None => {
                println!("\nHash of the contract: {}", &call_access_view.hash)
            }
        }
        Ok(())
    }
}
