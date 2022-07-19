import { useState } from 'react';
import { Outlet, useLoaderData } from 'react-router-dom';
import { Listbox } from '@headlessui/react';
import SelectorIcon from '../icons/selector.js';

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
						name="organization"
						value={selectedOrganization}
						onChange={setSelectedOrganization}
					>
						<div className="relative border-y border-y-neutral-800">
							<Listbox.Label className="w-full px-4 text-[0.625rem] uppercase tracking-widest leading-none">
								Organization
							</Listbox.Label>
							<div className="h-1" />
							<div className="relative flex">
								<Listbox.Button className="w-full px-4 py-2 flex justify-between items-center text-left bg-neutral-800">
									<span>{selectedOrganization}</span>
									<SelectorIcon />
								</Listbox.Button>
								<Listbox.Options className="absolute left-full w-full ml-2 border border-neutral-800">
									{organization.options.map(
										(organization) => {
											return (
												<Listbox.Option
													key={organization}
													value={organization}
													className={({
														active,
														selected,
													}) =>
														`w-full px-4 py-2 cursor-pointer ${
															selected
																? 'bg-neutral-800'
																: active
																? 'bg-neutral-700'
																: 'bg-neutral-900'
														}`
													}
												>
													{organization}
												</Listbox.Option>
											);
										}
									)}
								</Listbox.Options>
							</div>
						</div>
					</Listbox>
					<div className="h-2"></div>
					<Listbox
						name="database"
						value={selectedDatabase}
						onChange={setSelectedDatabase}
					>
						<div className="relative border-y border-y-neutral-800">
							<Listbox.Label className="w-full px-4 text-[0.625rem] uppercase tracking-widest leading-none">
								Database
							</Listbox.Label>
							<div className="h-1" />
							<div className="relative flex">
								<Listbox.Button className="w-full px-4 py-2 flex justify-between items-center text-left bg-neutral-800">
									<span>{selectedDatabase}</span>
									<SelectorIcon />
								</Listbox.Button>
								<Listbox.Options className="absolute left-full w-full ml-2 border border-neutral-800">
									{database.options.map((database) => {
										return (
											<Listbox.Option
												key={database}
												value={database}
												className={({
													active,
													selected,
												}) =>
													`w-full px-4 py-2 cursor-pointer ${
														selected
															? 'bg-neutral-800'
															: active
															? 'bg-neutral-700'
															: 'bg-neutral-900'
													}`
												}
											>
												{database}
											</Listbox.Option>
										);
									})}
								</Listbox.Options>
							</div>
						</div>
					</Listbox>
					<div className="h-2"></div>
					<Listbox
						name="branch"
						value={selectedBranch}
						onChange={setSelectedBranch}
					>
						<div className="relative border-y border-y-neutral-800">
							<Listbox.Label className="w-full px-4 text-[0.625rem] uppercase tracking-widest leading-none">
								Branch
							</Listbox.Label>
							<div className="h-1" />
							<div className="relative flex">
								<Listbox.Button className="w-full px-4 py-2 flex justify-between items-center text-left bg-neutral-800">
									<span>{selectedBranch}</span>
									<SelectorIcon />
								</Listbox.Button>
								<Listbox.Options className="absolute left-full w-full ml-2 border border-neutral-800">
									{branch.options.map((branch) => {
										return (
											<Listbox.Option
												key={branch}
												value={branch}
												className={({
													active,
													selected,
												}) =>
													`w-full px-4 py-2 cursor-pointer ${
														selected
															? 'bg-neutral-800'
															: active
															? 'bg-neutral-700'
															: 'bg-neutral-900'
													}`
												}
											>
												{branch}
											</Listbox.Option>
										);
									})}
								</Listbox.Options>
							</div>
						</div>
					</Listbox>
					<div className="h-2"></div>
					<Listbox
						name="schema"
						value={selectedSchema}
						onChange={setSelectedSchema}
					>
						<div className="relative border-y border-y-neutral-800">
							<Listbox.Label className="w-full px-4 text-[0.625rem] uppercase tracking-widest leading-none">
								Schema
							</Listbox.Label>
							<div className="h-1" />
							<div className="relative flex">
								<Listbox.Button className="w-full px-4 py-2 flex justify-between items-center text-left bg-neutral-800">
									<span>{selectedSchema}</span>
									<SelectorIcon />
								</Listbox.Button>
								<Listbox.Options className="absolute left-full w-full ml-2 border border-neutral-800">
									{schema.options.map((schema) => {
										return (
											<Listbox.Option
												key={schema}
												value={schema}
												className={({
													active,
													selected,
												}) =>
													`w-full px-4 py-2 cursor-pointer ${
														selected
															? 'bg-neutral-800'
															: active
															? 'bg-neutral-700'
															: 'bg-neutral-900'
													}`
												}
											>
												{schema}
											</Listbox.Option>
										);
									})}
								</Listbox.Options>
							</div>
						</div>
					</Listbox>
					<div className="h-2 border-b border-b-neutral-800"></div>
				</div>
			</aside>
			<main className="flex-1">
				<Outlet />
			</main>
		</>
	);
}
