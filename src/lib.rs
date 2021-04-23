//! This is a lightweight crate for verifying NUBAN numbers
//! for all Nigerian bank accounts as was directed by the CBN.

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Nuban {
    bank_code: String,
    account_number: String,
}

impl Nuban {
    pub fn new(bank_code: &str, account_number: &str) -> Result<Self, &'static str> {

        if bank_code.len() != 3 || account_number.len() != 10 {
            return Err("Validation Error: invalid bank code or account number");
        }

        Ok(Nuban {
            bank_code: bank_code.to_string(),
            account_number: account_number.to_string(),
        })
    }

    pub fn get_bank_name(&self) -> Result<&str, &str> {
        let banks = Self::banks();
        let bank_name = banks.get(self.bank_code());
        match bank_name {
            Some(_name) => Ok(bank_name.unwrap()),
            None => Err("Bank not found."),
        }
    }

    pub fn is_valid(&self) -> bool {
        let check_digit = self.account_number.chars().last().unwrap();
        let check_digit = check_digit.to_digit(10).unwrap() as u8;
        self.calculate_check_digit() == check_digit
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    pub fn bank_code(&self) -> &str {
        &self.bank_code
    }

    pub fn calculate_check_digit(&self) -> u8 {
        // The Approved NUBAN format: [ABC][DEFGHIJKL][M], where
        //   -       ABC : 3-digit Bank Code
        //   - DEFGHIJKL : NUBAN Account Serial Number
        //   -         M : NUBAN Check Digit
        // https://www.cbn.gov.ng/OUT/2011/CIRCULARS/BSPD/NUBAN%20PROPOSALS%20V%200%204-%2003%2009%202010.PDF
        // let numbers = format!("{}{}", , &self.account_number[..9]);

        let bank_code = self.bank_code.chars();
        let account_number = self.account_number.chars().take(9);
        let nuban_chars = bank_code.chain(account_number);
        let nuban_digits = nuban_chars.map(|num| num.to_digit(10).unwrap());
        let seed = [3, 7, 3, 3, 7, 3, 3, 7, 3, 3, 7, 3].iter();
        let check_sum: u32 = seed.zip(nuban_digits).map(|(l, r)| l * r).sum();
        match 10 - (check_sum % 10) as u8 {
            10 => 0,
            x => x,
        }
    }

    pub fn banks() -> HashMap<&'static str, &'static str> {
        [
            ("044", "Access Bank"),
            ("014", "Afribank"),
            ("023", "Citibank"),
            ("063", "Diamond Bank"),
            ("050", "Ecobank"),
            ("040", "Equitorial Trust Bank"),
            ("011", "First Bank"),
            ("214", "FCMB"),
            ("070", "Fidelity"),
            ("085", "FinBank"),
            ("058", "Guaranty Trust Bank"),
            ("069", "Intercontinentl Bank"),
            ("056", "Oceanic Bank"),
            ("082", "BankPhb"),
            ("076", "Skye Bank"),
            ("084", "SpringBank"),
            ("221", "StanbicIBTC"),
            ("068", "Standard Chartered Bank"),
            ("232", "Sterling Bank"),
            ("033", "United Bank For Africa"),
            ("032", "Union Bank"),
            ("035", "Wema Bank"),
            ("057", "Zenith Bank"),
            ("215", "Unity Bank"),
        ]
        .iter()
        .copied()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_returns_new_nuban_instance() {
        let account = Nuban::new("058", "0982736625");
        assert_eq!(
            account.unwrap(),
            Nuban {
                bank_code: String::from("058"),
                account_number: String::from("0982736625")
            }
        );
    }

    #[test]
    fn test_returns_false_for_invalid_account() {
        let account = Nuban::new("058", "0982736625").unwrap();
        assert!(!account.is_valid());
    }

    #[test]
    fn test_returns_true_for_valid_account() {
        let account = Nuban::new("058", "0152792740").unwrap();
        assert!(account.is_valid());
    }

    #[test]
    fn test_calculate_check_digit() {
        let account = Nuban::new("058", "0152792740").unwrap();
        let correct_check_digit = account.calculate_check_digit();
        assert_eq!(correct_check_digit, 0);
    }

    #[test]
    fn test_get_bank_name() {
        let account = Nuban::new("058", "0152792740").unwrap();
        assert_eq!(account.get_bank_name().unwrap(), String::from("Guaranty Trust Bank"));
    }
}
