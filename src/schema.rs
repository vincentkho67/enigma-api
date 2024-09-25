// @generated automatically by Diesel CLI.

diesel::table! {
    attendance (id) {
        id -> Int4,
        user_course_id -> Int4,
        date -> Timestamp,
        #[max_length = 64]
        status -> Varchar,
        #[max_length = 64]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    courses (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 16]
        course_type -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
        duration_in_days -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        first_name -> Nullable<Varchar>,
        #[max_length = 64]
        last_name -> Nullable<Varchar>,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 64]
        phone_number -> Nullable<Varchar>,
        #[max_length = 64]
        oauth_provider -> Nullable<Varchar>,
        #[max_length = 64]
        oauth_id -> Nullable<Varchar>,
        #[max_length = 64]
        profile_picture_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users_courses (id) {
        id -> Int4,
        user_id -> Int4,
        course_id -> Int4,
        total_attendance -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(attendance -> users_courses (user_course_id));
diesel::joinable!(users_courses -> courses (course_id));
diesel::joinable!(users_courses -> users (user_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendance,
    courses,
    roles,
    users,
    users_courses,
    users_roles,
);
