use tokio::sync::mpsc;

pub fn bus<S, R>() -> (LeftBus<S, R>, RightBus<S, R>) {
    let (tx1, rx1) = mpsc::channel(100);
    let (tx2, rx2) = mpsc::channel(100);

    (LeftBus { tx: tx1, rx: rx2 }, RightBus { tx: tx2, rx: rx1 })
}

pub struct LeftBus<S, R> {
    tx: mpsc::Sender<S>,
    rx: mpsc::Receiver<R>,
}

impl<S, R> LeftBus<S, R> {
    pub async fn send(&self, v: S) {
        self.tx.send(v).await.unwrap();
    }

    pub async fn recv(&mut self) -> R {
        self.rx.recv().await.unwrap()
    }
}

pub struct RightBus<S, R> {
    rx: mpsc::Receiver<S>,
    tx: mpsc::Sender<R>,
}

impl<S, R> RightBus<S, R> {
    pub async fn send(&self, v: R) {
        self.tx.send(v).await.unwrap();
    }

    pub async fn recv(&mut self) -> S {
        self.rx.recv().await.unwrap()
    }
}
