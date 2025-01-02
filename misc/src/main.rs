fn main() {

    let mut i = 0;
    loop {
        i+=1;
        println!("YO");
        if i > 10 {
            break;
        }
    }

    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(res, 20);

    let mut n = 1;
    while n <= 100 {
        analyze_fb(n);
        n+=1;
    }

    for n in 1..=100 {
        analyze_fb(n);
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("BB"),
            _ => println!("Hello {}", name),
        }
    }

    // match
    let number = 13;
    match number {
        1 => println!("One!"),
        18..28 => println!("cool"), 
        _ => println!("other"),
    }

}

fn analyze_fb(n: i32) {
    let mut hit = false;
    if n % 3 == 0 {
        print!("fizz");
        hit = true;
    }
    if n % 5 == 0 {
        print!("buzz");
        hit = true;
    }
    if hit {
        println!("")
    } else {
        println!("{}", n);
    }
}