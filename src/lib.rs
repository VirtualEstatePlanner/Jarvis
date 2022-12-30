extern crate csv;

use std::error::Error;
use std::fs::File;

struct Transaction {
    cleared_date: String,
    posted_date: String,
    amount: f64,
    transaction_type: TransactionType,
//    check_number: u32,
    reference_number: u64,
    recipient: String,
    location: String,
//    phone_numbers: Vec<String>,
//    industry: String,
    category: ExpenseCategory,
}

#[derive(PartialEq)]
enum TransactionType {
    Check,
    DebitCard,
    CreditCard,
    Online,
}

enum ExpenseCategory {
    Rent,
    Mortgage,
    HomeInsurance,
    HomeUtilities,
    HomeMaintenance,
    Transportation,
    AutoInsurance,
    Gas,
    AutoMaintenance,
    HealthInsurance,
    MedicalExpenses,
    LifeInsurance,
    Education,
    Childcare,
    Entertainment,
    Gifts,
    PersonalCare,
    Clothing,
    Groceries,
    EatingOut,
    Travel,
    Vacation,
    Hobbies,
    Fitness,
    Insurance,
    Investments,
    Taxes,
    LegalFees,
    CreditCardFees,
    BankFees,
    Phone,
    Internet,
    Cable,
    StreamingServices,
    Music,
    Magazines,
    Newspapers,
    Books,
    Subscriptions,
    Charity,
    Miscellaneous,
}

impl Transaction {
    fn new(
        cleared_date: String,
        posted_date: String,
        reference_number: u64,
        amount: f64,
        transaction_type: TransactionType,
//        check_number: u16,
        recipient: String,
        location: String,
        location_description: String,
//        location_description: String,
//        location_gps_coordinates: String,
// etc.
        phone_numbers: String,
        industry: String,
        category: ExpenseCategory,
        catchall_field: String
    ) -> Self {
        Self {
            cleared_date,
            posted_date,
            amount,
            transaction_type,
//            check_number,
            reference_number,
            recipient,
            location,
//            phone_numbers,
//            industry,
            category,
        }
    }

}

fn read_transactions_from_csv(filename: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut transactions = Vec::new();

    for result in reader.records() {
        let record = result?;
        let cleared_date = record[0].to_string();
        let posted_date = record[1].to_string();
        let amount = record[2].replace(",", "").parse::<f64>()?;
        let transaction_type = match record[3].as_ref() {
            "check" => TransactionType::Check,
            "debit card" => TransactionType::DebitCard,
            "credit card" => TransactionType::CreditCard,
            "online" => TransactionType::Online,
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid transaction type"))),
        };
        let check_num= if transaction_type == TransactionType::Check {
            Some(record[4].parse::<u32>()?)
        } else {
            None
        };
        let reference_number: String = record[5].to_string();
        let recipient: String = if record[6] == "" {
            None
        } else {
            Some(record[6].to_string())
        };
        let location = if record[7] == "" {
            None
        } else {
            Some(record[7].to_string())
        };
/*        let phone_numbers = record[8]
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();
        let industry = if record[9] == "" {
            None
        } else {
            Some(record[9].to_string())
        }; */
        let category = match record[10].as_ref() {
            "rent" => ExpenseCategory::Rent,
            "mortgage" => ExpenseCategory::Mortgage,
            "home insurance" => ExpenseCategory::HomeInsurance,
            "home utilities" => ExpenseCategory::HomeUtilities,
            "home maintenance" => ExpenseCategory::HomeMaintenance,
            "transportation" => ExpenseCategory::Transportation,
            "auto insurance" => ExpenseCategory::AutoInsurance,
            "gas" => ExpenseCategory::Gas,
            "auto maintenance" => ExpenseCategory::AutoMaintenance,
            "health insurance" => ExpenseCategory::HealthInsurance,
            "medical expenses" => ExpenseCategory::MedicalExpenses,
            "life insurance" => ExpenseCategory::LifeInsurance,
            "education" => ExpenseCategory::Education,
            "childcare" => ExpenseCategory::Childcare,
            "books" => ExpenseCategory::Books,
            "subscriptions" => ExpenseCategory::Subscriptions,
            "charity" => ExpenseCategory::Charity,
            "miscellaneous" => ExpenseCategory::Miscellaneous,
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid expense category"))),
        };

        let transaction = Transaction::new(
            cleared_date,
            posted_date,
            amount,
            transaction_type,
            //check_number,
            reference_number,
            recipient,
            location,
            //phone_numbers,
            //industry,
            category,
        );
        transactions.push(transaction);
    }

    Ok(transactions)
}

fn total_spent(transactions: &[Transaction]) -> f64 {
    transactions.iter().map(|t| t.amount).sum()
}

fn average_transaction_amount(transactions: &[Transaction]) -> f64 {
    let total_amount: _ = transactions.iter().map(|t| t.amount).sum();
    let num_transactions = transactions.len() as f64;
    total_amount / num_transactions
}

fn print_transaction_summary(transactions: &[Transaction]) {
    let total_spent = total_spent(transactions);
    let average_amount = average_transaction_amount(transactions);
    println!("Total spent: {:.2}", total_spent);
    println!("Average transaction amount: {:.2}", average_amount);
}
