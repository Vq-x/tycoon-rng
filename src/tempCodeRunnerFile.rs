println!(
        "one ore: {:?} one ore times total amount: {:?}",
        hand_of_poseidon.process_ores(&mut vec![ores[0].clone()]),
        hand_of_poseidon.process_ores(&mut vec![ores[0].clone()]) * ores.len() as f64
    );