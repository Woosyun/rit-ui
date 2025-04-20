mod itest;
use itest::*;

#[test]
pub fn initialize_repository() -> rit::Result<()> {
    let user = Client::build("initialize-repository")?;
    user.init()
}

#[test]
pub fn status_after_initialize_repository() -> rit::Result<()> {
    let client = Client::build("status-before-initialize-repository")?;
    client.init()?;

    client.try_status()
}

#[test]
pub fn status_after_first_work() -> rit::Result<()> {
    let mut client = Client::build("status-after-first-work")?;
    client.init()?;

    client.work()?;
    client.try_status()
}
#[test]
pub fn commit_once() -> rit::Result<()> {
    let mut client = Client::build("status-after-first-work")?;
    client.init()?;

    client.work()?;
    client.try_commit()
}
#[test]
pub fn status_after_first_commit() -> rit::Result<()> {
    let mut client = Client::build("status-after-first-commit")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;
    client.try_status()
}

#[test]
pub fn status_after_second_work() -> rit::Result<()> {
    let mut client = Client::build("status-after-second-work")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;

    client.work()?;
    client.try_status()
}
#[test]
pub fn commit_twice() -> rit::Result<()> {
    let mut client = Client::build("commit-twice")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;

    client.work()?;
    client.try_commit()
}
#[test]
pub fn status_after_second_commit() -> rit::Result<()> {
    let mut client = Client::build("status-after-second-commit")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;

    client.work()?;
    client.try_commit()?;
    client.try_status()
}
