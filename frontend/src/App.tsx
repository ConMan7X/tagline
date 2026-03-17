import { useState } from "react"

import MovieInfo from "./components/MovieInfo"
import "./App.css"

const MAX_HINTS = 4;

export default function App() {
	const [numHints, setNumHints] = useState(0);
	const [giveUp, setGiveUp] = useState(false);

	return (
		<div className="App">
			<h1>Tagline</h1>
			<MovieInfo numHints={numHints} giveUp={giveUp} />
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
	)
}