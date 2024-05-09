fn main() {
    //let amount: i32 = 500 * 100;
    //println!("Total price is Ksh.{} ", amount);
 

    laptop(100000, 200000);
    book(80, 100);
    phone(50000, 45000);

    
   //different format

    println!("4) Total price of two books is Ksh.{}", price_4(2000));

    price_5(300, 500);
    let bag_3: i32 = price_5(300, 500);
    println!("5) Total price of two bags is Ksh.{}", bag_3);

    }

fn laptop(price_1: i32, price_2: i32) {
    let price_3: i32 = price_1 + price_2;
    println!("1) Total price of laptop is Ksh.{}", price_3);
}

fn book(price_1: i32, price_2: i32) -> i32 {
    let price_3: i32 = price_1 + price_2;
    println!("2) Total price of book is  Ksh.{}", price_3); 
    price_3
}

fn phone(price_1: i32, price_2: i32) -> i32 {
    let price_3: i32 = price_1 + price_2;
    println!("3) Total price of phone is Ksh.{}", price_3);
    return price_3;
}


// different format
fn price_4(book_1: i32) -> i32{
   return 1000 + book_1;
}

fn price_5(bag_1: i32, bag_2: i32) -> i32 {
   return bag_1 + bag_2;
}



















