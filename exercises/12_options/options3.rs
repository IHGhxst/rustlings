// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// on set p en tant que ref et on change _ par None pour handle la situation de panic, enfin on ajoute le drop pour éviter les erreurs de compilations

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        None => panic!("no match!"),
    }
    drop(y); // Fix without deleting this line.
}
