use crate::tmdb;

pub async fn cli(id: u32) {
    println!("Welcome to MovieTag!");
    
    let movie_data = tmdb::fetch_movie_data(&id.to_string()).await.unwrap_or_else(|e| {
        eprintln!("Error fetching movie data: {}", e);
        std::process::exit(1);
    });

    if movie_data.tagline.is_empty() {
        println!("No tagline available for this movie: {}, {}. Exiting.", id, movie_data.title);
        return;
    }

    let mut num_hints = 0;

    loop {
        println!("Tagline: {}", movie_data.tagline);
        println!("Please enter a guess for the movie title (or type 'help' for help):");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        if guess == "exit" {
            break;
        }

        if guess == "help" {
            println!("Available commands:");
            println!("  exit - Quit the game");
            println!("  hint - Get a hint about the movie");
            println!("  giveup - Reveal the movie title");
            println!("  help - Show this help message");
            continue;
        }

        if guess == "giveup" {
            println!("The movie is '{}'.", movie_data.title);
            break;
        }

        if guess == "hint" {
            num_hints += 1;
            match num_hints {
                1 => println!("Hint 1: The movie was released in {}.", movie_data.year),
                2 => println!("Hint 2: The movie was directed by {}.", movie_data.director),
                3 => println!("Hint 3: The lead actors are {}.", movie_data.lead_actors.join(", ")),
                4 => println!("Hint 4: The genre of the movie is {}.", movie_data.genre),
                _ => println!("No more hints available!"),
            }
            continue;
        }

        if guess.eq_ignore_ascii_case(&movie_data.title) {
            println!("Correct! The movie is indeed '{}'.", movie_data.title);
            break;
        } else {
            println!("Incorrect guess. Try again!");
        }
    }
}