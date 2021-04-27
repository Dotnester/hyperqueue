use crate::transfer::connection::ClientConnection;
use crate::transfer::messages::{FromClientMessage, ToClientMessage};
use crate::client::worker::print_worker_info;
use crate::common::error::error;
use crate::client::utils::handle_message;
use crate::client::utils::OutputStyle;
use crate::client::globalsettings::GlobalSettings;

pub async fn get_worker_list(connection: &mut ClientConnection, gsettings: &GlobalSettings) -> crate::Result<()> {
    match handle_message(connection.send_and_receive(FromClientMessage::WorkerList).await)? {
        ToClientMessage::WorkerListResponse(mut msg) => {
            msg.workers.sort_unstable_by_key(|w| w.id);
            print_worker_info(msg.workers, gsettings);
        }
        msg => return error(format!("Received an invalid message {:?}", msg))
    }
    Ok(())
}
