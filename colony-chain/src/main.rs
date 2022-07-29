use pbc_colony_common::*;
use pdao_polkadot_colony_chain::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "80".to_owned())
        .parse::<u16>()
        .unwrap();
    println!("RUN ON PORT {}", port);
    serde_tc::http::run_server(
        port,
        vec![(
            "astar".to_owned(),
            serde_tc::http::create_http_object(Arc::new(Astar {}) as Arc<dyn ColonyChain>),
        )]
        .into_iter()
        .collect(),
    )
    .await;
}
