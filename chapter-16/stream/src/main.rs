use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("like"),
            String::from("you"),
            String::from("oh")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("good"),
            String::from("hell"),
            String::from("oops")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rev in rx {
        println!("got {}", rev);
    }


}
