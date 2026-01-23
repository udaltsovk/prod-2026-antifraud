SELECT
    count(*)
FROM
    users
WHERE
    (
        $1::user_role [] IS NULL
        OR role = ANY($1)
    )
    AND (
        $2::bool IS NULL
        OR is_active = $2
    )
