use tokio::sync::broadcast::Sender;

use crate::{TxSender, TX_NEWS, TX_NOTIFY};

pub enum SubscribeSender {
    NofifyDely(TxSender),
    News(TxSender),
}

fn publish(tx: Sender<TxSender>, payload: TxSender) {
    match tx.send(payload) {
        Ok(_) => {}
        Err(err) => {
            println!("TX_NOTIFY send error: {}", err);
        }
    }
}

impl SubscribeSender {
    pub fn send(self) {
        match self {
            SubscribeSender::NofifyDely(tx_sender) => {
                let tx = TX_NOTIFY.to_owned();
                publish(tx, tx_sender)
            }
            SubscribeSender::News(tx_sender) => {
                let tx = TX_NEWS.to_owned();
                publish(tx, tx_sender)
            }
        }
    }
}
