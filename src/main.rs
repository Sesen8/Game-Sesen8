use rand::Rng;
use std::io;


#[derive(Debug, Clone, Copy)]
enum Country {
    USA,
    CANADA,
    UK,
    ITALY,
    GERMANY,
    MEXICO,
    HONDURAS,
    COLOMBIA,
    ERITREA,
    ETHIOPIA,
    SPAIN,
    FRANCE,
    INDIA,
    AUSTRALIA,
    BRAZIL,
    JAPAN,
    SOUTH_AFRICA,
    RUSSIA,
    ARGENTINA,
    TURKEY,
    
}

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Country {
    fn name(&self) -> &'static str {
        match *self {
            Country::USA => "USA",
            Country::CANADA => "Canada",
            Country::UK => "UK",
            Country::ITALY => "Italy",
            Country::GERMANY => "Germany",
            Country::MEXICO => "Mexico",
            Country::HONDURAS => "Honduras",
            Country::COLOMBIA => "Colombia",
            Country::ERITREA => "Eritrea",
            Country::ETHIOPIA => "Ethiopia",
            Country::SPAIN => "Spain",
            Country::FRANCE => "France",
            Country::INDIA => "India",
            Country::AUSTRALIA => "Australia",
            Country::BRAZIL => "Brazil",
            Country::JAPAN => "Japan",
            Country::SOUTH_AFRICA => "South Africa",
            Country::RUSSIA => "Russia",
            Country::ARGENTINA => "Argentina",
            Country::TURKEY => "Turkey",
            
        }
    }

    // FOR MEDIUM LEVEL
    fn capital(&self) -> &'static str {
        match *self {
            Country::USA => "Washington, D.C.",
            Country::CANADA => "Ottawa",
            Country::UK => "London",
            Country::ITALY => "Rome",
            Country::GERMANY => "Berlin",
            Country::MEXICO => "Mexico City",
            Country::HONDURAS => "Tegucigalpa",
            Country::COLOMBIA => "Bogotá",
            Country::ERITREA => "Asmara",
            Country::ETHIOPIA => "Addis Ababa",
            Country::SPAIN => "Madrid",
            Country::FRANCE => "Paris",
            Country::INDIA => "New Delhi",
            Country::AUSTRALIA => "Canberra",
            Country::BRAZIL => "Brasília",
            Country::JAPAN => "Tokyo",
            Country::SOUTH_AFRICA => "Pretoria",
            Country::RUSSIA => "Moscow",
            Country::ARGENTINA => "Buenos Aires",
            Country::TURKEY => "Ankara",
            
        }
    }

    // FOR EASY LEVEL
    fn continent(&self) -> &'static str {
        match *self {
            Country::USA | Country::CANADA => "North America",
            Country::UK | Country::FRANCE => "Europe",
            Country::ITALY | Country::GERMANY => "Europe",
            Country::MEXICO | Country::HONDURAS | Country::COLOMBIA => "North America",
            Country::ERITREA | Country::ETHIOPIA => "Africa",
            Country::SPAIN => "Europe",
            Country::INDIA => "Asia",
            Country::AUSTRALIA => "Australia",
            Country::BRAZIL | Country::ARGENTINA => "South America",
            Country::JAPAN => "Asia",
            Country::SOUTH_AFRICA => "Africa",
            Country::RUSSIA => "Europe/Asia",
            Country::TURKEY => "Asia/Europe",
            
        }
    }

    // FOR HARD LEVEL
    fn language(&self) -> &'static str {
        match *self {
            Country::USA => "English",
            Country::CANADA => "English and French",
            Country::UK | Country::AUSTRALIA => "English",
            Country::ITALY => "Italian",
            Country::GERMANY => "German",
            Country::MEXICO => "Spanish",
            Country::HONDURAS | Country::COLOMBIA => "Spanish",
            Country::ERITREA => "Tigrinya",
            Country::ETHIOPIA => "Amharic",
            Country::SPAIN => "Spanish",
            Country::FRANCE => "French",
            Country::INDIA => "Hindi",
            Country::BRAZIL => "Portuguese",
            Country::JAPAN => "Japanese",
            Country::SOUTH_AFRICA => "Multilingual",
            Country::RUSSIA => "Russian",
            Country::ARGENTINA => "Spanish",
            Country::TURKEY => "Turkish",
            
        }
    }
}

fn create_countries() -> Vec<Country> {
    vec![
        Country::USA,
        Country::CANADA,
        Country::UK,
        Country::ITALY,
        Country::GERMANY,
        Country::MEXICO,
        Country::HONDURAS,
        Country::COLOMBIA,
        Country::ERITREA,
        Country::ETHIOPIA,
        Country::SPAIN,
        Country::FRANCE,
        Country::INDIA,
        Country::AUSTRALIA,
        Country::BRAZIL,
        Country::JAPAN,
        Country::SOUTH_AFRICA,
        Country::RUSSIA,
        Country::ARGENTINA,
        Country::TURKEY,
    ]
    
}

fn ask_question(country: &Country, difficulty: &Difficulty) {
    println!(" ");
    match difficulty {
        Difficulty::Easy => println!("In which continent is {} located?", country.name()),
        Difficulty::Medium => println!("What is the capital of {}?", country.name()),
        Difficulty::Hard => println!("What language do they speak in {}?", country.name()),
    }
}

// USING RAND
fn random_country() -> Country {
    let countries = vec![
        Country::USA,
        Country::CANADA,
        Country::UK,
        Country::ITALY,
        Country::GERMANY,
        Country::MEXICO,
        Country::HONDURAS,
        Country::COLOMBIA,
        Country::ERITREA,
        Country::ETHIOPIA,
        Country::SPAIN,
        Country::FRANCE,
        Country::INDIA,
        Country::AUSTRALIA,
        Country::BRAZIL,
        Country::JAPAN,
        Country::SOUTH_AFRICA,
        Country::RUSSIA,
        Country::ARGENTINA,
        Country::TURKEY,
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..countries.len());
    countries[index]
}


fn main() {
    let mut score = 0;
    let mut player_difficulty = Difficulty::Easy; //placeholder for now

    // GAME TITLE
    println!(" ");
    println!("Welcome Countries Trivia!");
    println!("This game has three difficulties: Easy, Medium and Hard!");
    println!("At any point in the game type...");
    println!("        'switch' to move between levels!");
    println!("        'exit' to stop playing!");
    println!("Have Fun and Learn!");




    loop {
        
        println!(" ");
        println!("Choose a difficulty level (Easy, Medium, Hard), or type 'exit' to quit:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let difficulty_choice = user_input.trim().to_lowercase();

        if difficulty_choice == "exit" {
            println!("Thanks for playing. Goodbye!");
            break;
        }

        player_difficulty = match difficulty_choice.as_str() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            _ => {
                println!("Invalid difficulty level. Please choose Easy, Medium, or Hard.");
                continue;
            }
        };

        
        // GAMEPLAY
        loop {
            let selected_country = random_country(); 
            ask_question(&selected_country, &player_difficulty);

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            let user_guess = user_input.trim().to_lowercase();

            
            match player_difficulty {
                Difficulty::Easy => {
                    if user_guess == "switch" {
                        break; 
                    }
                    if user_guess == "exit" {
                        println!("Thanks for playing. Goodbye!");
                        return; 
                    }

                    if user_guess == selected_country.continent().to_lowercase() {
                        println!("Correct!");
                        score += 1;
                    } else {
                        println!("Incorrect. The correct answer is {}.", selected_country.continent());
                    }
                }


                Difficulty::Medium => {
                    if user_guess == "switch" {
                        break; 
                    }
                    if user_guess == "exit" {
                        println!("Thanks for playing. Goodbye!");
                        return; 
                    }

                    if user_guess == selected_country.capital().to_lowercase() {
                        println!("Correct!");
                        score += 2;
                    } else {
                        println!("Incorrect. The correct answer is {}.", selected_country.capital());
                    }
                }



                Difficulty::Hard => {
                    if user_guess == "switch" {
                        break; 
                    }
                    if user_guess == "exit" {
                        println!("Thanks for playing. Goodbye!");
                        return; 
                    }

                    if user_guess == selected_country.language().to_lowercase() {
                        println!("Correct!");
                        score += 3;
                    } else {
                        println!("Incorrect. The correct answer is {}.", selected_country.language());
                    }
                }
            }

            println!("Your current score: {}", score);
        }
    }
}