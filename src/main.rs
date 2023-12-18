use std::time::{SystemTime, Duration};
use tqdm::{tqdm, Iter};
use rand::Rng;
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8, FheInt64};
use tfhe::prelude::*;

fn main() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);
    let mut rng = rand::thread_rng();

    let mut enc_numbers = vec![Vec::new(); 50];
    let mut sources = vec![Vec::new(); 50];

    println!("Generating encrypted numbers...");
    for _ in tqdm(0..100) {
        for shift in 0..50 {
            let number: f64 = rng.gen::<f64>() * 127f64;     // 0000...0XXXXXXX
            let number = 128 + number.round() as i64;   // 0000...1XXXXXXX
            let number = number << shift;            // 000..1XX..000000
            
            sources[shift].push(number);
            enc_numbers[shift].push(FheInt64::encrypt(number, &client_key));
        }


    }

    // let clear_a = 27u8;
    // let clear_b = 128u8;

    // let a = FheUint8::encrypt(clear_a, &client_key);
    // let b = FheUint8::encrypt(clear_b, &client_key);

    //Server-side
    set_server_key(server_key);

    let mut times: Vec<Vec<Duration>> = vec![Vec::new(); 50]; // Initialize times with 10 empty vectors


    println!("making zi operations...");
    for _ in 0..10 {
        for (index_of_shift, same_shift_nums) in enc_numbers
            .clone()
            .into_iter()
            .enumerate()
            .tqdm() 
        {
            for num in same_shift_nums.into_iter() {
                let now = SystemTime::now();
                let _ = num + 1;
                let duration  = now.elapsed();


                if times[index_of_shift].is_empty() {
                    times[index_of_shift] = vec![];
                }

                times[index_of_shift].push(duration.unwrap());
            }
        }
    }
    
    for (index_of_shift, same_shift_nums) in sources.into_iter().enumerate() {
        let average_duration = average_durations(&times[index_of_shift]);
        println!("i: {:#?} -> {:#?}", same_shift_nums[0], average_duration);
    }



    // //Client-side
    // let decrypted_result: u8 = result.decrypt(&client_key);

    // let clear_result = clear_a + clear_b;

    // assert_eq!(decrypted_result, clear_result);
}

fn average_durations(durations: &[Duration]) -> Duration {
    let sum: Duration = durations.iter().sum();
    sum / durations.len() as u32
}