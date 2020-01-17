SELECT EXISTS (
    SELECT
        1
    FROM
        members
    WHERE
        conversation_id = @conversation_id AND
        user_id = @user_id
    LIMIT 1
)
