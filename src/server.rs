
use salvo::prelude::*;
use tokio::net::ToSocketAddrs;





pub async fn webserver(addr: impl ToSocketAddrs + Send + 'static, router: Router) -> Result<(), crate::Error> {

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

    server.serve({

        Service::new(Router::new().hoop(CatchPanic::new()).push(router)).catcher(crate::routes::init_catcher()?)

    }).await;

    Ok(())

}