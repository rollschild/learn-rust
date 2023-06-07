fn nba_stars() {
    println!("Here is the list of stars that I like in NBA:");
    let stars = ["kobe", "kawhi", "jimmy butler"];
    for star in stars.iter() {
        println!("{}", &star); // & borrows for read-only access
    }
}

fn extract_csv() {
    let player_data = "\
    player name,height (cm)
    Kobe Bryant,198
    Kawhi Leonard,201
    Jimmy Butler,201
    ";

    let records = player_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // Vec<_> - infer the type of elements
        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();

        // cfg! checks configuration at compile time
        if cfg!(debug_assertions) {
            // prints to stderr
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        //                             inline type annotation
        if let Ok(height) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, height);
        }
    }
}
fn main() {
    // nba_stars();
    extract_csv();
}
