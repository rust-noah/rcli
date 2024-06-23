use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let tasks = (0..5)
        .map(|i| {
            task::spawn(async move {
                println!("task {}", i);
            })
        })
        .collect::<Vec<_>>();

    // 等待所有任务完成
    for task in tasks {
        task.await.unwrap();
    }

    // 等待一段时间，以保证所有输出都打印完毕
    sleep(Duration::from_secs(1)).await;
}
