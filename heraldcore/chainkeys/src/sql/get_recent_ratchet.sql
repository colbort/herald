SELECT
  generation, next_ix, base_key, ratchet_key
FROM
  ratchet_states
WHERE
  conversation_id = @cid
  AND public_key = @pk
ORDER BY
  generation DESC
LIMIT
  1
