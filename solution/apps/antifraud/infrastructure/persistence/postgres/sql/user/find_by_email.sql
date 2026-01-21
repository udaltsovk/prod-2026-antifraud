SELECT
    id,
    email,
    full_name,
    password_hash,
    age,
    gender AS "gender: StoredUserGender",
    martial_status AS "martial_status: StoredUserMartialStatus",
    region,
    role AS "role: StoredUserRole",
    is_active,
    created_at,
    updated_at
FROM users
WHERE email = $1
