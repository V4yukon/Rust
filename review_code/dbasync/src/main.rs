use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]

async fn main() {
    let db: Vec<u32> = vec![1, 4, 7, 2, 5, 8];
    
    let arc_db = Arc::new(Mutex::new(db));
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();



    let task_a = tokio::task::spawn(async move {
        let mut db = arc_db.lock().await;
        db[4] = 50;
        assert_eq!(db[4], 50);
    });

    let task_b = tokio::task::spawn(async move{
        let mut db = arc_db2.lock().await;
        db[4] = 100;
        assert_eq!(db[4], 100);
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();


    println!("{:?}", arc_db3.lock().await);


}
