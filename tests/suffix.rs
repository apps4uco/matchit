use matchit::{InsertError, MatchError, Router};

fn router()->Result<Router<&'static str>,InsertError> {
    let mut router = Router::new();
    router.insert("/home", "Welcome!")?;
    router.insert("/users/:id", "A User")?;
    router.insert("/users/:id.png", "A User Photo Currently not working")?;
    router.insert("/users/:id/x.png", "Another User Photo that works")?;
    Ok(router)
}

#[test]
fn match_img()->Result<(),MatchError>{

    let router=router().unwrap();
    let matched = router.at("/users/978")?;
    assert_eq!(matched.params.get("id"), Some("978"));
    assert_eq!(*matched.value, "A User");
    Ok(())
}