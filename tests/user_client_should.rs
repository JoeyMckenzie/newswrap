use newswrap::client::HackerNewsClient;

#[tokio::test]
async fn return_ok_when_item_is_valid() {
    // arrange
    let client = HackerNewsClient::new();
    let username = "joeymckenzie";

    // act
    let user_result = client.users.get_user(username).await;
    assert!(user_result.is_ok());

    // assert
    let user = user_result.unwrap();
    assert!(user.has_about_section());
    assert_eq!(user.about.unwrap(), "I like computers and rust.");
}

#[tokio::test]
async fn return_error_when_user_is_not_found() {
    // arrange
    let client = HackerNewsClient::new();
    let username = "joeymckenzie-does-not-exist";

    // act
    let user_result = client.users.get_user(username).await;

    // assert
    assert!(user_result.is_err());
}
