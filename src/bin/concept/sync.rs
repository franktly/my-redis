#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex, MutexGuard, RwLock, Condvar};
use std::thread;
use std::time::Duration;
use tokio::sync::Semaphore;


#[allow(dead_code)]
fn single_thread_mutex()
{
    let m = Mutex::new(3);
    {
        let mut num = m.lock().unwrap();
        *num = 33;
    }
    println!("m = {:?}", m);
}

#[allow(dead_code)]
fn single_thread_mutex_deadlock()
{
    let m = Mutex::new(3);
    let mut num = m.lock().unwrap();
    *num = 33;

    drop(num); // free lock manually

    // Apply for another lock when lock is NOT freed above if not free lock manually 
    // by `drop(num)`
    let mut num2 = m.lock().unwrap();
    *num2 = 11;
    println!("m = {:?}", m);
}

#[allow(dead_code)]
fn multi_thread_mutex()
{
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);

        // create sub-thread and clone & copy owner to sub-thread from parent-thread
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    // joint all threads to finish
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

}

#[allow(dead_code)]
fn single_thread_deadlock()
{
    let data = Mutex::new(0);
    println!("deadlock begins");
    let _d1 = data.lock();
    let _d2 = data.lock();
    println!("deadlock finished");
}

lazy_static!{

    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

#[allow(dead_code)]
fn multi_thread_deadlock()
{
    let mut children = vec![];
    for i_thread in 0..2{
        children.push(thread::spawn(move || {
            for _ in 0..1{
                if i_thread % 2 == 0{
                    let _guard: MutexGuard<i64> = MUTEX1.lock().unwrap();
                    println!("Thread {} lock MUTEX1 and prepare to lock MUTEX2", i_thread);

                    // current thread sleep a while for another thread to lock MUTEX2
                    thread::sleep(Duration::from_millis(10));

                    let _guard = MUTEX2.lock().unwrap();
                    // let _guard = MUTEX2.try_lock().unwrap();
                    println!("Thread {} lock MUTEX2 result", i_thread);
                }
                else{
                    let _guard = MUTEX2.lock().unwrap();

                    println!("Thread {} lock MUTEX2 and prepare to lock MUTEX1", i_thread);

                    // current thread sleep a while for another thread to lock MUTEX2
                    thread::sleep(Duration::from_millis(10));

                    let _guard = MUTEX1.lock().unwrap();
                    // let _guard = MUTEX1.try_lock().unwrap();
                    println!("Thread {} lock MUTEX1 result", i_thread);
                }
            }
        }));
    }

    for child in children{
        let _ = child.join();
    }
    println!("Deadlock NOT happened!!!");
}

#[allow(dead_code)]
fn rw_lock()
{
    let  lock = RwLock::new(3);

    // allow multi-read at the same time
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 3);
        assert_eq!(*r2, 3);
    } // read lock dropped automatically here

    // allow only-one write at the same time
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 4);

        // below code would panic for read & write exist at the same time
        /*
         * let r1 = lock.read();
         * println!("{:?}", r1);
         */
    } // write lock dropped automatically here

}

#[allow(dead_code)]
fn cond_var()
{
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let clone_flag = flag.clone();
    let clone_cond = cond.clone();

    // thread 1
    // wait condvar and reset flag and clone flag to false
    let handle = thread::spawn(move || {
        let mut f = *clone_flag.lock().unwrap();
        let mut counter = 0;

        // loop 3 times
        while counter < 3{
            while !f{
                f = * clone_cond.wait(clone_flag.lock().unwrap()).unwrap();
            }

            {
                f = false;
                *clone_flag.lock().unwrap()  = false;
            }

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    // thread 2
    // set flag and clone flag to true and notify condvar to THREAD1
    let mut counter = 0;
    // loop 3 times
    loop{
        thread::sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {break;}
        println!("outer counter: {}", counter);
        cond.notify_one();
    }

    handle.join().unwrap();
    println!("{:?}", flag);
}

/*
 * 
 * fn main()
 * {
 *     // single_thread_mutex();
 *     // single_thread_mutex_deadlock();
 *     // multi_thread_mutex();
 *     // single_thread_deadlock();
 *     // multi_thread_deadlock();
 *     // rw_lock();
 *     // cond_var();
 * }
 * 
 */

#[tokio::main]
async fn main()
{
    let smp = Arc::new(Semaphore::new(3));
    let mut handles = Vec::new();

    for id in 0..5{
        let permit = smp.clone().acquire_owned().await.unwrap();
        handles.push(tokio::spawn(async move {
            thread::sleep(Duration::from_millis(333));
            println!("runing work{:?}", id);
            drop(permit);
        }));
    }

    for handle in handles{
        handle.await.unwrap();
    }
}
