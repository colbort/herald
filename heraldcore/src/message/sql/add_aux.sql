INSERT OR IGNORE INTO
  messages(
    msg_id,
    author,
    conversation_id,
    aux_item,
    send_status,
    insertion_ts,
    server_ts,
    expiration_ts,
    is_reply
  )
VALUES(
  @msg_id,
  @author,
  @conversation_id,
  @aux_item,
  @send_status,
  @insertion_ts,
  @server_ts,
  @expiration_ts,
  0
)
