use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: Mutex<u32>,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: Mutex::new(0) }
    }

    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
}

fn main() {
    let counter = Arc::new(Counter::new());

    // Создаем несколько потоков для инкрементации счетчика
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1_000_000 {
                    counter.increment();
                }
            })
        })
        .collect();

    // Ожидаем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Выводим итоговое значение
    println!("Final count: {}", counter.count.lock().unwrap());
}