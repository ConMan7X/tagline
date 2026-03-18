import { useState } from "react"

import MovieInfo from "./components/MovieInfo"
import "./App.css"

const MAX_HINTS = 4;

export default function App() {
	const [numHints, setNumHints] = useState(0);
	const [giveUp, setGiveUp] = useState(false);
	const [guess, setGuess] = useState("");
	const [result, setResult] = useState<"correct" | "incorrect" | null>(null);
	const [movie, setMovie] = useState("");

	const submitGuess = () => {
		if (!movie || !guess.trim()) return;
		if (guess.trim().toLowerCase() === movie.trim().toLowerCase()) {
			setResult("correct");
		} else {
			setResult("incorrect");
			setGuess("");
		}
	};

	const gameOver = result === "correct" || giveUp;

	return (
		<div className="App">
			<h1>Tagline</h1>
			<div>
				<MovieInfo numHints={numHints} onMovieLoaded={setMovie}/>
				<button
					onClick={() => setNumHints(prev => Math.min(prev + 1, MAX_HINTS))}
					disabled={giveUp || numHints >= MAX_HINTS}
				>
					Get Hint {numHints < MAX_HINTS ? `(${MAX_HINTS - numHints} left)` : "(none left)"}
				</button>
				<button
					onClick={() => setGiveUp(true)}
					disabled={giveUp}
				>
					Give Up
				</button>
			</div>
			<div className="GuessSection">
				<input
					type="text"
					placeholder="Enter your guess here..."
					disabled={giveUp}
					value={guess}
					onChange={(e) => setGuess(e.target.value)}
					onKeyDown={e => e.key === "Enter" && submitGuess()}
				/>
				<button disabled={giveUp} onClick={() => submitGuess()}>
					Submit Guess
				</button>
			</div>
			{result === "correct" && <p>Correct! The movie was "{movie}".</p>}
			{result === "incorrect" && <p>Incorrect, try again!</p>}
			{giveUp && <p>The movie was "{movie}".</p>}
			{gameOver && <button onClick={() => {
				window.location.reload();
			}}>Play Again</button>}
		</div>
	)
}