use tokio::sync::broadcast::Sender;

use crate::{TxSender, TX_NOTIFY};

pub enum NotifySenderType {
    Dely(TxSender),
}

fn publish(tx: Sender<TxSender>, payload: TxSender) {
    match tx.send(payload) {
        Ok(_) => {}
        Err(err) => {
            println!("TX_NOTIFY send error: {}", err);
        }
    }
}

pub fn send(notify_sender_type: NotifySenderType) {
    match notify_sender_type {
        NotifySenderType::Dely(tx_sender) => {
            let tx = TX_NOTIFY.clone();
            publish(tx, tx_sender)
        }
    }
}
