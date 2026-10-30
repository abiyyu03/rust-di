use std::io;

fn print_header(is_result: bool) {
    if is_result {
        println!("================================");
        println!("ANALYSIS RESULT");
        println!("================================");
    } else {
        println!("================================");
        println!(" EXPENSE ANALYZER");
        println!("================================");
    }
}

fn analyze_budget() -> io::Result<()> {
    let mut monthly_budget = String::new();
    let mut today_expense = String::new();
    let mut transaction_count = String::new();

    println!("Monthly Budget: ");
    io::stdin().read_line(&mut monthly_budget)?;

    println!("Today Expense: ");
    io::stdin().read_line(&mut today_expense)?;

    println!("Transaction Count: ");
    io::stdin().read_line(&mut transaction_count)?;
    // let stdin_reader = std::io::stdin();

    let monthly_budget_num: i32 = monthly_budget
        .trim()
        .parse()
        .expect("Please type a valid i32 number!");
    let today_expense_num: i32 = today_expense
        .trim()
        .parse()
        .expect("Please type a valid i32 number!");
    let transaction_count_num: i32 = transaction_count
        .trim()
        .parse()
        .expect("Please type a valid i32 number!");

    // Monthly Budget       : Rp3.000.000
    // Today's Expenses     : Rp125.000
    // Transaction Count    : 4
    // Average Transaction  : Rp31.250
    // Budget Usage         : 4.16%
    // Expense Status       : Hemat
    // Remaining Budget     : Rp2.875.000
    let avg_transaction: i32 = today_expense_num / transaction_count_num;
    let budget_usage: f32 = ((today_expense_num as f32 / monthly_budget_num as f32) * 100.0) as f32;
    let remaining_budget: i32 = monthly_budget_num - today_expense_num;

    println!("Monthly Budget: {monthly_budget_num}");
    println!("Today Expense: {today_expense_num}");
    println!("Transaction Count: {transaction_count}");
    println!("Average Transaction: {avg_transaction}");
    println!("Budget Usage: {budget_usage}%");
    println!("Expense Status: {}", determine_expense_status(budget_usage));
    println!("Remaining Budget : {remaining_budget}");
    Ok(())
}

fn determine_expense_status(budget_usage: f32) -> String {
    if budget_usage >= 80.0 && budget_usage <= 100.0 {
        return String::from("Boros pisan");
    } else if budget_usage >= 60.0 && budget_usage <= 79.0 {
        return String::from("Udah lumayan kepake");
    } else if budget_usage >= 30.0 && budget_usage <= 59.0 {
        return String::from("Masih lumayan aman");
    } else if budget_usage >= 0.0 && budget_usage <= 29.0 {
        return String::from("Hemat pisan !!");
    } else {
        return String::from("Tidak diketahui !");
    }
}

fn main() {
    print_header(true);
    let _ = analyze_budget();
}
