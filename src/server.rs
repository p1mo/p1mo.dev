use std::sync::Arc;

use crate::Error;
use salvo::prelude::*;
use tokio::{net::ToSocketAddrs, task::JoinHandle};





#[derive(Debug, Clone)]
pub enum ServerKind {
    Parker,
    Website
}



pub async fn webserver(kind: ServerKind, addr: impl ToSocketAddrs + Send + 'static, router: Router) -> Result<(), crate::Error> {

    let acceptor = TcpListener::new(addr).bind().await;
    let server = Server::new(acceptor);
    let handle = server.handle();

    tokio::spawn(async move {

        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(windows)]
        let terminate = async {
            tokio::signal::windows::ctrl_c()
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        tokio::select! {
            _ = ctrl_c => tracing::info!("ctrl_c signal received"),
            _ = terminate => tracing::info!("terminate signal received"),
        };

        handle.stop_graceful(None);

    });

    let router = {
        Router::new()
            .hoop(CatchPanic::new())
            .hoop(affix_state::inject(Arc::new(kind.clone())))
            .push(router)
    };

    server.serve({
        Service::new(router).catcher(crate::routes::init_catcher(kind)?)
    }).await;

    Ok(())

}



pub type OptinalJoinHandle = Option<JoinHandle<Result<(), Error>>>;



pub async fn optional_task(
    task: OptinalJoinHandle,
) -> Result<Option<()>, Error> {
    match task {
        Some(handle) => Ok(Some(handle.await??)),
        None => Ok(None),
    }
}