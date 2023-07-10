use newswrap::{
    client::HackerNewsClient, errors::HackerNewsClientError, items::stories::HackerNewsStory,
};

#[tokio::test]
async fn return_ok_when_item_is_valid() {
    // arrange
    let client = HackerNewsClient::new();
    let story_id = 8863_u32;

    // act
    let item_result = client.items.get_item(story_id).await;
    assert!(item_result.is_ok());

    // assert
    let item = item_result.unwrap();
    assert!(item.is_story());
    let story: Result<HackerNewsStory, HackerNewsClientError> = item.try_into();
    assert!(story.is_ok());
}

#[tokio::test]
async fn return_ok_when_using_typed_clients() {
    // arrange
    let client = HackerNewsClient::new();
    let story_id = 8863_u32;

    // act
    let item_result = client.items.get_story(story_id).await;

    // assert
    assert!(item_result.is_ok());
}

#[tokio::test]
async fn return_err_on_invalid_typed_client() {
    // arrange
    let client = HackerNewsClient::new();
    let not_a_poll_id = 69_u32;

    // act
    let item_result = client.items.get_poll(not_a_poll_id).await;

    // assert
    assert!(item_result.is_err());
}
