// 引入需要的库
use futures::future::join_all;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// 定义异步函数来进行数据处理
async fn process_data(data: String) -> String {
    // 假设这里有一些复杂的数据处理逻辑
    data.to_uppercase()
}

// 异步函数，用于从网站获取数据
async fn fetch_data_from_website(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

// 异步函数，用于保存数据到文件
async fn save_to_file(filename: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(filename).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

// 异步主函数，处理所有任务
async fn main_task() -> Result<(), Box<dyn std::error::Error>> {
    let urls = [
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];

    // 创建一个 Vec 来保存所有任务
    let mut tasks = vec![];

    // 并发地发起网络请求并处理每个响应
    for (idx, &url) in urls.iter().enumerate() {
        let task = async move {
            let data = fetch_data_from_website(url).await?;
            let processed_data = process_data(data).await;
            let filename = format!("output{}.txt", idx);
            save_to_file(&filename, &processed_data).await?;
            Ok(())
        };
        tasks.push(task);
    }

    // 并发地执行所有任务
    let results: Vec<Result<(), Box<dyn std::error::Error>>> = join_all(tasks).await;
    for result in results {
        if let Err(err) = result {
            eprintln!("任务执行出错: {:?}", err);
        }
    }

    Ok(())
}

// 入口函数，调用异步主函数并运行事件循环
#[tokio::main]
async fn main() {
    if let Err(err) = main_task().await {
        eprintln!("程序执行出错: {:?}", err);
    }
}
