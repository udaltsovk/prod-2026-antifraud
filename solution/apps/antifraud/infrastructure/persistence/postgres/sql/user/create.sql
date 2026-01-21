INSERT INTO users (
   id,
   email,
   full_name,
   password_hash,
   age,
   gender,
   martial_status,
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
        true,
        now(),
        now()
)
RETURNING
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
