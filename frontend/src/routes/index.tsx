import * as React from 'react';
import { Outlet, useLoaderData } from 'react-router-dom';
import { Listbox } from '@headlessui/react';
import ChevronDown from '../icons/chevron-down.js';

export async function loader() {
	return {
		organizations: ['izznatsir', 'scalefleet'],
		selected: 'izznatsir',
	};
}

export default function Index() {
	const { selected, organizations } = useLoaderData() as {
		organizations: string[];
		selected: string;
	};

	const [selectedOrganization, setSelectedOrganization] =
		React.useState(selected);

	return (
		<>
			<aside className="w-60 py-8">
				<div>
					<Listbox
						name="organization"
						value={selectedOrganization}
						onChange={setSelectedOrganization}
					>
						<div className="relative">
							<Listbox.Label className="w-full px-4">
								Organization
							</Listbox.Label>
							<div className="h-2" />
							<Listbox.Button className="w-full px-4 py-2 flex justify-between items-center text-left bg-neutral-800">
								<span>{selectedOrganization}</span>
								<ChevronDown />
							</Listbox.Button>
							<Listbox.Options className="absolute w-full py-2">
								{organizations.map((organization) => {
									return (
										<Listbox.Option
											key={organization}
											value={organization}
											className={({ active, selected }) =>
												`w-full px-4 py-2 cursor-pointer ${
													selected
														? 'bg-cyan-800'
														: active
														? 'bg-neutral-700'
														: 'bg-neutral-800'
												}`
											}
										>
											{organization}
										</Listbox.Option>
									);
								})}
							</Listbox.Options>
						</div>
					</Listbox>
				</div>
				<div className="h-6"></div>
				<ul>
					<li>Schemas</li>
					<li>Queries</li>
				</ul>
			</aside>
			<main className="flex-1">
				<Outlet />
			</main>
		</>
	);
}
