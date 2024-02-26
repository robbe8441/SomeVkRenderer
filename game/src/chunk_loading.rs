use super::*;

#[system]
pub fn do_something(
    #[state] vec: &mut Vec<thread::JoinHandle<f64>>
) {
    use noise::{NoiseFn, SuperSimplex};

    // max the limit of threads to 6 to it wont lag that much
    // TODO probalbly need to adjust it to the users threads
    if vec.len() < 6 {
        let handle = thread::spawn(move || {
            let mut val = 0.0;
            let noise = SuperSimplex::new(0);

            for _ in 0..3000000 {
                val += noise.get([0.0, 1.0, val]);
            }
            val
        });

        vec.push(handle);
    }

    let mut num = 0;

    for i in (0..vec.len()).rev() {
        let handle = &vec[i];
        if num >= 3 {
            return;
        }

        if handle.is_finished() {
            let val = match vec.remove(i).join() {
                Ok(r) => r,
                _ => {
                    return;
                }
            };
            num += 1;
            println!("GIGACHAD :  {}", val);
        }
    }
}


