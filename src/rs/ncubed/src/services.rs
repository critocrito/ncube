use crate::errors::DataStoreError;
use crate::stores::NcubeStore;
use anyhow::Result;
use ncube_data::NcubeConfig;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum NcubeStoreCmd {
    IsBootstrapped(oneshot::Sender<Result<bool, DataStoreError>>),
    ShowConfig(oneshot::Sender<Result<NcubeConfig, DataStoreError>>),
    InsertSetting(oneshot::Sender<Result<(), DataStoreError>>, String, String),
}

pub async fn ncube_store_service<T: NcubeStore>(
    mut rx: mpsc::Receiver<NcubeStoreCmd>,
    mut ncube_store: T,
) -> Result<()> {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            NcubeStoreCmd::IsBootstrapped(tx) => match ncube_store.is_bootstrapped().await {
                Ok(is_bootstrapped) => {
                    let _ = tx.send(Ok(is_bootstrapped));
                }
                Err(err) => {
                    let _ = tx.send(Err(err));
                }
            },
            NcubeStoreCmd::ShowConfig(tx) => match ncube_store.show().await {
                Ok(config) => {
                    let _ = tx.send(Ok(config));
                }
                Err(err) => {
                    let _ = tx.send(Err(err));
                }
            },
            NcubeStoreCmd::InsertSetting(tx, name, value) => {
                match ncube_store.insert(&name, &value).await {
                    Ok(()) => {
                        let _ = tx.send(Ok(()));
                    }
                    Err(err) => {
                        let _ = tx.send(Err(err));
                    }
                }
            }
        }
    }
    Ok(())
}
