pub fn float_to_data(fl: f32) -> [u8; 4] {
    let mut data = (fl).to_be_bytes();
    data.reverse();
    return data;
    
}

pub fn combine_data(data1: [u8; 4], data2: [u8; 4]) -> [u8; 8] {
    let mut comb_data = [0; 8];

    let (left, right) = comb_data.split_at_mut(data1.len());

    left.copy_from_slice(&data1);
    right.copy_from_slice(&data2);

    return comb_data;
    
}