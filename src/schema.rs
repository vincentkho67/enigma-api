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
    users (id) {
        id -> Int4,
        #[max_length = 64]
        first_name -> Varchar,
        #[max_length = 64]
        last_name -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        #[max_length = 64]
        phone_number -> Nullable<Varchar>,
        #[max_length = 64]
        oauth_provider -> Nullable<Varchar>,
        #[max_length = 64]
        oauth_id -> Nullable<Varchar>,
        #[max_length = 64]
        profile_picture_url -> Nullable<Varchar>,
        #[max_length = 16]
        role -> Varchar,
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

diesel::joinable!(attendance -> users_courses (user_course_id));
diesel::joinable!(users_courses -> courses (course_id));
diesel::joinable!(users_courses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendance,
    courses,
    users,
    users_courses,
);
