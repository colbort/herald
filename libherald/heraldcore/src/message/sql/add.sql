INSERT INTO
  messages(
    msg_id,
    author,
    conversation_id,
    body,
    ts
  )
VALUES(@1, @2, @3, @4, @5)
