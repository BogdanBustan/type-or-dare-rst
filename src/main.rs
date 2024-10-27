use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
struct ValidationError(String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation error: {}", self.0)
    }
}

impl StdError for ValidationError {}

#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Debug)]
enum AgeData {
    Valid(i32),
    Invalid(String),
}

fn generate_data(valid: bool) -> Vec<(i32, String, AgeData)> {
    if valid {
        vec![
            (1, "Alice".to_string(), AgeData::Valid(25)),
            (2, "Bob".to_string(), AgeData::Valid(30)),
            (3, "Charlie".to_string(), AgeData::Valid(35)),
        ]
    } else {
        vec![
            (1, "Alice".to_string(), AgeData::Valid(25)),
            (2, "Bob".to_string(), AgeData::Invalid("thirty".to_string())),
            (3, "Charlie".to_string(), AgeData::Valid(35)),
        ]
    }
}

fn validate_data(data: Vec<(i32, String, AgeData)>) -> Result<Vec<User>, ValidationError> {
    let mut users = Vec::new();

    for (id_data, name_data, age_data) in data {
        let age = match age_data {
            AgeData::Valid(age) => age,
            AgeData::Invalid(invalid_value) => {
                return Err(ValidationError(format!("Invalid age value: {}", invalid_value)))
            }
        };
        let id = match id_data {
            id if id > 0 => id,
            _ => return Err(ValidationError("Invalid id value".to_string())),
        };
        let name = match name_data.len() {
            len if len > 0 => name_data,
            _ => return Err(ValidationError("Invalid name value".to_string())),
        };
        users.push(User { id, name, age });
    }

    Ok(users)
}

fn add_is_adult(users: Vec<User>) -> Result<Vec<(User, bool)>, ValidationError> {
    Ok(users
        .into_iter()
        .map(|user| (user.clone(), user.age >= 18))
        .collect())
}

fn summarize_data(users_with_adult: Vec<(User, bool)>) -> Result<String, ValidationError> {
    let total_age: i32 = users_with_adult.iter().map(|(user, _)| user.age).sum();
    let avg_age = total_age as f64 / users_with_adult.len() as f64;
    Ok(format!("Average age is {:.1}", avg_age))
}

fn process_user_data(valid: bool) -> Result<String, Box<dyn StdError>> {
    let data = generate_data(valid);
    validate_data(data)
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .and_then(|users| {
            add_is_adult(users)
                .map_err(|e| Box::new(e) as Box<dyn StdError>)
        })
        .and_then(|users_with_adult| {
            summarize_data(users_with_adult)
                .map_err(|e| Box::new(e) as Box<dyn StdError>)
        })
}

fn main() {
    match process_user_data(true) {
        Ok(summary) => println!("Success: {}", summary),
        Err(error) => println!("Error: {}", error),
    }

    match process_user_data(false) {
        Ok(summary) => println!("Success: {}", summary),
        Err(error) => println!("Error: {}", error),
    }
}