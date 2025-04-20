mod itest;
use itest::*;

#[test]
pub fn checkout_when_workspace_is_not_clean() -> rit::Result<()> {
    let mut client = Client::build("checkout-when-workspace-is-not-clean")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;
    client.try_branch_create("new_branch")?;
    
    client.work()?;
    client.try_checkout("new_branch")
}

#[test]
pub fn checkout_from_clean_workspace() -> rit::Result<()> {
    let mut client = Client::build("checkout-from-clean-workspace")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;
    
    client.try_branch_create("new_branch")?;
    client.try_checkout("new_branch")?;

    client.work()?;
    client.try_commit()?;
    client.try_checkout("main")
}

#[test]
pub fn checkout_while_working() -> rit::Result<()> {
    let mut client = Client::build("checkout-while-working")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;

    client.work()?;
    client.try_branch_create("new_branch")?;
    client.try_checkout("new_branch")?;
    client.try_commit()
}
