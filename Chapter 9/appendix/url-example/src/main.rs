extern crate url;

use url::Url;

fn main() {
    let url = Url::parse("git+https://foo:bar@gitlab.com/gitlab-org/gitlab-ce/blob/master/config/routes/development.rb#L8").unwrap();
    println!("Scheme: {}", url.scheme());
    println!("Username: {}", url.username());
    println!("Password: {}", url.password().unwrap());
    println!("Fragment: {}", url.fragment().unwrap());
    println!("Host: {:?}", url.host().unwrap());
}
