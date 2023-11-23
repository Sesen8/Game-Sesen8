use rand::Rng;
use std::io;


#[derive(Debug, Clone, Copy)]
enum Country {
    USA, CANADA, UK, ITALY, GERMANY, MEXICO, HONDURAS, COLOMBIA, ERITREA, ETHIOPIA, SPAIN,
    FRANCE, INDIA, AUSTRALIA, BRAZIL, JAPAN, SOUTH_AFRICA, RUSSIA, ARGENTINA, TURKEY,
    NIGERIA, EGYPT, PAKISTAN, SAUDI_ARABIA, SOUTH_KOREA, INDONESIA, VIETNAM,
    MOROCCO, UKRAINE, POLAND, SWEDEN, NORWAY, NEW_ZEALAND,
    THAILAND, MALAYSIA, PERU, CHILE, SWITZERLAND,
    
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
            Country::NIGERIA => "Nigeria",
            Country::EGYPT => "Egypt",
            Country::PAKISTAN => "Pakistan",
            Country::SAUDI_ARABIA => "Saudi Arabia",
            Country::SOUTH_KOREA => "South Korea",
            Country::INDONESIA => "Indonesia",
            Country::VIETNAM => "Vietnam",
            Country::MOROCCO => "Morocco",
            Country::UKRAINE => "Ukraine",
            Country::POLAND => "Poland",
            Country::SWEDEN => "Sweden",
            Country::NORWAY => "Norway",
            Country::NEW_ZEALAND => "New Zealand",
            Country::THAILAND => "Thailand",
            Country::MALAYSIA => "Malaysia",
            Country::PERU => "Peru",
            Country::CHILE => "Chile",
            Country::SWITZERLAND => "Switzerland",
            
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
            Country::NIGERIA => "Abuja",
            Country::EGYPT => "Cairo",
            Country::PAKISTAN => "Islamabad",
            Country::SAUDI_ARABIA => "Riyadh",
            Country::SOUTH_KOREA => "Seoul",
            Country::INDONESIA => "Jakarta",
            Country::VIETNAM => "Hanoi",
            Country::MOROCCO => "Rabat",
            Country::UKRAINE => "Kyiv",
            Country::POLAND => "Warsaw",
            Country::SWEDEN => "Stockholm",
            Country::NORWAY => "Oslo",
            Country::NEW_ZEALAND => "Wellington",
            Country::THAILAND => "Bangkok",
            Country::MALAYSIA => "Kuala Lumpur",
            Country::PERU => "Lima",
            Country::CHILE => "Santiago",
            Country::SWITZERLAND => "Bern",
            
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
            Country::AUSTRALIA | Country::NEW_ZEALAND => "Australia",
            Country::BRAZIL | Country::ARGENTINA | Country::PERU | Country::CHILE => "South America",
            Country::JAPAN => "Asia",
            Country::SOUTH_AFRICA | Country::MOROCCO => "Africa",
            Country::RUSSIA | Country::UKRAINE => "Europe",
            Country::POLAND | Country::SWEDEN | Country::NORWAY => "Europe",
            Country::THAILAND | Country::MALAYSIA => "Asia",
            Country::NIGERIA | Country::EGYPT | Country::SAUDI_ARABIA => "Africa",
            Country::SOUTH_KOREA | Country::INDONESIA | Country::VIETNAM => "Asia",
            Country::SWITZERLAND => "Europe",
            Country::TURKEY => "Asia",
            Country::PAKISTAN => "Asia",
            
        }
    }

    // FOR HARD LEVEL
    fn language(&self) -> &'static str {
        match *self {
            Country::USA => "English",
            Country::CANADA => "English",
            Country::UK | Country::AUSTRALIA | Country::NEW_ZEALAND => "English",
            Country::ITALY => "Italian",
            Country::GERMANY | Country::SWITZERLAND => "German",
            Country::MEXICO => "Spanish",
            Country::HONDURAS | Country::COLOMBIA => "Spanish",
            Country::ERITREA => "Tigrinya",
            Country::ETHIOPIA => "Amharic",
            Country::SPAIN => "Spanish",
            Country::FRANCE => "French",
            Country::INDIA => "Hindi",
            Country::BRAZIL => "Portuguese",
            Country::ARGENTINA => "Spanish",
            Country::TURKEY => "Turkish",
            Country::NIGERIA => "English",
            Country::EGYPT => "Arabic",
            Country::PAKISTAN => "Urdu",
            Country::SAUDI_ARABIA => "Arabic",
            Country::SOUTH_KOREA => "Korean",
            Country::INDONESIA => "Indonesian",
            Country::VIETNAM => "Vietnamese",
            Country::JAPAN => "Japanese",
            Country::MOROCCO => "Arabic",
            Country::UKRAINE | Country::POLAND | Country::SWEDEN | Country::NORWAY => "English",
            Country::THAILAND => "Thai",
            Country::MALAYSIA => "Malay",
            Country::PERU | Country::CHILE => "Spanish",
            Country::RUSSIA => "Russian",
            Country::SOUTH_AFRICA => "English",
            
        }
    }
}

fn create_countries() -> Vec<Country> {
    vec![
        Country::USA, Country::CANADA, Country::UK,
        Country::ITALY, Country::GERMANY, Country::MEXICO,
        Country::HONDURAS, Country::COLOMBIA, Country::ERITREA,
        Country::ETHIOPIA, Country::SPAIN, Country::FRANCE,
        Country::INDIA, Country::AUSTRALIA,
        Country::BRAZIL, Country::JAPAN, Country::SOUTH_AFRICA,
        Country::RUSSIA, Country::ARGENTINA,
        Country::TURKEY, Country::NIGERIA, Country::EGYPT,
        Country::PAKISTAN, Country::SAUDI_ARABIA,
        Country::SOUTH_KOREA, Country::INDONESIA,
        Country::VIETNAM, Country::MOROCCO,
        Country::UKRAINE, Country::POLAND, Country::SWEDEN,
        Country::NORWAY, Country::NEW_ZEALAND, Country::THAILAND,
        Country::MALAYSIA, Country::PERU,
        Country::CHILE, Country::SWITZERLAND,
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
        Country::USA, Country::CANADA, Country::UK, Country::ITALY,
        Country::GERMANY, Country::MEXICO, Country::HONDURAS,
        Country::COLOMBIA, Country::ERITREA, Country::ETHIOPIA,
        Country::SPAIN, Country::FRANCE, Country::INDIA,
        Country::AUSTRALIA, Country::BRAZIL,
        Country::JAPAN, Country::SOUTH_AFRICA, Country::RUSSIA,
        Country::ARGENTINA, Country::TURKEY, Country::NIGERIA,
        Country::EGYPT, Country::PAKISTAN,
        Country::SAUDI_ARABIA, Country::SOUTH_KOREA, Country::INDONESIA,
        Country::VIETNAM, Country::MOROCCO,
        Country::UKRAINE, Country::POLAND, Country::SWEDEN,
        Country::NORWAY, Country::NEW_ZEALAND, Country::THAILAND,
        Country::MALAYSIA, Country::PERU,
        Country::CHILE, Country::SWITZERLAND,
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
        println!("Choose a difficulty level!!! (Easy, Medium, Hard), or type 'exit' to quit:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("huh? try again please?");
        let difficulty_choice = user_input.trim().to_lowercase();

        if difficulty_choice == "exit" {
            println!("Thanks for playing!!!! Have a great day!!!!!");
            break;
        }

        player_difficulty = match difficulty_choice.as_str() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            _ => {
                println!("Thats not on my list hmm. Please choose Easy, Medium, or Hard.");
                continue;
            }
        };

        
        // GAMEPLAY
        loop {
            let selected_country = random_country(); 
            ask_question(&selected_country, &player_difficulty);

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("huh? try again please?");
            let user_guess = user_input.trim().to_lowercase();

            
            match player_difficulty {
                Difficulty::Easy => {
                    if user_guess == "switch" {
                        break; 
                    }
                    if user_guess == "exit" {
                        println!("Thanks for playing!!!! Have a great day!!!!");
                        return; 
                    }

                    if user_guess == selected_country.continent().to_lowercase() {
                        println!("You got it Correct!!!!!!");
                        score += 1;
                    } else {
                        println!("Aww thats Incorrect :( The correct answer is {}.", selected_country.continent());
                    }
                }


                Difficulty::Medium => {
                    if user_guess == "switch" {
                        break; 
                    }
                    if user_guess == "exit" {
                        println!("Thanks for playing!!!! Have a great day!!!!");
                        return; 
                    }

                    if user_guess == selected_country.capital().to_lowercase() {
                        println!("You got it Correct!!!!!!");
                        score += 2;
                    } else {
                        println!("Aww thats Incorrect :( The correct answer is {}.", selected_country.capital());
                    }
                }



                Difficulty::Hard => {
                    if user_guess == "switch" {
                        break; 
                    }
                    if user_guess == "exit" {
                        println!("Thanks for playing!!!! Have a great day!!!!");
                        return; 
                    }

                    if user_guess == selected_country.language().to_lowercase() {
                        println!("You got it Correct!!!!!");
                        score += 3;
                    } else {
                        println!("Aww thats Incorrect :( The correct answer is {}.", selected_country.language());
                    }
                }
            }

            println!("Your current score: {}", score);
        }
    }
}