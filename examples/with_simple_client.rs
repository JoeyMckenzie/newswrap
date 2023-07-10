use newswrap::{client::HackerNewsClient, errors::HackerNewsClientError};

#[tokio::main]
async fn main() -> Result<(), HackerNewsClientError> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Optionally build your client with a configured request timeout in seconds or a custom duration
    let _client_with_timeout = HackerNewsClient::new_with_timeout_secs(2);
    let _client_with_duration =
        HackerNewsClient::new_with_timeout_duration(std::time::Duration::from_millis(500));

    // Call various endpoints with your client instance
    let generic_item = client.items.get_item(69).await?;
    dbg!(&generic_item);

    // Determine what the item type is
    let item_type = generic_item.get_item_type();
    dbg!(item_type);

    // Check if the item is job
    assert!(generic_item.is_story());

    // Conveniently request typed items for known IDs
    let story_item = client.items.get_story(69).await?;
    dbg!(&story_item);

    // Get child comments associated to the story
    let comment = client
        .items
        .get_item(*story_item.comments.first().unwrap())
        .await?;
    dbg!(comment);

    // Errors will occur when requesting an incorrectly typed item
    let not_a_story_item = client.items.get_story(192327).await;
    assert!(not_a_story_item.is_err());
    dbg!(not_a_story_item.unwrap_err());

    // Get poll and poll options
    let poll = client.items.get_poll(126809).await?;
    dbg!(&poll);

    let first_poll_option = poll.poll_options.first().unwrap();
    let poll_option = client.items.get_poll_option(*first_poll_option).await?;
    dbg!(poll_option);

    // Retrieve user information
    let user = client.users.get_user("joeymckenzie").await?;
    dbg!(&user);

    if let Some(about_section) = user.about {
        println!("{}", about_section);
    }

    Ok(())
}
