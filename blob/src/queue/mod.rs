use deadpool::Runtime;
use deadpool_lapin::Config;
use anyhow::Result;
use lapin::options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};

mod consumer;

pub async fn init()  -> Result<()>{
    let connstring = dotenv::var("RABBITMQ_ADDR").expect("请设置RabbitMQ连接地址RABBITMQ_ADDR");
    let session_message_queue_name = dotenv::var("RABBITMQ_SESSION_MESSAGE_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_SESSION_MESSAGE_QUEUE_NAME");
    let exc_name = dotenv::var("RABBITMQ_MESSAGE_EXCHANGE").expect("请设置RabbitMQ交换机名称RABBITMQ_MESSAGE_EXCHANGE");
    print!("RabbitMQ连接地址: {}", connstring);
    let mut cfg = Config::default();
    cfg.url = Some(connstring.into());
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    let channel = pool.get().await?.create_channel().await?;
    let _queue = channel
            .queue_declare(
                &session_message_queue_name,
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?; 
    let _exchange = channel
            .exchange_declare(
                &exc_name,
                lapin::ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                Default::default(),
            )
            .await?;
    channel
            .queue_bind(
                &session_message_queue_name, 
                &exc_name, 
                "#", 
                QueueBindOptions::default(), 
                Default::default()).await?;
    print!("RabbitMQ队列名称: {}", session_message_queue_name);
    // Start RabbitMQ consumer in a separate task
    // let message_service_clone = message_service.clone();
    tokio::spawn(async move {
        if let Err(e) = consumer::consume_messages(channel, &session_message_queue_name).await {
            log::error!("Error consuming messages: {}", e);
        }
    });
    Ok(())
}

