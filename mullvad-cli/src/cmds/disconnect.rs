use crate::{new_rpc_client, Command, Result};

pub struct Disconnect;

#[mullvad_management_interface::async_trait]
impl Command for Disconnect {
    fn name(&self) -> &'static str {
        "disconnect"
    }

    fn clap_subcommand(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name())
            .about("Command the client to disconnect the VPN tunnel")
    }

    async fn run(&self, _: &clap::ArgMatches<'_>) -> Result<()> {
        let mut rpc = new_rpc_client().await?;
        rpc.disconnect_tunnel(()).await?;
        Ok(())
    }
}
