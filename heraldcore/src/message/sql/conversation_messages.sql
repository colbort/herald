SELECT
  messages.msg_id,
  author,
  conversation_id,
  body,
  op_msg_id,
  ts,
  send_status
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  conversation_id = ?
ORDER BY
  ts ASC