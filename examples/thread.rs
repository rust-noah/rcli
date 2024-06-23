use std::thread;
use std::time::Duration;

fn main() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("thread {}", i);
            })
        })
        .collect();

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 等待一段时间，以保证所有线程的输出都打印出来
    thread::sleep(Duration::from_secs(1));
}
