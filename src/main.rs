use crate::util::ISolution;
use std::collections::HashMap;

mod gift_shop;
mod lobby;
mod secret_entrance;
mod printing_dept;
mod util;

fn get_all_solutions() -> HashMap<String, Box<dyn ISolution>> {
    let mut map: HashMap<String, Box<dyn ISolution>> = HashMap::new();
    add_fn("25.1", secret_entrance::SecretEntrance, &mut map);
    add_fn("25.2", gift_shop::GiftShop, &mut map);
    add_fn("25.3", lobby::Lobby, &mut map);
    add_fn("25.4", printing_dept::PrintingDepartment, &mut map);
    map
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        exit();
    }

    let arg1 = args[1].as_str();
    let all_solutions = get_all_solutions();
    let ids = expand_ids(arg1, all_solutions.keys().map(|k| k.as_str()).collect());
    if ids.len() == 0 {
        exit();
    }

    let all_solutions = get_all_solutions();
    let mut this_solutions = Vec::new();
    for id in ids.into_iter() {
        if let Some(solution_fn) = all_solutions.get(&id) {
            let solution = solution_fn.solve();
            this_solutions.push((id, solution));
        }
    }

    if this_solutions.len() == 0 {
        exit();
    }

    // TODO: Sort, Table widths, Progress
    println!("{:-<24}", "");
    println!("{:^24}", "Solutions");
    println!("{:-<24}", "");
    println!("  ID | Part 1 | Part 2");
    println!("{:-<5}|{:-^8}|{:-^8}", "", "", "");
    for solution in this_solutions {
        println!(
            "{:<5}|{:^8}|{:^8}",
            solution.0, solution.1.part_one, solution.1.part_two
        );
    }
}

fn add_fn<T>(id: &str, s: T, map: &mut HashMap<String, Box<dyn ISolution>>)
where
    T: ISolution + 'static,
{
    map.insert(id.to_string(), Box::new(s));
}

fn expand_ids(arg1: &str, all_ids: Vec<&str>) -> Vec<String> {
    if let Some(_) = arg1.split_once(".") {
        return vec![arg1.to_string()];
    };

    return all_ids
        .iter()
        .filter(|i| i.starts_with(arg1))
        .map(|i| i.to_string())
        .collect();
}

fn exit() -> ! {
    println!("arg should be [YY].[1], [YY].[2] or [YY], eg: 25 or 25.2");
    std::process::exit(1)
}
