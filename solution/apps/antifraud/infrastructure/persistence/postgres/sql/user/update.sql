UPDATE
    users
SET
    full_name = $2,
    age = $3,
    gender = $4,
    marital_status = $5,
    region = $6,
    role = $7,
    is_active = $8,
    updated_at = NOW()
WHERE
    id = $1
RETURNING
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
