pub async fn connect(rabbit_url: &String) -> lapin::Connection {
    let future_connection = lapin::Connection::connect(
        &rabbit_url,
        lapin::ConnectionProperties::default(),
    );

    let connection_result = future_connection.await;

    return match connection_result {
        Ok(connection) => connection,
        Err(error) => panic!("Error: {:?}", error),
    };
}