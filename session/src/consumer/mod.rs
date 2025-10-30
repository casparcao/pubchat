use deadpool::Runtime;
use deadpool_lapin::Config;
use anyhow::Result;
use lapin::options::QueueDeclareOptions;

mod message;

pub async fn init()  -> Result<()>{
    let connstring = dotenv::var("RABBITMQ_ADDR").expect("请设置RabbitMQ连接地址RABBITMQ_ADDR");
    let queue_name = dotenv::var("RABBITMQ_QUEUE_NAME").expect("请设置RabbitMQ队列名称RABBITMQ_QUEUE_NAME");
    print!("RabbitMQ连接地址: {}", connstring);
    let mut cfg = Config::default();
    cfg.url = Some(connstring.into());
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    let channel = pool.get().await?.create_channel().await?;
    let _queue = channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?; 
    print!("RabbitMQ队列名称: {}", queue_name);
    // Start RabbitMQ consumer in a separate task
    // let message_service_clone = message_service.clone();
    tokio::spawn(async move {
        if let Err(e) = message::consume_messages(channel, &queue_name).await {
            eprintln!("Error consuming messages: {}", e);
        }
    });
    Ok(())
}
