pub fn euler1() -> i32 {
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let mut sum: usize = 0;
    const limit: usize = 1000;
    const size_5: usize = limit / 5;
    let array_5: [usize; size_5] = core::array::from_fn(|i| i * 5);
    const size_3: usize = limit / 3;
    let array_3: [usize; size_3] = core::array::from_fn(|i| i * 3);

    println!("{:?}", array_5);
    println!("{:?}", array_3);

    for i in 0..size_5 {
        sum += array_5[i];
    }
    for i in 0..size_3 {
        sum += array_3[i];
    }
    println!("First sum: {}", sum);

    let mut sum: i32 = 0;
    // Add all multiples of 3
    let mut n = 0;
    while 3 * n < 1000 {
        sum += 3 * n;
        n+=1;
    }

    // Add all multiples of 5
    let mut n = 0;
    while 5 * n < 1000 {
        sum += 5 * n;
        n+=1;
    }

    sum
}
