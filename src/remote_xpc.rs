use libc::size_t;
use crate::{xpc_msg_id_t, xpc_object_t, xpc_handler_t, xpc_finalizer_t};
use std::collections::VecDeque;
use std::sync::Barrier;


struct xpc_stream_pair_t {
    send: quinn::SendStream,
    receive: quinn::RecvStream
}

struct xpc_rich_error_t {}

enum xpc_remote_version_t {
    VERSION_1 = 1,
    VERSION_2 = 2,
    VERSION_3 = 3,
    VERSION_4 = 4,
    VERSION_5 = 5
}

type xpc_session_cancel_handler_t = fn(xpc_rich_error_t);
type xpc_session_incoming_message_handler_t = fn(xpc_object_t);
type xpc_session_reply_handler_t = fn(reply: xpc_object_t, error: xpc_rich_error_t);

type xpc_connection_handler_t = fn(connection: xpc_remote_connection_t);

type xpc_next_stage_handler_t = xpc_handler_t;


enum xpc_remote_state_t {}

enum xpc_remote_direction_t {
    OUTBOUND,
    INBOUND
}

enum xpc_listener_type_t {

}

struct xpc_endpoint_t {}

struct xpc_remote_channel {
    conn: xpc_remote_connection_t,
    error: xpc_rich_error_t,
    event_handler: xpc_handler_t,
    msg_handler: xpc_session_incoming_message_handler_t,
    parent_io: xpc_remote_connection_t,
    queue: VecDeque<xpc_remote_message>,
    state: xpc_remote_state_t,
    stream_direction: xpc_remote_direction_t,
    stream_io: xpc_stream_pair_t,
    tx_complete: xpc_handler_t,
    wire_version: xpc_remote_version_t
}

struct xpc_remote_message {
    body: xpc_object_t,
    link_stage_next: xpc_next_stage_handler_t,
    msg_id: xpc_msg_id_t,
    ool: *mut u8,
    ool_length: size_t,
    wants_reply: bool,
}

struct xpc_remote_connection_t {

}

struct xpc_remote_listener {
    accept_handler: xpc_connection_handler_t,
    cancel_handler: xpc_session_cancel_handler_t,
    canceled: bool,
    queue: VecDeque<xpc_remote_message>,
    listener_type: xpc_listener_type_t,
    barrier: Barrier
}

struct xpc_remote_outstanding_reply {
    link_stage_next: xpc_next_stage_handler_t,
    msg_id: xpc_msg_id_t,
    reply_handler: xpc_session_reply_handler_t,
    reply_queue: VecDeque<xpc_remote_message>
}

struct xpc_remote_pending_stream {
    channel: xpc_remote_channel,
    completion_handler: xpc_finalizer_t,
    direction: xpc_remote_direction_t,
    error: xpc_rich_error_t,
    io: xpc_stream_pair_t,
    link_stage_next: xpc_next_stage_handler_t,
    stream_id: *str
}

struct xpc_remote_stream {
    channel: xpc_remote_channel,
    link_stage_next: xpc_next_stage_handler_t
}