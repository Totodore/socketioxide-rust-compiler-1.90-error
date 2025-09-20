use socketioxide::{SocketIo, handler::ConnectHandler};

fn main() {}

async fn fails() {
    let (_svc, io) = SocketIo::new_svc();

    #[derive(Debug)]
    struct MyError;

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    let handler = || async move || Err(MyError);

    io.ns(
        "/",
        { async || {} }
            .with(handler())
            .with(handler())
            .with(handler()),
    );
}

async fn works_1() {
    let (_svc, io) = SocketIo::new_svc();

    #[derive(Debug)]
    struct MyError;

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    let handler = || async move || Err(MyError);

    io.ns("/", { async || {} }.with(handler()).with(handler()));
}

async fn works_2() {
    let (_svc, io) = SocketIo::new_svc();

    let handler = || async move || Err("");

    io.ns(
        "/",
        { async || {} }
            .with(handler())
            .with(handler())
            .with(handler()),
    );
}
