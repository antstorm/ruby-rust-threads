#[macro_use]
extern crate rutie;

use rutie::{Class, Object, NilClass, Thread};
use std::sync::mpsc;

class!(RustRunner);

methods!(
    RustRunner,
    _rtself,

    fn run() -> NilClass {
        let (tx1, rx1): (mpsc::SyncSender<u8>, mpsc::Receiver<u8>) = mpsc::sync_channel(1);
        let (tx2, rx2): (mpsc::SyncSender<u8>, mpsc::Receiver<u8>) = mpsc::sync_channel(1);

        // Spawn a Ruby consumer thread
        Thread::new(move || {
            println!("Entering Ruby thread...");
            while let Ok(_msg) = Thread::call_without_gvl(|| { rx1.recv() }, Some(|| {})) {
                println!("Ruby thread got a message");
            }

            println!("Ruby thread finished");

            NilClass::new()
        });

        // Spawn a Rust consumer thread
        std::thread::spawn(move || {
            println!("Entering Rust thread...");

            while let Ok(_msg) = rx2.recv() {
                println!("Rust thread got a message");
            }

            println!("Rust thread finished");
        });

        // Spawn a Rust producer thread
        std::thread::spawn(move || {
            let duration = std::time::Duration::from_millis(5000);

            loop {
                tx1.send(1).expect("Unable to send to a Ruby thread");
                tx2.send(2).expect("Unable to send to a Rust thread");
                std::thread::sleep(duration);
            }
        });

        NilClass::new()
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn init_rust() {
    Class::new("RustRunner", None).define(|klass| {
        klass.def_self("run", run);
    });
}
