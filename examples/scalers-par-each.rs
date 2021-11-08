use memmap::Mmap;
use nscl_evt::NsclData;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    sync::mpsc::{channel, Receiver},
    thread,
};

mod scalers_print;

fn add_map(rx: Receiver<((u32, usize), u32)>) -> HashMap<(u32, usize), u32> {
    let mut total = HashMap::new();
    for (k, v) in rx {
        *total.entry(k).or_insert(0) += v;
    }
    total
}

fn main() {
    // Make sure all files are there before starting
    let files = std::env::args()
        .skip(1)
        .map(|a| File::open(a).unwrap())
        .collect::<Vec<_>>();

    let (tx, rx) = channel();
    let add_handle = thread::spawn(move || add_map(rx));

    for f in files {
        let m = unsafe { Mmap::map(&f) }.unwrap();
        let d = NsclData::new(&m);
        d.filter(|e| e.type_id() == 20)
            .par_bridge()
            .for_each_with(tx.clone(), |tx, e| {
                let source_id = e.body_header().source_id().unwrap();
                let scalers = e.ring_item().as_periodic_scalers().unwrap().scalers();

                for (i, s) in scalers.into_iter().enumerate() {
                    tx.send(((source_id, i), s)).unwrap();
                }
            });
    }

    drop(tx);
    let scaler_totals = add_handle.join().unwrap();
    scalers_print::print(scaler_totals);
}
