use std::collections::HashMap;

//pub fn print(scaler_totals: HashMap<(u32, usize), u32>) {
//    let mut scaler_totals = scaler_totals.into_iter().collect::<Vec<_>>();
//    scaler_totals.sort_by_key(|((_, x), _)| *x);
//    scaler_totals.sort_by_key(|((x, _), _)| *x);
//
//    for (k, v) in scaler_totals {
//        println!("{:?}: {}", k, v);
//    }
//}

pub fn print(scaler_totals: HashMap<(u32, usize), u32>) {
    let total: u32 = scaler_totals.into_iter().map(|(_k, v)| v).sum();

    println!("{}", total);
}
