# Cosmos Gallery

**Author**: Gurleen Dhaliwal

## Project Vision

Cosmos Gallery is a user-friendly web application that allows users to view and vote on NASA's daily space images. The project is built using the Rust programming language and the Axum web framework, with a focus on creating a powerful and efficient backend system. The frontend is designed to be simple and intuitive, showcasing the capabilities of the backend.

## Features

- The backend is developed in Rust using the Axum web framework.
- Postgresql is utilized for database management, interfaced through the SQLx Rust framework.
- The backend offers a REST API, which the frontend uses for all interactions with the backend.
- The backend dynamically queries the NASA APOD (Astronomy Picture of the Day) API for its content.
- The frontend is designed with simplicity in mind, employing static, templated HTML pages.
- User accounts are password-protected and authentication is required for login. Once logged in, users are authenticated via JWT (JSON Web Tokens).
- Users can save and delete their previously saved choices.
- Each APOD API call made by a user is stored in a database table, creating an effective cache system.

## How It Works

Cosmos Gallery operates on a backend created in Rust using the Axum web framework. The backend fetches the daily space image from the NASA APOD API and stores the results in a Postgresql database, acting as a cache. The frontend communicates with the backend via a REST API, retrieving the APOD for display to the user. Users can register and login to the system, receiving JWT tokens for authentication in subsequent interactions. Users have the ability to upvote or downvote APODs, with the voting data stored in the Postgresql database.

## What Worked

- Successfully queries the NASA APOD API for daily space images.
- Caches APOD data in a Postgresql database for efficient retrieval.
- Displays the APOD to users through static, templated HTML pages.
- Supports user account creation and authentication using JWT tokens.
- Allows users to upvote or downvote APODs, with the voting data stored in the database.

## What Didn't Work

- Encountered challenges in handling database errors and edge cases.
- Initially struggled with the complexity of JWT token-based user authentication.

## Lessons Learned

- Rust and the Axum framework are powerful tools for building efficient and reliable web applications.
- Caching APOD data in a Postgresql database enhances performance and reduces dependency on the NASA API.
- JWT tokens offer a secure and efficient method for user authentication.
- Proper error handling and edge case management are essential for a robust and reliable web service.

## How to Use the Admin Account

1. Visit: [http://127.0.0.1:3000/](http://127.0.0.1:3000/)
2. Login with email: `sf` and password: `sf`
3. View the Astronomy Picture of the Day.
4. Click "Show All" to display all APODs.
5. Note: Each time a user registers or logs in, the system checks for the latest APOD and adds it to the database if not already present.

