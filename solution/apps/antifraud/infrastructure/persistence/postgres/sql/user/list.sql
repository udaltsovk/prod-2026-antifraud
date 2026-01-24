SELECT
    id,
    email,
    full_name,
    password_hash,
    age,
    gender AS "gender: _",
    marital_status AS "marital_status: _",
    region,
    role AS "role: _",
    is_active,
    created_at,
    updated_at
FROM
    users
ORDER BY
    created_at
LIMIT
    $1 OFFSET $2
