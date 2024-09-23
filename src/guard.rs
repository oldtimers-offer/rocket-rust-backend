use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

// AdminUser Struct (Guard)
pub struct AdminUser;

// Implementation for checking if user is an admin
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        // Here you would check session/cookie data, or a database check
        // For this example, we just check for an "admin" header
        if request.headers().get_one("X-Admin-Token") == Some("admin-secret-token") {
            Outcome::Success(AdminUser)
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

// User Struct (Guard)
pub struct User;

// Implementation for checking if user is a regular user
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        // Similarly, check session or token for regular user access
        if request.headers().get_one("X-User-Token").is_some() {
            Outcome::Success(User)
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

// SuperAdmin Struct (Guard)
pub struct SuperAdmin;

// Implementation for checking if user is an admin
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SuperAdmin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        // Here you would check session/cookie data, or a database check
        // For this example, we just check for an "admin" header
        if request.headers().get_one("Super-Admin-Token") == Some("super-admin-secret-token") {
            Outcome::Success(SuperAdmin)
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}
