// from [Condition variable not playing well with thread::sleep](http://stackoverflow.com/questions/42353366/condition-variable-not-playing-well-with-threadsleep)

use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::time;

#[derive(PartialEq, Debug)]
enum Command {
    Idle,
    ReadRegister(u32),
}

fn poll_thread(sync_pair: Arc<(Mutex<Command>, Condvar)>) {
    let &(ref mutex, ref cvar) = &*sync_pair;

    loop {
        let mut flag = mutex.lock().unwrap();
        while *flag == Command::Idle {
            flag = cvar.wait(flag).unwrap();
        }

        match *flag {
            Command::Idle => {
                println!("WHAT IMPOSSIBLE!");
                panic!();
            }
            Command::ReadRegister(i) => {
                println!("You want me to read {}?", i);
                thread::sleep(time::Duration::from_millis(450));
                println!("Ok, here it is: {}", 42);
            }
        }
    }
}

pub fn main() {
    let pair = Arc::new((Mutex::new(Command::Idle), Condvar::new()));
    let pclone = pair.clone();
    let rx_thread = thread::spawn(|| poll_thread(pclone));

    let &(ref mutex, ref cvar) = &*pair;

    for i in 0..10 {
        thread::sleep(time::Duration::from_millis(500));
        if i == 4 {
            println!("Setting ReadRegister");
            let mut flag = mutex.lock().unwrap();
            *flag = Command::ReadRegister(5);
            println!("flag is = {:?}", *flag);
            cvar.notify_one();
        } else if i == 8 {
            println!("Setting Idle");
            let mut flag = mutex.lock().unwrap();
            *flag = Command::Idle;
            println!("flag is = {:?}", *flag);
            cvar.notify_one();
        }
    }
    println!("after notify_one()");

    rx_thread.join();
}

