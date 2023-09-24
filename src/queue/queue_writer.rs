use lapin::{types::ShortString, types::AMQPValue};

pub async fn add_to_queue(message: &Vec<Vec<Vec<u8>>>, queue: &str, connection: &lapin::Connection) {
    let channel_result = connection.create_channel().await;

    let channel = match channel_result {
        Ok(channel) => channel,
        Err(error) => panic!("Error: {:?}", error),
    };

    channel
        .queue_declare(
            queue,
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .unwrap();
    
    let message = message.clone();

    let mut file_counter: i32 = 0;

    for file in message {
        file_counter += 1;

        let mut chunk_counter: i32 = 0;

        for chunk in file {
            chunk_counter += 1;

            let mut default_table = lapin::types::FieldTable::default();

            default_table.insert(ShortString::from("file"), AMQPValue::LongInt(file_counter));
            default_table.insert(ShortString::from("chunk"), AMQPValue::LongInt(chunk_counter));

            channel
                .basic_publish(
                    "",
                    queue,
                    lapin::options::BasicPublishOptions::default(),
                    chunk.as_slice(),
                    lapin::BasicProperties::default().with_headers(default_table),
                )
                .await
                .unwrap();
        }
    }

    connection.status();
}