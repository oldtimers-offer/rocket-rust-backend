#[macro_use]
extern crate rocket;
use crate::guard::AdminUser;
use crate::guard::SuperAdmin;
use crate::guard::User;

use rocket::response::Redirect;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::io;

mod guard;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "The World Is Mine!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/login")]
fn login() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Login Page");
    context.insert("message", "Please enter your credentials:");
    Template::render("login", &context)
}

#[get("/admin")]
fn admin_panel(_admin: AdminUser) -> &'static str {
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin", rank = 2)]
fn admin_panel_user(_user: User) -> &'static str {
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect(_super_admin: SuperAdmin) -> Redirect {
    Redirect::to(uri!(world))
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index, world])
// }

// second approach
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/hello",
            routes![
                index,
                world,
                delay,
                blocking_task,
                admin_panel,
                admin_panel_user,
                login,
                admin_panel_redirect,
            ],
        )
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
