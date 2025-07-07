use crate::db_utils;
use crate::news::news_repository;
use crate::newsletter::newsletter_repository;
use chrono::{Duration, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::thread;
use tokio::runtime::Handle;
use tracing::info;

pub async fn run() {
    info!("start schedule");
    if let Ok(pool) = db_utils::get_pool().await {
        match newsletter_repository::find_all_active(&pool).await {
            Ok(v) => {
                info!("{:?}", v);
                for n in v.iter() {
                    n.send_notify(&pool).await;
                }
            }
            Err(_) => info!("get newsletter error"),
        };
        match news_repository::find_all_active(&pool).await {
            Ok(v) => {
                info!("{:?}", v);
                for n in v.iter() {
                    n.send_notify(&pool).await;
                }
            }
            Err(_) => info!("get news error"),
        }
    } else {
        info!("start schedule error")
    }
}

pub async fn newsletter_scheduler() {
    let expression = "0   *   *    *           *       *          *";
    let schedule = Schedule::from_str(expression).unwrap();
    let handle = Handle::current();

    thread::spawn(move || {
        for datetime in schedule.upcoming(Utc) {
            let now = Utc::now();
            let wait_time = datetime - now;

            if wait_time > Duration::zero() {
                thread::sleep(wait_time.to_std().unwrap());
            }

            handle.spawn(async {
                run().await;
            });
        }
    });
}
