use std::collections::HashSet;

const SUBSTITUTION: [u8; 256] = [
    167, 97, 100, 212, 185, 16, 128, 181, 98, 191, 160, 203, 116, 194, 29, 179, 124, 172, 82, 71,
    13, 3, 50, 102, 90, 202, 119, 51, 63, 5, 86, 176, 64, 180, 210, 28, 227, 107, 69, 222, 171,
    137, 36, 153, 217, 45, 156, 4, 229, 46, 78, 26, 61, 37, 10, 238, 62, 32, 183, 248, 166, 43,
    134, 87, 99, 214, 211, 109, 254, 83, 60, 34, 12, 24, 81, 49, 123, 162, 155, 75, 38, 249, 129,
    88, 205, 73, 25, 94, 215, 120, 225, 193, 135, 186, 67, 242, 163, 221, 170, 255, 187, 204, 17,
    237, 117, 92, 2, 144, 11, 189, 244, 20, 224, 57, 142, 138, 219, 101, 70, 74, 247, 122, 197,
    108, 113, 209, 165, 77, 6, 58, 140, 53, 169, 95, 150, 200, 131, 226, 110, 1, 230, 159, 245,
    127, 177, 239, 228, 196, 146, 152, 41, 241, 85, 65, 157, 44, 126, 121, 190, 232, 27, 130, 168,
    182, 68, 0, 48, 253, 76, 19, 173, 136, 133, 188, 80, 56, 105, 103, 125, 206, 233, 18, 158, 7,
    84, 30, 72, 33, 240, 54, 234, 47, 89, 154, 114, 112, 79, 115, 246, 207, 195, 141, 218, 148,
    178, 35, 42, 223, 252, 145, 216, 220, 201, 199, 208, 66, 147, 192, 231, 106, 236, 250, 243, 31,
    104, 118, 55, 132, 15, 251, 9, 235, 21, 184, 151, 111, 52, 161, 198, 40, 139, 175, 8, 213, 59,
    22, 143, 91, 174, 96, 93, 149, 14, 39, 23, 164,
];

fn substitute(mut input: [u8; 6]) -> [u8; 6] {
    for i in input.iter_mut() {
        *i = SUBSTITUTION[*i as usize];
    }
    input
}

fn mix(mut input: [u8; 6]) -> [u8; 6] {
    for i in 0..6 {
        input[i] = input[i].wrapping_add(input[(i + 1) % 6]).wrapping_add(42);
    }
    input
}

fn hash(mut input: [u8; 6]) -> [u8; 6] {
    for _ in 0..420 {
        input = substitute(input);
        input = mix(input);
        input = substitute(input);
        input = mix(input);
        input = mix(input);
    }

    input
}

fn main() {
    let mut set: HashSet<[u8; 6]> = HashSet::new();
    let mut collisions = 0;
    let solution = [74, 139, 62, 163, 189, 180];
    for i in 0..0x1000000 {
        let string = format!("{:6x}", i);
        let hash = hash(string.as_bytes().try_into().unwrap());
        if !(set.insert(hash)) {
            collisions += 1;
        }
        if i % 0x100000 == 0 {
            println!("{} {:?}", i, hash);
        }
        if solution == hash {
            println!("solution: {}", string);
        }
    }
    println!("collisions: {}", collisions);
}
