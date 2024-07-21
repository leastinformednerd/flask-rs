//!Dead simple 
pub struct App<State, S = ()>
where
    State: Clone,
    S: Clone + Send + Sync + 'static,
{
    router: axum::Router<S>,
    state: State,
}
pub enum Method {
    Get,
    Post,
}

#[macro_export]
macro_rules! route {
    ($app:ident, $(($path:literal, $func:ident, $method:ident)),+) => {
        $app
        $(
            .with_route($path, $func, $method)
        )+
    };
    ($app:ident, $($path:ident, $func:ident, $method:ident),+) => {
        $app
        $(
            .with_route($path, $func)
        )+
    };
}

impl<State, S> App<State,S>
where
    State: Clone,
    S: Clone + Send + Sync + 'static,
{
    pub fn new(state: State) -> Self {
        Self {
            router: axum::Router::new(),
            state: state
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// 
    pub fn with_route<Handler, T>(mut self, path: &str, handler: Handler, method: Method) -> Self 
    where
        Handler: axum::handler::Handler<T, S>,
        T: 'static
    {
        // This could be more DRY (by moving the match into the method_router arg) but that seems kind of unreadable
        match method {
            Method::Get => self.router = self.router.route(path,axum::routing::get(handler)),
            Method::Post => self.router = self.router.route(path,axum::routing::post(handler)),
        }

        self
    }

    ///Hmmm...
    pub async fn serve(self, addr: &'static str) -> () {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.router).await.unwrap();
    }

}

#[cfg(test)]
mod test {
    use super::*;

    async fn _handler() -> &'static str {
        "Hello world"
    }

    #[test]
    fn route_macro(){
        let _app: App<(), ()> = App::new(());
        use Method::{Get, Post};
        let _app = route!(_app, ("/",_handler, Get), ("/",_handler, Post));
    }
}