import { useState } from 'react';
import { Outlet, useLoaderData } from 'react-router-dom';
import Listbox from '../components/listbox.js';

export async function loader() {
	return {
		organization: {
			options: ['izznatsir', 'scalefleet'],
			selected: 'izznatsir',
		},
		database: {
			options: ['scalefleet'],
			selected: 'scalefleet',
		},
		branch: {
			options: ['main', 'dev'],
			selected: 'dev',
		},
		schema: {
			options: ['public'],
			selected: 'public',
		},
	};
}

export default function Index() {
	const { organization, database, branch, schema } = useLoaderData() as {
		organization: { options: string[]; selected: string };
		database: { options: string[]; selected: string };
		branch: { options: string[]; selected: string };
		schema: { options: string[]; selected: string };
	};

	const [selectedOrganization, setSelectedOrganization] = useState(
		organization.selected
	);
	const [selectedDatabase, setSelectedDatabase] = useState(database.selected);
	const [selectedBranch, setSelectedBranch] = useState(branch.selected);
	const [selectedSchema, setSelectedSchema] = useState(schema.selected);

	return (
		<>
			<aside className="w-60 py-8 border-r border-r-neutral-800">
				<ul className="flex flex-col gap-2">
					<li className="px-4 py-2 border-y border-y-neutral-800 cursor-pointer hover:bg-neutral-700">
						Tables
					</li>
					<li className="px-4 py-2 border-y border-y-neutral-800 cursor-pointer hover:bg-neutral-700">
						Queries
					</li>
				</ul>
				<div className="h-12"></div>
				<div>
					<Listbox
						name="Organization"
						options={organization.options}
						selected={selectedOrganization}
						setSelected={setSelectedOrganization}
					/>
					<div className="h-2"></div>
					<Listbox
						name="Database"
						options={database.options}
						selected={selectedDatabase}
						setSelected={setSelectedDatabase}
					/>
					<div className="h-2"></div>
					<Listbox
						name="Branch"
						options={branch.options}
						selected={selectedBranch}
						setSelected={setSelectedBranch}
					/>
					<div className="h-2"></div>
					<Listbox
						name="Schema"
						options={schema.options}
						selected={selectedSchema}
						setSelected={setSelectedSchema}
					/>
					<div className="h-2 border-b border-b-neutral-800"></div>
				</div>
			</aside>
			<main className="flex-1">
				<Outlet />
			</main>
		</>
	);
}
