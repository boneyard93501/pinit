use core_affinity;
use std::thread;
use num_cpus;

fn main() {

    println!("available:      {}", num_cpus::get());
    println!("physical:       {}", num_cpus::get_physical());

    let core_ids = core_affinity::get_core_ids().unwrap();
    println!("pinnable count: {}\nids: {:?}", core_ids.len(), core_ids);


    let handles = core_ids.into_iter().map(|id| {
        thread::spawn(move || {
            println!("id: {:?}", id);
            // Pin this thread to a single CPU core.
            let res = core_affinity::set_for_current(id);
            println!("res: {}", res);
            if res {
              // Do more work after this.
              println!("pinned: {}", res);
            }
        })
    }).collect::<Vec<_>>();

    println!("handles: {:?}", handles);
    /*
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
     */
}

