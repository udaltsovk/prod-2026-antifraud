SELECT
    id,
    email,
    full_name,
    password_hash,
    age,
    gender AS "gender: _",
    martial_status AS "martial_status: _",
    region,
    role AS "role: _",
    is_active,
    created_at,
    updated_at
FROM users
WHERE id = $1
