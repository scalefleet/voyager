import { Listbox as L } from '@headlessui/react';
import SelectorIcon from '../icons/selector.js';

interface ListboxProps {
	name: string;
	options: string[];
	selected: string;
	setSelected: (selected: string) => void;
}

export default function Listbox({
	name,
	options,
	selected,
	setSelected,
}: ListboxProps) {
	return (
		<L name={name.toLowerCase()} value={selected} onChange={setSelected}>
			<div className="relative border-y border-y-neutral-800">
				<L.Label className="w-full px-4 text-[0.625rem] uppercase tracking-widest leading-none">
					{name}
				</L.Label>
				<div className="h-1" />
				<div className="relative flex">
					<L.Button className="w-full px-4 py-2 flex justify-between items-center text-left bg-neutral-800">
						<span>{selected}</span>
						<SelectorIcon />
					</L.Button>
					<L.Options className="absolute left-full w-full ml-2 border border-neutral-800">
						{options.map((option) => {
							return (
								<L.Option
									key={option}
									value={option}
									className={({ active, selected }) =>
										`w-full px-4 py-2 cursor-pointer ${
											selected
												? 'bg-neutral-800'
												: active
												? 'bg-neutral-700'
												: 'bg-neutral-900'
										}`
									}
								>
									{option}
								</L.Option>
							);
						})}
					</L.Options>
				</div>
			</div>
		</L>
	);
}
