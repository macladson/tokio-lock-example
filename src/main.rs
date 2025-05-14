use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

const TASK_COUNT: usize = 1000;
const HEARTBEAT_SECS: u64 = 5;
const TASK_COOLDOWN_MILLIS: u64 = 5;

#[tokio::main]
async fn main() {
    let mut heartbeat = 0;
    let mutex = Arc::new(Mutex::new(0));
    let rwlock = Arc::new(RwLock::new(0));

    for _ in 0..TASK_COUNT{
        let mutex_clone = Arc::clone(&mutex);
        let rwlock_clone = Arc::clone(&rwlock);

        tokio::spawn(async move {
            loop {
                {
                    let mut guard = mutex_clone.lock().await;
                    *guard += 1;
                }

                {
                    let guard = rwlock_clone.read().await;
                    let _ = *guard;
                }

                {
                    let mut guard = rwlock_clone.write().await;
                    *guard += 1;
                }

                tokio::time::sleep(Duration::from_millis(TASK_COOLDOWN_MILLIS)).await;
            }
        });
    }

    loop {
        println!("Heartbeat {heartbeat}");
        heartbeat += 1;
        tokio::time::sleep(Duration::from_secs(HEARTBEAT_SECS)).await;
    }
}
