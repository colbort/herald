SELECT
  count(*)
FROM
  sqlite_master
WHERE
  type = 'table'
  AND name = 'conversations'