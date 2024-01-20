use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for idx in 0..50 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            //// 在这里执行任务...
            println!("idx={}", idx);
            sleep(Duration::from_secs(1)).await;
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}