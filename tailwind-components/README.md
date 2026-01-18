### Tailwind setup

```
package.json
{
	"scripts": {
		"build:css": "tailwindcss -i style/input.css -o style/output.css",
		"watch:css": "tailwindcss -i style/input.css -o style/output.css --watch"
	},
	"dependencies": {
		"@tailwindcss/cli": "^4.0.17",
		"tailwindcss": "^4.0.17"
	}
}

Add in index.html

 <link data-trunk rel="css" href="/style/output.css" />

 tailwind.config.js
 /** @type {import('tailwindcss').Config} */
export default {
	darkMode: "class",
	content: {
		files: ["./src/**/*.rs", "./input.css", "./index.html"],
	},
	theme: {
		container: {
			center: true,
			padding: "2rem",
			screens: {
				"2xl": "1400px",
			},
		},
		extend: {
		}
	},
};

```

### Run the Program

```bash
Terminal1: trunk serve --open

Terminal2:
npm i
npm run watch:css
```

### UI CLI - LEPTOS UI

```
cargo install ui-cli
ui init

copy tailwind.css to input.css
delete tailwind.css

ui add component-name

cargo add leptos_router
```
