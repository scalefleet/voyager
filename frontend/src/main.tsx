import * as React from 'react';
import * as ReactDOM from 'react-dom/client';

import './main.css';

function Main() {
	return (
		<p className="text-center">Voyager for PlanetScale from ScaleFleet.</p>
	);
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<Main />
	</React.StrictMode>
);
