use slack_hook2::{PayloadBuilder, Slack};

#[tokio::main]
async fn main() {
  // futures::join!(notify_to_slack());
  // futures::executor::block_on(notify_to_slack());
  notify_to_slack().await;
}

async fn notify_to_slack() {
  let url = "https://hooks.slack.com/services/H8Y348RH/JOA89W4UJ/657t7878H&H9J";

  let slack = Slack::new(url).unwrap();
  let p = PayloadBuilder::new()
    .text("Hello World!!")
    .channel("#notify")
    .username("rust-slack")
    .icon_emoji(":rocket:")
    .build()
    .unwrap();

  let res = slack.send(&p).await;
  match res {
    Ok(()) => println!("ok"),
    Err(x) => println!("ERR: {:?}", x),
  }
}
