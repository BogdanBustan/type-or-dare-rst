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

fn generate_data(valid: bool) -> Vec<(i32, String, String)> {
    if valid {
        vec![
            (1, "Alice".to_string(), "25".to_string()),
            (2, "Bob".to_string(), "30".to_string()),
            (3, "Charlie".to_string(), "35".to_string()),
        ]
    } else {
        vec![
            (1, "Alice".to_string(), "25".to_string()),
            (2, "Bob".to_string(), "thirty".to_string()),
            (3, "Charlie".to_string(), "35".to_string()),
        ]
    }
}

fn validate_data(data: Vec<(i32, String, String)>) -> Result<Vec<User>, ValidationError> {
    let mut users = Vec::new();

    for (id_data, name_data, age_data) in data {
        let age = age_data.parse::<i32>().map_err(|_| {
            ValidationError(format!("Invalid age value: {}", age_data))
        })?;

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

    let oldest_user = users_with_adult.iter()
        .max_by_key(|(user, _)| user.age)
        .map(|(user, _)| user)
        .unwrap();

    let adult_count = users_with_adult.iter()
        .filter(|(_, is_adult)| *is_adult)
        .count();

    Ok(format!(
        "Summary:\n\
         - Average age is {:.1}\n\
         - Oldest user is {} (ID: {}) at age {}\n\
         - Number of adults: {}",
        avg_age,
        oldest_user.name,
        oldest_user.id,
        oldest_user.age,
        adult_count
    ))
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
    // Test with valid data
    match process_user_data(true) {
        Ok(summary) => println!("Success:\n{}", summary),
        Err(error) => println!("Error: {}", error),
    }

    println!("\n---\n");

    // Test with invalid data
    match process_user_data(false) {
        Ok(summary) => println!("Success:\n{}", summary),
        Err(error) => println!("Error: {}", error),
    }
}