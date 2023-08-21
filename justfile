# start
start:
	pnpm tauri dev

start-windows:
	pnpm tauri dev --target x86_64-pc-windows-gnu

# package management
pnpm-add package:
	pnpm i {{package}}

pnpm-add-dev package:
	pnpm i -D {{package}}

pnpm-remove package:
	pnpm remove {{package}}

cargo-add package:
	cargo add {{package}}

cargo-remove package:
	cargo remove {{package}}
