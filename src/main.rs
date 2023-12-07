use core_affinity;
use std::thread;
use num_cpus;

fn main() {

    println!("available cpus : {}", num_cpus::get());
    println!("physical  cpus : {}", num_cpus::get_physical());

    let core_ids = core_affinity::get_core_ids().unwrap();
    println!("pinnable count : {}\npinnable ids   : {:?}\n\n", core_ids.len(), core_ids.iter().map(|id| id.id).collect::<Vec<_>>());


    let handles = core_ids.into_iter().map(|id| {
        thread::spawn(move || {
            // Pin this thread to a single CPU core.
            let res = core_affinity::set_for_current(id);
            println!("core id: {}, pinned: {}", id.id, res);
            if res {
              // Do more work after this.
              println!("pinned: {}", res);
            }
        })
    }).collect::<Vec<_>>();

    // println!("handles: {:?}", handles);
    thread::sleep(std::time::Duration::from_secs(2)); 

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    println!("\n\njoined");
}

