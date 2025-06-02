use tokio::sync::broadcast::Sender;

use crate::{TxSender, TX_NOTIFY};

pub enum NotifySender {
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

impl NotifySender {
    pub fn send(self) {
        match self {
            NotifySender::Dely(tx_sender) => {
                let tx = TX_NOTIFY.to_owned();
                publish(tx, tx_sender)
            }
        }
    }
}
