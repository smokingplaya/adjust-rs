use std::any::type_name;
use axum::Router;

/// ``Controller`` - trait required for the ``use_controller`` method on ``axum::Router``.
///
/// The ``register`` method is called at the moment of ``axum`` web service initialization, and is applied at the moment of ``axum::Router`` creation.
pub trait Controller<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn register(self, router: Router<S>) -> Router<S>;
}

/// ``ApplyControllerOnRouter`` is an internal trait for ``axum::Router``.
///
/// Adds two methods - ``use_controller`` and ``use_controllers``.
pub(crate) trait ApplyControllerOnRouter<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn use_controller<T>(self, controller: T) -> Router<S>
  where
    T: Controller<S>;

  fn use_controllers<T, const SIZE: usize>(self, controllers: [T; SIZE]) -> Router<S>
  where
    T: Controller<S>;
}

impl<S> ApplyControllerOnRouter<S> for Router<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn use_controller<T>(self, controller: T) -> Router<S>
  where
    T: Controller<S>,
  {
    log::debug!("Using {}", type_name::<T>());

    controller.register(self)
  }

  fn use_controllers<T, const SIZE: usize>(self, controllers: [T; SIZE]) -> Router<S>
  where
    T: Controller<S>,
  {
    controllers.into_iter().fold(self, |router, controller| {
      router.use_controller(controller)
    })
  }
}
