use crate::protocols::mediated_connection::invitee::states::requested::RequestedState;
use messages::diddoc::aries::diddoc::AriesDidDoc;
use messages::protocols::connection::invite::Invitation;
use messages::protocols::connection::request::Request;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvitedState {
    pub invitation: Invitation,
    pub did_doc: AriesDidDoc,
}

impl From<(InvitedState, Request, AriesDidDoc)> for RequestedState {
    fn from((_state, request, did_doc): (InvitedState, Request, AriesDidDoc)) -> RequestedState {
        trace!("ConnectionInvitee: transit state from InvitedState to RequestedState");
        RequestedState { request, did_doc }
    }
}
