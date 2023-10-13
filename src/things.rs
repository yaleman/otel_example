use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::MAX_LEWPS;

#[instrument(level = "info", name = "do_other_stuff")]
pub async fn do_other_stuff() {
    info!("This is totes not do_stuff.");
    possibly_slow_thing().await;
}

#[instrument(level = "info", name = "do_stuff", fields(request_id))]
pub async fn do_stuff() {
    tracing::Span::current().record("request_id", &Uuid::new_v4().to_string());
    info!("This event will be logged in the do_stuff span.");
    possibly_slow_thing().await;
}

#[instrument(level = "info", name = "possibly_slow_thing", fields(request_id))]
async fn possibly_slow_thing() {
    tracing::Span::current().record("request_id", &Uuid::new_v4().to_string());
    let sleeptime = rand::random::<u8>();
    if sleeptime > 192 {
        error!("This event is slow! Time was {}ns", &sleeptime);
    } else {
        info!("The event was fine");
    }
}

pub async fn do_stuff_loop() {
    let mut loop_counter: u64 = 0;
    loop {
        do_stuff().await;
        // tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        // counter.add(1, &[KeyValue::new("key", "value")]);

        loop_counter += 1;
        if loop_counter % 10 == 0 {
            println!("do_stuff loop counter is {}", loop_counter);
        }
        if loop_counter > MAX_LEWPS {
            break;
        }
    }
}

pub async fn do_other_stuff_loop() {
    let mut loop_counter: u64 = 0;
    loop {
        do_other_stuff().await;

        loop_counter += 1;
        if loop_counter % 10 == 0 {
            println!("do_other_stuff loop_counter is {}", loop_counter);
        }
        if loop_counter > MAX_LEWPS {
            break;
        }
    }
}
