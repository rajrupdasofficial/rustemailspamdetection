use csv::ReaderBuilder;
use std::error::Error;
use std::fs;
use std::path::Path;

// Simple Naive Bayes Classifier for Spam Detection
struct SpamClassifier {
    spam_words: Vec<String>,
    ham_words: Vec<String>,
    spam_count: usize,
    ham_count: usize,
}

impl SpamClassifier {
    fn new() -> Self {
        SpamClassifier {
            spam_words: Vec::new(),
            ham_words: Vec::new(),
            spam_count: 0,
            ham_count: 0,
        }
    }

    fn train(&mut self, emails: &Vec<(String, String)>) {
        for (label, content) in emails {
            let words: Vec<String> = content
                .to_lowercase()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if label == "spam" {
                self.spam_words.extend(words);
                self.spam_count += 1;
            } else {
                self.ham_words.extend(words);
                self.ham_count += 1;
            }
        }
    }

    fn predict(&self, message: &str) -> bool {
        let message_words: Vec<String> = message
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let spam_indicators = [
            "free",
            "win",
            "urgent",
            "lottery",
            "click here",
            "limited offer",
            "$$$",
            "winner",
            "prize",
            "congratulations",
        ];

        let spam_word_matches = message_words
            .iter()
            .filter(|word| spam_indicators.contains(&word.as_str()))
            .count();

        // Simple heuristic: More than 2 spam indicators suggests spam
        spam_word_matches > 2
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "emails.csv";

    // Ensure dataset exists
    if !Path::new(file_path).exists() {
        create_default_dataset(file_path)?;
    }

    // Load emails
    let emails = load_data(file_path)?;

    // Train classifier
    let mut classifier = SpamClassifier::new();
    classifier.train(&emails);

    // Interactive mode
    loop {
        println!("\nSpam Detection Tool");
        println!("1. Check an email message");
        println!("2. Exit");
        print!("Enter your choice (1/2): ");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                print!("Enter the email message to check for spam: ");
                let mut input_message = String::new();
                std::io::stdin().read_line(&mut input_message)?;

                let result = classifier.predict(input_message.trim());

                if result {
                    println!("🚨 SPAM DETECTED! This message appears to be spam.");
                } else {
                    println!("✅ NO SPAM DETECTED. This message seems safe.");
                }
            }
            "2" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}

// Create a default dataset if none exists
fn create_default_dataset(file_path: &str) -> Result<(), Box<dyn Error>> {
    let default_data = "label,content\n\
        spam,Congratulations! You've won a free iPhone! Click here to claim now!!!\n\
        ham,Hi John, can we schedule a meeting to discuss the project next week?\n\
        spam,URGENT: You've been selected for an exclusive lottery. Claim your $10,000 prize NOW!\n\
        ham,Please find attached the quarterly report for your review.\n\
        spam,GET RICH QUICK! Make $5000 per week working from home. No experience needed!\n\
        ham,Meeting minutes from today's team discussion are attached.\n\
        spam,Limited time offer! 90% OFF all products. Buy now before it's gone!\n\
        ham,Could you please send me the updated client contact list?\n\
        spam,You are the WINNER of our mega sweepstakes! Claim your prize immediately!\n\
        ham,Thank you for your recent order. Your package will be shipped soon.\n\
        spam,FREE VIAGRA! Lowest prices guaranteed. Buy now!\n\
        ham,Please confirm your attendance for the upcoming conference.\n\
        spam,Make millions from home! Our proven system guarantees success!!!\n\
        ham,Your monthly bank statement is now available for review.\n\
        spam,ATTENTION: Your computer is infected. Click here to fix immediately!\n\
        ham,Draft proposal for the new marketing strategy is ready for your feedback.\n\
        spam,Exclusive offer: Become a millionaire overnight! No investment required!\n\
        ham,Reminder: Performance review meetings are scheduled for next week.\n\
        spam,WIN BIG! Mega casino bonus waiting for you. No deposit needed!\n\
        ham,Invoice #1234 for services rendered is attached for your records.\n";

    fs::write(file_path, default_data)?;
    println!("Created default spam dataset: {}", file_path);
    Ok(())
}

// Load data from CSV
fn load_data(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let mut emails: Vec<(String, String)> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let label = record.get(0).unwrap_or("ham").to_string();
        let content = record.get(1).unwrap_or("").to_string();
        emails.push((label, content));
    }

    Ok(emails)
}
