import 'unplugin-icons/types/svelte';

// See https://svelte.dev/docs/kit/types#app.d.ts

// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	type HTMLEvent<T extends HTMLElement> = MouseEvent & {
		currentTarget: EventTarget & T;
	};

	namespace Footer {
		type Action = {
			label: string;
			disabled?: boolean;
		} & (
				| { type: 'button'; onClick: () => void; }
				| { type: 'link'; href: string; target?: string; }
			);
	}
}

export { };
