use std::time::Duration;
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};

const FPS: usize = 60;

const FREQUENCY: usize = 4194304;

const WAIT_TICKS: usize = FREQUENCY / FPS;

pub struct ClockSyncer {
    subscriber: broadcast::Receiver<()>,
    finisher: mpsc::Sender<()>,
}

impl ClockSyncer {
    pub async fn receive_tick(&mut self) {
        self.subscriber.recv().await.unwrap();
    }

    pub async fn finish_tick(&self) {
        self.finisher.send(()).await.unwrap();
    }
}

pub struct Clock {
    ticks: u32,
    syncer_count: usize,
    ticker: broadcast::Sender<()>,
    _ticker_backup: broadcast::Receiver<()>,
    finish_rx: mpsc::Receiver<()>,
    finish_tx: mpsc::Sender<()>,
}

impl Clock {
    pub fn new() -> Self {
        let (ticker, ticker_backup) = broadcast::channel(100);
        let (finish_tx, finish_rx) = mpsc::channel(100);

        Self {
            ticks: 0,
            syncer_count: 0,
            ticker,
            _ticker_backup: ticker_backup,
            finish_rx,
            finish_tx,
        }
    }

    pub fn syncer(&mut self) -> ClockSyncer {
        self.syncer_count += 1;
        ClockSyncer {
            subscriber: self.ticker.subscribe(),
            finisher: self.finish_tx.clone(),
        }
    }

    pub async fn run(mut self) {
        loop {
            self.ticks = 0;

            let sleep = sleep(Duration::from_secs(1) / FPS as u32);

            self.ticks += 1;

            self.ticker.send(()).unwrap();

            for _ in 0..self.syncer_count {
                self.finish_rx.recv().await.unwrap();
            }

            sleep.await;
        }
    }
}
