UPDATE ratchet_states
  SET deprecated = 1
WHERE
  public_key = @pk
