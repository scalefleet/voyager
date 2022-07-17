import 'uno.css';

import * as React from 'react';
import * as ReactDOM from 'react-dom/client';

function Main() {
	return <p className="">Voyager for PlanetScale from ScaleFleet.</p>;
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<Main />
	</React.StrictMode>
);
