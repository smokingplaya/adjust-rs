use axum::{extract::Path, routing::get, Router};
use dixxxie::{controller::Controller, response::HttpResult};

use crate::{models::hi::MessageDto, service::test::TestService, AppState};

pub struct TestController;

impl TestController {
  async fn say_hi(
    Path(name): Path<String>
  ) -> HttpResult<MessageDto> {
    TestService::say_hi(name)
  }
}

impl Controller<AppState> for TestController {
  fn register(self, router: Router<AppState>) -> Router<AppState> {
    router
      .nest("/test",
      Router::new()
        .route("/sayHi/{name}", get(Self::say_hi))
    )
  }
}