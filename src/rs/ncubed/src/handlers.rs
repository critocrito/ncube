use crate::errors::RouteRejection;
use crate::services::NcubeStoreCmd;
use ncubed::types::NcubeConfigRequest;
use tokio::sync::{mpsc, oneshot};
use warp;

pub mod ncube_config {
    use super::*;

    pub async fn show(
        mut s: mpsc::Sender<NcubeStoreCmd>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let (tx, rx) = oneshot::channel();

        let _ = s
            .send(NcubeStoreCmd::IsBootstrapped(tx))
            .await
            .map_err(|_| RouteRejection::ChannelError)?;

        if let Ok(false) = rx.await.map_err(|_| RouteRejection::ChannelError)? {
            return Err(warp::reject::not_found());
        }

        let (tx, rx) = oneshot::channel();

        let _ = s
            .send(NcubeStoreCmd::ShowConfig(tx))
            .await
            .map_err(|_| RouteRejection::ChannelError)?;

        let config = rx
            .await
            .map_err(|_| RouteRejection::ChannelError)?
            .map_err(|_| RouteRejection::DataError)?;

        Ok(warp::reply::json(&config))
    }

    pub async fn create(
        settings: NcubeConfigRequest,
        mut s: mpsc::Sender<NcubeStoreCmd>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let (tx, rx) = oneshot::channel();

        let _ = s
            .send(NcubeStoreCmd::IsBootstrapped(tx))
            .await
            .map_err(|_| RouteRejection::ChannelError)?;

        if let Ok(true) = rx.await.map_err(|_| RouteRejection::ChannelError)? {
            return Err(warp::reject());
        }

        for setting in settings {
            let (tx, _rx) = oneshot::channel();

            let _ = s
                .send(NcubeStoreCmd::InsertSetting(
                    tx,
                    setting.name,
                    setting.value,
                ))
                .await
                .map_err(|_| RouteRejection::ChannelError)?;
        }

        Ok(warp::reply())
    }

    pub async fn insert(
        settings: NcubeConfigRequest,
        mut s: mpsc::Sender<NcubeStoreCmd>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let (tx, rx) = oneshot::channel();

        let _ = s
            .send(NcubeStoreCmd::IsBootstrapped(tx))
            .await
            .map_err(|_| RouteRejection::ChannelError)?;

        if let Ok(false) = rx.await.map_err(|_| RouteRejection::ChannelError)? {
            return Err(warp::reject());
        }

        for setting in settings {
            let (tx, _rx) = oneshot::channel();

            let _ = s
                .send(NcubeStoreCmd::InsertSetting(
                    tx,
                    setting.name,
                    setting.value,
                ))
                .await
                .map_err(|_| RouteRejection::ChannelError)?;
        }

        Ok(warp::reply())
    }
}
