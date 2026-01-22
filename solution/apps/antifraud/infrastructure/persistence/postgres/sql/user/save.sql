INSERT INTO users (
   id,
   email,
   full_name,
   password_hash,
   age,
   gender,
   marital_status,
   region,
   role,
   is_active,
   created_at,
   updated_at
)
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        now()
)
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
