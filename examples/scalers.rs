use memmap::Mmap;
use nscl_evt::NsclData;
use std::{collections::HashMap, fs::File};

fn main() {
    let mut scaler_totals = HashMap::new();

    // Make sure all files are there before starting
    let files = std::env::args()
        .skip(1)
        .map(|a| File::open(a).unwrap())
        .collect::<Vec<_>>();

    for f in files {
        let m = unsafe { Mmap::map(&f) }.unwrap();
        let d = NsclData::new(&m);
        for e in d.filter(|e| e.type_id() == 20) {
            let source_id = e.body_header().source_id().unwrap();
            let scalers = e.ring_item().as_periodic_scalers().unwrap().scalers();

            for (i, s) in scalers.into_iter().enumerate() {
                *scaler_totals.entry((source_id, i)).or_insert(0) += s;
            }
        }
    }

    let mut scaler_totals = scaler_totals.into_iter().collect::<Vec<_>>();
    scaler_totals.sort_by_key(|((_, x), _)| *x);
    scaler_totals.sort_by_key(|((x, _), _)| *x);

    for (k, v) in scaler_totals {
        println!("{:?}: {}", k, v);
    }
}
