#[derive(Debug)]
#[allow(dead_code)]
enum Test {
    A,
    B,
    C,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Computer {
    Ram(u8),
    CpuCores(u8),
}

fn main() {
    /*let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximu is configured to be {}", max),
        _ => (),
    }*/

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Do the thing!");
    }

    let ttest = Test::A;
    if let Test::A = ttest {
        println!("The test {:?} ...", ttest);
    }

    let computer = Computer::Ram(32);
    if let Computer::Ram(size) = computer {
        println!("{:?}GB", size);
    }
}
