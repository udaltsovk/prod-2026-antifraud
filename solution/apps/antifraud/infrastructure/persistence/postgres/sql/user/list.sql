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
FROM users
WHERE
    ($1::user_role[] IS NULL OR role = ANY($1)) AND ($2::bool IS NULL OR is_active = $2)
ORDER BY created_at
LIMIT $3
OFFSET $4;
