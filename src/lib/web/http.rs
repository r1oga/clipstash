use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;
use rocket::response::{status, Redirect};
use rocket::{uri, State};
use rocket::http::uri::fmt::UriQueryArgument::Raw;

use crate::data::Database;
use crate::{Db, HitCounter, service};
use crate::service::{action, ask};
use crate::web::{form, renderer::Renderer, PageError, ctx, PASSWORD_COOKIE};
use crate::{ServiceError, ShortCode};
use crate::domain::clip::field::Content;
use crate::web::ctx::*;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let ctx = Home::default();
    RawHtml(renderer.render(ctx, &[]))
}


#[rocket::post("/", data = "<form>")]
pub async fn new_clip(
    // rocket can only call a function for a route if all data exists.
    // form data may not exist or be incorrect
    // using Contextual allows to accept invalid form data
    form: Form<Contextual<'_, form::NewClip>>,
    db: &State<Db>,
    renderer: &State<Renderer<'_>>,
) -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner(); // to get Contextual
    if let Some(value) = form.value {
        let req = ask::NewClip {
            // these values comes from from::NewClip, which uses field::*, so we know all these fields are already validated
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        };

        match action::new_clip(req, db.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("internal error: {:?}", e);
                Err((
                    Status::InternalServerError,
                    RawHtml(
                        renderer.render(ctx::Home::default(),
                                        &["A server error occurred. Please try again."],
                        )
                    )
                ))
            }
        }
    } else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;
                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("unhandled error: {:?}", err);
                    "An error occurred, please try again"
                }
            })
            .collect::<Vec<_>>();

        Err((
            Status::BadRequest,
            RawHtml(
                renderer.render_with_data(
                    ctx::Home::default(),
                    ("clip", &form.context),
                    &errors,
                )
            )
        ))
    }
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    db: &State<Db>,
    hit_counter: &State<HitCounter>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    fn render_with_status<T: PageCtx + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<RawHtml<String>>, PageError> {
        Ok(status::Custom(status, RawHtml(renderer.render(context, &[]))))
    }

    match action::get_clip(shortcode.clone().into(), db.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
            _ => Err(PageError::Internal("server error".to_owned()))
        }
    }
}

#[rocket::post("/clip/<shortcode>", data = "<form>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::GetPasswordProtectedClip>>,
    shortcode: ShortCode,
    hit_counter: &State<HitCounter>,
    db: &State<Db>,
    renderer: &State<Renderer<'_>>,
) -> Result<RawHtml<String>, PageError> {
    if let Some(form) = &form.into_inner().value {
        let req = ask::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };

        match action::get_clip(req, db.get_pool()).await {
            Ok(clip) => {
                hit_counter.hit(shortcode.clone(), 1);
                let context = ctx::ViewClip::new(clip);

                // adding cookie
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(RawHtml(renderer.render(context, &[])))
            }
            Err(e) => match e {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                }
                ServiceError::NotFound => Err(PageError::NotFound("clip not found".to_owned())),
                _ => Err(PageError::Internal("server error".to_owned()))
            }
        }
    } else {
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(RawHtml(renderer.render(context, &["A password is required to view this clip"])))
    }
}

#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: ShortCode,
    hit_counter: &State<HitCounter>,
    db: &State<Db>,
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;
    let req = ask::GetClip {
        shortcode: shortcode.clone(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_pwd| Password::new(raw_pwd.to_string()).ok())
            .flatten() // Option(Option()) -> Option()
            .unwrap_or_else(Password::default),
    };

    match action::get_clip(req, db.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            Ok(status::Custom(Status::Ok, clip.content.into_inner()))
        }
        Err(e) => match e {
            ServiceError::PermissionError(msg) => Ok(status::Custom(Status::Unauthorized, msg)),
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError)
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        home,
        get_clip,
        new_clip,
        submit_clip_password,
        get_raw_clip
    ]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "Something went wrong"
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}

#[cfg(test)]
pub mod test {
    use crate::data::Db;
    use crate::test::async_runtime;
    use crate::web::test::client;
    use rocket::http::Status;
    use serde_json::Value::String;

    #[test]
    fn gets_home() {
        let client = client();
        let res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);
    }

    #[test]
    fn not_found__error_on_unknown_clip_shortcode() {
        let client = client();
        let res = client.get("/clip/foo").dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }

    #[test]
    fn requires_pwd_if_defined() {
        use crate::domain::clip::field::{Content, Expires, Password, Title};
        use crate::service;
        use rocket::http::{ContentType, Cookie};

        let rt = async_runtime();

        let client = client();
        let db = client.rocket().state::<Db>().unwrap();

        let req = service::ask::NewClip {
            content: Content::new("content").unwrap(),
            expires: Expires::default(),
            password: Password::new("123".to_owned()).unwrap(),
            title: Title::default(),
        };
        let clip = rt
            .block_on(async move { service::action::new_clip(req, db.get_pool()).await })
            .unwrap();
        let response = client
            .get(format!("/clip/{}", clip.shortcode.as_str()))
            .dispatch();

        // Unauthorized error when no password is provided
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        // Get clip when the password is provided
        let response = client
            .post(format!("/clip/{}", clip.shortcode.as_str()))
            .header(ContentType::Form)
            .body("password=123")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "123"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Get clip when the password is provided, but incorrect
        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "abc"))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }
}

