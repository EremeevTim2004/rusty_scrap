use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(request::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    
    let res = request::get(
            "https://www.rust-lang.org/en-US/"
        )
        .await()?
        .text()
        .await()?;

    println!("links on web page 'rust-lang.com' : ");

    Document::from(res::as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
