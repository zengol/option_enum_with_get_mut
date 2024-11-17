// TODO:
// 1) Safely access the first account in the 'accounts' vector using the 
//    .first_mut() method. 
// 2) '.first_mut()' returns an Option whose Some variant is a mutable ref to 
//     an Account. Use a 'match' statement to figure out if
//     you have a Some or a None
// 3) In the Some case, set the balance of the account to 30, then print the account
// 4) In the None case, print the message "No account found"
// Hint: You might have to add in the 'mut' keyword somewhere...

#[derive(Debug)]
struct Account {
    balance: i32
}

fn main() {
    let mut accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 }
    ];
    // Aprendimos y utilizamos un nuevo metodo llamado get_mut() que retorna un ref mut a un elemento o subslice dependiendo
    // tipo de indice o None si el indice esta fuera de los límites. 
    let access_secure = accounts.get_mut(67);
    // Add code here:
    match access_secure {
        Some (account) => {
            //account.balance = 30;
            println!("{:#?}", account);
        }
        None => {
            println!("No account found!");
        }
    }
}







