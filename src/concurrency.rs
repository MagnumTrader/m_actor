pub type OneShotSender<T> = tokio::sync::oneshot::Sender<T>;
pub type OneShotReader<T> = tokio::sync::oneshot::Receiver<T>;

pub type MpscSender<T> = tokio::sync::mpsc::Sender<T>;
pub type MpscReciever<T> = tokio::sync::mpsc::Receiver<T>;

pub type UnboundedMpscSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub type UnboundedMpscReciever<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

pub type BroadcastSender<T> = tokio::sync::broadcast::Sender<T>;
pub type BroadcastReciever<T> = tokio::sync::broadcast::Receiver<T>;

pub fn oneshot<T>() -> (OneShotSender<T>, OneShotReader<T>) {
    tokio::sync::oneshot::channel()
}

pub fn mpsc_bounded<T>(bound: usize) -> (MpscSender<T>, MpscReciever<T>) {
    tokio::sync::mpsc::channel(bound)
}

pub fn mpsc_unbounded<T>() -> (UnboundedMpscSender<T>, UnboundedMpscReciever<T>) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn broadcast<T: Clone>(bound: usize) -> (BroadcastSender<T>, BroadcastReciever<T>) {
    tokio::sync::broadcast::channel(bound)
}
