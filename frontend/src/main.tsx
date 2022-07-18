import * as React from 'react';
import * as ReactDOM from 'react-dom/client';
import { DataBrowserRouter, Route } from 'react-router-dom';
import Index, { loader as indexLoader } from './routes/index.js';

import './main.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<DataBrowserRouter>
			<Route path="/" element={<Index />} loader={indexLoader}></Route>
		</DataBrowserRouter>
	</React.StrictMode>
);
