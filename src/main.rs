// mod application;
// mod domain;
// mod infrastructure;
// mod macros;
// mod presentation;

// use axum::{Router, routing::get};
// use std::{env, io::Error, sync::Arc, time::Duration};

// #[tokio::main]
fn main() {
    // let db = connection().await.inspect_err(|e| {
    //     log(LogEntry::Error(e.kind(), e.to_string()));
    // })?;
    // let db = Arc::new(db);

    // schema(db.clone()).await.inspect_err(|e| {
    //     log(LogEntry::Error(e.kind(), e.to_string()));
    // })?;

    // let services = dependency_injection(db.clone());

    // let move_services = services.clone();
    // tokio::spawn({
    //     async move {
    //         let services = move_services.clone();
    //         loop {
    //             println!("Delivering Karma...");
    //             let vec_karma = futures::executor::block_on(async {
    //                 services.providers.karma.get(KarmaFilters::default()).await
    //             });

    //             if let Err(e) = vec_karma {
    //                 log(LogEntry::Error(e.kind(), e.to_string()));
    //             } else if let Err(e) =
    //                 use_case_karma_deliver(services.clone(), vec_karma.unwrap()).await
    //             {
    //                 log(LogEntry::Error(e.kind(), e.to_string()));
    //             }

    //             println!("Karma Delivered!");
    //             tokio::time::sleep(Duration::from_secs(60)).await;
    //         }
    //     }
    // });

    // match env::args().nth(1).as_deref() {
    //     Some("migrate") => execute_migration(db.clone()).await.inspect_err(|e| {
    //         log(LogEntry::Error(e.kind(), e.to_string()));
    //     }),
    //     _ => {
    //         let app = Router::new()
    //             .route("/preto_no_branco.ico", get(handler_section_favicon))
    //             .merge(section_router(services.clone()))
    //             .nest("/collection", collection_router(services.clone()))
    //             .nest("/view", view_router(services.clone()))
    //             .nest("/table", table_router(services.clone()))
    //             .nest("/operation", operation_router(services.clone()));

    //         let listener = tokio::net::TcpListener::bind("0.0.0.0:6174").await.unwrap();
    //         println!("Listening on: {}", listener.local_addr().unwrap());
    //         axum::serve(listener, app).await?;
    //         Ok(())
    //     }
    // }?;

    // Ok(())
    println!("RustArt")
}
