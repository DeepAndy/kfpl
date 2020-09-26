use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "Port Forward";

#[derive(Default)]
pub struct PortForward {
    port: String
}

impl PortForward {
    pub fn with_port(mut self, p: &str) -> Self {
        self.port = p.to_owned();
        self
    }
}

impl Nameable for PortForward {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for PortForward {
    async fn is_present(&self) -> Result<bool> {
        // TODO: This is a hack: eventually, check to see if the port is already open.
        Ok(false)
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("kubectl")
            .arg("port-forward")
            .arg("-n")
            .arg("kubeflow")
            .arg("svc/ml-pipeline-ui")
            .arg(format!("{}:80", self.port))
            .status().await
            .status_to_unit()
            .context("Unable to start the port-forward.")?;

        // This is a blocking call...

        Ok(())
    }
}