import { useState, useEffect } from "react";

import type { Movie } from "../interfaces";
import "../App.css";

export default function MovieInfo({ numHints, onMovieLoaded }: { numHints: number; onMovieLoaded: (title: string) => void}) {
    const [movie, setMovie] = useState<Movie | null>(null);
    const [loading, setLoading] = useState(true)

	useEffect(() => {
		fetchMovie();
	}, []);

	const fetchMovie = async() => {
		// try {
		// 	const response = await fetch('/api/movie');
		// 	const data = await response.json();
		// 	setMovie(data);
		// } catch (error) {
		// 	console.error('Error fetching movie:', error);
		// } finally {
		// 	setLoading(false);
		// }

		const sampleMovie: Movie = {
			title: "Inception",
			tagline: "Your mind is the scene of the crime.",
			year: 2010,
			director: "Christopher Nolan",
			lead_actors: ["Leonardo DiCaprio", "Joseph Gordon-Levitt", "Ellen Page"],
			genre: "Science Fiction"
		};
		setMovie(sampleMovie);
		setLoading(false);
		onMovieLoaded(sampleMovie.title);
	}

	if (loading) {
		return <div>Loading...</div>
	}

	if (!movie) {
		return <div>Failed to load movie.</div>;
	}

	const hints: string[] = [];
	if (numHints >= 1) hints.push(`The movie was released in ${movie.year}.`);
	if (numHints >= 2) hints.push(`The movie was directed by ${movie.director}.`);
	if (numHints >= 3) hints.push(`The lead actors in the movie are ${movie.lead_actors.join(", ")}.`);
	if (numHints >= 4) hints.push(`The genre of the movie is ${movie.genre}.`);
	if (numHints >= 5) hints.push(`There are no more hints available.`);

	return (
		<div className="MovieInfo">
			<p>Tagline: {movie.tagline}</p>
			{hints.map((hint, index) => (
				<p key={index}>hint {index + 1}: {hint}</p>
			))}
		</div>
	);
}