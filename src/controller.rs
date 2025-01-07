use axum::Router;

pub trait Controller<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn register(&self, router: Router<S>) -> Router<S>;
}

pub trait ApplyControllerOnRouter<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn apply_controller<T>(self, controller: T) -> Router<S>
  where
    T: Controller<S>;
}

impl<S> ApplyControllerOnRouter<S> for Router<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn apply_controller<T>(self, controller: T) -> Router<S>
  where
    T: Controller<S>,
  {
    controller.register(self)
  }
}