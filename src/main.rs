use log::{debug, info};
use std::io::Write;
use std::sync::Arc;
use std::{thread, time::Duration};
use tokio::sync::{mpsc, Mutex};

#[cxx::bridge(namespace = "dev::pnkv")]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type DepthAISource;

        fn post_frame(src: &mut DepthAISource, data: &[u8], size: u32);
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("include/depthai_wrapper.h");

        type DepthAIClient;

        fn new_depthai_client() -> UniquePtr<DepthAIClient>;
        fn next_frame(&self, src: &mut DepthAISource) -> i32;
    }
}

pub struct DepthAISource {
    file: std::fs::File,
}

impl DepthAISource {
    pub fn new() -> Self {
        Self {
            file: std::fs::File::create("video.h264").unwrap(),
        }
    }
    pub fn close(&mut self) {
        self.file.flush().unwrap();
    }
}

pub fn post_frame(src: &mut DepthAISource, data: &[u8], size: u32) {
    debug!("post_frame: {} {}", data.len(), size);
    src.file.write_all(data).unwrap();
}

fn setup_logger(debug: bool) {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    });
    if debug {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();
}

#[tokio::main]
async fn main() {
    setup_logger(true);

    debug!("Starting");

    let src = DepthAISource::new();

    let shared_src = Arc::new(Mutex::new(src));
    let shared_src_clone = shared_src.clone();

    let (stopper_tx, stopper_rx) = mpsc::channel(1);
    let stopper_rx_arc = Arc::new(Mutex::new(stopper_rx));

    thread::spawn(move || {
        let mut locked_src = shared_src.blocking_lock();
        let client = ffi::new_depthai_client();
        let mut stopper_rx = stopper_rx_arc.blocking_lock();

        while !stopper_rx.try_recv().is_ok() {
            let ret = client.next_frame(&mut locked_src);
            debug!("next_frame: {}", ret);
            thread::sleep(Duration::from_millis(33)); // 30fps
        }
    });

    tokio::select! {
      _ = tokio::signal::ctrl_c() => {
          debug!("Stopping - recived interrupt signal");
          stopper_tx.send(()).await.unwrap();
      }
    };

    shared_src_clone.lock().await.close();

    info!("To view the encoded data, convert the stream file (.h265) into a video file (.mp4) using a command below:");
    info!("ffmpeg -framerate 30 -i video.h264 -c copy video.mp4");

    debug!("Done");
}
