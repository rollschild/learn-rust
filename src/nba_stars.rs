pub fn nba_stars() {
    println!("Here is the list of stars that I like in NBA:");
    let stars = ["kobe", "kawhi", "jimmy butler"];
    for star in stars.iter() {
        println!("{}", &star); // & borrows for read-only access
    }
}
