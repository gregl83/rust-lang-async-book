use futures::{
    join,
    executor::block_on
};

async fn broward() -> usize { 12 }

async fn miami_dade() -> usize {
    let first = miami().await;
    let second = north_miami().await;
    first + second
}

async fn north_miami() -> usize { 7 }

async fn miami() -> usize { 10 }

async fn count_votes() -> (usize, usize) {
    let md = miami_dade();
    let b = broward();
    join!(md, b)
}

fn main() {
    let counties = block_on(count_votes());
    let total_votes = counties.0 + counties.1;
    println!("total votes: {}", total_votes);
}