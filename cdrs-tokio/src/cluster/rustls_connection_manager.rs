use async_trait::async_trait;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::cluster::connection_manager::{
    startup, ConnectionManager, ThreadSafeReconnectionPolicy,
};
use crate::cluster::{KeyspaceHolder, NodeRustlsConfig};
use crate::compression::Compression;
use crate::error::Result;
use crate::frame::Frame;
use crate::transport::{CdrsTransport, TransportRustls};

pub struct RustlsConnectionManager {
    config: NodeRustlsConfig,
    keyspace_holder: Arc<KeyspaceHolder>,
    compression: Compression,
    event_handler: Option<Sender<Frame>>,
    connection: Mutex<Option<Arc<TransportRustls>>>,
}

#[async_trait]
impl ConnectionManager<TransportRustls> for RustlsConnectionManager {
    async fn connection(
        &self,
        reconnection_policy: &ThreadSafeReconnectionPolicy,
    ) -> Result<Arc<TransportRustls>> {
        let mut connection = self.connection.lock().await;
        if let Some(connection) = connection.deref() {
            if !connection.is_broken() {
                return Ok(connection.clone());
            }
        }

        let mut schedule = reconnection_policy.new_node_schedule();

        loop {
            let transport = self.establish_connection().await;
            match transport {
                Ok(transport) => {
                    let transport = Arc::new(transport);
                    *connection = Some(transport.clone());

                    return Ok(transport);
                }
                Err(error) => {
                    let delay = schedule.next_delay().ok_or(error)?;
                    sleep(delay).await;
                }
            }
        }
    }

    fn addr(&self) -> SocketAddr {
        self.config.addr
    }
}

impl RustlsConnectionManager {
    pub fn new(
        config: NodeRustlsConfig,
        keyspace_holder: Arc<KeyspaceHolder>,
        compression: Compression,
        event_handler: Option<Sender<Frame>>,
    ) -> Self {
        RustlsConnectionManager {
            config,
            keyspace_holder,
            compression,
            event_handler,
            connection: Default::default(),
        }
    }

    async fn establish_connection(&self) -> Result<TransportRustls> {
        let transport = TransportRustls::new(
            self.config.addr,
            self.config.dns_name.clone(),
            self.config.config.clone(),
            self.keyspace_holder.clone(),
            self.event_handler.clone(),
            self.compression,
        )
        .await?;

        startup(
            &transport,
            self.config.authenticator.deref(),
            self.keyspace_holder.deref(),
            self.compression,
        )
        .await?;

        Ok(transport)
    }
}