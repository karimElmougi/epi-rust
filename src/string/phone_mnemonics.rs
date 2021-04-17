pub fn mnemonics(phone_number: &str) -> Vec<String> {
    let mut mnemonics = mnemonics_rec(phone_number);
    for mnemonic in mnemonics.iter_mut() {
        unsafe { mnemonic.as_bytes_mut() }.reverse();
    }
    mnemonics
}

pub fn mnemonics_rec(phone_number: &str) -> Vec<String> {
    static MAP: [&[char]; 10] = [
        &[],
        &[],
        &['A', 'B', 'C'],
        &['D', 'E', 'F'],
        &['G', 'H', 'I'],
        &['J', 'K', 'L'],
        &['M', 'N', 'O'],
        &['P', 'Q', 'R', 'S'],
        &['T', 'U', 'V'],
        &['W', 'X', 'Y', 'Z'],
    ];

    if phone_number.is_empty() {
        return vec![];
    }

    let digit = phone_number.chars().next().unwrap().to_digit(10).unwrap() as usize;
    if phone_number.len() == 1 {
        return MAP[digit].iter().map(|c| c.to_string()).collect();
    } else {
        let submnemonics = mnemonics_rec(&phone_number[1..]);

        if MAP[digit].is_empty() {
            return submnemonics;
        }

        let mut mnemonics = vec![];
        for &c in MAP[digit] {
            for mnemonic in submnemonics.iter() {
                let mut mnemonic = mnemonic.clone();
                mnemonic.push(c);
                mnemonics.push(mnemonic);
            }
        }

        mnemonics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let number = "123";
        let result = ["AD", "AE", "AF", "BD", "BE", "BF", "CD", "CE", "CF"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!((mnemonics(number)), result);
    }
}
