use dotenv::dotenv;
use github_flows::{listen_to_event, EventPayload, GithubLogin::Default};
use slack_flows::send_message_to_channel;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());

    listen_to_event(
        &Default,
        &github_owner,
        &github_repo,
        vec!["fork"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        let name = forkee.owner.unwrap().login;
        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");

        let text = format!("{name} forked your repo to {html_url}\n{time}");
        send_message_to_channel(&slack_workspace, &slack_channel, text);
    }
}
