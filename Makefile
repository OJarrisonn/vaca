all: windows unix

windows:
	cargo build --release --target x86_64-pc-windows-gnu

unix:
	cargo build --release --target x86_64-unknown-linux-gnu